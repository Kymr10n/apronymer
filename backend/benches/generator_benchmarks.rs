use apronymer::{generate_apronyms, generate_apronyms_with_limit};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

/// Benchmark small workloads (sequential processing expected)
fn bench_small_workload(c: &mut Criterion) {
    let mut group = c.benchmark_group("small_workload");

    let small_terms = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ];

    group.bench_function("3_terms_frag1_minmax2-3", |b| {
        b.iter(|| {
            generate_apronyms_with_limit(
                black_box(small_terms.clone()),
                black_box(1),
                black_box(2),
                black_box(3),
                black_box(50),
            )
        })
    });

    group.finish();
}

/// Benchmark large workloads (parallel processing expected)
fn bench_large_workload(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_workload");

    let large_terms = vec![
        "adventure".to_string(),
        "beautiful".to_string(),
        "creative".to_string(),
        "delightful".to_string(),
        "energetic".to_string(),
        "fantastic".to_string(),
    ];

    group.bench_function("6_terms_frag2_minmax4-6", |b| {
        b.iter(|| {
            generate_apronyms_with_limit(
                black_box(large_terms.clone()),
                black_box(2),
                black_box(4),
                black_box(6),
                black_box(50),
            )
        })
    });

    group.finish();
}

/// Benchmark parallel vs sequential threshold boundary
fn bench_threshold_boundary(c: &mut Criterion) {
    let mut group = c.benchmark_group("threshold_boundary");

    // Just below parallel threshold (should use sequential)
    let terms_below = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
    ];

    // Just above parallel threshold (should use parallel)
    let terms_above = vec![
        "adventure".to_string(),
        "beautiful".to_string(),
        "creative".to_string(),
        "delightful".to_string(),
        "energetic".to_string(),
    ];

    group.bench_function("below_threshold_4_terms", |b| {
        b.iter(|| {
            generate_apronyms(
                black_box(terms_below.clone()),
                black_box(2),
                black_box(3),
                black_box(4),
            )
        })
    });

    group.bench_function("above_threshold_5_terms", |b| {
        b.iter(|| {
            generate_apronyms(
                black_box(terms_above.clone()),
                black_box(2),
                black_box(3),
                black_box(4),
            )
        })
    });

    group.finish();
}

/// Benchmark early termination effectiveness
fn bench_early_termination(c: &mut Criterion) {
    let mut group = c.benchmark_group("early_termination");

    let terms = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
    ];

    for limit in [10, 50, 100, 500].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(limit), limit, |b, &limit| {
            b.iter(|| {
                generate_apronyms_with_limit(
                    black_box(terms.clone()),
                    black_box(2),
                    black_box(3),
                    black_box(4),
                    black_box(limit),
                )
            })
        });
    }

    group.finish();
}

/// Benchmark different fragment lengths
fn bench_fragment_lengths(c: &mut Criterion) {
    let mut group = c.benchmark_group("fragment_lengths");

    let terms = vec![
        "application".to_string(),
        "beautiful".to_string(),
        "creative".to_string(),
        "delightful".to_string(),
    ];

    for frag_len in [1, 2, 3].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(frag_len),
            frag_len,
            |b, &frag_len| {
                b.iter(|| {
                    generate_apronyms_with_limit(
                        black_box(terms.clone()),
                        black_box(frag_len),
                        black_box(3),
                        black_box(4),
                        black_box(100),
                    )
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_small_workload,
    bench_large_workload,
    bench_threshold_boundary,
    bench_early_termination,
    bench_fragment_lengths
);
criterion_main!(benches);
