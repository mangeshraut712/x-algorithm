//! Twitter Snowflake ID utilities
//!
//! Snowflake IDs encode timestamp information that can be extracted.

use std::time::Duration;

/// Twitter epoch (November 4, 2010 01:42:54.657 UTC)
const TWITTER_EPOCH: i64 = 1288834974657;

/// Extract the creation timestamp from a snowflake ID
pub fn timestamp_millis(snowflake_id: i64) -> i64 {
    (snowflake_id >> 22) + TWITTER_EPOCH
}

/// Get the duration since the snowflake was created
pub fn duration_since_creation_opt(snowflake_id: i64) -> Option<Duration> {
    let creation_ms = timestamp_millis(snowflake_id);
    let now_ms = chrono::Utc::now().timestamp_millis();
    
    if now_ms > creation_ms {
        Some(Duration::from_millis((now_ms - creation_ms) as u64))
    } else {
        None
    }
}

/// Create a snowflake ID from a timestamp (for testing)
pub fn from_timestamp(timestamp_ms: i64) -> i64 {
    (timestamp_ms - TWITTER_EPOCH) << 22
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_extraction() {
        // A known tweet ID
        let tweet_id: i64 = 1234567890123456789;
        let ts = timestamp_millis(tweet_id);
        
        // Should be a reasonable timestamp (after Twitter epoch)
        assert!(ts > TWITTER_EPOCH);
    }

    #[test]
    fn test_roundtrip() {
        let now_ms = chrono::Utc::now().timestamp_millis();
        let snowflake = from_timestamp(now_ms);
        let extracted = timestamp_millis(snowflake);
        
        assert_eq!(now_ms, extracted);
    }
}
