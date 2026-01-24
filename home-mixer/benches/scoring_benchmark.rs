// Copyright 2026 X.AI Corp.
// Benchmarks for HomeMixer scoring performance
// Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use home_mixer::params;

/// Benchmark score calculation using raw weights
fn score_calculation_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Score Calculation");
    
    // Simulate different engagement levels
    for num_candidates in [10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("calculate_weighted_score", num_candidates),
            num_candidates,
            |b, &n| {
                // Prepare mock probability scores
                let candidates: Vec<(f64, f64, f64, f64, f64)> = (0..n)
                    .map(|i| {
                        let base = (i as f64) / (n as f64);
                        (
                            base * 0.1,        // favorite
                            base * 0.05,       // reply
                            base * 0.03,       // retweet
                            base * 0.08,       // profile_click
                            base * 0.02,       // vqv
                        )
                    })
                    .collect();
                
                b.iter(|| {
                    let scores: Vec<f64> = candidates.iter().map(|(fav, reply, rt, pc, vqv)| {
                        black_box(
                            fav * params::FAVORITE_WEIGHT +
                            reply * params::REPLY_WEIGHT +
                            rt * params::RETWEET_WEIGHT +
                            pc * params::PROFILE_CLICK_WEIGHT +
                            vqv * params::VQV_WEIGHT
                        )
                    }).collect();
                    scores
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark weight sum computation
fn weight_validation_benchmark(c: &mut Criterion) {
    c.bench_function("validate_weights_sum", |b| {
        b.iter(|| {
            black_box(params::WEIGHTS_SUM);
            black_box(params::NEGATIVE_WEIGHTS_SUM);
        });
    });
}

/// Benchmark freshness decay calculation
fn freshness_decay_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Freshness Decay");
    
    let decay_hours = params::FRESHNESS_DECAY_HOURS;
    
    for age_hours in [1.0, 6.0, 12.0, 24.0, 48.0].iter() {
        group.bench_with_input(
            BenchmarkId::new("calculate_decay", age_hours),
            age_hours,
            |b, &age| {
                b.iter(|| {
                    // Exponential decay formula
                    black_box(0.5f64.powf(age / decay_hours))
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark author diversity decay
fn author_diversity_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Author Diversity");
    
    let decay = params::AUTHOR_DIVERSITY_DECAY;
    
    for num_posts in [1, 2, 3, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::new("diversity_penalty", num_posts),
            num_posts,
            |b, &n| {
                b.iter(|| {
                    black_box(decay.powi(n - 1))
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    score_calculation_benchmark,
    weight_validation_benchmark,
    freshness_decay_benchmark,
    author_diversity_benchmark
);

criterion_main!(benches);
