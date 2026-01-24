---
name: prepare-github-pr
description: Prepare and submit a pull request to a GitHub repository
---

# Prepare GitHub PR Skill

Prepare and submit pull requests to open source repositories.

## When to Use

Use when:
- Contributing to open source projects
- Submitting features or fixes
- Proposing documentation improvements

## Steps

### 1. Clone/Fork the Target Repo
```bash
# Clone the official repo
git clone https://github.com/ORG/REPO.git target-repo
cd target-repo

# Create feature branch
git checkout -b feat/your-feature-name
```

### 2. Make Your Changes

Follow the project's contribution guidelines if they exist.

### 3. Stage and Commit
```bash
git add -A
git commit -m "feat: descriptive title

## Summary
Brief description of changes

## Details
- Change 1
- Change 2

## Testing
How you tested the changes"
```

### 4. Fork on GitHub

1. Go to the repo on GitHub
2. Click "Fork" button
3. This creates your fork

### 5. Push to Your Fork
```bash
# Add your fork as remote
git remote add myfork https://github.com/YOUR_USERNAME/REPO.git

# Push your branch
git push myfork feat/your-feature-name
```

### 6. Create Pull Request

1. Go to your fork on GitHub
2. Click "Compare & pull request"
3. Fill in:
   - Clear title
   - Detailed description
   - Link to related issues
   - Screenshots if applicable
4. Submit

## PR Description Template

```markdown
## Summary

Brief description of what this PR does.

## Changes

- [ ] Change 1
- [ ] Change 2
- [ ] Change 3

## Why

Explain the motivation for these changes.

## Testing

How you tested the changes.

## Screenshots (if applicable)

## Checklist

- [ ] Code follows project style
- [ ] Tests pass
- [ ] Documentation updated
- [ ] No breaking changes
```

## Tips for Getting Merged

1. **Read CONTRIBUTING.md** first
2. **Keep PRs focused** - one feature/fix per PR
3. **Write clear descriptions**
4. **Respond to feedback promptly**
5. **Be patient** - maintainers are busy
6. **Build visibility** - share on social media

## After Submission

- Watch for review comments
- Make requested changes quickly
- Be professional and collaborative
- Thank reviewers for their time
