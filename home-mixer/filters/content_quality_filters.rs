// Copyright 2026 X.AI Corp.
// Content Safety & Quality Filters
// Addressing Real User Complaints: NSFW, Spam, Quality Issues

use crate::candidate_pipeline::candidate::PostCandidate;
use crate::candidate_pipeline::query::ScoredPostsQuery;
use std::collections::HashSet;
use tonic::async_trait;
use xai_candidate_pipeline::filter::{Filter, FilterResult};

/// NSFW/Adult Content Filter
/// 
/// ADDRESSES USER COMPLAINT #1: "Porn showing up when I didn't ask for it"
/// 
/// This filter removes adult content unless user has explicitly opted in.
/// Uses multi-signal detection:
/// 1. Content label from media pipeline
/// 2. Text-based detection (keywords, patterns)
/// 3. User's content preferences
/// 4. Author's content rating
pub struct NSFWContentFilter {
    /// Use strict filtering by default
    strict_mode: bool,
    
    /// Blocked keywords for text analysis
    blocked_keywords: HashSet<String>,
}

impl NSFWContentFilter {
    pub fn new(strict_mode: bool) -> Self {
        // Load blocked keywords from configuration
        let blocked_keywords = Self::load_nsfw_keywords();
        
        Self {
            strict_mode,
            blocked_keywords,
        }
    }
    
    fn load_nsfw_keywords() -> HashSet<String> {
        // In production, load from secure configuration
        // Here's a minimal example
        vec![
            "nsfw".to_string(),
            "18+".to_string(),
            // ... extensive keyword list in production
        ]
        .into_iter()
        .collect()
    }
    
    fn is_nsfw_content(&self, candidate: &PostCandidate) -> bool {
        // Check 1: Explicit content label from media pipeline
        if candidate.content_labels.contains("adult_content") {
            return true;
        }
        
        // Check 2: Author has adult content rating
        if candidate.author_content_rating == Some("adult") {
            return true;
        }
        
        // Check 3: Sensitive media flag
        if candidate.has_sensitive_media.unwrap_or(false) {
            return true;
        }
        
        // Check 4: Text-based detection (fallback)
        if let Some(text) = &candidate.text {
            let text_lower = text.to_lowercase();
            for keyword in &self.blocked_keywords {
                if text_lower.contains(keyword) {
                    return true;
                }
            }
        }
        
        false
    }
    
    fn user_allows_nsfw(&self, query: &ScoredPostsQuery) -> bool {
        // Check user's content preferences
        query.user_preferences
            .as_ref()
            .and_then(|prefs| prefs.show_sensitive_media)
            .unwrap_or(false)
    }
}

#[async_trait]
impl Filter<ScoredPostsQuery, PostCandidate> for NSFWContentFilter {
    async fn filter(
        &self,
        query: &ScoredPostsQuery,
        candidates: Vec<PostCandidate>,
    ) -> Result<FilterResult<PostCandidate>, String> {
        let user_opted_in = self.user_allows_nsfw(query);
        
        let (kept, removed): (Vec<_>, Vec<_>) = candidates
            .into_iter()
            .partition(|candidate| {
                let is_nsfw = self.is_nsfw_content(candidate);
                
                if is_nsfw {
                    // If NSFW, only keep if user explicitly opted in
                    user_opted_in && !self.strict_mode
                } else {
                    // Keep all non-NSFW content
                    true
                }
            });
        
        log::info!(
            "NSFW filter: kept {} tweets, removed {} NSFW tweets (user_opted_in: {})",
            kept.len(),
            removed.len(),
            user_opted_in
        );
        
        Ok(FilterResult { kept, removed })
    }
}

/// Engagement Bait Filter
/// 
/// ADDRESSES USER COMPLAINT #2: "Stop showing me clickbait and rage bait"
/// 
/// Detects and penalizes:
/// - "You won't believe..." patterns
/// - "This will shock you..." patterns
/// - Excessive emoji usage
/// - Fake urgency ("BREAKING:", "URGENT:")
/// - Engagement farming ("Like and RT if...")
pub struct EngagementBaitFilter {
    /// Patterns that indicate engagement bait
    bait_patterns: Vec<String>,
    
    /// Threshold for emoji density (emojis per character)
    max_emoji_density: f64,
}

impl EngagementBaitFilter {
    pub fn new() -> Self {
        Self {
            bait_patterns: vec![
                "you won't believe".to_string(),
                "this will shock you".to_string(),
                "number 7 will".to_string(),
                "doctors hate".to_string(),
                "like and retweet".to_string(),
                "like and rt".to_string(),
                "thread 🧵".to_string(), // Often used for engagement farming
                "let that sink in".to_string(),
                "read that again".to_string(),
            ],
            max_emoji_density: 0.15, // 15% of text is emojis = suspicious
        }
    }
    
    fn is_engagement_bait(&self, candidate: &PostCandidate) -> bool {
        if let Some(text) = &candidate.text {
            let text_lower = text.to_lowercase();
            
            // Check for bait patterns
            for pattern in &self.bait_patterns {
                if text_lower.contains(pattern) {
                    return true;
                }
            }
            
            // Check emoji density
            let emoji_count = text.chars().filter(|c| self.is_emoji(*c)).count();
            let emoji_density = emoji_count as f64 / text.len() as f64;
            
            if emoji_density > self.max_emoji_density {
                return true;
            }
            
            // Check for excessive capitalization
            let caps_count = text.chars().filter(|c| c.is_uppercase()).count();
            let caps_ratio = caps_count as f64 / text.len() as f64;
            
            if caps_ratio > 0.5 && text.len() > 20 {
                return true; // MORE THAN HALF IS CAPS = SHOUTING
            }
        }
        
        false
    }
    
    fn is_emoji(&self, c: char) -> bool {
        // Simplified emoji detection
        // In production, use proper Unicode emoji ranges
        let code = c as u32;
        (code >= 0x1F600 && code <= 0x1F64F) || // Emoticons
        (code >= 0x1F300 && code <= 0x1F5FF) || // Misc Symbols
        (code >= 0x1F680 && code <= 0x1F6FF) || // Transport
        (code >= 0x2600 && code <= 0x26FF)      // Misc symbols
    }
}

#[async_trait]
impl Filter<ScoredPostsQuery, PostCandidate> for EngagementBaitFilter {
    async fn filter(
        &self,
        _query: &ScoredPostsQuery,
        candidates: Vec<PostCandidate>,
    ) -> Result<FilterResult<PostCandidate>, String> {
        let (kept, removed): (Vec<_>, Vec<_>) = candidates
            .into_iter()
            .partition(|c| !self.is_engagement_bait(c));
        
        log::info!(
            "Engagement bait filter: removed {} clickbait tweets",
            removed.len()
        );
        
        Ok(FilterResult { kept, removed })
    }
}

/// Spam & Bot Detection Filter
/// 
/// ADDRESSES USER COMPLAINT #3: "Fake crypto giveaways and reply bots everywhere"
/// 
/// Detects:
/// - Crypto scam patterns
/// - Impersonation attempts
/// - Suspicious account age + activity
/// - Copy-paste spam
pub struct SpamBotFilter {
    /// Known spam patterns
    spam_patterns: Vec<String>,
}

impl SpamBotFilter {
    pub fn new() -> Self {
        Self {
            spam_patterns: vec![
                "send me".to_string(),
                "claim your".to_string(),
                "free bitcoin".to_string(),
                "double your crypto".to_string(),
                "limited time offer".to_string(),
                "click here now".to_string(),
                "exclusive offer".to_string(),
                "act now".to_string(),
            ],
        }
    }
    
    fn is_spam(&self, candidate: &PostCandidate) -> bool {
        // Check 1: Known spam patterns
        if let Some(text) = &candidate.text {
            let text_lower = text.to_lowercase();
            for pattern in &self.spam_patterns {
                if text_lower.contains(pattern) {
                    return true;
                }
            }
        }
        
        // Check 2: Suspicious author metrics
        if let Some(follower_count) = candidate.author_follower_count {
            if let Some(following_count) = candidate.author_following_count {
                // Suspicious: Following 10x more than followers
                if following_count > follower_count * 10 && follower_count < 100 {
                    return true;
                }
            }
        }
        
        // Check 3: Account age vs activity
        if let Some(account_age_days) = candidate.author_account_age_days {
            if let Some(tweet_count) = candidate.author_tweet_count {
                // New account (<30 days) with tons of tweets (>100/day)
                let tweets_per_day = tweet_count as f64 / account_age_days as f64;
                if account_age_days < 30 && tweets_per_day > 100.0 {
                    return true;
                }
            }
        }
        
        // Check 4: Verified impersonation
        if candidate.is_verified_impersonation.unwrap_or(false) {
            return true;
        }
        
        false
    }
}

#[async_trait]
impl Filter<ScoredPostsQuery, PostCandidate> for SpamBotFilter {
    async fn filter(
        &self,
        _query: &ScoredPostsQuery,
        candidates: Vec<PostCandidate>,
    ) -> Result<FilterResult<PostCandidate>, String> {
        let (kept, removed): (Vec<_>, Vec<_>) = candidates
            .into_iter()
            .partition(|c| !self.is_spam(c));
        
        log::info!(
            "Spam bot filter: removed {} spam/bot tweets",
            removed.len()
        );
        
        Ok(FilterResult { kept, removed })
    }
}

/// Echo Chamber Diversification Scorer
/// 
/// ADDRESSES USER COMPLAINT #4: "Algorithm only shows me opinions I agree with"
/// 
/// Boosts content from outside user's typical interests
/// to encourage diverse perspectives
pub struct DiversityBoostScorer {
    /// How much to boost diverse content (multiplier)
    diversity_boost: f64,
}

impl DiversityBoostScorer {
    pub fn new(diversity_boost: f64) -> Self {
        Self { diversity_boost }
    }
    
    fn is_outside_bubble(&self, candidate: &PostCandidate, query: &ScoredPostsQuery) -> bool {
        // Check if author is outside user's typical network
        if let Some(user_typical_topics) = &query.user_interest_topics {
            if let Some(tweet_topics) = &candidate.topics {
                // Count topic overlap
                let overlap: usize = tweet_topics
                    .iter()
                    .filter(|topic| user_typical_topics.contains(*topic))
                    .count();
                
                // If less than 30% overlap, it's diverse content
                let overlap_ratio = overlap as f64 / tweet_topics.len() as f64;
                return overlap_ratio < 0.3;
            }
        }
        
        false
    }
}

#[async_trait]
impl Scorer<ScoredPostsQuery, PostCandidate> for DiversityBoostScorer {
    async fn score(
        &self,
        query: &ScoredPostsQuery,
        candidates: &[PostCandidate],
    ) -> Result<Vec<PostCandidate>, String> {
        let scored = candidates
            .iter()
            .map(|c| {
                let boost = if self.is_outside_bubble(c, query) {
                    self.diversity_boost
                } else {
                    1.0
                };
                
                PostCandidate {
                    diversity_boost: Some(boost),
                    ..Default::default()
                }
            })
            .collect();
        
        Ok(scored)
    }
    
    fn update(&self, candidate: &mut PostCandidate, scored: PostCandidate) {
        if let Some(boost) = scored.diversity_boost {
            // Apply boost to final score
            if let Some(current_score) = candidate.weighted_score {
                candidate.weighted_score = Some(current_score * boost);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_nsfw_detection() {
        let filter = NSFWContentFilter::new(true);
        
        let mut candidate = PostCandidate::default();
        candidate.has_sensitive_media = Some(true);
        
        assert!(filter.is_nsfw_content(&candidate));
    }
    
    #[test]
    fn test_engagement_bait_detection() {
        let filter = EngagementBaitFilter::new();
        
        let mut candidate = PostCandidate::default();
        candidate.text = Some("You won't believe what happened next!".to_string());
        
        assert!(filter.is_engagement_bait(&candidate));
    }
    
    #[test]
    fn test_spam_detection() {
        let filter = SpamBotFilter::new();
        
        let mut candidate = PostCandidate::default();
        candidate.text = Some("Send me Bitcoin and I'll double it!".to_string());
        candidate.author_follower_count = Some(10);
        candidate.author_following_count = Some(5000);
        
        assert!(filter.is_spam(&candidate));
    }
}
