// Copyright 2026 X.AI Corp.
// Production-ready configuration and metrics system

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

// ============================================================
// CONFIGURATION
// ============================================================

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub caching: CachingConfig,
    pub batching: BatchingConfig,
    pub personalization: PersonalizationConfig,
    pub safety: SafetyConfig,
    pub features: FeatureFlags,
    pub metrics: MetricsConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CachingConfig {
    pub enabled: bool,
    pub user_cache_size: usize,
    pub trending_cache_size: usize,
    pub trending_ttl_secs: u64,
    pub user_cache_ttl_secs: u64,
    pub enable_cache_warming: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchingConfig {
    pub enabled: bool,
    pub max_batch_size: usize,
    pub max_wait_time_ms: u64,
    pub max_concurrent_batches: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PersonalizationConfig {
    pub enabled: bool,
    pub num_clusters: usize,
    pub enable_auto_refresh: bool,
    pub refresh_interval_hours: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SafetyConfig {
    pub enable_nsfw_filter: bool,
    pub nsfw_strict_mode: bool,
    pub enable_spam_filter: bool,
    pub enable_engagement_bait_filter: bool,
    pub enable_diversity_boost: bool,
    pub diversity_boost_multiplier: f64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub caching_rollout_percent: u8,
    pub batching_rollout_percent: u8,
    pub personalization_rollout_percent: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub port: u16,
    pub enable_tracing: bool,
}

impl Default for CachingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            user_cache_size: 10_000_000,
            trending_cache_size: 100_000,
            trending_ttl_secs: 300,
            user_cache_ttl_secs: 3600,
            enable_cache_warming: false,
        }
    }
}

impl Default for BatchingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_batch_size: 128,
            max_wait_time_ms: 5,
            max_concurrent_batches: 4,
        }
    }
}

impl Default for PersonalizationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            num_clusters: 100,
            enable_auto_refresh: false,
            refresh_interval_hours: 24,
        }
    }
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            enable_nsfw_filter: true,
            nsfw_strict_mode: true,
            enable_spam_filter: true,
            enable_engagement_bait_filter: true,
            enable_diversity_boost: false,
            diversity_boost_multiplier: 1.3,
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            port: 9090,
            enable_tracing: false,
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            caching: CachingConfig {
                enabled: env_bool("ENABLE_PHOENIX_CACHING", false),
                user_cache_size: env_usize("CACHE_SIZE", 10_000_000),
                trending_cache_size: env_usize("TRENDING_CACHE_SIZE", 100_000),
                trending_ttl_secs: env_u64("TRENDING_TTL_SECS", 300),
                user_cache_ttl_secs: env_u64("CACHE_TTL_SECS", 3600),
                enable_cache_warming: env_bool("ENABLE_CACHE_WARMING", false),
            },
            batching: BatchingConfig {
                enabled: env_bool("ENABLE_PHOENIX_BATCHING", false),
                max_batch_size: env_usize("BATCH_SIZE", 128),
                max_wait_time_ms: env_u64("BATCH_TIMEOUT_MS", 5),
                max_concurrent_batches: env_usize("MAX_CONCURRENT_BATCHES", 4),
            },
            personalization: PersonalizationConfig {
                enabled: env_bool("ENABLE_PERSONALIZATION", false),
                num_clusters: env_usize("NUM_USER_CLUSTERS", 100),
                enable_auto_refresh: env_bool("AUTO_REFRESH_CLUSTERS", false),
                refresh_interval_hours: env_u64("CLUSTER_REFRESH_HOURS", 24),
            },
            safety: SafetyConfig {
                enable_nsfw_filter: env_bool("ENABLE_NSFW_FILTER", true),
                nsfw_strict_mode: env_bool("NSFW_STRICT_MODE", true),
                enable_spam_filter: env_bool("ENABLE_SPAM_FILTER", true),
                enable_engagement_bait_filter: env_bool("ENABLE_ENGAGEMENT_BAIT_FILTER", true),
                enable_diversity_boost: env_bool("ENABLE_DIVERSITY_BOOST", false),
                diversity_boost_multiplier: env_f64("DIVERSITY_BOOST_MULTIPLIER", 1.3),
            },
            features: FeatureFlags {
                caching_rollout_percent: env_u8("CACHING_ROLLOUT_PERCENT", 0),
                batching_rollout_percent: env_u8("BATCHING_ROLLOUT_PERCENT", 0),
                personalization_rollout_percent: env_u8("PERSONALIZATION_ROLLOUT_PERCENT", 0),
            },
            metrics: MetricsConfig {
                enabled: env_bool("METRICS_ENABLED", true),
                port: env_u16("METRICS_PORT", 9090),
                enable_tracing: env_bool("ENABLE_TRACING", false),
            },
        }
    }
    
    pub fn should_use_caching(&self, user_id: u64) -> bool {
        self.caching.enabled && is_in_rollout(user_id, self.features.caching_rollout_percent)
    }
    
    pub fn should_use_batching(&self, user_id: u64) -> bool {
        self.batching.enabled && is_in_rollout(user_id, self.features.batching_rollout_percent)
    }
    
    pub fn should_use_personalization(&self, user_id: u64) -> bool {
        self.personalization.enabled && is_in_rollout(user_id, self.features.personalization_rollout_percent)
    }
}

fn is_in_rollout(user_id: u64, percent: u8) -> bool {
    if percent >= 100 { return true; }
    if percent == 0 { return false; }
    (user_id % 100) < percent as u64
}

fn env_bool(key: &str, default: bool) -> bool {
    std::env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn env_usize(key: &str, default: usize) -> usize {
    std::env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn env_u64(key: &str, default: u64) -> u64 {
    std::env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn env_u8(key: &str, default: u8) -> u8 {
    std::env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn env_u16(key: &str, default: u16) -> u16 {
    std::env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn env_f64(key: &str, default: f64) -> f64 {
    std::env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

// ============================================================
// METRICS
// ============================================================

#[derive(Default)]
pub struct Metrics {
    // Latency
    pub feed_latency_sum_ms: AtomicU64,
    pub feed_latency_count: AtomicU64,
    
    // Throughput
    pub requests_total: AtomicU64,
    pub requests_success: AtomicU64,
    pub requests_error: AtomicU64,
    
    // Cache
    pub cache_hits: AtomicU64,
    pub cache_misses: AtomicU64,
    
    // Batching
    pub batch_size_sum: AtomicU64,
    pub batch_count: AtomicU64,
    
    // GPU
    pub gpu_inference_time_sum_ms: AtomicU64,
    pub gpu_inference_count: AtomicU64,
    
    // Safety filters
    pub nsfw_filtered: AtomicU64,
    pub spam_filtered: AtomicU64,
    pub clickbait_filtered: AtomicU64,
    
    // Personalization
    pub personalized_requests: AtomicU64,
}

impl Metrics {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }
    
    pub fn record_request(&self, latency_ms: u64, success: bool) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
        self.feed_latency_sum_ms.fetch_add(latency_ms, Ordering::Relaxed);
        self.feed_latency_count.fetch_add(1, Ordering::Relaxed);
        
        if success {
            self.requests_success.fetch_add(1, Ordering::Relaxed);
        } else {
            self.requests_error.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    pub fn record_cache_access(&self, hit: bool) {
        if hit {
            self.cache_hits.fetch_add(1, Ordering::Relaxed);
        } else {
            self.cache_misses.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    pub fn record_batch(&self, size: usize) {
        self.batch_size_sum.fetch_add(size as u64, Ordering::Relaxed);
        self.batch_count.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn record_filter(&self, filter_type: FilterType) {
        match filter_type {
            FilterType::Nsfw => self.nsfw_filtered.fetch_add(1, Ordering::Relaxed),
            FilterType::Spam => self.spam_filtered.fetch_add(1, Ordering::Relaxed),
            FilterType::Clickbait => self.clickbait_filtered.fetch_add(1, Ordering::Relaxed),
        };
    }
    
    pub fn avg_latency_ms(&self) -> f64 {
        let sum = self.feed_latency_sum_ms.load(Ordering::Relaxed);
        let count = self.feed_latency_count.load(Ordering::Relaxed);
        if count == 0 { 0.0 } else { sum as f64 / count as f64 }
    }
    
    pub fn cache_hit_rate(&self) -> f64 {
        let hits = self.cache_hits.load(Ordering::Relaxed);
        let misses = self.cache_misses.load(Ordering::Relaxed);
        let total = hits + misses;
        if total == 0 { 0.0 } else { hits as f64 / total as f64 }
    }
    
    pub fn avg_batch_size(&self) -> f64 {
        let sum = self.batch_size_sum.load(Ordering::Relaxed);
        let count = self.batch_count.load(Ordering::Relaxed);
        if count == 0 { 0.0 } else { sum as f64 / count as f64 }
    }
    
    pub fn error_rate(&self) -> f64 {
        let total = self.requests_total.load(Ordering::Relaxed);
        let errors = self.requests_error.load(Ordering::Relaxed);
        if total == 0 { 0.0 } else { errors as f64 / total as f64 }
    }
    
    pub fn to_prometheus(&self) -> String {
        format!(
            r#"# HELP feed_latency_ms Average feed generation latency
# TYPE feed_latency_ms gauge
feed_latency_ms {:.2}

# HELP requests_total Total number of requests
# TYPE requests_total counter
requests_total {}

# HELP cache_hit_rate Cache hit rate
# TYPE cache_hit_rate gauge
cache_hit_rate {:.4}

# HELP avg_batch_size Average batch size
# TYPE avg_batch_size gauge
avg_batch_size {:.2}

# HELP error_rate Error rate
# TYPE error_rate gauge
error_rate {:.6}

# HELP nsfw_filtered Total NSFW content filtered
# TYPE nsfw_filtered counter
nsfw_filtered {}

# HELP spam_filtered Total spam content filtered
# TYPE spam_filtered counter
spam_filtered {}

# HELP clickbait_filtered Total clickbait content filtered
# TYPE clickbait_filtered counter
clickbait_filtered {}
"#,
            self.avg_latency_ms(),
            self.requests_total.load(Ordering::Relaxed),
            self.cache_hit_rate(),
            self.avg_batch_size(),
            self.error_rate(),
            self.nsfw_filtered.load(Ordering::Relaxed),
            self.spam_filtered.load(Ordering::Relaxed),
            self.clickbait_filtered.load(Ordering::Relaxed),
        )
    }
}

pub enum FilterType {
    Nsfw,
    Spam,
    Clickbait,
}

// ============================================================
// REQUEST CONTEXT
// ============================================================

pub struct RequestContext {
    pub request_id: String,
    pub user_id: u64,
    pub start_time: Instant,
    pub config: Arc<Config>,
    pub metrics: Arc<Metrics>,
}

impl RequestContext {
    pub fn new(user_id: u64, config: Arc<Config>, metrics: Arc<Metrics>) -> Self {
        Self {
            request_id: generate_request_id(),
            user_id,
            start_time: Instant::now(),
            config,
            metrics,
        }
    }
    
    pub fn elapsed_ms(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64
    }
    
    pub fn finish(&self, success: bool) {
        self.metrics.record_request(self.elapsed_ms(), success);
    }
}

fn generate_request_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("req_{:x}", timestamp)
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(!config.caching.enabled);
        assert!(!config.batching.enabled);
        assert!(config.safety.enable_nsfw_filter);
    }
    
    #[test]
    fn test_rollout_logic() {
        // User 0-9 should be in 10% rollout
        for user_id in 0..10u64 {
            assert!(is_in_rollout(user_id, 10));
        }
        // User 10-99 should NOT be in 10% rollout
        for user_id in 10..100u64 {
            assert!(!is_in_rollout(user_id, 10));
        }
        // 100% rollout
        assert!(is_in_rollout(999, 100));
        // 0% rollout
        assert!(!is_in_rollout(0, 0));
    }
    
    #[test]
    fn test_metrics() {
        let metrics = Metrics::new();
        
        metrics.record_request(50, true);
        metrics.record_request(60, true);
        metrics.record_request(70, false);
        
        assert_eq!(metrics.requests_total.load(Ordering::Relaxed), 3);
        assert_eq!(metrics.requests_success.load(Ordering::Relaxed), 2);
        assert_eq!(metrics.requests_error.load(Ordering::Relaxed), 1);
        assert!((metrics.avg_latency_ms() - 60.0).abs() < 0.01);
    }
    
    #[test]
    fn test_cache_hit_rate() {
        let metrics = Metrics::new();
        
        for _ in 0..70 {
            metrics.record_cache_access(true);
        }
        for _ in 0..30 {
            metrics.record_cache_access(false);
        }
        
        assert!((metrics.cache_hit_rate() - 0.7).abs() < 0.01);
    }
}
