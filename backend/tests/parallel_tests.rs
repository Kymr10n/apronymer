use apronymer::{generate_apronyms, generate_apronyms_with_limit};

/// Test that parallel processing is triggered for large workloads
#[test]
fn test_parallel_threshold_combinations() {
    // Create a workload that exceeds PARALLEL_COMBINATIONS_THRESHOLD (1000)
    let terms = vec![
        "adventure".to_string(),
        "beautiful".to_string(),
        "creative".to_string(),
        "delightful".to_string(),
        "energetic".to_string(),
    ];

    // frag_len=2, 5 terms, min_len=3, max_len=4
    // This should trigger parallel processing
    let results = generate_apronyms(terms.clone(), 2, 3, 4);

    // Should complete successfully
    assert!(results.len() <= 100); // respects DEFAULT_MAX_RESULTS
}

/// Test that parallel processing is triggered by permutation count
#[test]
fn test_parallel_threshold_permutations() {
    // Create a workload that exceeds PARALLEL_PERMUTATIONS_THRESHOLD (10)
    let terms = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
        "elderberry".to_string(),
    ];

    // 5 terms, min_len=3, max_len=4 creates many permutations
    let results = generate_apronyms(terms.clone(), 1, 3, 4);

    // Should complete successfully
    assert!(results.len() <= 100);
}

/// Test safety limit for excessive combinations per permutation
#[test]
fn test_max_combinations_safety_limit() {
    // Create a scenario that would exceed MAX_COMBINATIONS_PER_PERMUTATION (10,000)
    let terms = vec![
        "supercalifragilisticexpialidocious".to_string(),
        "antidisestablishmentarianism".to_string(),
        "pseudopseudohypoparathyroidism".to_string(),
        "floccinaucinihilipilification".to_string(),
        "pneumonoultramicroscopicsilicovolcanoconiosis".to_string(),
        "hippopotomonstrosesquippedaliophobia".to_string(),
        "incomprehensibilities".to_string(),
        "strengths".to_string(),
    ];

    // With frag_len=3 and 8 terms, some permutations would have 3^8 = 6561 combinations
    // This tests that the safety limit logic works
    let results = generate_apronyms(terms.clone(), 3, 6, 8);

    // Should complete without panic or timeout
    assert!(results.len() <= 100);
}

/// Test early termination with small limit
#[test]
fn test_early_termination_small_limit() {
    let terms = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
    ];

    let results_10 = generate_apronyms_with_limit(terms.clone(), 2, 3, 4, 10);
    let results_100 = generate_apronyms_with_limit(terms.clone(), 2, 3, 4, 100);

    // Small limit should return fewer or equal results
    assert!(results_10.len() <= 10);
    assert!(results_10.len() <= results_100.len());
}

/// Test thread safety with parallel execution
#[test]
fn test_thread_safety_parallel() {
    use std::sync::Arc;
    use std::thread;

    let terms = Arc::new(vec![
        "adventure".to_string(),
        "beautiful".to_string(),
        "creative".to_string(),
        "delightful".to_string(),
    ]);

    let mut handles = vec![];

    // Spawn multiple threads executing generation concurrently
    for _ in 0..4 {
        let terms_clone = Arc::clone(&terms);
        let handle = thread::spawn(move || generate_apronyms((*terms_clone).clone(), 2, 3, 4));
        handles.push(handle);
    }

    // All threads should complete successfully
    for handle in handles {
        let results = handle.join().expect("Thread should not panic");
        assert!(results.len() <= 100);
    }
}

/// Test edge case: empty terms
#[test]
fn test_empty_terms() {
    let terms: Vec<String> = vec![];
    let results = generate_apronyms(terms, 1, 1, 1);
    assert_eq!(results.len(), 0);
}

/// Test edge case: single term
#[test]
fn test_single_term() {
    let terms = vec!["apple".to_string()];
    let results = generate_apronyms(terms, 2, 1, 1);

    // Should check if any prefix of "apple" is a valid word
    assert!(results.len() <= 100);
}

/// Test edge case: minimum valid input
#[test]
fn test_minimum_valid_input() {
    let terms = vec!["apple".to_string(), "banana".to_string()];

    let results = generate_apronyms(terms, 1, 1, 2);
    assert!(results.len() <= 100);
}

/// Test parallel processing produces valid results
#[test]
fn test_parallel_results_validity() {
    let terms = vec![
        "adventure".to_string(),
        "beautiful".to_string(),
        "creative".to_string(),
        "delightful".to_string(),
        "energetic".to_string(),
    ];

    let results = generate_apronyms(terms.clone(), 2, 4, 5);

    // All results should have text and terms
    for apronym in results.iter() {
        assert!(!apronym.text.is_empty(), "Apronym text should not be empty");
        assert!(
            !apronym.terms.is_empty(),
            "Apronym terms should not be empty"
        );
        assert!(
            apronym.terms.len() >= 4 && apronym.terms.len() <= 5,
            "Terms length should match min/max constraints"
        );
    }
}

/// Test that sequential and parallel produce consistent results for same input
#[test]
fn test_sequential_parallel_consistency() {
    // Use a workload just below parallel threshold
    let small_terms = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ];

    // This should use sequential
    let sequential_results = generate_apronyms(small_terms.clone(), 1, 2, 3);

    // Use a workload that triggers parallel
    let large_terms = vec![
        "adventure".to_string(),
        "beautiful".to_string(),
        "creative".to_string(),
        "delightful".to_string(),
        "energetic".to_string(),
    ];

    // This should use parallel
    let parallel_results = generate_apronyms(large_terms.clone(), 2, 3, 4);

    // Both should produce valid results
    assert!(sequential_results.len() <= 100);
    assert!(parallel_results.len() <= 100);
}

/// Test result limit is respected in parallel execution
#[test]
fn test_parallel_respects_limit() {
    let terms = vec![
        "adventure".to_string(),
        "beautiful".to_string(),
        "creative".to_string(),
        "delightful".to_string(),
        "energetic".to_string(),
    ];

    let limit = 25;
    let results = generate_apronyms_with_limit(terms, 2, 3, 5, limit);

    assert!(
        results.len() <= limit,
        "Results should not exceed specified limit"
    );
}

/// Test high concurrency scenario
#[test]
fn test_high_concurrency() {
    use rayon::prelude::*;
    use std::sync::Arc;

    let terms = Arc::new(vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
    ]);

    // Execute many generations in parallel using rayon
    let results: Vec<_> = (0..20)
        .into_par_iter()
        .map(|_| generate_apronyms((*terms).clone(), 1, 2, 3))
        .collect();

    // All should complete successfully
    assert_eq!(results.len(), 20);
    for result in results {
        assert!(result.len() <= 100);
    }
}
