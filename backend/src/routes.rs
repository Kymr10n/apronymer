// Route handlers and API types for the backend
use axum::{Json, Router, response::IntoResponse, routing::post, extract::Extension};
use serde::{Deserialize, Serialize};

use crate::generator::generate_apronyms; // Apronym generation logic
use crate::validator::validate_generate_request; // Request validation
use crate::dictionary; // Dictionary access
use crate::rate_limiter::{RateLimiter, RateLimiterStats}; // Rate limiter

/// Request payload for generating apronyms
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub terms: Vec<String>,   // Input terms
    pub frag_len: usize,      // Number of characters to take from each term (1-3)
    pub min_len: usize,       // Minimum apronym length
    pub max_len: usize,       // Maximum apronym length
}

/// Apronym result type
#[derive(Debug, Serialize, Deserialize)]
pub struct Apronym {
    pub text: String,         // The generated apronym
    pub terms: Vec<String>,   // Terms used to create the apronym
}

/// Handler to generate apronyms based on input terms
pub async fn generate(Json(payload): Json<GenerateRequest>) -> impl IntoResponse {
    tracing::info!("Handling /generate: terms={:?}, frag_len={}, min_len={}, max_len={}", payload.terms, payload.frag_len, payload.min_len, payload.max_len);
    if let Err(err) = validate_generate_request(&payload) {
        tracing::warn!("Validation failed for /generate: terms={:?}, frag_len={}, min_len={}, max_len={}", payload.terms, payload.frag_len, payload.min_len, payload.max_len);
        return err;
    }
    let results = generate_apronyms(payload.terms, payload.frag_len, payload.min_len, payload.max_len);
    tracing::info!("Generated {} apronyms", results.len());
    Json(results).into_response()
}

/// Health status response
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub dictionary_words: usize,
    pub dictionary_available: bool,
    pub rate_limiter: Option<RateLimiterStats>,
    pub timestamp: String,
}

/// Health check endpoint (no API key required)
pub async fn health(Extension(rate_limiter): Extension<RateLimiter>) -> impl IntoResponse {
    let (word_count, has_words) = dictionary::get_dictionary_stats();
    let rate_limiter_stats = rate_limiter.get_stats();
    
    let health_response = HealthResponse {
        status: if has_words { "healthy".to_string() } else { "degraded".to_string() },
        dictionary_words: word_count,
        dictionary_available: has_words,
        rate_limiter: Some(rate_limiter_stats),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    
    tracing::info!("Health check: status={}, dictionary_words={}, dictionary_available={}, rate_limiter_clients={}", 
        health_response.status, health_response.dictionary_words, health_response.dictionary_available,
        health_response.rate_limiter.as_ref().map_or(0, |rl| rl.active_clients));
    
    Json(health_response)
}

/// Define the API routes for the application
pub fn routes() -> Router {
    Router::new()
        .route("/generate", post(generate)) // POST /api/generate
}