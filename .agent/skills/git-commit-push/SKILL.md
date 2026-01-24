---
name: git-commit-push
description: Stage, commit with conventional message, and push changes to remote
---

# Git Commit & Push Skill

Quickly commit and push changes with proper conventional commit messages.

## When to Use

Use this skill after making code changes that need to be committed and pushed.

## Steps

### 1. Check Status
// turbo
```bash
git status
```

### 2. Stage All Changes
// turbo
```bash
git add -A
```

### 3. Create Commit

Use conventional commit format:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation
- `refactor:` - Code refactoring
- `test:` - Tests
- `chore:` - Maintenance

```bash
git commit -m "TYPE: brief description

- Detail 1
- Detail 2"
```

### 4. Push to Remote
```bash
git push origin BRANCH_NAME
```

## Commit Message Template

```
TYPE(scope): short description

## What Changed
- Change 1
- Change 2

## Why
Brief explanation of motivation
```

## Common Patterns

### Feature Addition
```bash
git commit -m "feat: add score calculator tool

- Interactive sliders for engagement probabilities
- Real-time score calculation
- Visual tier indicators"
```

### Bug Fix
```bash
git commit -m "fix: resolve CI build failures

- Update dependency versions
- Fix formatting issues
- Address clippy warnings"
```

### Documentation
```bash
git commit -m "docs: add architecture documentation

- System overview diagram
- Component descriptions
- Scoring formula explanation"
```

## Notes
- Always pull before push if working with others
- Use `--force` only when absolutely necessary
- Check branch name before pushing
