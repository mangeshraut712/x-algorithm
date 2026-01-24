---
name: rust-ci-fix
description: Fix Rust CI failures including build, test, clippy, and formatting issues
---

# Rust CI Fix Skill

This skill fixes common Rust CI failures systematically.

## When to Use

Use this skill when:
- GitHub Actions CI is failing
- Build errors occur
- Clippy warnings need fixing
- Formatting issues exist
- Dependency problems arise

## Steps

### 1. Identify the Problem
// turbo
```bash
# Check current build status
cargo build 2>&1 | head -50
```

### 2. Run Clippy to Find Issues
// turbo
```bash
cargo clippy --all-targets 2>&1 | head -100
```

### 3. Check Formatting
// turbo
```bash
cargo fmt --check 2>&1
```

### 4. Fix Formatting Automatically
```bash
cargo fmt
```

### 5. Update Dependencies if Needed
```bash
cargo update
```

### 6. Run Tests
// turbo
```bash
cargo test --verbose 2>&1 | head -100
```

### 7. Commit and Push Fixes
```bash
git add -A && git commit -m "fix: resolve CI failures" && git push origin main
```

## Common Issues and Solutions

### Missing Dependencies
- Add to Cargo.toml with proper versions
- Use workspace dependencies when available

### Clippy Warnings
- Address each warning specifically
- Use `#[allow(...)]` only as last resort

### Formatting Issues
- Run `cargo fmt` before committing
- Check import ordering

### Test Failures
- View test output with `cargo test -- --nocapture`
- Check for missing test dependencies

## Notes
- Always run locally before pushing
- Check the specific CI job that's failing
- Review the CI workflow file for context
