use apronymer::routes::{generate, GenerateRequest};
use axum::response::IntoResponse;

#[tokio::test]
async fn test_generate_handler_validation_error() {
    let payload = GenerateRequest {
        terms: vec!["a".to_string(), "b".to_string()], // too few terms
        frag_len: 2,
        min_len: 3,
        max_len: 3,
    };
    let response = generate(axum::Json(payload)).await.into_response();
    // Should be a validation error (400)
    assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_generate_handler_valid_request() {
    let payload = GenerateRequest {
        terms: vec!["Alpha".to_string(), "Echo".to_string(), "India".to_string()],
        frag_len: 1,
        min_len: 2,
        max_len: 3,
    };
    let response = generate(axum::Json(payload)).await.into_response();
    // Should succeed (200)
    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[tokio::test]
async fn test_generate_handler_duplicate_terms() {
    let payload = GenerateRequest {
        terms: vec!["Alpha".to_string(), "Alpha".to_string(), "Echo".to_string()],
        frag_len: 1,
        min_len: 2,
        max_len: 3,
    };
    let response = generate(axum::Json(payload)).await.into_response();
    // Should be a validation error (400)
    assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
}
