# Contributing to X Algorithm

First off, thank you for considering contributing to this project! 🎉

This project aims to make the X (Twitter) algorithm accessible, understandable, and useful for everyone.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Style Guidelines](#style-guidelines)
- [Recognition](#recognition)

---

## 📜 Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](../CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

---

## 🤝 How Can I Contribute?

### 🐛 Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates.

When creating a bug report, include:
- **Clear title** describing the issue
- **Steps to reproduce** the behavior
- **Expected behavior** vs actual behavior
- **Environment details** (OS, Rust version, etc.)
- **Code snippets** if applicable

### 💡 Suggesting Enhancements

Enhancement suggestions are welcome! Please:
- Use a **clear, descriptive title**
- Provide a **detailed description** of the proposed feature
- Explain **why this would be useful**
- Include **examples** if possible

### 📝 Documentation Improvements

Documentation is crucial. You can help by:
- Fixing typos and grammar
- Adding examples and clarifications
- Translating documentation
- Creating tutorials or guides

### 💻 Code Contributions

#### Good First Issues

Look for issues labeled `good first issue` - these are great for newcomers!

#### Areas We Need Help

| Area | Description | Difficulty |
|------|-------------|------------|
| **Documentation** | Improve README, add examples | Easy |
| **Tests** | Add unit tests, integration tests | Medium |
| **Visualizations** | Create algorithm visualizations | Medium |
| **Performance** | Optimize scoring algorithms | Hard |
| **New Features** | Implement missing components | Hard |

---

## 🛠️ Development Setup

### Prerequisites

- Rust 1.70+ ([install](https://rustup.rs/))
- Git

### Setup Steps

```bash
# Clone the repository
git clone https://github.com/mangeshraut712/x-algorithm.git
cd x-algorithm

# Build the project
cargo build

# Run tests
cargo test

# Run clippy (linting)
cargo clippy

# Format code
cargo fmt
```

### Project Structure

```
x-algorithm/
├── candidate-pipeline/   # Core framework (start here!)
├── home-mixer/           # Timeline service
├── thunder/              # Post store
├── tools/                # Utilities and calculators
└── docs/                 # Documentation
```

### Which Crate to Work On?

| Crate | Status | Best For |
|-------|--------|----------|
| `candidate-pipeline` | ✅ Fully Working | New contributors |
| `home-mixer` | 🔧 In Progress | Experienced Rust devs |
| `thunder` | 🔧 In Progress | Systems programming focus |

---

## 🔄 Pull Request Process

### 1. Fork & Branch

```bash
# Fork the repo on GitHub, then:
git clone https://github.com/YOUR_USERNAME/x-algorithm.git
cd x-algorithm

# Create a feature branch
git checkout -b feature/your-feature-name
```

### 2. Make Changes

- Write clean, documented code
- Add tests for new functionality
- Update documentation if needed

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run clippy
cargo clippy -- -D warnings

# Format code
cargo fmt
```

### 4. Commit

Use clear commit messages:

```
feat: add new scoring algorithm for video content

- Implement VQV (Video Quality View) scoring
- Add tests for video duration threshold
- Update documentation
```

Commit message prefixes:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation
- `test:` - Tests
- `refactor:` - Code refactoring
- `perf:` - Performance improvement

### 5. Push & Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub with:
- Clear description of changes
- Link to related issue (if any)
- Screenshots (if UI changes)

### 6. Review Process

- Maintainers will review your PR
- Address any feedback
- Once approved, it will be merged!

---

## 📏 Style Guidelines

### Rust Code Style

```rust
// Good: Clear, documented code
/// Computes the weighted score for a candidate post.
///
/// # Arguments
/// * `candidate` - The post candidate to score
///
/// # Returns
/// The weighted engagement score
fn compute_weighted_score(candidate: &PostCandidate) -> f64 {
    // Implementation
}

// Good: Meaningful variable names
let reply_weight = 27.0;
let profile_click_weight = 12.0;

// Bad: Unclear names
let w1 = 27.0;
let w2 = 12.0;
```

### Documentation Style

- Use clear, concise language
- Include code examples
- Explain the "why," not just the "what"
- Keep README files updated

### Commit Style

- Use present tense ("add feature" not "added feature")
- Use imperative mood ("move cursor" not "moves cursor")
- Limit first line to 72 characters
- Reference issues when applicable

---

## 🏆 Recognition

Contributors are recognized in multiple ways:

### Contributors List

All contributors are listed in our README and CONTRIBUTORS.md file.

### Types of Contributions Recognized

- 💻 Code
- 📖 Documentation
- 🐛 Bug reports
- 💡 Ideas
- 🎨 Design
- 📢 Talks/Blog posts
- 🔧 Tools

### Hall of Fame

Significant contributors may be featured in:
- Project README
- Release notes
- Social media shoutouts

---

## ❓ Questions?

- Open a [Discussion](https://github.com/mangeshraut712/x-algorithm/discussions)
- Check existing [Issues](https://github.com/mangeshraut712/x-algorithm/issues)
- Read the [Documentation](./ARCHITECTURE.md)

---

## 📄 License

By contributing, you agree that your contributions will be licensed under the Apache 2.0 License.

---

**Thank you for contributing! 🙏**

Every contribution, no matter how small, makes this project better.
