//! Phoenix Candidate Pipeline
//!
//! This is the main pipeline that orchestrates candidate retrieval, filtering, and scoring.

use crate::candidate_pipeline::candidate::PostCandidate;
use crate::candidate_pipeline::query::ScoredPostsQuery;
use crate::params;
use candidate_pipeline::candidate_pipeline::CandidatePipeline;
use candidate_pipeline::filter::Filter;
use candidate_pipeline::hydrator::Hydrator;
use candidate_pipeline::query_hydrator::QueryHydrator;
use candidate_pipeline::scorer::Scorer;
use candidate_pipeline::selector::Selector;
use candidate_pipeline::side_effect::SideEffect;
use candidate_pipeline::source::Source;
use std::sync::Arc;
use tonic::async_trait;

/// Phoenix Candidate Pipeline implementation
pub struct PhoenixCandidatePipeline {
    query_hydrators: Vec<Box<dyn QueryHydrator<ScoredPostsQuery>>>,
    sources: Vec<Box<dyn Source<ScoredPostsQuery, PostCandidate>>>,
    hydrators: Vec<Box<dyn Hydrator<ScoredPostsQuery, PostCandidate>>>,
    filters: Vec<Box<dyn Filter<ScoredPostsQuery, PostCandidate>>>,
    scorers: Vec<Box<dyn Scorer<ScoredPostsQuery, PostCandidate>>>,
    selector: Box<dyn Selector<ScoredPostsQuery, PostCandidate>>,
    post_selection_hydrators: Vec<Box<dyn Hydrator<ScoredPostsQuery, PostCandidate>>>,
    post_selection_filters: Vec<Box<dyn Filter<ScoredPostsQuery, PostCandidate>>>,
    side_effects: Arc<Vec<Box<dyn SideEffect<ScoredPostsQuery, PostCandidate>>>>,
}

impl PhoenixCandidatePipeline {
    /// Create a production pipeline configuration
    pub async fn prod() -> Self {
        // For open-source compatibility, we create a minimal pipeline
        // In production, this would include real client connections
        PhoenixCandidatePipeline {
            query_hydrators: vec![],
            sources: vec![],
            hydrators: vec![],
            filters: vec![],
            scorers: vec![],
            selector: Box::new(TopKSelector::new(params::RESULT_SIZE)),
            post_selection_hydrators: vec![],
            post_selection_filters: vec![],
            side_effects: Arc::new(vec![]),
        }
    }
}

/// Simple top-K selector
struct TopKSelector {
    k: usize,
}

impl TopKSelector {
    fn new(k: usize) -> Self {
        Self { k }
    }
}

impl Selector<ScoredPostsQuery, PostCandidate> for TopKSelector {
    fn score(&self, candidate: &PostCandidate) -> f64 {
        candidate.score.unwrap_or(0.0)
    }

    fn size(&self) -> Option<usize> {
        Some(self.k)
    }
}

#[async_trait]
impl CandidatePipeline<ScoredPostsQuery, PostCandidate> for PhoenixCandidatePipeline {
    fn query_hydrators(&self) -> &[Box<dyn QueryHydrator<ScoredPostsQuery>>] {
        &self.query_hydrators
    }

    fn sources(&self) -> &[Box<dyn Source<ScoredPostsQuery, PostCandidate>>] {
        &self.sources
    }

    fn hydrators(&self) -> &[Box<dyn Hydrator<ScoredPostsQuery, PostCandidate>>] {
        &self.hydrators
    }

    fn filters(&self) -> &[Box<dyn Filter<ScoredPostsQuery, PostCandidate>>] {
        &self.filters
    }

    fn scorers(&self) -> &[Box<dyn Scorer<ScoredPostsQuery, PostCandidate>>] {
        &self.scorers
    }

    fn selector(&self) -> &dyn Selector<ScoredPostsQuery, PostCandidate> {
        self.selector.as_ref()
    }

    fn post_selection_hydrators(&self) -> &[Box<dyn Hydrator<ScoredPostsQuery, PostCandidate>>] {
        &self.post_selection_hydrators
    }

    fn post_selection_filters(&self) -> &[Box<dyn Filter<ScoredPostsQuery, PostCandidate>>] {
        &self.post_selection_filters
    }

    fn side_effects(&self) -> Arc<Vec<Box<dyn SideEffect<ScoredPostsQuery, PostCandidate>>>> {
        Arc::clone(&self.side_effects)
    }

    fn result_size(&self) -> usize {
        params::RESULT_SIZE
    }
}
