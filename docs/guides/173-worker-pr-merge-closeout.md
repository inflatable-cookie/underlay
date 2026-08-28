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

The wrapper captures the reviewed `headRefOid`, merges with
`--match-head-commit` and `-R`, verifies the merged PR still records that exact
OID, then plans local cleanup against that reviewed OID.

Equivalent raw GitHub CLI flags:

```bash
REVIEWED=$(gh pr view <pr-number> -R OWNER/REPO --json headRefOid --jq .headRefOid)
gh pr merge <pr-number> --squash --delete-branch \
  --match-head-commit "$REVIEWED" -R OWNER/REPO
gh pr view <pr-number> -R OWNER/REPO --json state,headRefOid
```

A `MERGED` state with the same `headRefOid` is the merge outcome. Local
worktree/branch cleanup is a separate step.

## Local cleanup after merge

Destructive cleanup is safe only when the local branch tip equals the reviewed
provider `headRefOid`. If the tips diverge, inspect manually — the local branch
may hold commits added after the merged head.

When tips match and a worktree still holds the branch:

```bash
git worktree remove /path/to-worker-worktree
git branch -D <head-branch>
```

The wrapper compares OIDs and prints those commands only on a match. It does
not delete operator files or worktrees.

## Boundaries

- Merge remains operator-authorised. Workers create PRs; they do not merge.
- This guide does not change review gates or required checks.
