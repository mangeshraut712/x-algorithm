# 𝕏 Algorithm

<div align="center">

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![CI](https://github.com/mangeshraut712/x-algorithm/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/mangeshraut712/x-algorithm/actions)
[![Website](https://img.shields.io/badge/Website-Live-green)](https://mangeshraut712.github.io/x-algorithm/)
[![Stars](https://img.shields.io/github/stars/mangeshraut712/x-algorithm?style=social)](https://github.com/mangeshraut712/x-algorithm/stargazers)

**The Open-Source Reference Implementation of X's "For You" Timeline Algorithm**

[🌐 Live Website](https://mangeshraut712.github.io/x-algorithm/) • [🚀 Post Analyzer](https://mangeshraut712.github.io/x-algorithm/tools/viral-score-analyzer.html) • [📊 Feed Simulator](https://mangeshraut712.github.io/x-algorithm/tools/feed-simulator.html) • [📖 Docs](./docs/ARCHITECTURE.md)

</div>

---

## ✨ What's New (January 2026)

- 🎨 **Complete X Platform Redesign** - All pages now match X's exact interface
- 🚀 **Post Analyzer** - Analyze posts with viral scoring and AI optimization
- 📊 **Feed Simulator** - See how posts are ranked in real-time
- ⏰ **Posting Time Optimizer** - Find the best time to post with heatmaps
- 🎯 **SimClusters Explorer** - Visual niche/topic clustering
- ⚖️ **Updated Weights** - Bookmark (4×), DM Share (2×), Block (-150×)

---

## 🎯 What This Is

This repository contains a **reference implementation** of the recommendation algorithm that powers X's (formerly Twitter) "For You" timeline, plus **7 interactive tools** to help you understand and optimize for it.

### What You'll Learn

- **The exact scoring weights** used to rank posts
- **How SimClusters work** for topic-based recommendations
- **Why certain content performs better** than others
- **How to optimize your content strategy** based on the algorithm

---

## 🔥 The Key Insight: Scoring Weights

The algorithm predicts engagement probabilities and multiplies them by these weights:

| Action | Weight | What This Means |
|--------|--------|-----------------|
| **💬 Reply** | **27.0** | 27x more valuable than a like |
| **👤 Profile Click** | **12.0** | Shows genuine interest |
| **🔖 Bookmark** | **4.0** | Strong save intent signal |
| **👥 Follow Author** | **4.0** | Quality author signal |
| **💬 Quote Tweet** | **2.0** | Conversation value |
| **📤 DM Share** | **2.0** | High-intent sharing |
| **❤️ Like** | 1.0 | Just the baseline |
| **🔁 Repost** | 1.0 | Standard amplification |
| **🎬 Video View** | 0.3 | Bonus if watched 2+ seconds |
| **🔇 Mute** | **-50.0** | Penalty |
| **😴 Not Interested** | **-74.0** | ⚠️ Major penalty |
| **🚫 Block** | **-150.0** | Severe penalty |
| **⚠️ Report** | **-369.0** | ☠️ Account killer |

> **TL;DR**: One reply is worth 27 likes. Blocks and reports destroy your reach.

---

## 🛠️ Interactive Tools

| Tool | Description | Try It |
|------|-------------|--------|
| 🚀 **Post Analyzer** | Analyze posts, get viral score, optimize content | [Open →](https://mangeshraut712.github.io/x-algorithm/tools/viral-score-analyzer.html) |
| 📊 **Feed Simulator** | See how the algorithm ranks competing posts | [Open →](https://mangeshraut712.github.io/x-algorithm/tools/feed-simulator.html) |
| ⏰ **Posting Time Optimizer** | Find optimal posting times with heatmaps | [Open →](https://mangeshraut712.github.io/x-algorithm/tools/posting-time-optimizer.html) |
| 🧮 **Score Calculator** | Calculate scores from engagement probabilities | [Open →](https://mangeshraut712.github.io/x-algorithm/tools/score-calculator.html) |
| 🔄 **Pipeline Visualization** | Explore the 6-stage ranking pipeline | [Open →](https://mangeshraut712.github.io/x-algorithm/tools/pipeline-visualization.html) |
| 🎯 **SimClusters Explorer** | Understand topic clustering and niche strategy | [Open →](https://mangeshraut712.github.io/x-algorithm/tools/simclusters-explorer.html) |

---

## 📦 Repository Structure

```
x-algorithm/
├── 📁 candidate-pipeline/     # Core Framework (Rust)
│   ├── filter.rs              # Content filtering traits
│   ├── scorer.rs              # Scoring algorithm traits  
│   ├── selector.rs            # Ranking and selection
│   └── candidate_pipeline.rs  # Pipeline orchestration
│
├── 📁 home-mixer/             # Timeline Service (Rust)
│   ├── params.rs              # ⭐ THE SCORING WEIGHTS
│   ├── config.rs              # Production-ready configuration
│   ├── scorers/               # Weighted scoring implementation
│   │   ├── weighted_scorer.rs # SIMD-optimized scoring
│   │   ├── phoenix_scorer.rs  # ML scoring with Grok
│   │   └── author_diversity_scorer.rs
│   ├── filters/               # Content safety filters
│   └── tests/                 # Integration tests
│
├── 📁 phoenix/                # ML Ranking Engine (Python/JAX)
│   └── ranker.py              # Grok-based engagement prediction
│
├── 📁 thunder/                # In-Network Posts (Rust)
│   ├── realtime_query.rs      # Real-time timeline queries
│   └── candidate_source.rs    # Following-based retrieval
│
├── 📁 tools/                  # Interactive Web Tools
│   ├── viral-score-analyzer.html   # 🚀 Post analyzer with optimization
│   ├── feed-simulator.html         # 📊 Feed ranking simulator
│   ├── posting-time-optimizer.html # ⏰ Best time to post
│   ├── score-calculator.html       # 🧮 Score calculator
│   ├── pipeline-visualization.html # 🔄 Pipeline explorer
│   └── simclusters-explorer.html   # 🎯 Niche clustering
│
├── 📁 docs/
│   ├── ARCHITECTURE.md        # System architecture deep dive
│   ├── POSTING_STRATEGY.md    # Content optimization guide
│   └── CONTRIBUTING.md        # How to contribute
│
└── index.html                 # Landing page (X-style design)
```

---

## 🚀 Quick Start

### For Developers

```bash
# Clone the repository
git clone https://github.com/mangeshraut712/x-algorithm.git
cd x-algorithm

# Build the Rust components
cargo build -p candidate-pipeline

# Run tests
cargo test -p candidate-pipeline

# Explore the scoring weights
cat home-mixer/params.rs
```

### For Creators

1. **[Open the Post Analyzer](https://mangeshraut712.github.io/x-algorithm/tools/viral-score-analyzer.html)** - Analyze your post before publishing
2. **[Try the Feed Simulator](https://mangeshraut712.github.io/x-algorithm/tools/feed-simulator.html)** - See how your post would rank
3. **[Find Your Best Time](https://mangeshraut712.github.io/x-algorithm/tools/posting-time-optimizer.html)** - Timezone-aware posting schedule
4. **[Read the Strategy Guide](./docs/POSTING_STRATEGY.md)** - Actionable optimization tips

---

## 🧠 How The Algorithm Works

```
User Opens App
       │
       ▼
┌──────────────────┐
│   HOME MIXER     │  ← Orchestration layer
└──────────────────┘
       │
       ├─────────────────────────────────┐
       ▼                                 ▼
┌──────────────┐                ┌──────────────────┐
│   THUNDER    │                │ PHOENIX          │
│ (Following)  │                │ (Discovery)      │
│  ~500 posts  │                │  ~1000 posts     │
│  Weight: 1.0 │                │  Weight: 0.7 OON │
└──────────────┘                └──────────────────┘
       │                                 │
       └─────────────┬───────────────────┘
                     ▼
            ┌──────────────────┐
            │    FILTERING     │  ← Spam, blocks, 7-day max age
            └──────────────────┘
                     │
                     ▼
            ┌──────────────────┐
            │    SCORING       │  ← Grok predictions × weights
            │                  │  ← Author diversity decay (0.8×)
            └──────────────────┘
                     │
                     ▼
            ┌──────────────────┐
            │   SELECTION      │  ← Top K by score + ad injection
            └──────────────────┘
                     │
                     ▼
              Your "For You" Feed
```

---

## 📊 Key Algorithm Parameters

From `home-mixer/params.rs`:

```rust
// Positive Signals
pub const REPLY_WEIGHT: f64 = 27.0;           // 🔥 HIGHEST
pub const PROFILE_CLICK_WEIGHT: f64 = 12.0;   // Shows interest
pub const BOOKMARK_WEIGHT: f64 = 4.0;         // Save intent
pub const FOLLOW_AUTHOR_WEIGHT: f64 = 4.0;    // Quality signal
pub const QUOTE_WEIGHT: f64 = 2.0;            // Conversation
pub const SHARE_VIA_DM_WEIGHT: f64 = 2.0;     // High intent

// Negative Signals
pub const NOT_INTERESTED_WEIGHT: f64 = -74.0;
pub const BLOCK_AUTHOR_WEIGHT: f64 = -150.0;
pub const REPORT_WEIGHT: f64 = -369.0;        // ☠️ 

// Modifiers
pub const OON_WEIGHT_FACTOR: f64 = 0.7;       // Out-of-network discount
pub const AUTHOR_DIVERSITY_DECAY: f64 = 0.8;  // Per-post decay
pub const FRESHNESS_DECAY_HOURS: f64 = 6.0;   // Half-life
```

---

## 🎯 TL;DR: How To Win

Based on the actual algorithm code:

1. **Create reply-worthy content** (27× weight!)
2. **Make it bookmark-worthy** (4× weight for saves)
3. **End with a question** (drives replies)
4. **Never put links in post body** (use "link in reply")
5. **Stay in your niche** (SimClusters matter)
6. **Reply to your comments** (author response boost)
7. **Space posts 3-4 hours** (diversity decay)
8. **Post at optimal times** (freshness decay: 6h half-life)

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [🏗️ Architecture](./docs/ARCHITECTURE.md) | Deep dive into system design |
| [🎯 Posting Strategy](./docs/POSTING_STRATEGY.md) | How to optimize for the algorithm |
| [🤝 Contributing](./docs/CONTRIBUTING.md) | How to contribute to this project |

---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](./docs/CONTRIBUTING.md) for guidelines.

### Ways to Contribute
- 📖 Improve documentation
- 🐛 Report bugs
- 💡 Suggest features
- 💻 Submit code
- 🎨 Create visualizations

---

## 📄 License

This project is licensed under the Apache 2.0 License - see [LICENSE](LICENSE) for details.

---

## ⭐ Support This Project

If this helped you understand the algorithm:

1. **Star this repo** ⭐
2. **Share the [Website](https://mangeshraut712.github.io/x-algorithm/)** with creator friends
3. **Contribute** to make it even better
4. **Follow** for updates

---

<div align="center">

**Made with ❤️ for the creator community**

[🌐 Website](https://mangeshraut712.github.io/x-algorithm/) • [Report Bug](https://github.com/mangeshraut712/x-algorithm/issues) • [Request Feature](https://github.com/mangeshraut712/x-algorithm/issues)

</div>
