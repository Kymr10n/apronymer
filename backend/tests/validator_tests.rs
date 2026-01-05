use apronymer::routes::GenerateRequest;
use apronymer::validator::validate_generate_request;

fn valid_request() -> GenerateRequest {
    GenerateRequest {
        terms: vec!["Alpha".to_string(), "Echo".to_string(), "India".to_string()],
        frag_len: 1, // 1 = first letter only
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
    let req = GenerateRequest {
        terms: vec![],
        frag_len: 1,
        min_len: 2,
        max_len: 3,
    };
    assert!(validate_generate_request(&req).is_err());
}

#[test]
fn test_duplicate_terms_fails() {
    let req = GenerateRequest {
        terms: vec!["Alpha".to_string(), "Alpha".to_string()],
        frag_len: 1,
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
    // With 3 terms and frag_len=1, max possible length is 3
    req.max_len = 4; // This should fail
    assert!(validate_generate_request(&req).is_err());

    // With 3 terms and frag_len=2, max possible length is 6
    req.frag_len = 2;
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
