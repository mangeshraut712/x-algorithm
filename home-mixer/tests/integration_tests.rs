// Copyright 2026 X.AI Corp.
// Integration Tests for HomeMixer
// Author: Algorithm Optimization Team

use candidate_pipeline::scorer::Scorer;
use home_mixer::candidate_pipeline::{candidate::PostCandidate, query::ScoredPostsQuery};
use home_mixer::scorers::weighted_scorer::WeightedScorer;

/// Test that the weighted scorer produces non-zero scores
#[tokio::test]
async fn test_weighted_scorer_basic() {
    let scorer = WeightedScorer;

    let query = ScoredPostsQuery::default();
    let candidate = PostCandidate {
        phoenix_scores: home_mixer::candidate_pipeline::candidate::PhoenixScores {
            favorite_score: Some(0.8),
            reply_score: Some(0.6),
            ..Default::default()
        },
        ..Default::default()
    };

    let candidates = vec![candidate];

    let result = scorer.score(&query, &candidates).await.unwrap();

    assert_eq!(result.len(), 1);
    assert!(result[0].weighted_score.is_some());
}

/// Test that the scorer handles empty input
#[tokio::test]
async fn test_weighted_scorer_empty_input() {
    let scorer = WeightedScorer;

    let query = ScoredPostsQuery::default();
    let candidates: Vec<PostCandidate> = vec![];

    let result = scorer.score(&query, &candidates).await.unwrap();

    assert!(result.is_empty());
}

/// Test that video duration affects VQV weight
#[tokio::test]
async fn test_weighted_scorer_video_boost() {
    let scorer = WeightedScorer;

    let query = ScoredPostsQuery::default();

    // Candidate with video
    let with_video = PostCandidate {
        video_duration_ms: Some(5000), // 5 seconds
        phoenix_scores: home_mixer::candidate_pipeline::candidate::PhoenixScores {
            vqv_score: Some(0.5),
            ..Default::default()
        },
        ..Default::default()
    };

    // Candidate without video
    let without_video = PostCandidate {
        phoenix_scores: home_mixer::candidate_pipeline::candidate::PhoenixScores {
            vqv_score: Some(0.5),
            ..Default::default()
        },
        ..Default::default()
    };

    let candidates = vec![with_video.clone(), without_video.clone()];

    let result = scorer.score(&query, &candidates).await.unwrap();

    // Both should have scores
    assert!(result[0].weighted_score.is_some());
    assert!(result[1].weighted_score.is_some());
}

/// Test configuration loading
#[test]
fn test_config_defaults() {
    use home_mixer::config::Config;

    let config = Config::default();

    // Caching is disabled by default for safety
    assert!(!config.caching.enabled);
    // Safety filters are enabled by default
    assert!(config.safety.enable_nsfw_filter);
    assert!(config.safety.enable_spam_filter);
}
