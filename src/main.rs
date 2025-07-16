use axum::{Router, routing::get, serve};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod routes;
mod generator;
mod dictionary;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/api", routes::routes())  // 👈 Mount your API routes
        .route("/hello", get(|| async { "Hello Axum 0.8!" }))
        .fallback_service(ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://{}", addr);

    serve(listener, app).await.unwrap();
}