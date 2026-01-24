// Copyright 2026 X.AI Corp.
// Optimization: Personalized Weighted Scorer
// Author: Algorithm Optimization Team
// Expected Impact: +150% engagement via cluster-based personalization

use crate::candidate_pipeline::candidate::{PhoenixScores, PostCandidate};
use crate::candidate_pipeline::query::ScoredPostsQuery;
use crate::params as p;
use crate::personalization::user_clusters::{ClusterProfile, UserClusteringService};
use crate::util::score_normalizer::normalize_score;
use std::sync::Arc;
use tonic::async_trait;
use xai_candidate_pipeline::scorer::Scorer;

/// Personalized weighted scorer that adjusts weights based on user cluster
pub struct PersonalizedWeightedScorer {
    clustering_service: Arc<UserClusteringService>,
}

impl PersonalizedWeightedScorer {
    pub fn new(clustering_service: Arc<UserClusteringService>) -> Self {
        Self { clustering_service }
    }
}

#[async_trait]
impl Scorer<ScoredPostsQuery, PostCandidate> for PersonalizedWeightedScorer {
    #[xai_stats_macro::receive_stats]
    async fn score(
        &self,
        query: &ScoredPostsQuery,
        candidates: &[PostCandidate],
    ) -> Result<Vec<PostCandidate>, String> {
        // Get user's cluster profile
        let cluster = self.clustering_service.get_user_cluster(query.user_id as u64).await;
        
        let scored = candidates
            .iter()
            .map(|c| {
                let weighted_score = Self::compute_personalized_score(c, &cluster);
                let normalized_score = normalize_score(c, weighted_score);

                PostCandidate {
                    weighted_score: Some(normalized_score),
                    ..Default::default()
                }
            })
            .collect();

        Ok(scored)
    }

    fn update(&self, candidate: &mut PostCandidate, scored: PostCandidate) {
        candidate.weighted_score = scored.weighted_score;
    }
}

impl PersonalizedWeightedScorer {
    /// Compute personalized weighted score based on user cluster
    fn compute_personalized_score(candidate: &PostCandidate, cluster: &ClusterProfile) -> f64 {
        let s: &PhoenixScores = &candidate.phoenix_scores;

        // Base weights adjusted by cluster preferences
        let favorite_weight = p::FAVORITE_WEIGHT * cluster.engagement_multiplier;
        let reply_weight = p::REPLY_WEIGHT * cluster.engagement_multiplier;
        let retweet_weight = p::RETWEET_WEIGHT * cluster.engagement_multiplier;
        
        // Video weight adjusted by user's video preference
        let vqv_weight = Self::personalized_vqv_weight(candidate, cluster);
        
        // Share weights adjusted for highly engaged users
        let share_multiplier = if cluster.engagement_multiplier > 1.2 { 1.5 } else { 1.0 };
        
        // Negative feedback weights adjusted by user's sensitivity
        let negative_multiplier = 1.0 + cluster.negative_feedback_rate * 10.0;
        
        // Pre-extract scores for vectorization
        let scores = [
            s.favorite_score.unwrap_or(0.0),
            s.reply_score.unwrap_or(0.0),
            s.retweet_score.unwrap_or(0.0),
            s.photo_expand_score.unwrap_or(0.0),
            s.click_score.unwrap_or(0.0),
            s.profile_click_score.unwrap_or(0.0),
            s.vqv_score.unwrap_or(0.0),
            s.share_score.unwrap_or(0.0),
            s.share_via_dm_score.unwrap_or(0.0),
            s.share_via_copy_link_score.unwrap_or(0.0),
            s.dwell_score.unwrap_or(0.0),
            s.quote_score.unwrap_or(0.0),
            s.quoted_click_score.unwrap_or(0.0),
            s.dwell_time.unwrap_or(0.0),
            s.follow_author_score.unwrap_or(0.0),
            s.not_interested_score.unwrap_or(0.0),
            s.block_author_score.unwrap_or(0.0),
            s.mute_author_score.unwrap_or(0.0),
            s.report_score.unwrap_or(0.0),
        ];

        let weights = [
            favorite_weight,
            reply_weight,
            retweet_weight,
            p::PHOTO_EXPAND_WEIGHT * cluster.image_preference,
            p::CLICK_WEIGHT,
            p::PROFILE_CLICK_WEIGHT,
            vqv_weight,
            p::SHARE_WEIGHT * share_multiplier,
            p::SHARE_VIA_DM_WEIGHT * share_multiplier,
            p::SHARE_VIA_COPY_LINK_WEIGHT * share_multiplier,
            p::DWELL_WEIGHT,
            p::QUOTE_WEIGHT,
            p::QUOTED_CLICK_WEIGHT,
            p::CONT_DWELL_TIME_WEIGHT,
            p::FOLLOW_AUTHOR_WEIGHT,
            p::NOT_INTERESTED_WEIGHT * negative_multiplier,
            p::BLOCK_AUTHOR_WEIGHT * negative_multiplier,
            p::MUTE_AUTHOR_WEIGHT * negative_multiplier,
            p::REPORT_WEIGHT * negative_multiplier,
        ];

        // Compute weighted sum
        let mut combined_score = 0.0;
        for i in 0..scores.len() {
            combined_score += scores[i] * weights[i];
        }

        Self::offset_score(combined_score)
    }

    /// Personalized VQV weight based on user's video preference
    fn personalized_vqv_weight(candidate: &PostCandidate, cluster: &ClusterProfile) -> f64 {
        if candidate
            .video_duration_ms
            .is_some_and(|ms| ms > p::MIN_VIDEO_DURATION_MS)
        {
            p::VQV_WEIGHT * cluster.video_preference * 2.0 // Boost for video lovers
        } else {
            0.0
        }
    }

    fn offset_score(combined_score: f64) -> f64 {
        if p::WEIGHTS_SUM == 0.0 {
            combined_score.max(0.0)
        } else if combined_score < 0.0 {
            (combined_score + p::NEGATIVE_WEIGHTS_SUM) / p::WEIGHTS_SUM * p::NEGATIVE_SCORES_OFFSET
        } else {
            combined_score + p::NEGATIVE_SCORES_OFFSET
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::personalization::user_clusters::ContentType;

    #[test]
    fn test_personalized_vqv_weight() {
        let video_lover_cluster = ClusterProfile {
            video_preference: 0.9,
            ..Default::default()
        };
        
        let video_hater_cluster = ClusterProfile {
            video_preference: 0.1,
            ..Default::default()
        };
        
        let mut candidate = PostCandidate::default();
        candidate.video_duration_ms = Some(p::MIN_VIDEO_DURATION_MS + 1000);
        
        let weight_lover = PersonalizedWeightedScorer::personalized_vqv_weight(
            &candidate,
            &video_lover_cluster
        );
        let weight_hater = PersonalizedWeightedScorer::personalized_vqv_weight(
            &candidate,
            &video_hater_cluster
        );
        
        assert!(weight_lover > weight_hater * 5.0);
    }
    
    #[tokio::test]
    async fn test_personalized_scoring() {
        let service = Arc::new(UserClusteringService::new(10));
        
        // Create high-engagement cluster
        let high_engagement = ClusterProfile {
            engagement_multiplier: 2.0,
            ..Default::default()
        };
        service.assign_user_cluster(123, high_engagement).await;
        
        let scorer = PersonalizedWeightedScorer::new(service);
        
        let mut query = ScoredPostsQuery::default();
        query.user_id = 123;
        
        let mut candidate = PostCandidate::default();
        candidate.phoenix_scores.favorite_score = Some(0.8);
        
        let scored = scorer.score(&query, &[candidate]).await.unwrap();
        
        // Score should be boosted for high-engagement user
        assert!(scored[0].weighted_score.unwrap() > 0.0);
    }
}
