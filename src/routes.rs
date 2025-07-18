use axum::{Router, routing::post, Json};
use serde::{Deserialize, Serialize};

use crate::generator::generate_apronyms;

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequest {
    terms: Vec<String>,
    min_len: usize,
    max_len: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Apronym {
    pub name: String,
    pub terms: Vec<String>,
}

pub async fn generate(Json(payload): Json<GenerateRequest>) -> Json<Vec<Apronym>> {
    let results = generate_apronyms(payload.terms, payload.min_len, payload.max_len);
    Json(results)
}

pub fn routes() -> Router {
    Router::new().route("/generate", post(generate))
}