// Request validation logic for the backend
use axum::{http::StatusCode, response::{IntoResponse, Response}};
use crate::routes::GenerateRequest;
use std::collections::HashSet;
use tracing;

/// Validate the GenerateRequest payload
/// Returns Ok(()) if valid, or an error response if invalid
pub fn validate_generate_request(payload: &GenerateRequest) -> Result<(), Response> {
    tracing::info!("Validating generate request: terms={:?}, term_len={}, min_len={}, max_len={}", payload.terms, payload.term_len, payload.min_len, payload.max_len);

    // Check for empty input
    if payload.terms.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Please enter at least one valid term.").into_response());
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
    
    // Calculate maximum possible apronym length (term_len * number_of_terms)
    let max_possible_length = payload.term_len * payload.terms.len();
    if payload.max_len > max_possible_length {
        return Err((StatusCode::BAD_REQUEST, format!("Max Length cannot exceed {} (Term Length × Number of Terms)", max_possible_length)).into_response());
    }
    
    // Check for valid term length (must be between 1 and 3)
    if payload.term_len < 1 || payload.term_len > 3 {
        return Err((StatusCode::BAD_REQUEST, "Term Length must be between 1 and 3.").into_response());
    }
    
    // Check that all terms have at least term_len characters
    let terms_too_short: Vec<_> = payload.terms.iter()
        .filter(|term| term.len() < payload.term_len)
        .collect();
        
    if !terms_too_short.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "All terms must have at least as many characters as the specified Term Length.").into_response());
    }
    
    // All terms are valid for the specified term_len
    if payload.terms.len() < payload.max_len {
        return Err((StatusCode::BAD_REQUEST, "The number of terms must be greater than or equal to Max Length.").into_response());
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::GenerateRequest;

    fn valid_request() -> GenerateRequest {
        GenerateRequest {
            terms: vec!["Alpha".to_string(), "Echo".to_string(), "India".to_string()],
            term_len: 1, // 1 = first letter only
            min_len: 2,
            max_len: 3,
        }
    }

    #[test]
    fn test_valid_request_passes() {
        let req = valid_request();
        assert!(validate_generate_request(&req).is_ok());
    }

    #[test]
    fn test_empty_terms_fails() {
        let req = GenerateRequest { terms: vec![], term_len: 1, min_len: 2, max_len: 3 };
        assert!(validate_generate_request(&req).is_err());
    }

    #[test]
    fn test_duplicate_terms_fails() {
        let req = GenerateRequest {
            terms: vec!["Alpha".to_string(), "Alpha".to_string()],
            term_len: 1,
            min_len: 1,
            max_len: 1,
        };
        assert!(validate_generate_request(&req).is_err());
    }

    #[test]
    fn test_invalid_length_fails() {
        let mut req = valid_request();
        req.min_len = 0;
        assert!(validate_generate_request(&req).is_err());

        req.min_len = 5;
        req.max_len = 4;
        assert!(validate_generate_request(&req).is_err());
    }

    #[test]
    fn test_max_length_exceeds_possible_fails() {
        let mut req = valid_request();
        // With 3 terms and term_len=1, max possible length is 3
        req.max_len = 4; // This should fail
        assert!(validate_generate_request(&req).is_err());
        
        // With 3 terms and term_len=2, max possible length is 6
        req.term_len = 2;
        req.max_len = 7; // This should fail
        assert!(validate_generate_request(&req).is_err());
    }

    #[test]
    fn test_non_alphabetic_terms_fail() {
        let mut req = valid_request();
        req.terms = vec!["123".to_string(), "Test".to_string()];
        assert!(validate_generate_request(&req).is_err());
    }

    #[test]
    fn test_missing_vowel_start_fails() {
        let mut req = valid_request();
        req.terms = vec!["Beta".to_string(), "Charlie".to_string()];
        assert!(validate_generate_request(&req).is_err());
    }
}