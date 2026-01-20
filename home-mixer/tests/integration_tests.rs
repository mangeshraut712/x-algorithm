// Copyright 2026 X.AI Corp.
// Integration Tests for Optimized Phoenix Scoring
// Author: Algorithm Optimization Team

use home_mixer::scorers::{
    cached_phoenix_scorer::{CachedPhoenixScorer, CacheConfig},
    batched_phoenix_scorer::{BatchedPhoenixScorer, BatchConfig},
    phoenix_scorer::PhoenixScorer,
};
use home_mixer::candidate_pipeline::{
    candidate::PostCandidate,
    query::ScoredPostsQuery,
};
use std::sync::Arc;
use std::time::Instant;
use xai_candidate_pipeline::scorer::Scorer;

#[tokio::test]
async fn test_caching_reduces_ml_calls() {
    // Setup: Create mock Phoenix scorer that counts calls
    let call_counter = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let counter_clone = call_counter.clone();
    
    let mock_scorer = Arc::new(MockPhoenixScorer::new(counter_clone));
    let cached_scorer = CachedPhoenixScorer::new(
        mock_scorer.clone(),
        CacheConfig::default(),
    );
    
    let query = create_test_query(user_id: 12345);
    let candidates = create_test_candidates(10);
    
    // First call - should hit ML
    let _result1 = cached_scorer.score(&query, &candidates).await.unwrap();
    assert_eq!(call_counter.load(std::sync::atomic::Ordering::Relaxed), 1);
    
    // Second call with same tweets - should hit cache
    let _result2 = cached_scorer.score(&query, &candidates).await.unwrap();
    assert_eq!(call_counter.load(std::sync::atomic::Ordering::Relaxed), 1);
    
    // Verify cache hit rate
    let stats = cached_scorer.cache_stats();
    assert!(stats.hit_rate > 0.5, "Cache hit rate should be > 50%");
}

#[tokio::test]
async fn test_batching_improves_throughput() {
    let scorer = Arc::new(create_test_phoenix_scorer());
    
    let batch_config = BatchConfig {
        max_batch_size: 64,
        max_wait_time: std::time::Duration::from_millis(10),
        max_concurrent_batches: 4,
    };
    
    let batched_scorer = Arc::new(BatchedPhoenixScorer::new(
        scorer.clone(),
        batch_config,
    ));
    
    // Spawn multiple concurrent requests
    let mut handles = vec![];
    for user_id in 0..100 {
        let scorer_clone = batched_scorer.clone();
        let handle = tokio::spawn(async move {
            let query = create_test_query(user_id);
            let candidates = create_test_candidates(10);
            scorer_clone.score(&query, &candidates).await
        });
        handles.push(handle);
    }
    
    // Wait for all to complete
    let start = Instant::now();
    for handle in handles {
        handle.await.unwrap().unwrap();
    }
    let elapsed = start.elapsed();
    
    // Batching should process 100 requests faster than sequential
    // With batching: ~100ms (assuming 10ms per batch, 10 batches)
    // Without batching: ~1000ms (assuming 10ms per request, 100 requests)
    assert!(elapsed.as_millis() < 500, "Batching should be fast");
    
    // Check batch stats
    let stats = batched_scorer.get_stats().await;
    assert!(stats.avg_batch_size > 5.0, "Should batch multiple requests");
}

#[tokio::test]
async fn test_cache_and_batch_together() {
    // Test the full optimization stack
    let scorer = Arc::new(create_test_phoenix_scorer());
    
    // Wrap with caching
    let cached = Arc::new(CachedPhoenixScorer::new(
        scorer,
        CacheConfig::default(),
    ));
    
    // Wrap with batching
    let batched_cached = Arc::new(BatchedPhoenixScorer::new(
        cached.clone(),
        BatchConfig::default(),
    ));
    
    // First batch of requests
    let mut handles = vec![];
    for _ in 0..50 {
        let scorer_clone = batched_cached.clone();
        let handle = tokio::spawn(async move {
            let query = create_test_query(12345);
            let candidates = create_test_candidates(10);
            scorer_clone.score(&query, &candidates).await
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap().unwrap();
    }
    
    // Second batch - should benefit from cache
    let mut handles = vec![];
    for _ in 0..50 {
        let scorer_clone = batched_cached.clone();
        let handle = tokio::spawn(async move {
            let query = create_test_query(12345);
            let candidates = create_test_candidates(10);
            scorer_clone.score(&query, &candidates).await
        });
        handles.push(handle);
    }
    
    let start = Instant::now();
    for handle in handles {
        handle.await.unwrap().unwrap();
    }
    let second_batch_time = start.elapsed();
    
    // Second batch should be significantly faster due to caching
    assert!(second_batch_time.as_millis() < 100, "Cached batch should be very fast");
}

#[tokio::test]
async fn test_optimized_pipeline_end_to_end() {
    // Full pipeline with all optimizations
    let pipeline = create_optimized_test_pipeline();
    
    let query = create_test_query(12345);
    
    let start = Instant::now();
    let result = pipeline.execute(query).await;
    let elapsed = start.elapsed();
    
    // Check results
    assert!(result.selected_candidates.len() > 0, "Should have candidates");
    assert!(elapsed.as_millis() < 200, "Optimized pipeline should be fast");
}

#[test]
fn test_weighted_scorer_optimization() {
    use home_mixer::scorers::weighted_scorer::WeightedScorer;
    
    let mut candidate = PostCandidate::default();
    candidate.phoenix_scores.favorite_score = Some(0.8);
    candidate.phoenix_scores.reply_score = Some(0.6);
    candidate.phoenix_scores.retweet_score = Some(0.7);
    
    let start = Instant::now();
    for _ in 0..10000 {
        let _score = WeightedScorer::compute_weighted_score(&candidate);
    }
    let elapsed = start.elapsed();
    
    // Optimized version should compute 10k scores in < 10ms
    assert!(elapsed.as_millis() < 10, "Optimized scoring should be very fast");
}

// Helper functions

fn create_test_query(user_id: u64) -> ScoredPostsQuery {
    ScoredPostsQuery {
        user_id,
        request_id: format!("test_{}", user_id),
        user_action_sequence: Some(vec![]),
        in_network_only: false,
        ..Default::default()
    }
}

fn create_test_candidates(count: usize) -> Vec<PostCandidate> {
    (0..count)
        .map(|i| PostCandidate {
            tweet_id: i as i64,
            author_id: (i * 10) as u64,
            ..Default::default()
        })
        .collect()
}

fn create_test_phoenix_scorer() -> PhoenixScorer {
    // Create mock or test Phoenix scorer
    // In real tests, you'd use dependency injection
    todo!("Implement test Phoenix scorer")
}

fn create_optimized_test_pipeline() -> impl CandidatePipeline<ScoredPostsQuery, PostCandidate> {
    // Create full pipeline with optimizations
    todo!("Implement test pipeline")
}

struct MockPhoenixScorer {
    call_counter: Arc<std::sync::atomic::AtomicU64>,
}

impl MockPhoenixScorer {
    fn new(counter: Arc<std::sync::atomic::AtomicU64>) -> Self {
        Self { call_counter: counter }
    }
}

#[async_trait::async_trait]
impl Scorer<ScoredPostsQuery, PostCandidate> for MockPhoenixScorer {
    async fn score(
        &self,
        _query: &ScoredPostsQuery,
        candidates: &[PostCandidate],
    ) -> Result<Vec<PostCandidate>, String> {
        self.call_counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        // Simulate ML inference delay
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        
        // Return candidates with dummy scores
        Ok(candidates.to_vec())
    }
    
    fn update(&self, _candidate: &mut PostCandidate, _scored: PostCandidate) {}
}
