//! Candidate Source for ThunderRealtime
//!
//! Provides in-network post candidates for the home timeline.
//! These are posts from accounts the user follows.

use serde::{Deserialize, Serialize};

/// Post candidate from Thunder (in-network)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThunderCandidate {
    /// Post ID
    pub post_id: i64,
    /// Author ID  
    pub author_id: i64,
    /// Author handle
    pub author_handle: String,
    /// Post content
    pub content: String,
    /// Timestamp (Unix epoch seconds)
    pub created_at: u64,
    /// Has media (image/video)
    pub has_media: bool,
    /// Is a reply
    pub is_reply: bool,
    /// Reply to post ID (if is_reply)
    pub reply_to_id: Option<i64>,
    /// Has external link
    pub has_link: bool,
    /// Engagement metrics snapshot
    pub engagement: EngagementSnapshot,
}

/// Snapshot of engagement metrics at retrieval time
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EngagementSnapshot {
    pub likes: u32,
    pub replies: u32,
    pub reposts: u32,
    pub bookmarks: u32,
    pub views: u64,
}

impl ThunderCandidate {
    /// Create a new candidate
    pub fn new(post_id: i64, author_id: i64, content: String, created_at: u64) -> Self {
        Self {
            post_id,
            author_id,
            author_handle: String::new(),
            content,
            created_at,
            has_media: false,
            is_reply: false,
            reply_to_id: None,
            has_link: false,
            engagement: EngagementSnapshot::default(),
        }
    }

    /// Calculate age in seconds
    pub fn age_seconds(&self, now: u64) -> u64 {
        now.saturating_sub(self.created_at)
    }

    /// Check if post is within max age (7 days)
    pub fn is_fresh(&self, now: u64, max_age_seconds: u64) -> bool {
        self.age_seconds(now) < max_age_seconds
    }
}

/// Source of in-network candidates
pub trait CandidateSource: Send + Sync {
    /// Fetch candidates for a user's following list
    fn fetch_candidates(
        &self,
        user_id: i64,
        following_ids: &[i64],
        limit: usize,
    ) -> Vec<ThunderCandidate>;
}

/// In-memory candidate source for testing/development
#[derive(Default)]
pub struct InMemoryCandidateSource {
    posts: Vec<ThunderCandidate>,
}

impl InMemoryCandidateSource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_post(&mut self, post: ThunderCandidate) {
        self.posts.push(post);
    }
}

impl CandidateSource for InMemoryCandidateSource {
    fn fetch_candidates(
        &self,
        _user_id: i64,
        following_ids: &[i64],
        limit: usize,
    ) -> Vec<ThunderCandidate> {
        self.posts
            .iter()
            .filter(|p| following_ids.contains(&p.author_id))
            .take(limit)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidate_freshness() {
        let candidate = ThunderCandidate::new(1, 100, "Test post".into(), 1000);
        
        // Post is 100 seconds old
        let now = 1100;
        assert!(candidate.is_fresh(now, 200));  // Within 200s limit
        assert!(!candidate.is_fresh(now, 50)); // Outside 50s limit
    }

    #[test]
    fn test_in_memory_source() {
        let mut source = InMemoryCandidateSource::new();
        source.add_post(ThunderCandidate::new(1, 100, "Post 1".into(), 1000));
        source.add_post(ThunderCandidate::new(2, 200, "Post 2".into(), 1001));
        source.add_post(ThunderCandidate::new(3, 100, "Post 3".into(), 1002));

        let candidates = source.fetch_candidates(1, &[100], 10);
        assert_eq!(candidates.len(), 2);
    }
}
