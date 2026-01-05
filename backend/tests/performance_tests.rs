use apronymer::{generate_apronyms, generate_apronyms_with_limit};
use std::time::Instant;

#[test]
fn test_performance_comparison() {
    let terms = vec![
        "Alpha".to_string(),
        "Echo".to_string(),
        "India".to_string(),
        "Oscar".to_string(),
        "Uniform".to_string(),
    ];

    // Test with limit (optimized)
    let start = Instant::now();
    let limited_results = generate_apronyms_with_limit(terms.clone(), 2, 3, 4, 20);
    let limited_duration = start.elapsed();

    // Test without limit (legacy behavior)
    let start = Instant::now();
    let unlimited_results = generate_apronyms(terms.clone(), 2, 3, 4);
    let unlimited_duration = start.elapsed();

    println!(
        "Limited results (20 max): {} apronyms in {:?}",
        limited_results.len(),
        limited_duration
    );
    println!(
        "Unlimited results: {} apronyms in {:?}",
        unlimited_results.len(),
        unlimited_duration
    );

    // The limited version should be faster when there are many results
    assert!(
        limited_results.len() <= 20,
        "Limited results should not exceed the limit"
    );
    assert!(
        !limited_results.is_empty(),
        "Should find at least some results"
    );
}

#[test]
fn test_early_termination_effectiveness() {
    let terms = vec![
        "Alpha".to_string(),
        "Echo".to_string(),
        "India".to_string(),
        "Oscar".to_string(),
    ];

    // Test with very small limit
    let start = Instant::now();
    let small_limit_results = generate_apronyms_with_limit(terms.clone(), 1, 3, 4, 5);
    let small_limit_duration = start.elapsed();

    // Test with larger limit
    let start = Instant::now();
    let large_limit_results = generate_apronyms_with_limit(terms.clone(), 1, 3, 4, 50);
    let large_limit_duration = start.elapsed();

    println!(
        "Small limit (5): {} apronyms in {:?}",
        small_limit_results.len(),
        small_limit_duration
    );
    println!(
        "Large limit (50): {} apronyms in {:?}",
        large_limit_results.len(),
        large_limit_duration
    );

    assert!(
        small_limit_results.len() <= 5,
        "Small limit should be respected"
    );
    assert!(
        large_limit_results.len() <= 50,
        "Large limit should be respected"
    );

    // With early termination, small limit should be significantly faster
    if large_limit_results.len() > 20 {
        assert!(
            small_limit_duration <= large_limit_duration,
            "Small limit should be faster or equal"
        );
    }
}
