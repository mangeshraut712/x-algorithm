# Contributing to X Algorithm

First off, thank you for considering contributing to the X Algorithm project! 🎉

This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Style Guide](#style-guide)

## Our Principles
1. **Transparency**: Every change should be understandable and its impact on user experience should be clear.
2. **Performance**: This algorithm runs at massive scale. Efficiency is paramount.
3. **Diversity**: We strive for a feed that is diverse and representative of different perspectives.
4. **Safety**: Protecting users from spam, violence, and harmful content is built into the pipeline.

## Code of Conduct

This project adheres to a Code of Conduct. By participating, you are expected to uphold this code. Please read [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Getting Started

### Prerequisites

- **Rust** 1.75+ (install via [rustup](https://rustup.rs/))
- **Python** 3.11+ (for phoenix/ ML components)
- **uv** (for Python dependency management)
- **Node.js** 18+ (optional, for web tools development)
- **Git**

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/x-algorithm.git
   cd x-algorithm
   ```

3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/mangeshraut712/x-algorithm.git
   ```

## Development Setup

### Rust Components (`home-mixer`, `thunder`, `candidate-pipeline`)
The Rust components use standard Cargo. 
To build:
```bash
cargo build --workspace
```
To run tests:
```bash
cargo test --workspace
```

### ML Components (`phoenix`)
The ML models are implemented in Python using JAX and Haiku. We use `uv` for dependency management.
```bash
cd phoenix
uv sync
```
To run model tests:
```bash
uv run pytest
```

## Making Changes

### Branch Naming

Use descriptive branch names:

- `feature/add-scoring-feature` - New features
- `fix/correct-weight-calculation` - Bug fixes
- `docs/update-api-reference` - Documentation
- `refactor/simplify-pipeline` - Code refactoring
- `test/add-integration-tests` - Test additions

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add bookmark weight to scoring
fix: correct freshness decay calculation
docs: update API reference
test: add weighted scorer benchmarks
refactor: simplify candidate pipeline
chore: update dependencies
```

## Testing

### Rust Tests
```bash
cargo test --workspace
```

### Python/ML Tests
```bash
cd phoenix
uv run pytest
```

## Submitting Changes

### Pull Request Process

1. **Update your fork:**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Create a feature branch:**
   ```bash
   git checkout -b feature/my-feature
   ```

3. **Make your changes and commit:**
   ```bash
   git add .
   git commit -m "feat: description of change"
   ```

4. **Push to your fork:**
   ```bash
   git push origin feature/my-feature
   ```

5. **Open a Pull Request** on GitHub

### PR Requirements

- [ ] All tests pass (`cargo test` and `uv run pytest`)
- [ ] Code builds without warnings
- [ ] Documentation updated if needed
- [ ] Commit messages follow conventions
- [ ] PR description explains the changes

## Style Guide

- **Rust**: Use `cargo fmt` and `cargo clippy`.
- **Python**: Use `ruff` for linting and formatting.
- **Web**: 4-space indentation, ES6+ patterns.

---

Thank you for contributing! 🚀
