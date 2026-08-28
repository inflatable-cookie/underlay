# 173 - Worker PR Merge Closeout

Status: active
Audience: operators and orchestrators merging worker PRs

## Problem

`gh pr merge --delete-branch` can exit non-zero after a successful provider
merge when the PR head branch is still checked out in a registered worktree.
Automation that treats that exit code as merge failure will retry or report the
wrong outcome.

## Required closeout command

Prefer the repo wrapper:

```bash
./scripts/merge-pr-closeout.sh <pr-number> --squash
```

Equivalent raw GitHub CLI flags — always pass `-R` so local branch deletion is
skipped while remote deletion still runs:

```bash
gh pr merge <pr-number> --squash --delete-branch -R OWNER/REPO
```

Then confirm provider state separately if needed:

```bash
gh pr view <pr-number> -R OWNER/REPO --json state,mergedAt
```

A `MERGED` state is the merge outcome. Local worktree/branch cleanup is a
separate step.

## Local cleanup after merge

Do not force-delete a branch that still belongs to a worktree. Remove the
worktree first, then the local branch from the primary checkout:

```bash
git worktree remove /path/to/worker-worktree
git branch -D <head-branch>
```

The wrapper inventories holding worktrees and prints those commands. It does
not delete operator files or worktrees.

## Boundaries

- Merge remains operator-authorised. Workers create PRs; they do not merge.
- This guide does not change review gates or required checks.
