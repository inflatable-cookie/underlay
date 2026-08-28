---
title: Papercuts wave 4 merge and leftover-tree worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: in-review
owner: Tom / papercuts orchestrator
created: 2026-08-27
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260827-235350-papercuts-wave4-merge-closeout.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

Wave 4 closed three Underlay papercuts: merge closeout that does not treat
worktree branch deletion as provider merge failure, retired top-level package
inventory in workspace-shape, and g09 front-door currentness (verified
aligned). Attention-marker CLI and `--` test args remain Effigy wave 4.

## Why It Matters

Automation mistakes local branch-cleanup failure for provider merge
failure. Local checkouts still look polyrepo-shaped after the migration
merges.

## Current State

- **Repository:** `/Users/tom/Dev/projects/underlay`
- **Worker branch:** `t3code/papercuts-wave4-merge-closeout`
- **Worker worktree:** `/Users/tom/.t3/worktrees/underlay/t3code-32efad7a`
- **Implementation status:** runway complete; worker PR opened and under
  orchestrator review. Do not start a second implementation worker.
- **Ready work items:** none remaining on this handoff. The three assigned
  papercuts are closed in `PAPERCUTS.md`.
- **Out of scope:** attention-marker CLI (Effigy wave 4); `--` test:unit
  (Effigy); skill JSON (Northstar wave 4); batch-card template (Northstar);
  GitHub Release on execute (Effigy protocol); reference postgres volume
  docs (Underlay Reference).
- **Canonical refs:** `PAPERCUTS.md`; `docs/guides/173-worker-pr-merge-closeout.md`;
  workspace-shape checker; PR16.
- **PR URL:** https://github.com/inflatable-cookie/underlay/pull/16
- **Merge authorisation:** absent; do not merge until the operator authorises

## Boundaries

- Do not delete operator files from existing checkouts. Inventory and
  commands only.
- Do not merge without explicit operator authorisation.
- Do not re-dispatch implementation for this handoff while status is
  `in-review`.

## Important Context

- PR13 merge already hit the worktree-delete failure.
- **Report to:** the operator.

## Suggested Next Move

Orchestrator re-review of PR16. After an approve-for-merge verdict and
explicit operator merge authorisation, merge with the exact reviewed head:

```bash
./scripts/merge-pr-closeout.sh 16 --reviewed-head <reviewed-sha> --squash
```

Then close this handoff.

## Completion Protocol

### Before you start

Implementation preflight is complete for this handoff. New agents must not
re-run the worker runway. If asked to continue this file, treat it as
review/merge coordination only.

### When the assigned runway is complete

1. Closed finished papercuts. Pushed PR16. Do not merge.

### Review and merge path

Awaiting orchestrator review. Merge is operator-authorised only.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; the PR.

### Handoff closeout

g09 currentness was already aligned; that entry was closed with evidence and
no checker was added.
