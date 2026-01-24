//! Realtime Query for Thunder
//!
//! Provides realtime query capabilities for fetching recent posts
//! from followed accounts with freshness filtering.

use crate::candidate_source::{CandidateSource, ThunderCandidate};
use crate::config::ThunderConfig;

/// Query parameters for fetching in-network posts
#[derive(Clone, Debug)]
pub struct RealtimeQuery {
    /// User requesting the feed
    pub user_id: i64,
    /// List of followed user IDs
    pub following_ids: Vec<i64>,
    /// Maximum posts to return
    pub limit: usize,
    /// Maximum post age in seconds (default: 7 days)
    pub max_age_seconds: u64,
    /// Exclude posts already seen (IDs)
    pub exclude_post_ids: Vec<i64>,
}

impl RealtimeQuery {
    /// Create a new query with defaults
    pub fn new(user_id: i64, following_ids: Vec<i64>) -> Self {
        Self {
            user_id,
            following_ids,
            limit: 100,
            max_age_seconds: 7 * 24 * 60 * 60, // 7 days
            exclude_post_ids: Vec::new(),
        }
    }

    /// Set the limit
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Set max age
    pub fn with_max_age(mut self, seconds: u64) -> Self {
        self.max_age_seconds = seconds;
        self
    }

    /// Add exclusions
    pub fn exclude(mut self, post_ids: Vec<i64>) -> Self {
        self.exclude_post_ids = post_ids;
        self
    }
}

/// Response from a realtime query
#[derive(Clone, Debug)]
pub struct RealtimeQueryResponse {
    /// The candidates matching the query
    pub candidates: Vec<ThunderCandidate>,
    /// Total candidates available (before limit)
    pub total_available: usize,
    /// Query execution time in ms
    pub query_time_ms: u64,
}

/// Execute a realtime query against the candidate source
pub fn execute_query<S: CandidateSource>(
    source: &S,
    query: &RealtimeQuery,
    _config: &ThunderConfig,
) -> RealtimeQueryResponse {
    let start = std::time::Instant::now();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Fetch candidates from source
    let all_candidates = source.fetch_candidates(
        query.user_id,
        &query.following_ids,
        query.limit * 2, // Fetch extra to allow for filtering
    );

    // Filter by freshness and exclusions
    let filtered: Vec<_> = all_candidates
        .into_iter()
        .filter(|c| c.is_fresh(now, query.max_age_seconds))
        .filter(|c| !query.exclude_post_ids.contains(&c.post_id))
        .collect();

    let total = filtered.len();
    let candidates: Vec<_> = filtered.into_iter().take(query.limit).collect();

    RealtimeQueryResponse {
        candidates,
        total_available: total,
        query_time_ms: start.elapsed().as_millis() as u64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::candidate_source::InMemoryCandidateSource;

    #[test]
    fn test_realtime_query() {
        let mut source = InMemoryCandidateSource::new();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Add some recent posts
        source.add_post(ThunderCandidate::new(1, 100, "Fresh post".into(), now - 100));
        source.add_post(ThunderCandidate::new(2, 100, "Also fresh".into(), now - 200));

        // Add an old post (should be filtered)
        source.add_post(ThunderCandidate::new(3, 100, "Old post".into(), now - 8 * 24 * 60 * 60));

        let query = RealtimeQuery::new(1, vec![100]);
        let config = ThunderConfig::default();
        let response = execute_query(&source, &query, &config);

        assert_eq!(response.candidates.len(), 2); // Only fresh posts
    }

    #[test]
    fn test_query_exclusions() {
        let mut source = InMemoryCandidateSource::new();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        source.add_post(ThunderCandidate::new(1, 100, "Post 1".into(), now - 100));
        source.add_post(ThunderCandidate::new(2, 100, "Post 2".into(), now - 200));
        source.add_post(ThunderCandidate::new(3, 100, "Post 3".into(), now - 300));

        let query = RealtimeQuery::new(1, vec![100]).exclude(vec![2]);
        let config = ThunderConfig::default();
        let response = execute_query(&source, &query, &config);

        assert_eq!(response.candidates.len(), 2);
        assert!(!response.candidates.iter().any(|c| c.post_id == 2));
    }
}
