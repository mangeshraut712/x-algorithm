//! Global parameters and constants for HomeMixer
//! Updated January 2026 based on real X algorithm behavior analysis

/// Maximum gRPC message size (16MB)
pub const MAX_GRPC_MESSAGE_SIZE: usize = 16 * 1024 * 1024;

/// Default result size for scored posts
pub const RESULT_SIZE: usize = 100;

/// Maximum post age in seconds (7 days)
pub const MAX_POST_AGE: u64 = 7 * 24 * 60 * 60;

/// Minimum video duration for VQV weight eligibility (milliseconds)
pub const MIN_VIDEO_DURATION_MS: i32 = 2000;

// ============================================================================
// Scoring Weights (from X Algorithm Analysis - January 2026)
// ============================================================================

// Positive Signals
pub const FAVORITE_WEIGHT: f64 = 1.0;         // Like
pub const REPLY_WEIGHT: f64 = 27.0;           // 🔥 HIGHEST - drives conversation
pub const RETWEET_WEIGHT: f64 = 1.0;          // Standard repost
pub const PHOTO_EXPAND_WEIGHT: f64 = 0.5;     // Image click (dwell signal)
pub const CLICK_WEIGHT: f64 = 0.5;            // Link click (but links are penalized)
pub const PROFILE_CLICK_WEIGHT: f64 = 12.0;   // 🔥 Very high - shows interest in author
pub const VQV_WEIGHT: f64 = 0.3;              // Video Quality View bonus
pub const SHARE_WEIGHT: f64 = 1.0;            // Generic share
pub const SHARE_VIA_DM_WEIGHT: f64 = 2.0;     // DM share = high intent signal
pub const SHARE_VIA_COPY_LINK_WEIGHT: f64 = 1.5; // Copy link = save intent
pub const DWELL_WEIGHT: f64 = 0.1;            // Time spent viewing
pub const QUOTE_WEIGHT: f64 = 2.0;            // Quote tweet adds conversation
pub const QUOTED_CLICK_WEIGHT: f64 = 0.5;     // Clicked to see quoted tweet
pub const CONT_DWELL_TIME_WEIGHT: f64 = 0.05; // Continuous dwell bonus
pub const FOLLOW_AUTHOR_WEIGHT: f64 = 4.0;    // Strong signal of author quality
pub const BOOKMARK_WEIGHT: f64 = 4.0;         // 🔥 Strong save intent signal

// Negative Signals
pub const NOT_INTERESTED_WEIGHT: f64 = -74.0; // ⚠️ Very negative
pub const BLOCK_AUTHOR_WEIGHT: f64 = -150.0;  // Severe penalty
pub const MUTE_AUTHOR_WEIGHT: f64 = -50.0;    // Penalty
pub const REPORT_WEIGHT: f64 = -369.0;        // ☠️ Account killer

// Author Response Bonus (replying to your own thread)
pub const AUTHOR_REPLY_BONUS: f64 = 1.5;      // Multiplier when author engages

// In-Network vs Out-of-Network
pub const IN_NETWORK_WEIGHT: f64 = 1.0;       // Full weight for followed accounts
pub const OON_WEIGHT_FACTOR: f64 = 0.7;       // Discount for discovery content

// Author Diversity (anti-spam)
pub const AUTHOR_DIVERSITY_DECAY: f64 = 0.8;  // Each additional post from same author

// Freshness
pub const FRESHNESS_DECAY_HOURS: f64 = 6.0;   // Half-life for post freshness

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
    + FOLLOW_AUTHOR_WEIGHT
    + BOOKMARK_WEIGHT;

pub const NEGATIVE_WEIGHTS_SUM: f64 =
    NOT_INTERESTED_WEIGHT.abs() + BLOCK_AUTHOR_WEIGHT.abs() + MUTE_AUTHOR_WEIGHT.abs() + REPORT_WEIGHT.abs();

pub const NEGATIVE_SCORES_OFFSET: f64 = 0.0;
