# 𝕏 Algorithm

<div align="center">

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![CI](https://github.com/mangeshraut712/x-algorithm/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/mangeshraut712/x-algorithm/actions)
[![Stars](https://img.shields.io/github/stars/mangeshraut712/x-algorithm?style=social)](https://github.com/mangeshraut712/x-algorithm/stargazers)

**The Open-Source Reference Implementation of X's "For You" Timeline Algorithm**

[📊 Score Calculator](./tools/score-calculator.html) • [📖 Architecture](./docs/ARCHITECTURE.md) • [🎯 Posting Strategy](./docs/POSTING_STRATEGY.md) • [🤝 Contributing](./docs/CONTRIBUTING.md)

</div>

---

## 🎯 What This Is

This repository contains a **reference implementation** of the recommendation algorithm that powers X's (formerly Twitter) "For You" timeline. It's designed to help creators, developers, and researchers understand exactly how content is ranked.

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
| **❤️ Like** | 1.0 | Just the baseline |
| **🔁 Retweet** | 1.0 | Standard amplification |
| **🎬 Video View (VQV)** | 0.3 | Bonus if watched 2+ seconds |
| **😴 Not Interested** | **-74.0** | ⚠️ Major penalty |
| **🚫 Report** | **-369.0** | ☠️ Account killer |

> **TL;DR**: One reply is worth 27 likes. Don't trigger "Not Interested" or you're dead.

---

## 📦 Repository Structure

```
x-algorithm/
├── 📁 candidate-pipeline/     # ✅ Core Framework (Fully Working)
│   ├── filter.rs              # Content filtering traits
│   ├── scorer.rs              # Scoring algorithm traits  
│   ├── selector.rs            # Ranking and selection
│   └── candidate_pipeline.rs  # Pipeline orchestration
│
├── 📁 home-mixer/             # 🔧 Timeline Service
│   ├── params.rs              # ⭐ THE SCORING WEIGHTS
│   ├── scorers/               # Weighted scoring implementation
│   ├── filters/               # Content safety filters
│   └── personalization/       # SimClusters and user clustering
│
├── 📁 thunder/                # 🔧 In-Memory Post Store
│
├── 📁 tools/
│   └── score-calculator.html  # 🧮 Interactive Score Calculator
│
└── 📁 docs/
    ├── ARCHITECTURE.md        # 🏗️ System Architecture Deep Dive
    ├── POSTING_STRATEGY.md    # 🎯 Content Optimization Guide
    ├── TWITTER_THREAD.md      # 🐦 Ready-to-Post Thread
    └── CONTRIBUTING.md        # 🤝 How to Contribute
```

---

## 🚀 Quick Start

### For Developers

```bash
# Clone the repository
git clone https://github.com/mangeshraut712/x-algorithm.git
cd x-algorithm

# Build the working components
cargo build -p candidate-pipeline

# Run tests
cargo test -p candidate-pipeline

# Explore the scoring weights
cat home-mixer/params.rs
```

### For Creators

1. **Read the [Posting Strategy Guide](./docs/POSTING_STRATEGY.md)** - Actionable tips based on the algorithm
2. **Use the [Score Calculator](./tools/score-calculator.html)** - Predict how your content will rank
3. **Try the [Post Analyzer](./tools/post-analyzer.html)** - Analyze your post before publishing
4. **Explore [SimClusters](./tools/simclusters-explorer.html)** - Understand niche-based distribution
5. **See the [Pipeline Visualization](./tools/pipeline-visualization.html)** - Interactive algorithm flow
6. **Check the [Twitter Thread](./docs/TWITTER_THREAD.md)** - Share this knowledge with your audience


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
│   THUNDER    │                │ PHOENIX RETRIEVAL│
│ (Following)  │                │ (Discovery)      │
└──────────────┘                └──────────────────┘
       │                                 │
       └─────────────┬───────────────────┘
                     ▼
            ┌──────────────────┐
            │    FILTERING     │  ← Remove spam, duplicates, etc.
            └──────────────────┘
                     │
                     ▼
            ┌──────────────────┐
            │    SCORING       │  ← ML predictions × weights
            └──────────────────┘
                     │
                     ▼
            ┌──────────────────┐
            │   SELECTION      │  ← Top K by score
            └──────────────────┘
                     │
                     ▼
              Your "For You" Feed
```

---

## 📊 Key Components Explained

### 1. Phoenix Scorer (ML Predictions)
Uses a Grok-based transformer to predict engagement probabilities:
- P(like), P(reply), P(retweet), P(follow), etc.

### 2. Weighted Scorer (Score Combination)
Combines predictions using the weights shown above.

### 3. SimClusters (Topic Matching)
Groups users and content into topic clusters. **Staying in your niche is crucial**.

### 4. Author Diversity Scorer
Prevents any single author from dominating your feed. **Quality > Quantity**.

### 5. Filters
Remove spam, NSFW, blocked users, old content, etc.

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [🏗️ Architecture](./docs/ARCHITECTURE.md) | Deep dive into system design |
| [🎯 Posting Strategy](./docs/POSTING_STRATEGY.md) | How to optimize for the algorithm |
| [🐦 Twitter Thread](./docs/TWITTER_THREAD.md) | Ready-to-post educational thread |
| [🤝 Contributing](./docs/CONTRIBUTING.md) | How to contribute to this project |

---

## 🛠️ Tools

### Score Calculator
Interactive tool to predict how your content will rank.

**[→ Open Score Calculator](./tools/score-calculator.html)**

Features:
- Adjust engagement probabilities
- See real-time score calculations
- Get optimization tips
- Understand the impact of negative signals

---

## 🎯 TL;DR: How To Win

Based on the actual algorithm code:

1. **Create reply-worthy content** (27x weight!)
2. **Make it shareable** (especially DM-worthy)
3. **Stop the scroll** (dwell time matters)
4. **Stay in your niche** (SimClusters are real)
5. **Never trigger blocks/mutes** (-74 to -369 weight)
6. **Reply to your comments** (author response boosts)
7. **Space your posts** (AuthorDiversityScorer decay)
8. **No links in main post** (link penalty is real)

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
2. **Share the [Posting Strategy](./docs/POSTING_STRATEGY.md)** with creator friends
3. **Contribute** to make it even better
4. **Follow** for updates

---

<div align="center">

**Made with ❤️ for the creator community**

[Report Bug](https://github.com/mangeshraut712/x-algorithm/issues) • [Request Feature](https://github.com/mangeshraut712/x-algorithm/issues) • [Discussions](https://github.com/mangeshraut712/x-algorithm/discussions)

</div>
