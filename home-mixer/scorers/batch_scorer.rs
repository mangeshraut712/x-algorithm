//! High-Performance Batch Scorer
//!
//! Optimized scoring implementation using batch processing and cache-friendly
//! data layouts for maximum throughput on large candidate sets.

use crate::params;

/// Batch score result
#[derive(Debug, Clone)]
pub struct BatchScoreResult {
    pub scores: Vec<f64>,
    pub processing_time_us: u64,
}

/// High-performance batch scorer using vectorized operations
pub struct BatchScorer {
    /// Pre-computed weight array for cache-friendly access
    weights: [f64; 16],
}

impl Default for BatchScorer {
    fn default() -> Self {
        Self::new()
    }
}

impl BatchScorer {
    pub fn new() -> Self {
        // Pack weights into contiguous array for cache efficiency
        Self {
            weights: [
                params::FAVORITE_WEIGHT,       // 0
                params::REPLY_WEIGHT,          // 1
                params::RETWEET_WEIGHT,        // 2
                params::PHOTO_EXPAND_WEIGHT,   // 3
                params::CLICK_WEIGHT,          // 4
                params::PROFILE_CLICK_WEIGHT,  // 5
                params::VQV_WEIGHT,            // 6
                params::SHARE_WEIGHT,          // 7
                params::SHARE_VIA_DM_WEIGHT,   // 8
                params::SHARE_VIA_COPY_LINK_WEIGHT, // 9
                params::DWELL_WEIGHT,          // 10
                params::QUOTE_WEIGHT,          // 11
                params::QUOTED_CLICK_WEIGHT,   // 12
                params::CONT_DWELL_TIME_WEIGHT, // 13
                params::FOLLOW_AUTHOR_WEIGHT,  // 14
                params::BOOKMARK_WEIGHT,       // 15
            ],
        }
    }

    /// Score a batch of candidates efficiently
    /// 
    /// Takes probability scores as a flattened array where each candidate
    /// has 16 probability values in the same order as weights.
    /// 
    /// # Arguments
    /// * `probabilities` - Flattened array of probabilities (len = num_candidates * 16)
    /// * `num_candidates` - Number of candidates to score
    /// 
    /// # Returns
    /// Vector of scores, one per candidate
    #[inline]
    pub fn score_batch(&self, probabilities: &[f64], num_candidates: usize) -> BatchScoreResult {
        let start = std::time::Instant::now();
        
        debug_assert_eq!(probabilities.len(), num_candidates * 16);
        
        let mut scores = Vec::with_capacity(num_candidates);
        
        // Process in chunks for cache locality
        for chunk_start in (0..num_candidates).step_by(8) {
            let chunk_end = (chunk_start + 8).min(num_candidates);
            
            for i in chunk_start..chunk_end {
                let base = i * 16;
                let score = self.score_single_candidate(&probabilities[base..base + 16]);
                scores.push(score);
            }
        }
        
        BatchScoreResult {
            scores,
            processing_time_us: start.elapsed().as_micros() as u64,
        }
    }

    /// Score a single candidate from its probability array
    #[inline(always)]
    fn score_single_candidate(&self, probs: &[f64]) -> f64 {
        // Unrolled loop for maximum performance
        // Compiler can vectorize this with appropriate target features
        probs[0] * self.weights[0]
            + probs[1] * self.weights[1]
            + probs[2] * self.weights[2]
            + probs[3] * self.weights[3]
            + probs[4] * self.weights[4]
            + probs[5] * self.weights[5]
            + probs[6] * self.weights[6]
            + probs[7] * self.weights[7]
            + probs[8] * self.weights[8]
            + probs[9] * self.weights[9]
            + probs[10] * self.weights[10]
            + probs[11] * self.weights[11]
            + probs[12] * self.weights[12]
            + probs[13] * self.weights[13]
            + probs[14] * self.weights[14]
            + probs[15] * self.weights[15]
    }

    /// Score with freshness decay applied
    #[inline]
    pub fn score_with_freshness(&self, base_score: f64, age_hours: f64) -> f64 {
        let decay = 0.5f64.powf(age_hours / params::FRESHNESS_DECAY_HOURS);
        base_score * decay
    }

    /// Apply author diversity penalty
    #[inline]
    pub fn apply_diversity_penalty(&self, score: f64, author_post_count: u32) -> f64 {
        if author_post_count <= 1 {
            score
        } else {
            score * params::AUTHOR_DIVERSITY_DECAY.powi(author_post_count as i32 - 1)
        }
    }

    /// Get total positive weights sum (for normalization)
    #[inline]
    pub fn positive_weights_sum(&self) -> f64 {
        params::WEIGHTS_SUM
    }
}

/// Optimized top-K selection using partial sort
pub fn select_top_k<T, F>(items: &mut [T], k: usize, score_fn: F) -> &[T]
where
    F: Fn(&T) -> f64,
{
    if k >= items.len() {
        return items;
    }

    // Use partial sort for O(n + k log k) complexity instead of O(n log n)
    items.select_nth_unstable_by(k, |a, b| {
        score_fn(b).partial_cmp(&score_fn(a)).unwrap_or(std::cmp::Ordering::Equal)
    });

    &items[..k]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_scoring() {
        let scorer = BatchScorer::new();
        
        // Create 100 candidates with random probabilities
        let num_candidates = 100;
        let probabilities: Vec<f64> = (0..num_candidates * 16)
            .map(|i| (i as f64 % 100.0) / 100.0)
            .collect();
        
        let result = scorer.score_batch(&probabilities, num_candidates);
        
        assert_eq!(result.scores.len(), num_candidates);
        assert!(result.scores.iter().all(|&s| s >= 0.0));
    }

    #[test]
    fn test_freshness_decay() {
        let scorer = BatchScorer::new();
        
        let base_score = 100.0;
        
        // At 0 hours, no decay
        let fresh = scorer.score_with_freshness(base_score, 0.0);
        assert!((fresh - 100.0).abs() < 0.01);
        
        // At half-life (6 hours), score should be 50%
        let half = scorer.score_with_freshness(base_score, 6.0);
        assert!((half - 50.0).abs() < 0.01);
        
        // At 12 hours, score should be 25%
        let quarter = scorer.score_with_freshness(base_score, 12.0);
        assert!((quarter - 25.0).abs() < 0.01);
    }

    #[test]
    fn test_diversity_penalty() {
        let scorer = BatchScorer::new();
        
        let base_score = 100.0;
        
        // First post: no penalty
        assert_eq!(scorer.apply_diversity_penalty(base_score, 1), 100.0);
        
        // Second post: 0.8x
        let second = scorer.apply_diversity_penalty(base_score, 2);
        assert!((second - 80.0).abs() < 0.01);
        
        // Third post: 0.64x (0.8^2)
        let third = scorer.apply_diversity_penalty(base_score, 3);
        assert!((third - 64.0).abs() < 0.01);
    }

    #[test]
    fn test_top_k_selection() {
        let mut items: Vec<i32> = (0..100).collect();
        let top_10 = select_top_k(&mut items, 10, |&x| x as f64);
        
        assert_eq!(top_10.len(), 10);
        // Top 10 should contain 99, 98, 97, ..., 90
        for &item in top_10 {
            assert!(item >= 90);
        }
    }
}
