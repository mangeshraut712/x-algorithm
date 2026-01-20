# X Algorithm Architecture Guide
## Deep Dive into the "For You" Feed Recommendation System

---

## 🏗️ System Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                      FOR YOU FEED REQUEST                           │
│                    (User opens Twitter/X app)                       │
└─────────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│                          HOME MIXER                                 │
│                    (Orchestration Layer)                            │
│  • Coordinates all pipeline stages                                  │
│  • Manages request lifecycle                                        │
│  • Handles caching and batching                                     │
└─────────────────────────────────────────────────────────────────────┘
                                 │
        ┌────────────────────────┴────────────────────────┐
        ▼                                                  ▼
┌───────────────────┐                          ┌───────────────────┐
│  QUERY HYDRATION  │                          │  QUERY HYDRATION  │
│                   │                          │                   │
│ User Action       │                          │ User Features     │
│ Sequence          │                          │ (following list,  │
│ (engagement       │                          │  preferences,     │
│  history)         │                          │  SimClusters)     │
└───────────────────┘                          └───────────────────┘
        │                                                  │
        └────────────────────────┬────────────────────────┘
                                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│                       CANDIDATE SOURCES                             │
│                                                                     │
│  ┌─────────────────────────┐    ┌─────────────────────────────┐    │
│  │        THUNDER          │    │     PHOENIX RETRIEVAL       │    │
│  │   (In-Network Posts)    │    │   (Out-of-Network Posts)    │    │
│  │                         │    │                             │    │
│  │  Posts from accounts    │    │   ML-based similarity       │    │
│  │  you follow             │    │   search across global      │    │
│  │                         │    │   corpus                    │    │
│  └─────────────────────────┘    └─────────────────────────────┘    │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│                          HYDRATION                                  │
│                                                                     │
│  Fetch additional data:                                             │
│  • Core post metadata (text, media, timestamps)                     │
│  • Author info (follower count, verification status)                │
│  • Media entities (images, videos, durations)                       │
│  • Engagement counts (likes, replies, retweets)                     │
│  • Visibility data (safety labels, content warnings)                │
└─────────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│                          FILTERING                                  │
│                                                                     │
│  Remove:                                                            │
│  ✗ Duplicates                    ✗ Muted keywords                  │
│  ✗ Old posts (AgeFilter)         ✗ Blocked authors                 │
│  ✗ Self-posts                    ✗ NSFW content (if disabled)      │
│  ✗ Previously seen posts         ✗ Spam/low-quality                │
└─────────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│                           SCORING                                   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    PHOENIX SCORER                            │   │
│  │                  (ML Predictions)                            │   │
│  │                                                              │   │
│  │  Grok-based Transformer predicts:                            │   │
│  │  P(like), P(reply), P(repost), P(click), P(follow)...       │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                           │                                         │
│                           ▼                                         │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                   WEIGHTED SCORER                            │   │
│  │              (Combine predictions)                           │   │
│  │                                                              │   │
│  │  Weighted Score = Σ (weight × P(action))                     │   │
│  │                                                              │   │
│  │  Reply: 27.0    Profile Click: 12.0    Like: 1.0            │   │
│  │  Not Interested: -74.0    Report: -369.0                    │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                           │                                         │
│                           ▼                                         │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │               AUTHOR DIVERSITY SCORER                        │   │
│  │                                                              │   │
│  │  Attenuates repeated author scores to ensure feed diversity  │   │
│  │  (Prevents one person from dominating your feed)             │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│                          SELECTION                                  │
│                                                                     │
│  Sort by final score, select top K candidates                       │
│  (Typically ~50-100 posts for initial feed)                         │
└─────────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│                  FILTERING (Post-Selection)                         │
│                                                                     │
│  Visibility filtering:                                              │
│  • Deleted posts                                                    │
│  • Spam that slipped through                                        │
│  • Violence/gore                                                    │
│  • Policy violations                                                │
└─────────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│                     RANKED FEED RESPONSE                            │
│                                                                     │
│  Final ordered list of posts for the user's "For You" feed         │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 📁 Codebase Structure

```
x-algorithm/
│
├── candidate-pipeline/          # Core Framework
│   ├── candidate_pipeline.rs    # Main pipeline orchestration
│   ├── filter.rs                # Filter trait definition
│   ├── scorer.rs                # Scorer trait definition
│   ├── hydrator.rs              # Hydrator trait definition
│   ├── selector.rs              # Selection/ranking logic
│   ├── source.rs                # Candidate source trait
│   └── side_effect.rs           # Post-processing effects
│
├── home-mixer/                  # Timeline Service
│   ├── main.rs                  # gRPC server entry point
│   ├── server.rs                # Request handling
│   ├── proto.rs                 # Protocol definitions
│   ├── params.rs                # ⭐ SCORING WEIGHTS
│   ├── config.rs                # Configuration & metrics
│   │
│   ├── candidate_pipeline/      # Pipeline Implementation
│   │   ├── phoenix_candidate_pipeline.rs  # Main pipeline
│   │   ├── candidate.rs         # PostCandidate struct
│   │   └── query.rs             # Query definitions
│   │
│   ├── scorers/                 # Scoring Algorithms
│   │   └── weighted_scorer.rs   # ⭐ MAIN SCORING LOGIC
│   │
│   ├── filters/                 # Content Filters
│   │   └── mod.rs               # Filter implementations
│   │
│   ├── personalization/         # User Clustering
│   │   └── user_clusters.rs     # SimClusters implementation
│   │
│   └── util/                    # Utilities
│       ├── snowflake.rs         # Tweet ID parsing
│       └── request_util.rs      # Request helpers
│
└── thunder/                     # In-Memory Post Store
    ├── main.rs                  # Thunder service
    └── lib.rs                   # Core functionality
```

---

## 🎯 The Scoring Formula

### Step 1: ML Prediction (Phoenix Scorer)

The Grok-based transformer predicts the probability of each action:

```
P(like)          → How likely user will like this post
P(reply)         → How likely user will reply
P(retweet)       → How likely user will retweet
P(quote)         → How likely user will quote tweet
P(click)         → How likely user will click to read more
P(profile_click) → How likely user will visit author profile
P(follow)        → How likely user will follow the author
P(video_view)    → How likely user will watch video (if applicable)
P(bookmark)      → How likely user will bookmark
P(share_dm)      → How likely user will share via DM
...
P(block)         → How likely user will block
P(mute)          → How likely user will mute
P(report)        → How likely user will report
P(not_interested)→ How likely user will mark "not interested"
```

### Step 2: Weighted Combination

```rust
// From params.rs
FAVORITE_WEIGHT       = 1.0
REPLY_WEIGHT          = 27.0    // 🔥 Highest positive weight
RETWEET_WEIGHT        = 1.0
PROFILE_CLICK_WEIGHT  = 12.0    // 🔥 Strong signal
VQV_WEIGHT            = 0.3     // Video quality view bonus
FOLLOW_AUTHOR_WEIGHT  = 0.0     // (tracked separately)

// Negative signals
NOT_INTERESTED_WEIGHT = -74.0   // ⚠️ Major penalty
BLOCK_AUTHOR_WEIGHT   = 0.0     // (handled differently)
MUTE_AUTHOR_WEIGHT    = 0.0     // (handled differently)
REPORT_WEIGHT         = -369.0  // ☠️ Severe penalty
```

### Step 3: Final Score Calculation

```rust
fn compute_weighted_score(candidate: &PostCandidate) -> f64 {
    let scores = [
        favorite_score,
        reply_score,
        retweet_score,
        // ... all predictions
    ];
    
    let weights = [
        FAVORITE_WEIGHT,
        REPLY_WEIGHT,
        RETWEET_WEIGHT,
        // ... all weights
    ];
    
    // Dot product
    let mut combined_score = 0.0;
    for i in 0..scores.len() {
        combined_score += scores[i] * weights[i];
    }
    
    offset_score(combined_score)
}
```

---

## 🏷️ SimClusters: The Niche System

SimClusters is X's way of understanding what topics users and content belong to.

### How It Works

1. **User Clustering**: Users are assigned to clusters based on their engagement patterns
2. **Content Clustering**: Posts are assigned to clusters based on who engages with them
3. **Matching**: Content is recommended to users in matching clusters

### Why "Staying in Your Lane" Matters

```
User's SimClusters: [Tech, AI, Crypto]
Post's SimClusters: [Tech, AI]          → ✅ High relevance
Post's SimClusters: [Cooking, Travel]   → ❌ Low/zero relevance
```

If you regularly post about tech but suddenly post about cooking:
- Your cooking post won't match your followers' clusters
- It will get near-zero distribution
- This can confuse your cluster assignment

---

## ⏱️ Author Diversity Scorer

Prevents any single author from dominating your feed.

```rust
// Pseudocode
for each candidate in feed:
    author_count[candidate.author] += 1
    
    // Apply decay multiplier
    if author_count[candidate.author] > 1:
        candidate.score *= decay_factor^(author_count - 1)
```

**Implication**: Posting 20 times a day means only your top 2-3 posts get full weight.

---

## 🔍 The Filter Chain

Posts go through multiple filters before scoring:

| Filter | Purpose |
|--------|---------|
| `AgeFilter` | Removes posts older than threshold |
| `DropDuplicatesFilter` | Removes duplicate content |
| `SelfTweetFilter` | Removes your own tweets |
| `MutedKeywordFilter` | Respects user mute settings |
| `AuthorSocialGraphFilter` | Blocks/mutes |
| `VFFilter` | Visibility/safety filtering |
| `ContentQualityFilter` | Spam/low-quality detection |

---

## 📊 Key Metrics Tracked

```rust
pub struct Metrics {
    // Latency
    feed_latency_sum_ms: AtomicU64,
    feed_latency_count: AtomicU64,
    
    // Throughput
    requests_total: AtomicU64,
    requests_success: AtomicU64,
    requests_error: AtomicU64,
    
    // Cache
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
    
    // Safety filters
    nsfw_filtered: AtomicU64,
    spam_filtered: AtomicU64,
    clickbait_filtered: AtomicU64,
    
    // Personalization
    personalized_requests: AtomicU64,
}
```

---

## 🚀 Performance Optimizations

### 1. Caching
- User-level score caching
- Trending content caching
- SimCluster assignment caching

### 2. Batching
- GPU inference batching (128 candidates/batch)
- Network request batching

### 3. SIMD Optimization
```rust
// The weighted scorer uses array-based computation
// for compiler auto-vectorization
let scores = [s1, s2, s3, ...];  // Aligned arrays
let weights = [w1, w2, w3, ...]; // Enable SIMD
```

---

## 🔧 Configuration

Environment variables control behavior:

```env
# Caching
ENABLE_PHOENIX_CACHING=true
CACHE_SIZE=10000000

# Batching
ENABLE_PHOENIX_BATCHING=true
BATCH_SIZE=128

# Safety
ENABLE_NSFW_FILTER=true
ENABLE_SPAM_FILTER=true

# Personalization
ENABLE_PERSONALIZATION=true
NUM_CLUSTERS=100
```

---

## 📚 Further Reading

- [Posting Strategy Guide](./POSTING_STRATEGY.md) - How to optimize for the algorithm
- [Contributing Guide](./CONTRIBUTING.md) - How to contribute to this project
- [API Reference](./API.md) - Detailed API documentation

---

*This documentation is based on analysis of the open-source X algorithm codebase.*
