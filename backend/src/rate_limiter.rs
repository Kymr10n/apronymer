// ...existing code...
// Rate limiting middleware using sliding window algorithm
use axum::{
    extract::ConnectInfo,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    body::Body,
};
use dashmap::DashMap;
use serde::{Serialize, Deserialize};
use std::{
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::time;

/// Rate limiter configuration
#[derive(Clone, Debug)]
pub struct RateLimiterConfig {
    /// Maximum number of requests allowed per window
    pub max_requests: u32,
    /// Time window duration
    pub window_duration: Duration,
    /// Cleanup interval for expired entries
    pub cleanup_interval: Duration,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            max_requests: 10,                                    // 10 requests
            window_duration: Duration::from_secs(60),            // per minute
            cleanup_interval: Duration::from_secs(300),          // cleanup every 5 minutes
        }
    }
}

/// Request timestamp entry for sliding window
#[derive(Debug, Clone)]
struct RequestEntry {
    timestamps: Vec<Instant>,
    last_cleanup: Instant,
}

impl RequestEntry {
    fn new() -> Self {
        Self {
            timestamps: Vec::new(),
            last_cleanup: Instant::now(),
        }
    }

    /// Add a new request timestamp and clean old ones
    fn add_request(&mut self, config: &RateLimiterConfig) -> bool {
        let now = Instant::now();
        
        // Clean old timestamps outside the window
        self.cleanup_old_timestamps(now, config.window_duration);
        
        // Check if we've exceeded the rate limit
        if self.timestamps.len() >= config.max_requests as usize {
            return false; // Rate limit exceeded
        }
        
        // Add the new request
        self.timestamps.push(now);
        true
    }

    /// Remove timestamps older than the window duration
    fn cleanup_old_timestamps(&mut self, now: Instant, window_duration: Duration) {
        let cutoff = now - window_duration;
        self.timestamps.retain(|&timestamp| timestamp >= cutoff);
        self.last_cleanup = now;
    }

    /// Check if this entry should be removed (no requests in window + cleanup interval)
    fn should_expire(&self, now: Instant, window_duration: Duration, cleanup_interval: Duration) -> bool {
        if self.timestamps.is_empty() {
            return now.duration_since(self.last_cleanup) > cleanup_interval;
        }
        
        // Check if all timestamps are older than window + cleanup interval
        let expire_cutoff = now - (window_duration + cleanup_interval);
        self.timestamps.iter().all(|&timestamp| timestamp < expire_cutoff)
    }
}

/// Rate limiter using sliding window algorithm
#[derive(Clone)]
pub struct RateLimiter {
    /// Client IP -> Request history mapping
    clients: Arc<DashMap<String, RequestEntry>>,
    /// Rate limiter configuration
    config: RateLimiterConfig,
}

impl RateLimiter {
    /// Create a new rate limiter with default configuration
    pub fn new() -> Self {
        Self::with_config(RateLimiterConfig::default())
    }

    /// Create a new rate limiter with custom configuration
    pub fn with_config(config: RateLimiterConfig) -> Self {
        let rate_limiter = Self {
            clients: Arc::new(DashMap::new()),
            config,
        };

        // Start cleanup task
        rate_limiter.start_cleanup_task();
        rate_limiter
    }

    /// Start a background task to periodically clean up expired entries
    fn start_cleanup_task(&self) {
        let clients = Arc::clone(&self.clients);
        let config = self.config.clone();

        tokio::spawn(async move {
            let mut interval = time::interval(config.cleanup_interval);
            loop {
                interval.tick().await;
                
                let now = Instant::now();
                let mut expired_keys = Vec::new();

                // Find expired entries
                for entry in clients.iter() {
                    if entry.value().should_expire(now, config.window_duration, config.cleanup_interval) {
                        expired_keys.push(entry.key().clone());
                    }
                }

                // Remove expired entries
                for key in expired_keys {
                    clients.remove(&key);
                }

                tracing::debug!("Rate limiter cleanup: {} active clients", clients.len());
            }
        });
    }

    /// Check if a request should be allowed for the given client IP
    pub fn check_rate_limit(&self, client_ip: &str) -> bool {
        let mut entry = self.clients.entry(client_ip.to_string()).or_insert_with(RequestEntry::new);
        entry.add_request(&self.config)
    }

    /// Get current statistics for monitoring
    pub fn get_stats(&self) -> RateLimiterStats {
        RateLimiterStats {
            active_clients: self.clients.len(),
            max_requests: self.config.max_requests,
            window_duration_secs: self.config.window_duration.as_secs(),
        }
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

/// Rate limiter statistics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimiterStats {
    pub active_clients: usize,
    pub max_requests: u32,
    pub window_duration_secs: u64,
}

/// Extract client IP from request, with fallback strategies
fn get_client_ip(req: &Request<Body>) -> String {
    // Try to get IP from ConnectInfo (most reliable)
    if let Some(ConnectInfo(addr)) = req.extensions().get::<ConnectInfo<SocketAddr>>() {
        return addr.ip().to_string();
    }

    // Fallback: Check X-Forwarded-For header (for load balancers/proxies)
    if let Some(forwarded_for) = req.headers().get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded_for.to_str() {
            // Take the first IP in the chain
            if let Some(first_ip) = forwarded_str.split(',').next() {
                return first_ip.trim().to_string();
            }
        }
    }

    // Fallback: Check X-Real-IP header
    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            return ip_str.to_string();
        }
    }

    // Final fallback: unknown client
    "unknown".to_string()
}

/// Axum middleware function for rate limiting
pub async fn rate_limit_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get rate limiter from request extensions (injected by main.rs)
    let rate_limiter = req
        .extensions()
        .get::<RateLimiter>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let client_ip = get_client_ip(&req);
    
    if rate_limiter.check_rate_limit(&client_ip) {
        tracing::debug!("Rate limit OK for client: {}", client_ip);
        Ok(next.run(req).await)
    } else {
        tracing::warn!("Rate limit exceeded for client: {}", client_ip);
        Err(StatusCode::TOO_MANY_REQUESTS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_request_entry_rate_limiting() {
        let config = RateLimiterConfig {
            max_requests: 2,
            window_duration: Duration::from_secs(60),
            cleanup_interval: Duration::from_secs(300),
        };

        let mut entry = RequestEntry::new();

        // First request should be allowed
        assert!(entry.add_request(&config));
        assert_eq!(entry.timestamps.len(), 1);

        // Second request should be allowed
        assert!(entry.add_request(&config));
        assert_eq!(entry.timestamps.len(), 2);

        // Third request should be denied (rate limit exceeded)
        assert!(!entry.add_request(&config));
        assert_eq!(entry.timestamps.len(), 2);
    }

    #[tokio::test]
    async fn test_rate_limiter_basic() {
        let config = RateLimiterConfig {
            max_requests: 3,
            window_duration: Duration::from_secs(60),
            cleanup_interval: Duration::from_secs(300),
        };

        let rate_limiter = RateLimiter::with_config(config);
        let client_ip = "192.168.1.1";

        // First 3 requests should be allowed
        assert!(rate_limiter.check_rate_limit(client_ip));
        assert!(rate_limiter.check_rate_limit(client_ip));
        assert!(rate_limiter.check_rate_limit(client_ip));

        // Fourth request should be denied
        assert!(!rate_limiter.check_rate_limit(client_ip));
    }

    #[tokio::test]
    async fn test_different_clients() {
        let rate_limiter = RateLimiter::new();
        
        let client1 = "192.168.1.1";
        let client2 = "192.168.1.2";

        // Each client should have their own rate limit
        assert!(rate_limiter.check_rate_limit(client1));
        assert!(rate_limiter.check_rate_limit(client2));
    }
}
