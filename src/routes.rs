use axum::{Router, routing::post, Json};
use serde::Deserialize;

use crate::generator::generate_apronyms;

#[derive(Deserialize)]
pub struct GenerateRequest {
    terms: Vec<String>,
    min_len: usize,
    max_len: usize,
}

pub async fn generate(Json(payload): Json<GenerateRequest>) -> Json<Vec<String>> {
    let results = generate_apronyms(payload.terms, payload.min_len, payload.max_len);
    Json(results)
}

pub fn routes() -> Router {
    Router::new().route("/generate", post(generate))
}