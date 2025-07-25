#[cfg(test)]
mod tests {
    #[test]
    fn test_env_var_api_key() {
        // This test checks that the API_KEY env var can be read (may be empty in test)
        let _ = std::env::var("API_KEY");
    }
}
// Main entry point for the backend server
use axum::{Router, serve};
use tower_http::trace::TraceLayer; // For request/response logging
use tower_http::cors::{CorsLayer, Any}; // For CORS support
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{fmt, EnvFilter}; // For logging
use std::env;
use std::time::Duration;

mod routes;      // API route handlers
mod validator;   // Request validation logic
mod generator;   // Apronym generation logic
mod dictionary;  // Dictionary lookup logic
mod rate_limiter; // Rate limiting middleware

use rate_limiter::{RateLimiter, RateLimiterConfig};

/// Middleware to check API key in request header
async fn require_api_key(
    req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next
) -> Result<axum::response::Response, axum::http::StatusCode> {
    // Get API key from environment
    let expected_key = std::env::var("API_KEY").unwrap_or_default();
    // Check header
    let key = req.headers().get("x-api-key").and_then(|v| v.to_str().ok());
    if key == Some(&expected_key) {
        Ok(next.run(req).await)
    } else {
        Err(axum::http::StatusCode::UNAUTHORIZED)
    }
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env if present
    if let Ok(iter) = dotenvy::from_path_iter(".env") {
        for item in iter.flatten() {
            // Only set if not already defined (Azure envs take priority)
            env::set_var(&item.0, env::var(&item.0).unwrap_or(item.1));
        }
    }

    // Initialize logging (tracing) with enhanced formatting
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    tracing::info!("🚀 Starting Apronymer Backend Server");
    tracing::info!("📋 Environment variables:");
    tracing::info!("  - HOST: {}", env::var("HOST").unwrap_or_else(|_| "127.0.0.1 (default)".to_string()));
    tracing::info!("  - PORT: {}", env::var("PORT").unwrap_or_else(|_| "3000 (default)".to_string()));
    tracing::info!("  - RUST_LOG: {}", env::var("RUST_LOG").unwrap_or_else(|_| "not set".to_string()));
    tracing::info!("  - API_KEY: {}", if env::var("API_KEY").is_ok() { "✅ set" } else { "❌ not set" });

    // Get host and port from environment or use defaults
    let host = env::var("HOST").expect("❌ HOST environment variable is missing");
    let port: u16 = env::var("PORT")
        .expect("❌ PORT environment variable is missing")
        .parse()
        .expect("❌ PORT must be a valid integer");
    
    tracing::info!("🌐 Binding to {}:{}", host, port);
    let addr = match format!("{host}:{port}").parse::<SocketAddr>() {
        Ok(addr) => addr,
        Err(e) => {
            tracing::error!("❌ Failed to parse address {}:{} - {}", host, port, e);
            std::process::exit(1);
        }
    };
    
    match TcpListener::bind(addr).await {
        Ok(listener) => {
            tracing::info!("✅ Successfully bound to {}", addr);
            
            // Initialize dictionary and check stats
            tracing::info!("🔄 Checking dictionary status...");
            let (word_count, has_words) = dictionary::get_dictionary_stats();
            if has_words {
                tracing::info!("✅ Dictionary ready with {} words", word_count);
            } else {
                tracing::error!("❌ Dictionary is empty or unavailable! Apronym generation will fail.");
                tracing::error!("🔧 Please check that the wordlist file is properly mounted in the container.");
            }

            // Initialize rate limiter
            tracing::info!("🔄 Setting up rate limiter...");
            let rate_limiter_config = RateLimiterConfig {
                max_requests: 10,                           // 10 requests
                window_duration: Duration::from_secs(60),   // per minute
                cleanup_interval: Duration::from_secs(300), // cleanup every 5 minutes
            };
            let rate_limiter = RateLimiter::with_config(rate_limiter_config);
            tracing::info!("✅ Rate limiter configured: 10 requests per minute per IP");

            // Build the Axum app with health endpoint accessible and API routes protected
            let app = Router::new()
                .route("/health", axum::routing::get(routes::health)) // Health endpoint without API key or rate limiting
                .nest("/api", 
                    routes::routes()
                        .layer(axum::middleware::from_fn(rate_limiter::rate_limit_middleware)) // Rate limiting first
                        .layer(axum::middleware::from_fn(require_api_key)) // Then API key check
                )
                .layer(axum::Extension(rate_limiter)) // Add rate limiter to all routes
                .layer(TraceLayer::new_for_http()) // Add request logging
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any) // Allow all origins (customize for production)
                        .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::OPTIONS])
                        .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::HeaderName::from_static("x-api-key")])
                );

            tracing::info!("🎯 Server ready! Listening on http://{}", addr);
            tracing::info!("🏥 Health endpoint: http://{}/health", addr);
            tracing::info!("🔗 API endpoints: http://{}/api/*", addr);

            // Start the server
            if let Err(e) = serve(listener, app).await {
                tracing::error!("❌ Server failed: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            tracing::error!("❌ Failed to bind to {}: {}", addr, e);
            std::process::exit(1);
        }
    }
}

#[test]
fn test_env_host_and_port() {
    std::env::set_var("HOST", "0.0.0.0");
    std::env::set_var("PORT", "3000");

    let host = std::env::var("HOST").unwrap();
    let port: u16 = std::env::var("PORT").unwrap().parse().unwrap();

    assert_eq!(host, "0.0.0.0");
    assert_eq!(port, 3000);
}
