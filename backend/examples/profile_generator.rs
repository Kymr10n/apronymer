/// Profiling harness for generator performance analysis
///
/// Run with: cargo run --release --example profile_generator
/// For flamegraph: cargo flamegraph --example profile_generator
use apronymer::{generate_apronyms, generate_apronyms_with_limit};
use std::time::Instant;

fn main() {
    println!("🔍 Profiling Apronym Generator\n");

    // Profile 1: Small workload (sequential expected)
    println!("📊 Profile 1: Small workload (3 terms)");
    let small_terms = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ];

    let start = Instant::now();
    let small_results = generate_apronyms(small_terms.clone(), 1, 2, 3);
    let small_duration = start.elapsed();
    println!(
        "   Results: {} apronyms in {:?}\n",
        small_results.len(),
        small_duration
    );

    // Profile 2: Medium workload (boundary case)
    println!("📊 Profile 2: Medium workload (5 terms) - threshold boundary");
    let medium_terms = vec![
        "adventure".to_string(),
        "beautiful".to_string(),
        "creative".to_string(),
        "delightful".to_string(),
        "energetic".to_string(),
    ];

    let start = Instant::now();
    let medium_results = generate_apronyms(medium_terms.clone(), 2, 3, 4);
    let medium_duration = start.elapsed();
    println!(
        "   Results: {} apronyms in {:?}\n",
        medium_results.len(),
        medium_duration
    );

    // Profile 3: Large workload (parallel expected)
    println!("📊 Profile 3: Large workload (6 terms) - parallel processing");
    let large_terms = vec![
        "adventure".to_string(),
        "beautiful".to_string(),
        "creative".to_string(),
        "delightful".to_string(),
        "energetic".to_string(),
        "fantastic".to_string(),
    ];

    let start = Instant::now();
    let large_results = generate_apronyms(large_terms.clone(), 2, 4, 6);
    let large_duration = start.elapsed();
    println!(
        "   Results: {} apronyms in {:?}\n",
        large_results.len(),
        large_duration
    );

    // Profile 4: Test early termination efficiency
    println!("📊 Profile 4: Early termination (limit=10 vs limit=100)");
    let test_terms = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
    ];

    let start = Instant::now();
    let limit_10 = generate_apronyms_with_limit(test_terms.clone(), 2, 3, 4, 10);
    let limit_10_duration = start.elapsed();

    let start = Instant::now();
    let limit_100 = generate_apronyms_with_limit(test_terms.clone(), 2, 3, 4, 100);
    let limit_100_duration = start.elapsed();

    println!(
        "   Limit 10:  {} apronyms in {:?}",
        limit_10.len(),
        limit_10_duration
    );
    println!(
        "   Limit 100: {} apronyms in {:?}\n",
        limit_100.len(),
        limit_100_duration
    );

    // Profile 5: Stress test - many iterations to amplify hotspots
    println!("📊 Profile 5: Stress test (100 iterations) - for profiler sampling");
    let stress_terms = vec![
        "alpha".to_string(),
        "beta".to_string(),
        "gamma".to_string(),
        "delta".to_string(),
    ];

    let start = Instant::now();
    let mut total_results = 0;
    for _ in 0..100 {
        let results = generate_apronyms(stress_terms.clone(), 2, 3, 4);
        total_results += results.len();
    }
    let stress_duration = start.elapsed();
    println!(
        "   Total: {} apronyms in {:?} ({:.2}ms per iteration)\n",
        total_results,
        stress_duration,
        stress_duration.as_secs_f64() * 10.0
    );

    println!("✅ Profiling complete!");
    println!("\n💡 Tips:");
    println!("   - Run with 'cargo flamegraph --example profile_generator' for flamegraph");
    println!("   - Run with 'perf record' and 'perf report' for detailed analysis");
    println!("   - Check target/criterion for detailed benchmark HTML reports");
}
