// ...existing code...
// Request validation logic for the backend
use axum::{http::StatusCode, response::{IntoResponse, Response}};
use crate::routes::GenerateRequest;
use std::collections::HashSet;

/// Validate the GenerateRequest payload
/// Returns Ok(()) if valid, or an error response if invalid
#[allow(clippy::result_large_err)]
pub fn validate_generate_request(payload: &GenerateRequest) -> Result<(), Response> {
    tracing::info!("Validating generate request: terms={:?}, frag_len={}, min_len={}, max_len={}", payload.terms, payload.frag_len, payload.min_len, payload.max_len);

    // Check for empty input
    if payload.terms.len() < 3 {
        return Err((StatusCode::BAD_REQUEST, "Please enter at least three valid terms.").into_response());
    }
    // Check for too many terms
    if payload.terms.len() > 10 {
        return Err((StatusCode::BAD_REQUEST, "Please enter no more than 10 terms.").into_response());
    }
    // Check for duplicate terms
    let unique_terms: HashSet<_> = payload.terms.iter().collect();
    if unique_terms.len() != payload.terms.len() {
        return Err((StatusCode::BAD_REQUEST, "Terms must be unique — please remove duplicates.").into_response());
    }
    // Check for valid min/max lengths
    if payload.min_len == 0 || payload.max_len == 0 {
        return Err((StatusCode::BAD_REQUEST, "Min Length and Max Length must both be at least 1.").into_response());
    }
    if payload.min_len > 10 || payload.max_len > 10 {
        return Err((StatusCode::BAD_REQUEST, "Min Length and Max Length must not exceed 10.").into_response());
    }
    if payload.max_len < payload.min_len {
        return Err((StatusCode::BAD_REQUEST, "Max Length must be greater than or equal to Min Length.").into_response());
    }
    
    // Only check max_len against number of terms
    if payload.max_len > payload.terms.len() {
        return Err((StatusCode::BAD_REQUEST, "Max Length cannot exceed the number of terms provided.").into_response());
    }
    // Safety check: prevent excessive computational complexity
    // This prevents potential DoS attacks or server overload
    let max_combinations = (payload.frag_len as u32).pow(payload.max_len as u32);
    if max_combinations > 10_000 {
        return Err((StatusCode::BAD_REQUEST, "Request would generate too many combinations. Please reduce Fragment Length or Max Length.").into_response());
    }
    
    // Check for valid fragment length (must be between 1 and 3)
    if payload.frag_len < 1 || payload.frag_len > 3 {
        return Err((StatusCode::BAD_REQUEST, "Fragment Length must be between 1 and 3.").into_response());
    }
    
    // Check that all terms have at least frag_len characters
    let terms_too_short: Vec<_> = payload.terms.iter()
        .filter(|term| term.len() < payload.frag_len)
        .collect();
        
    if !terms_too_short.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "All terms must have at least as many characters as the specified Fragment Length.").into_response());
    }
    
    // All terms are valid for the specified frag_len
    // Check for only alphabetic terms
    if payload.terms.iter().any(|t| !t.chars().all(|c| c.is_alphabetic())) {
        return Err((StatusCode::BAD_REQUEST, "Terms must only contain letters (A-Z)." ).into_response());
    }
    // Check for at least one term starting with a vowel
    if !payload.terms.iter().any(|t| matches!(t.chars().next(), Some(c) if "aeiouAEIOU".contains(c))) {
        return Err((StatusCode::BAD_REQUEST, "At least one term must start with a vowel (A, E, I, O, U)." ).into_response());
    }
    Ok(())
}

