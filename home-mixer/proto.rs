//! Mock proto structures for open-source compatibility
//! 
//! This module provides mock implementations of internal X.AI proto types
//! to allow the project to compile without proprietary dependencies.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Home Mixer Proto Types
// ============================================================================

/// File descriptor set for gRPC reflection (empty placeholder)
pub const FILE_DESCRIPTOR_SET: &[u8] = &[];

/// Served type enum
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum ServedType {
    #[default]
    Unknown = 0,
    InNetwork = 1,
    OutOfNetwork = 2,
    Promoted = 3,
}

/// Impression bloom filter entry
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ImpressionBloomFilterEntry {
    pub filter_data: Vec<u8>,
    pub num_bits: i32,
    pub num_hashes: i32,
}

/// Scored post response
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScoredPostsResponse {
    pub scored_posts: Vec<ScoredPost>,
}

/// Individual scored post
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScoredPost {
    pub tweet_id: u64,
    pub author_id: u64,
    pub retweeted_tweet_id: u64,
    pub retweeted_user_id: u64,
    pub in_reply_to_tweet_id: u64,
    pub score: f32,
    pub in_network: bool,
    pub served_type: i32,
    pub last_scored_timestamp_ms: u64,
    pub prediction_request_id: u64,
    pub ancestors: Vec<u64>,
    pub screen_names: HashMap<u64, String>,
    pub visibility_reason: Option<VisibilityReason>,
}

/// Visibility reason
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct VisibilityReason {
    pub filtered_reason: Option<FilteredReason>,
}

/// Scored posts query
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScoredPostsQuery {
    pub viewer_id: u64,
    pub client_app_id: i64,
    pub country_code: String,
    pub language_code: String,
    pub seen_ids: Vec<i64>,
    pub served_ids: Vec<i64>,
    pub in_network_only: bool,
    pub is_bottom_request: bool,
    pub bloom_filter_entries: Vec<ImpressionBloomFilterEntry>,
}

// ============================================================================
// Visibility Filtering Types
// ============================================================================

/// Filtered reason enum
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilteredReason {
    #[default]
    None,
    Blocked,
    Muted,
    Nsfw,
    Spam,
    LowQuality,
    Hidden,
}

/// Visibility action
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    #[default]
    Allow,
    Drop,
    Interstitial,
    LocalizedInterstitial,
    SoftIntervention,
}

// ============================================================================
// RecSys Proto Types
// ============================================================================

/// Action name enum for predictions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(usize)]
pub enum ActionName {
    ServerTweetFav = 0,
    ServerTweetReply = 1,
    ServerTweetRetweet = 2,
    ClientTweetPhotoExpand = 3,
    ClientTweetClick = 4,
    ClientTweetClickProfile = 5,
    ClientTweetVideoQualityView = 6,
    ClientTweetShare = 7,
    ClientTweetClickSendViaDirectMessage = 8,
    ClientTweetShareViaCopyLink = 9,
    ClientTweetRecapDwelled = 10,
    ServerTweetQuote = 11,
    ClientQuotedTweetClick = 12,
    ClientTweetFollowAuthor = 13,
    ClientTweetNotInterestedIn = 14,
    ClientTweetBlockAuthor = 15,
    ClientTweetMuteAuthor = 16,
    ClientTweetReport = 17,
}

/// Continuous action name enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(usize)]
pub enum ContinuousActionName {
    DwellTime = 0,
}

/// User action sequence
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UserActionSequence {
    pub actions: Vec<UserAction>,
}

/// Individual user action
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UserAction {
    pub action_type: i32,
    pub tweet_id: u64,
    pub timestamp_ms: u64,
}

/// Tweet info for predictions
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TweetInfo {
    pub tweet_id: u64,
    pub author_id: u64,
}

/// Prediction response
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PredictNextActionsResponse {
    pub distribution_sets: Vec<DistributionSet>,
}

/// Distribution set
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DistributionSet {
    pub candidate_distributions: Vec<CandidateDistribution>,
}

/// Candidate distribution
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CandidateDistribution {
    pub candidate: Option<TweetInfo>,
    pub top_log_probs: Vec<f32>,
    pub continuous_actions_values: Vec<f32>,
}

// ============================================================================
// Twitter Context Types
// ============================================================================

/// Twitter context viewer
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TwitterContextViewer {
    pub user_id: i64,
    pub client_application_id: i64,
    pub request_country_code: String,
    pub request_language_code: String,
}

/// Trait for getting viewer context
pub trait GetTwitterContextViewer {
    fn get_viewer(&self) -> Option<TwitterContextViewer>;
}

// ============================================================================
// gRPC Service Definitions (Mock)
// ============================================================================

pub mod scored_posts_service_server {
    use super::*;
    use tonic::{Request, Response, Status};

    #[tonic::async_trait]
    pub trait ScoredPostsService: Send + Sync + 'static {
        async fn get_scored_posts(
            &self,
            request: Request<ScoredPostsQuery>,
        ) -> Result<Response<ScoredPostsResponse>, Status>;
    }

    #[derive(Clone)]
    pub struct ScoredPostsServiceServer<T: ScoredPostsService> {
        inner: std::sync::Arc<T>,
    }

    impl<T: ScoredPostsService> ScoredPostsServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self {
                inner: std::sync::Arc::new(inner),
            }
        }

        pub fn max_decoding_message_size(self, _size: usize) -> Self {
            self
        }

        pub fn max_encoding_message_size(self, _size: usize) -> Self {
            self
        }

        pub fn accept_compressed(self, _encoding: tonic::codec::CompressionEncoding) -> Self {
            self
        }

        pub fn send_compressed(self, _encoding: tonic::codec::CompressionEncoding) -> Self {
            self
        }
    }

    impl<T: ScoredPostsService> tonic::server::NamedService for ScoredPostsServiceServer<T> {
        const NAME: &'static str = "home_mixer.ScoredPostsService";
    }
}
