//! Scorers for ranking candidates
//!
//! Note: Some scorers require internal clients and are disabled for open-source compatibility.

pub mod weighted_scorer;
pub mod batch_scorer;

// The following modules require internal clients and are commented out for open-source builds:
// pub mod author_diversity_scorer;
// pub mod batched_phoenix_scorer;
// pub mod cached_phoenix_scorer;
// pub mod oon_scorer;
// pub mod personalized_weighted_scorer;
// pub mod phoenix_scorer;

