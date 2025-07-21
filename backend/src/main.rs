use axum::{Router, routing::get, serve};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;

mod routes;
mod validator;
mod generator;
mod dictionary;

#[tokio::main]
async fn main() {
    // Set up basic tracing subscriber for logging
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .nest("/api", routes::routes())  // 👈 Mount your API routes
        .route("/hello", get(|| async { "Hello Axum 0.8!" }))
        .fallback_service(ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Listening on http://{}", addr);

    serve(listener, app).await.unwrap();
}