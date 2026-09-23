// Behavioral analytics middleware: logs request metadata (IP, User-Agent, Referrer)
// and flags suspicious patterns (unusual/non-browser User-Agents). These structured
// log lines are designed to be picked up by Azure Monitor / Application Insights via
// the Container App's log stream (see docs/analytics.md or README for KQL examples).
use axum::{body::Body, extract::ConnectInfo, http::Request, middleware::Next, response::Response};
use std::net::SocketAddr;

/// User-Agent substrings commonly associated with scripts, bots, or non-browser
/// HTTP clients rather than real end-users browsing the app.
const NON_BROWSER_UA_MARKERS: &[&str] = &[
    "curl",
    "wget",
    "python-requests",
    "python-urllib",
    "go-http-client",
    "java/",
    "okhttp",
    "libwww-perl",
    "scrapy",
    "bot",
    "spider",
    "crawler",
    "httpclient",
    "axios",
    "postmanruntime",
];

/// Extract the client IP from the request, preferring proxy headers (as set by
/// Azure Container Apps' ingress) and falling back to the socket address.
fn get_client_ip(req: &Request<Body>) -> String {
    if let Some(forwarded_for) = req.headers().get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded_for.to_str() {
            if let Some(first_ip) = forwarded_str.split(',').next() {
                let ip = first_ip.trim();
                if !ip.is_empty() {
                    return ip.to_string();
                }
            }
        }
    }

    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            if !ip_str.is_empty() {
                return ip_str.to_string();
            }
        }
    }

    if let Some(ConnectInfo(addr)) = req.extensions().get::<ConnectInfo<SocketAddr>>() {
        return addr.ip().to_string();
    }

    "unknown".to_string()
}

/// Determine whether a User-Agent string looks like a non-browser / scripted
/// client, or is missing entirely (also considered suspicious).
pub fn is_suspicious_user_agent(user_agent: Option<&str>) -> bool {
    match user_agent {
        None => true,
        Some(ua) => {
            let ua_trimmed = ua.trim();
            if ua_trimmed.is_empty() {
                return true;
            }
            let ua_lower = ua_trimmed.to_lowercase();
            NON_BROWSER_UA_MARKERS
                .iter()
                .any(|marker| ua_lower.contains(marker))
        }
    }
}

/// Axum middleware that logs behavioral analytics data (IP, User-Agent, Referrer,
/// method, path) for every request, flagging unusual/non-browser clients so that
/// they can be surfaced via log-based alerts in Azure Monitor / Application Insights.
pub async fn analytics_middleware(req: Request<Body>, next: Next) -> Response {
    let client_ip = get_client_ip(&req);
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    let user_agent = req
        .headers()
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let referrer = req
        .headers()
        .get(axum::http::header::REFERER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "none".to_string());

    let suspicious = is_suspicious_user_agent(user_agent.as_deref());
    let user_agent_display = user_agent.unwrap_or_else(|| "none".to_string());

    if suspicious {
        tracing::warn!(
            ip = %client_ip,
            method = %method,
            path = %path,
            user_agent = %user_agent_display,
            referrer = %referrer,
            "🚨 Suspicious request: unusual or missing User-Agent"
        );
    } else {
        tracing::info!(
            ip = %client_ip,
            method = %method,
            path = %path,
            user_agent = %user_agent_display,
            referrer = %referrer,
            "📊 Request analytics"
        );
    }

    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_user_agent_is_suspicious() {
        assert!(is_suspicious_user_agent(None));
        assert!(is_suspicious_user_agent(Some("")));
        assert!(is_suspicious_user_agent(Some("   ")));
    }

    #[test]
    fn test_script_user_agents_are_suspicious() {
        assert!(is_suspicious_user_agent(Some("curl/8.4.0")));
        assert!(is_suspicious_user_agent(Some("Wget/1.21.3")));
        assert!(is_suspicious_user_agent(Some(
            "python-requests/2.31.0"
        )));
        assert!(is_suspicious_user_agent(Some("Go-http-client/1.1")));
        assert!(is_suspicious_user_agent(Some(
            "Mozilla/5.0 (compatible; SomeBot/1.0)"
        )));
        assert!(is_suspicious_user_agent(Some("PostmanRuntime/7.36.0")));
    }

    #[test]
    fn test_browser_user_agents_are_not_suspicious() {
        assert!(!is_suspicious_user_agent(Some(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
        )));
        assert!(!is_suspicious_user_agent(Some(
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1"
        )));
    }

    #[test]
    fn test_get_client_ip_from_forwarded_header() {
        let req = Request::builder()
            .header("x-forwarded-for", "203.0.113.5, 10.0.0.1")
            .body(Body::empty())
            .unwrap();
        assert_eq!(get_client_ip(&req), "203.0.113.5");
    }

    #[test]
    fn test_get_client_ip_from_real_ip_header() {
        let req = Request::builder()
            .header("x-real-ip", "198.51.100.7")
            .body(Body::empty())
            .unwrap();
        assert_eq!(get_client_ip(&req), "198.51.100.7");
    }

    #[test]
    fn test_get_client_ip_unknown_fallback() {
        let req = Request::builder().body(Body::empty()).unwrap();
        assert_eq!(get_client_ip(&req), "unknown");
    }
}
