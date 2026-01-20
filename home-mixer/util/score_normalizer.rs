//! Score normalization utilities

use crate::candidate_pipeline::candidate::PostCandidate;

/// Normalize a weighted score for a candidate
pub fn normalize_score(_candidate: &PostCandidate, score: f64) -> f64 {
    // Simple normalization - can be expanded with more sophisticated logic
    score.max(0.0)
}
