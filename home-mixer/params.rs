//! Global parameters and constants for HomeMixer

/// Maximum gRPC message size (16MB)
pub const MAX_GRPC_MESSAGE_SIZE: usize = 16 * 1024 * 1024;

/// Default result size for scored posts
pub const RESULT_SIZE: usize = 100;

/// Maximum post age in seconds (7 days)
pub const MAX_POST_AGE: u64 = 7 * 24 * 60 * 60;

/// Minimum video duration for VQV weight eligibility (milliseconds)
pub const MIN_VIDEO_DURATION_MS: i32 = 2000;

// ============================================================================
// Scoring Weights
// ============================================================================

pub const FAVORITE_WEIGHT: f64 = 1.0;
pub const REPLY_WEIGHT: f64 = 27.0;
pub const RETWEET_WEIGHT: f64 = 1.0;
pub const PHOTO_EXPAND_WEIGHT: f64 = 0.0;
pub const CLICK_WEIGHT: f64 = 0.0;
pub const PROFILE_CLICK_WEIGHT: f64 = 12.0;
pub const VQV_WEIGHT: f64 = 0.3;
pub const SHARE_WEIGHT: f64 = 0.0;
pub const SHARE_VIA_DM_WEIGHT: f64 = 0.0;
pub const SHARE_VIA_COPY_LINK_WEIGHT: f64 = 0.0;
pub const DWELL_WEIGHT: f64 = 0.0;
pub const QUOTE_WEIGHT: f64 = 0.0;
pub const QUOTED_CLICK_WEIGHT: f64 = 0.0;
pub const CONT_DWELL_TIME_WEIGHT: f64 = 0.0;
pub const FOLLOW_AUTHOR_WEIGHT: f64 = 0.0;
pub const NOT_INTERESTED_WEIGHT: f64 = -74.0;
pub const BLOCK_AUTHOR_WEIGHT: f64 = 0.0;
pub const MUTE_AUTHOR_WEIGHT: f64 = 0.0;
pub const REPORT_WEIGHT: f64 = -369.0;

// Weight calculation helpers
pub const WEIGHTS_SUM: f64 = FAVORITE_WEIGHT
    + REPLY_WEIGHT
    + RETWEET_WEIGHT
    + PHOTO_EXPAND_WEIGHT
    + CLICK_WEIGHT
    + PROFILE_CLICK_WEIGHT
    + VQV_WEIGHT
    + SHARE_WEIGHT
    + SHARE_VIA_DM_WEIGHT
    + SHARE_VIA_COPY_LINK_WEIGHT
    + DWELL_WEIGHT
    + QUOTE_WEIGHT
    + QUOTED_CLICK_WEIGHT
    + CONT_DWELL_TIME_WEIGHT
    + FOLLOW_AUTHOR_WEIGHT;

pub const NEGATIVE_WEIGHTS_SUM: f64 =
    NOT_INTERESTED_WEIGHT.abs() + BLOCK_AUTHOR_WEIGHT.abs() + MUTE_AUTHOR_WEIGHT.abs() + REPORT_WEIGHT.abs();

pub const NEGATIVE_SCORES_OFFSET: f64 = 0.0;
