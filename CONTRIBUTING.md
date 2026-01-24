# Contributing to X Algorithm

Thank you for your interest in contributing to the X Recommendation Algorithm! This project is a core part of X's transparency mission, and we welcome community involvement.

## Table of Contents
- [Our Principles](#our-principles)
- [How to Contribute](#how-to-contribute)
- [Development Environment](#development-environment)
- [Code Style](#code-style)
- [Reporting Issues](#reporting-issues)

## Our Principles
1. **Transparency**: Every change should be understandable and its impact on user experience should be clear.
2. **Performance**: This algorithm runs at massive scale. Efficiency is paramount.
3. **Diversity**: We strive for a feed that is diverse and representative of different perspectives.
4. **Safety**: Protecting users from spam, violence, and harmful content is built into the pipeline.

## How to Contribute
We welcome various types of contributions:
- **Optimization**: Performance improvements in the Rust-based pipeline or JAX-based ML model.
- **Explainability**: Tools or documentation that help people understand why certain content is recommended.
- **Testing**: Adding benchmarks or robust unit tests for the complex scoring logic.
- **Documentation**: Clarifying the architecture and design decisions.

### Submitting a Pull Request
1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/amazing-feature`).
3. Commit your changes with descriptive messages.
4. Push to the branch (`git push origin feature/amazing-feature`).
5. Open a Pull Request with a detailed description of your changes and why they are beneficial.

## Development Environment

### Rust Components (`home-mixer`, `thunder`, `candidate-pipeline`)
The Rust components use standard Cargo. 
To build:
```bash
cargo build
```
To run tests:
```bash
cargo test
```

### ML Components (`phoenix`)
The ML models are implemented in Python using JAX and Haiku. We recommend using `uv` for dependency management.
```bash
cd phoenix
uv sync
```
To run model tests:
```bash
pytest
```

## Code Style
- **Rust**: We follow standard Rust idioms. Use `cargo fmt` and `cargo clippy`.
- **Python**: We use `ruff` for linting and formatting.

## Reporting Issues
Please use the GitHub Issue Tracker to report bugs or suggest enhancements. When reporting a bug, please include:
- A clear description of the issue.
- Steps to reproduce (if applicable).
- Expected vs. actual behavior.

---
*Note: This is an open-source release of the production algorithm. Some internal infrastructure details are intentionally omitted or simplified.*
