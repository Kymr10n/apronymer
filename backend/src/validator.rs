use axum::{http::StatusCode, response::{IntoResponse, Response}};
use crate::routes::GenerateRequest;
use std::collections::HashSet;

/// Validate the GenerateRequest payload
pub fn validate_generate_request(payload: &GenerateRequest) -> Result<(), Response> {
    if payload.terms.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Please enter at least one valid term.").into_response());
    }

    if payload.terms.len() > 10 {
        return Err((StatusCode::BAD_REQUEST, "Please enter no more than 10 terms.").into_response());
    }

    let unique_terms: HashSet<_> = payload.terms.iter().collect();
    if unique_terms.len() != payload.terms.len() {
        return Err((StatusCode::BAD_REQUEST, "Terms must be unique — please remove duplicates.").into_response());
    }

    if payload.min_len == 0 || payload.max_len == 0 {
        return Err((StatusCode::BAD_REQUEST, "Min Length and Max Length must both be at least 1.").into_response());
    }

    if payload.min_len > 10 || payload.max_len > 10 {
        return Err((StatusCode::BAD_REQUEST, "Min Length and Max Length must not exceed 10.").into_response());
    }

    if payload.max_len < payload.min_len {
        return Err((StatusCode::BAD_REQUEST, "Max Length must be greater than or equal to Min Length.").into_response());
    }

    if payload.terms.len() < payload.max_len {
        return Err((StatusCode::BAD_REQUEST, "The number of terms must be greater than or equal to Max Length.").into_response());
    }

    if payload.terms.iter().any(|t| !t.chars().all(|c| c.is_alphabetic())) {
        return Err((StatusCode::BAD_REQUEST, "Terms must only contain letters (A-Z).").into_response());
    }

    if !payload.terms.iter().any(|t| {
        t.chars()
            .next()
            .map(|c| "aeiouAEIOU".contains(c))
            .unwrap_or(false)
    }) {
        return Err((StatusCode::BAD_REQUEST, "At least one term must start with a vowel (A, E, I, O, U).").into_response());
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
        let req = GenerateRequest { terms: vec![], min_len: 2, max_len: 3 };
        assert!(validate_generate_request(&req).is_err());
    }

    #[test]
    fn test_duplicate_terms_fails() {
        let req = GenerateRequest {
            terms: vec!["Alpha".to_string(), "Alpha".to_string()],
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