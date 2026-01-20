# Contributing to X Algorithm

Thank you for your interest in contributing to the X Algorithm project! This document provides guidelines and information for contributors.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Submitting Changes](#submitting-changes)
- [Code Style](#code-style)
- [Testing](#testing)
- [Documentation](#documentation)
- [Reporting Issues](#reporting-issues)

## 🤝 Code of Conduct

This project follows a code of conduct to ensure a welcoming environment for all contributors. By participating, you agree to:

- Be respectful and inclusive
- Focus on constructive feedback
- Accept responsibility for mistakes
- Show empathy towards other contributors
- Help create a positive community

## 🚀 Getting Started

### Prerequisites

- **Rust**: 1.70+ ([install](https://rustup.rs/))
- **Python**: 3.8+ ([install](https://python.org))
- **Git**: Latest version
- **Cargo**: Included with Rust

### Quick Setup

```bash
# Fork and clone the repository
git clone https://github.com/your-username/x-algorithm.git
cd x-algorithm

# Set up upstream remote
git remote add upstream https://github.com/mangeshraut712/x-algorithm.git

# Install dependencies and build
cd home-mixer && cargo build --release
```

## 💡 How to Contribute

### Types of Contributions

- 🐛 **Bug fixes** - Fix existing issues
- ✨ **Features** - Add new functionality
- 📚 **Documentation** - Improve docs or add examples
- 🧪 **Tests** - Add or improve test coverage
- 🎨 **UI/UX** - Improve user interfaces
- 🔧 **Tools** - Development tools and scripts

### Finding Issues to Work On

1. Check [open issues](https://github.com/mangeshraut712/x-algorithm/issues) labeled `good first issue`
2. Look for issues labeled `help wanted`
3. Check the [project board](https://github.com/mangeshraut712/x-algorithm/projects) for planned work

## 🛠️ Development Setup

### Rust Development

```bash
# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Run with optimizations
cargo build --workspace --release

# Check code quality
cargo clippy --workspace
cargo fmt --workspace --check
```

### Python Development (Phoenix)

```bash
cd phoenix

# Install dependencies
pip install -r requirements.txt

# Run tests
python -m pytest

# Run model training/inference
python run_retrieval.py
```

### Running the Full System

```bash
# Start all services
docker-compose up -d

# Or run individual components
./scripts/deploy.sh
```

## 📝 Submitting Changes

### 1. Create a Branch

```bash
# Create and switch to a feature branch
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/issue-number-description
```

### 2. Make Changes

- Follow the [code style guidelines](#code-style)
- Add tests for new functionality
- Update documentation as needed
- Ensure all tests pass

### 3. Commit Changes

```bash
# Stage your changes
git add .

# Commit with a clear message
git commit -m "feat: add user clustering optimization

- Implements k-means clustering for user segmentation
- Reduces personalization latency by 40%
- Adds comprehensive test coverage

Closes #123"
```

### 4. Push and Create PR

```bash
# Push your branch
git push origin feature/your-feature-name

# Create a Pull Request on GitHub
```

### Pull Request Guidelines

- **Title**: Use conventional commit format (e.g., "feat:", "fix:", "docs:")
- **Description**: Explain what changes and why
- **Screenshots**: Include for UI changes
- **Tests**: Ensure CI passes
- **Breaking Changes**: Clearly document any breaking changes

## 🎨 Code Style

### Rust Code Style

- Follow the [official Rust style guide](https://doc.rust-lang.org/style-guide/)
- Use `cargo fmt` to format code
- Use `cargo clippy` for linting
- Maximum line length: 100 characters

### Python Code Style

- Follow [PEP 8](https://pep8.org/)
- Use [Black](https://black.readthedocs.io/) for formatting
- Use [flake8](https://flake8.pycqa.org/) for linting

### Commit Messages

Use [conventional commits](https://conventionalcommits.org/):

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Testing
- `chore`: Maintenance

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run specific test
cargo test test_name

# Run with coverage
cargo tarpaulin --workspace
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test implementation
        assert_eq!(result, expected);
    }
}
```

### Test Coverage

- Aim for >80% code coverage
- Test edge cases and error conditions
- Include integration tests for complex features

## 📚 Documentation

### Code Documentation

```rust
/// Brief description of the function
///
/// # Arguments
/// * `param1` - Description of param1
/// * `param2` - Description of param2
///
/// # Returns
/// Description of return value
///
/// # Examples
/// ```
/// let result = my_function(1, 2);
/// assert_eq!(result, 3);
/// ```
pub fn my_function(param1: i32, param2: i32) -> i32 {
    // implementation
}
```

### Updating Documentation

- Update README.md for significant changes
- Add examples for new features
- Keep API documentation current
- Update deployment guides as needed

## 🐛 Reporting Issues

When reporting bugs, please include:

- **Clear title** describing the issue
- **Steps to reproduce** the problem
- **Expected vs actual behavior**
- **Environment details** (OS, Rust/Python versions)
- **Error messages** and stack traces
- **Screenshots** if applicable

## 📞 Getting Help

- **Discussions**: Use [GitHub Discussions](https://github.com/mangeshraut712/x-algorithm/discussions) for questions
- **Issues**: Use [GitHub Issues](https://github.com/mangeshraut712/x-algorithm/issues) for bugs/features
- **Documentation**: Check [docs/](docs/) first

## 🎉 Recognition

Contributors will be recognized in:
- Repository README
- Release notes
- Contributor acknowledgments

Thank you for contributing to X Algorithm! 🚀
