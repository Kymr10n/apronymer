use apronymer::generator::generate_apronyms;

#[test]
fn test_generate_apronyms_valid_input() {
    let terms = vec!["Alpha".to_string(), "Echo".to_string(), "India".to_string()];
    let _results = generate_apronyms(terms, 1, 2, 3);
    // Can't assert exact contents due to dictionary check, but should not panic
    // This test ensures the function completes without errors
}

#[test]
fn test_generate_apronyms_empty_terms() {
    let terms: Vec<String> = vec![];
    let results = generate_apronyms(terms, 1, 1, 3);
    assert!(results.is_empty(), "Empty terms should return empty results");
}

#[test]
fn test_generate_apronyms_single_term() {
    let terms = vec!["Test".to_string()];
    let _results = generate_apronyms(terms, 2, 1, 1);
    // Should generate apronyms with prefixes "T" and "Te"
    // Results depend on dictionary, but function should not panic
}

#[test]
fn test_generate_apronyms_different_frag_lengths() {
    let terms = vec!["Auto".to_string(), "Baum".to_string()];
    
    // Test with frag_len = 1 (first letters only)
    let _results_1 = generate_apronyms(terms.clone(), 1, 2, 2);
    
    // Test with frag_len = 2 (up to 2 characters)
    let _results_2 = generate_apronyms(terms.clone(), 2, 2, 2);
    
    // Test with frag_len = 3 (up to 3 characters)
    let _results_3 = generate_apronyms(terms, 3, 2, 2);
    
    // Results may vary based on dictionary, but should not panic
}

#[test]
fn test_generate_apronyms_min_max_len_constraints() {
    let terms = vec!["One".to_string(), "Two".to_string(), "Three".to_string()];
    
    // Test min_len = max_len = 2 (exactly 2 terms)
    let _results_2 = generate_apronyms(terms.clone(), 1, 2, 2);
    
    // Test min_len = max_len = 3 (exactly 3 terms)
    let _results_3 = generate_apronyms(terms.clone(), 1, 3, 3);
    
    // Test min_len = 1, max_len = 3 (1 to 3 terms)
    let _results_range = generate_apronyms(terms, 1, 1, 3);
    
    // All should complete without panic
}

#[test]
fn test_apronym_structure() {
    // Test that Apronym structure is correctly populated
    let terms = vec!["Test".to_string(), "Word".to_string()];
    let results = generate_apronyms(terms.clone(), 1, 2, 2);
    
    // Verify that all results have non-empty text and terms
    for apronym in &results {
        assert!(!apronym.text.is_empty(), "Apronym text should not be empty");
        assert!(!apronym.terms.is_empty(), "Apronym terms should not be empty");
        assert!(apronym.terms.len() >= 1 && apronym.terms.len() <= 2, 
            "Apronym should have 1-2 terms as requested");
        
        // Verify that all terms are from the original input
        for term in &apronym.terms {
            assert!(terms.contains(term), "All apronym terms should be from input terms");
        }
    }
}
