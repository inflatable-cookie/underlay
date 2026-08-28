# 2026-08-28 - Papercuts Wave 4 Merge And Leftover-Tree Closeout

## Outcome

Closed three Underlay papercuts on the worker runway:

1. `gh pr merge --delete-branch` worktree false failure
2. retired top-level package leftovers after workspace-shape fast-forwards
3. g09 front-door currentness (verified already aligned; no checker added)

## Changes

- `scripts/merge-pr-closeout.sh` — require caller `--reviewed-head`, compare
  live provider head before merge, merge with `--match-head-commit` and `-R`,
  verify that exact OID after merge, then plan local cleanup only when the
  local tip matches
- `scripts/lib/merge-pr-closeout-cleanup.ts` — non-mutating pre-merge assert,
  post-merge verify, and cleanup planners with fixture tests, including
  review-to-merge head change before wrapper invocation
- `docs/guides/173-worker-pr-merge-closeout.md` — operator/orchestrator merge
  closeout contract
- workspace-shape `retired-top-level-package` rule — inventory leftover
  top-level dirs that mirror live `apps/*` or `packages/*` members; suggest
  POSIX-quoted `rm -rf --` only for disposable leftover children, otherwise
  inspect/relocate

## g09 Currentness Evidence

- generation README Status: `complete`
- queue entries: 62; all `[x]`
- card Status mismatches vs checkboxes: 0
- card Status counts: `{ complete: 62 }`
- roadmap / generation-index / product-guardrails already show no active
  generation

## Validation

- `effigy check:workspace-shape`
- `./scripts/merge-pr-closeout.sh --help`
- `git diff --check`

## PR

https://github.com/inflatable-cookie/underlay/pull/16

## Boundaries

- no merge
- no operator file deletion
- Effigy attention-marker / `--` test args / skill JSON / batch-card template /
  release GitHub publish / reference postgres volume docs remain out of scope

## Next Task

Orchestrator review of the PR. Merge is operator-authorised only.
