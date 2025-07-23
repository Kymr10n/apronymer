// Main entry point for the backend server
use axum::{Router, serve};
use tower_http::trace::TraceLayer; // For request/response logging
use tower_http::cors::{CorsLayer, Any}; // For CORS support
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber; // For logging
use dotenvy::dotenv; // For loading .env files
use std::env;

mod routes;      // API route handlers
mod validator;   // Request validation logic
mod generator;   // Apronym generation logic
mod dictionary;  // Dictionary lookup logic

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
    dotenv().ok();
    // Initialize logging (tracing)
    tracing_subscriber::fmt::init();

    // Get host and port from environment or use defaults
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let addr = format!("{}:{}", host, port).parse::<SocketAddr>().expect("Invalid HOST or PORT");
    let listener = TcpListener::bind(addr).await.unwrap();

    // Build the Axum app with health endpoint accessible and API routes protected
    let app = Router::new()
        .route("/health", axum::routing::get(routes::health)) // Health endpoint without API key
        .nest("/api", routes::routes().layer(axum::middleware::from_fn(require_api_key))) // API routes with API key
        .layer(TraceLayer::new_for_http()) // Add request logging
        .layer(
            CorsLayer::new()
                .allow_origin(Any) // Allow all origins (customize for production)
                .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
                .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::HeaderName::from_static("x-api-key")])
        );

    tracing::info!("Listening on http://{}", addr);

    // Start the server
    serve(listener, app).await.unwrap();
}