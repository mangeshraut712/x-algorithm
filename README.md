# X Algorithm

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![CI](https://github.com/mangeshraut712/x-algorithm/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/mangeshraut712/x-algorithm/actions)

**An open-source reference implementation of X's "For You" timeline recommendation algorithm** - featuring ML-based ranking, content safety filters, and performance optimizations.

> **Note**: This is an open-source adaptation of X's internal algorithm. Some components require internal infrastructure and are provided as reference implementations.

## 🎯 Overview

This repository demonstrates the architecture and algorithms behind X's content ranking system.

### Core Components

| Component | Description | Status |
|-----------|-------------|--------|
| **candidate-pipeline** | Generic framework for scoring/filtering pipelines | ✅ Complete |
| **home-mixer** | Timeline ranking service (reference implementation) | 🔧 In Progress |
| **thunder** | In-memory post store (reference implementation) | 🔧 In Progress |

## 🏗️ Architecture

```
User Request → Home Mixer → [Sources → Hydrators → Filters → Scorers → Selection] → Feed
                               ↓
                     ┌─────────────────┐
                     │ Phoenix Scorer  │  (ML-based ranking)
                     │ Weighted Scorer │  (Feature-weighted ranking)
                     └─────────────────┘
```

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/mangeshraut712/x-algorithm.git
cd x-algorithm

# Build the candidate-pipeline crate (fully functional)
cargo build -p candidate-pipeline --release

# Run tests
cargo test -p candidate-pipeline
```

## 📦 Project Structure

```
x-algorithm/
├── candidate-pipeline/   # Core framework for candidate scoring
│   ├── filter.rs         # Filter trait and implementations
│   ├── scorer.rs         # Scorer trait for ranking
│   ├── selector.rs       # Selection/truncation logic
│   └── candidate_pipeline.rs  # Pipeline orchestration
├── home-mixer/           # Timeline ranking service
│   ├── scorers/          # Weighted scoring implementations
│   ├── filters/          # Content safety filters
│   ├── params.rs         # Scoring weights and constants
│   └── config.rs         # Configuration management
├── thunder/              # In-memory post store (reference)
├── scripts/              # Deployment automation
└── visualizations/       # Performance dashboard
```

## 🔧 Key Features

### Candidate Pipeline Framework
The `candidate-pipeline` crate provides a generic, extensible framework for building ranking pipelines:

- **Sources**: Fetch candidates from various backends
- **Hydrators**: Enrich candidates with additional data
- **Filters**: Remove candidates that don't meet criteria
- **Scorers**: Compute ranking scores
- **Selectors**: Sort and truncate results

### Weighted Scoring
```rust
// Scoring weights (from params.rs)
FAVORITE_WEIGHT: 1.0
REPLY_WEIGHT: 27.0        // Replies are highly valued
RETWEET_WEIGHT: 1.0
PROFILE_CLICK_WEIGHT: 12.0
NOT_INTERESTED_WEIGHT: -74.0  // Strong negative signal
REPORT_WEIGHT: -369.0     // Very strong negative signal
```

## 📊 Performance Targets

| Metric | Target |
|--------|--------|
| Latency (p50) | < 50ms |
| Cache Hit Rate | > 50% |
| GPU Utilization | > 80% |

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 🔒 Security

For security concerns, please review our [Security Policy](SECURITY.md).

## 📄 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

---

**⭐ Star this repository** if you find it useful!

**🐛 Found a bug?** [Open an issue](https://github.com/mangeshraut712/x-algorithm/issues)

**💡 Have an idea?** [Start a discussion](https://github.com/mangeshraut712/x-algorithm/discussions)
