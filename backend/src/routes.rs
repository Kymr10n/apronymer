// Route handlers and API types for the backend
use axum::{Json, Router, response::{IntoResponse, Response}, routing::post, http::StatusCode};
use serde::{Deserialize, Serialize};
use tracing;

use crate::generator::generate_apronyms; // Apronym generation logic
use crate::validator::validate_generate_request; // Request validation

/// Request payload for generating apronyms
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub terms: Vec<String>,   // Input terms
    pub min_len: usize,       // Minimum apronym length
    pub max_len: usize,       // Maximum apronym length
}

/// Apronym result type
#[derive(Debug, Serialize, Deserialize)]
pub struct Apronym {
    pub name: String,         // The generated apronym
    pub terms: Vec<String>,   // Terms used to create the apronym
}

/// Handler to generate apronyms based on input terms
pub async fn generate(Json(payload): Json<GenerateRequest>) -> impl IntoResponse {
    tracing::info!("Handling /generate: terms={:?}, min_len={}, max_len={}", payload.terms, payload.min_len, payload.max_len);
    if let Err(err) = validate_generate_request(&payload) {
        tracing::warn!("Validation failed for /generate: terms={:?}, min_len={}, max_len={}", payload.terms, payload.min_len, payload.max_len);
        return err;
    }
    let results = generate_apronyms(payload.terms, payload.min_len, payload.max_len);
    tracing::info!("Generated {} apronyms", results.len());
    Json(results).into_response()
}

/// Define the API routes for the application
pub fn routes() -> Router {
    Router::new()
        .route("/generate", post(generate)) // POST /api/generate
}