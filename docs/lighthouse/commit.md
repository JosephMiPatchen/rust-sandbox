# Lighthouse Contribution Workflow

## Understanding Open Source vs Internal Development

### Why Forking is Required

Unlike internal development (e.g., at Amazon where you have write access to repositories), open source projects require forking because:

- **No write permissions**: Random contributors cannot push directly to `sigp/lighthouse`
- **Trust model**: You must prove your code is good before it gets merged
- **Security**: Prevents malicious actors from pushing harmful code
- **Your fork = your staging area**: It's where you have write permissions

**Core maintainers** of Lighthouse have write access and can push branches directly, but still use PRs for code review.

### The Three-Repository Model

```
[sigp/lighthouse] (upstream)
   ↓ You READ from here (sync)
   
[your-username/lighthouse] (origin - on GitHub)
   ↑ You WRITE here (push)
   
[local clone]
   ↑ You WORK here (commit)
```

---

## Why Feature Branches?

**Q: Why can't I just work on `unstable` directly?**

**A:** Feature branches keep your work isolated and your `unstable` branch clean:

- **Clean syncing**: Your `unstable` can fast-forward merge with upstream without merge commits
- **Multiple PRs**: You can work on several features simultaneously
- **Isolation**: Changes in one PR don't affect another
- **Easy cleanup**: Delete the branch after merge

If you work directly on `unstable`, syncing with upstream creates merge commits and tangles your work with others' changes.

---

## GitHub PR Conventions

GitHub's PR system expects:
- **Separate feature branches** for each PR
- **Descriptive branch names**: `fix-toctou-vulnerability` not `patch1` or `fix`
- **One logical change per PR**: Don't mix unrelated fixes
- **Rebase before PR**: Ensures your changes apply cleanly to current codebase

The Lighthouse project specifically requires:
- **Always branch from `unstable`**
- **Always target `unstable`** in your PR
- **Rebase, don't merge**: Keeps history linear and clean

---

## 1. Setup (One-Time Only)

```bash
git clone https://github.com/your-username/lighthouse.git
cd lighthouse
git remote add upstream https://github.com/sigp/lighthouse.git
```

**What this does:**
- `origin` = your fork (where you push)
- `upstream` = sigp/lighthouse (source of truth)

---

## 2. Start New Feature/Fix

```bash
git checkout unstable
git pull upstream unstable
git checkout -b fix-descriptive-name
```

**Why sync first?**
- Ensures you're branching from the latest code
- Avoids conflicts later
- Your local `unstable` mirrors upstream

---

## 3. Make Changes

```bash
# Edit files...
cargo test
```

**Best practices:**
- Test locally before committing
- Make atomic commits (one logical change per commit)
- Follow the project's code style

---

## 4. Commit

```bash
git add <files>
git commit -m "Brief description

Detailed explanation

Fixes #ISSUE_NUMBER"
```

**Good commit messages:**
- First line: Brief summary (50 chars or less)
- Blank line
- Detailed explanation of what and why
- Reference issue number with `Fixes #123`

---

## 5. Sync With Upstream (CRITICAL!)

```bash
git fetch upstream
git rebase upstream/unstable
```

**What rebase does:**
```
Before:  upstream A---B---C---D---E
         your fix A---B---C---X---Y

After:   upstream A---B---C---D---E
         your fix A---B---C---D---E---X'---Y'
```

It replays your commits on top of the latest upstream code.

**If conflicts occur:**
```bash
# Fix conflicts in files, then:
git add <fixed-files>
git rebase --continue
```

**Why this step matters:**
- Catches conflicts before maintainers see them
- Ensures your changes work with current codebase
- Prevents the "47 extra commits" problem (PR #8016)
- Makes your PR clean and reviewable

---

## 6. Push to Your Fork

```bash
git push origin fix-descriptive-name
```

**Note:** This pushes to YOUR fork on GitHub (origin), not the main repo (upstream).

If you've already pushed and rebased:
```bash
git push origin fix-descriptive-name --force-with-lease
```

---

## 7. Create Pull Request

1. Go to your fork on GitHub
2. Click "Compare & pull request"
3. **Ensure:** `sigp/lighthouse:unstable` ← `your-username/lighthouse:fix-descriptive-name`
4. Write description and submit

**PR description template:**
```markdown
## Description
Brief summary of the change

## Proposed Changes
- Change 1
- Change 2

## Issue Addressed
Fixes #123

## Testing
How you tested the changes
```

---

## 8. If Requested Changes

```bash
# Make changes...
git add <files>
git commit -m "Address review feedback"
git push origin fix-descriptive-name
```

The PR will automatically update with new commits.

**You don't need to:**
- Close and recreate the PR
- Force push (unless you're amending commits)

---

## 9. After Merge

```bash
git checkout unstable
git pull upstream unstable
git branch -d fix-descriptive-name
```

**What happens:**
1. Your local `unstable` syncs with upstream (now includes YOUR merged work!)
2. Feature branch is deleted (no longer needed)
3. You're ready to start the next feature

**Note:** Your local `unstable` being "stale" during development is normal and fine. You only sync it when starting new work or after a merge.

---

## Quick Reference: Repository States

```
Start:     unstable synced → branch off → work on feature
           (unstable gets "stale" - that's OK!)
           
Finish:    PR merged → sync unstable → delete feature branch
           (unstable updated, ready for next feature)
```

---

## Common Pitfalls

❌ **Branching from `stable` instead of `unstable`**
❌ **Not rebasing before creating PR**
❌ **Working directly on `unstable`**
❌ **Forgetting to sync before starting new work**

✅ **Always branch from `unstable`**
✅ **Always rebase before pushing**
✅ **Use descriptive branch names**
✅ **Sync `unstable` before each new feature**