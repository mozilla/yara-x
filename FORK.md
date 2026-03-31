# Fork of VirusTotal/yara-x

This repository is a fork of [VirusTotal/yara-x](https://github.com/VirusTotal/yara-x).

## Keeping in sync with upstream

A [Sync Upstream](.github/workflows/sync_upstream.yaml) workflow runs automatically every working day (Monday–Friday at 08:00 UTC). It can also be triggered manually from the Actions tab.

When new commits are detected on `upstream/main`, the workflow:

1. Creates a branch named `sync/upstream-<short-sha>`
2. Merges the upstream changes into it
3. Opens a Pull Request targeting the `amo` branch

**To apply upstream changes, use the "Merge" button on the Pull Request.** No manual steps are needed in the normal case.

## Handling merge failures

If the merge fails due to a conflict, the workflow will open a GitHub issue instead.

The issue links to the failed workflow run for context. To resolve it manually:

```bash
# Add the upstream remote if you haven't already
git remote add upstream https://github.com/VirusTotal/yara-x.git
git fetch upstream

# Make sure you have the latest changes locally
git checkout amo
git pull origin amo
git checkout -b sync-upstream

# Merge upstream/main and resolve conflicts
git merge --no-ff upstream/main

# After resolving all conflicts, commit and push
git add .
# You can adapt the commit message
git commit -m "Resolve merge conflicts"
git push origin sync-upstream
```

Then open a Pull Request from `sync-upstream` into `amo` and merge it once CI passes. You can reference the issue number in the PR description (e.g., `Fixes #123`).
