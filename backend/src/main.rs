use axum::{Router, routing::get, serve};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;
use dotenvy::dotenv;
use std::env;

mod routes;
mod validator;
mod generator;
mod dictionary;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env if present
    tracing_subscriber::fmt::init();

    // Get host and port from environment or use defaults
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let addr = format!("{}:{}", host, port).parse::<SocketAddr>().expect("Invalid HOST or PORT");
    let listener = TcpListener::bind(addr).await.unwrap();
    let app = Router::new()
        .nest("/api", routes::routes())
        .route("/hello", get(|| async { "Hello Axum 0.8!" }))
        .fallback_service(ServeDir::new("static"))
        .layer(TraceLayer::new_for_http());

    tracing::info!("Listening on http://{}", addr);

    serve(listener, app).await.unwrap();
}