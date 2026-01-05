/// Detailed timing analysis to identify bottlenecks
///
/// Run with: cargo run --release --example timing_analysis
use apronymer::generate_apronyms_with_limit;
use std::time::Instant;

fn main() {
    println!("⏱️  Detailed Timing Analysis\n");

    // Test 1: Dictionary loading impact (first call)
    println!("Test 1: First generation (includes dictionary load)");
    let terms = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ];
    let start = Instant::now();
    let _ = generate_apronyms_with_limit(terms.clone(), 1, 2, 3, 10);
    println!("  Time: {:?}\n", start.elapsed());

    // Test 2: Sequential vs Parallel comparison
    println!("Test 2: Sequential threshold (4 terms, should be sequential)");
    let seq_terms = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
    ];

    let mut seq_times = Vec::new();
    for i in 0..10 {
        let start = Instant::now();
        let results = generate_apronyms_with_limit(seq_terms.clone(), 2, 3, 4, 50);
        let duration = start.elapsed();
        seq_times.push(duration);
        if i == 0 {
            println!(
                "  Run {}: {:?} ({} results)",
                i + 1,
                duration,
                results.len()
            );
        }
    }
    let avg_seq = seq_times.iter().sum::<std::time::Duration>() / seq_times.len() as u32;
    println!("  Average over 10 runs: {:?}\n", avg_seq);

    // Test 3: Parallel threshold (5 terms, should trigger parallel)
    println!("Test 3: Parallel threshold (5 terms, should be parallel)");
    let par_terms = vec![
        "adventure".to_string(),
        "beautiful".to_string(),
        "creative".to_string(),
        "delightful".to_string(),
        "energetic".to_string(),
    ];

    let mut par_times = Vec::new();
    for i in 0..10 {
        let start = Instant::now();
        let results = generate_apronyms_with_limit(par_terms.clone(), 2, 3, 4, 50);
        let duration = start.elapsed();
        par_times.push(duration);
        if i == 0 {
            println!(
                "  Run {}: {:?} ({} results)",
                i + 1,
                duration,
                results.len()
            );
        }
    }
    let avg_par = par_times.iter().sum::<std::time::Duration>() / par_times.len() as u32;
    println!("  Average over 10 runs: {:?}\n", avg_par);

    // Test 4: Large workload with fragments
    println!("Test 4: Large workload (6 terms, frag=2, complex)");
    let large_terms = vec![
        "adventure".to_string(),
        "beautiful".to_string(),
        "creative".to_string(),
        "delightful".to_string(),
        "energetic".to_string(),
        "fantastic".to_string(),
    ];

    let mut large_times = Vec::new();
    for i in 0..5 {
        let start = Instant::now();
        let results = generate_apronyms_with_limit(large_terms.clone(), 2, 4, 6, 100);
        let duration = start.elapsed();
        large_times.push(duration);
        if i == 0 {
            println!(
                "  Run {}: {:?} ({} results)",
                i + 1,
                duration,
                results.len()
            );
        }
    }
    let avg_large = large_times.iter().sum::<std::time::Duration>() / large_times.len() as u32;
    println!("  Average over 5 runs: {:?}\n", avg_large);

    // Analysis
    println!("📊 Analysis:");
    println!("  Sequential (4 terms):  {:?}", avg_seq);
    println!("  Parallel (5 terms):    {:?}", avg_par);
    println!(
        "  Speedup ratio:         {:.2}x {}",
        avg_seq.as_secs_f64() / avg_par.as_secs_f64(),
        if avg_par < avg_seq {
            "FASTER"
        } else {
            "SLOWER (parallel overhead)"
        }
    );
    println!("  Large workload:        {:?}", avg_large);
}
