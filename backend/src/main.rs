// Main entry point for the backend server
use axum::{Router, routing::get, serve};
use tower_http::services::ServeDir; // For serving static files
use tower_http::trace::TraceLayer; // For request/response logging
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber; // For logging
use dotenvy::dotenv; // For loading .env files
use std::env;

mod routes;      // API route handlers
mod validator;   // Request validation logic
mod generator;   // Apronym generation logic
mod dictionary;  // Dictionary lookup logic

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

    // Build the Axum app with API routes and static file serving
    let app = Router::new()
        .nest("/api", routes::routes()) // Mount API routes at /api
        .route("/hello", get(|| async { "Hello Axum 0.8!" })) // Example route
        .fallback_service(ServeDir::new("static")) // Serve static files
        .layer(TraceLayer::new_for_http()); // Add request logging

    tracing::info!("Listening on http://{}", addr);

    // Start the server
    serve(listener, app).await.unwrap();
}