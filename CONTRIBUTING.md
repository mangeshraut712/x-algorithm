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

## Code of Conduct

This project adheres to a Code of Conduct. By participating, you are expected to uphold this code. Please read [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Getting Started

### Prerequisites

- **Rust** 1.75+ (install via [rustup](https://rustup.rs/))
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

### Build the Project

```bash
# Build all crates
cargo build --workspace

# Build in release mode
cargo build --workspace --release
```

### Run Tests

```bash
# Run all tests
cargo test --workspace

# Run tests with output
cargo test --workspace -- --nocapture

# Run specific test
cargo test test_weighted_scorer
```

### Run the Services

```bash
# HomeMixer HTTP Server
cargo run -p home-mixer

# Thunder Service
cargo run -p thunder

# With logging
RUST_LOG=info cargo run -p home-mixer
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

**Examples:**

```bash
git commit -m "feat: add new engagement heatmap tool"
git commit -m "fix: resolve ownership error in user_clusters"
git commit -m "docs: add API reference for scoring endpoint"
```

## Testing

### Unit Tests

Add unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_calculation() {
        // Your test here
    }

    #[tokio::test]
    async fn test_async_function() {
        // Async test here
    }
}
```

### Integration Tests

Add integration tests in `tests/` directories:

```rust
// home-mixer/tests/integration_tests.rs
#[tokio::test]
async fn test_end_to_end_scoring() {
    // Full pipeline test
}
```

### Benchmarks

Run benchmarks with:

```bash
cargo bench -p home-mixer
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

- [ ] All tests pass (`cargo test --workspace`)
- [ ] Code builds without warnings (`cargo build --workspace`)
- [ ] Documentation updated if needed
- [ ] Commit messages follow conventions
- [ ] PR description explains the changes

## Style Guide

### Rust Code Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` to format code
- Use `cargo clippy` for linting
- Document public APIs with doc comments

```rust
/// Calculate the weighted score for a post candidate.
///
/// # Arguments
///
/// * `candidate` - The post candidate with probability scores
/// * `weights` - The scoring weights configuration
///
/// # Returns
///
/// The computed weighted score as a f64
pub fn calculate_score(candidate: &PostCandidate, weights: &Weights) -> f64 {
    // Implementation
}
```

### HTML/JavaScript Style

- Use 4-space indentation
- Follow modern ES6+ patterns
- Keep CSS modular with CSS custom properties
- Ensure accessibility (proper labels, ARIA attributes)

### Documentation

- Keep README.md up to date
- Document API changes in docs/API_REFERENCE.md
- Add inline comments for complex algorithms

## Areas for Contribution

### Good First Issues

Look for issues labeled `good-first-issue` for beginner-friendly tasks.

### Wanted Contributions

- **New Tools**: Interactive web tools for the algorithm
- **Performance**: Optimization of scoring pipelines
- **Testing**: More comprehensive test coverage
- **Documentation**: Improved explanations and examples
- **Visualization**: Better visual representations of the algorithm

## Questions?

- Open a [GitHub Discussion](https://github.com/mangeshraut712/x-algorithm/discussions)
- Check existing [Issues](https://github.com/mangeshraut712/x-algorithm/issues)

---

Thank you for contributing! 🚀
