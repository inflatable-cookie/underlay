---
title: Papercuts wave 4 merge and leftover-tree worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom / papercuts orchestrator
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260827-235350-papercuts-wave4-merge-closeout.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

Wave 3 banned machine-local contract paths. Remaining Underlay papercuts
include `gh pr merge --delete-branch` failing after a successful merge
because the local branch still belongs to a worktree, leftover
`apps/`/`packages/` trees after workspace-shape fast-forwards, and g09
front-door currentness. Attention-marker CLI and `--` test args belong to
Effigy wave 4.

You are the Underlay implementation worker. Do not start an Effigy worker
from this repo.

## Why It Matters

Automation mistakes local branch-cleanup failure for provider merge
failure. Local checkouts still look polyrepo-shaped after the migration
merges.

## Current State

- **Repository:** `/Users/tom/Dev/projects/underlay`
- **Planning branch:** `main`
- **Planning base commit:** `350da4c27fcbbe1acdb2ac3d8ea5ec3dc5504812`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved
  to that SHA before this handoff was created.
- **Planning checkout:** clean before this handoff file was created.
- **Worker mode:** implementation worker dispatched by the orchestrator.
- **Worker branch:** `worker/papercuts-wave4-merge-closeout`
- **Worker worktree:** launcher first. `.agents.local.env` was absent;
  ask before creating a manual fallback. Never use `/tmp`.
- **Ready work items, in order:**
  1. `gh pr merge --delete-branch` reports failure after a successful
     merge
  2. Workspace-shape fast-forwards leave retired local package trees
     behind
  3. Northstar refresh found multi-week front-door drift after g09
     closeout — only a cheap `qa:northstar` checkbox/Status check, not a
     historical rewrite
- **Out of scope:** attention-marker CLI (Effigy wave 4); `--` test:unit
  (Effigy); skill JSON (Northstar wave 4); batch-card template (Northstar);
  GitHub Release on execute (Effigy protocol); reference postgres volume
  docs (Underlay Reference).
- **Canonical refs:** `PAPERCUTS.md`; worker merge/closeout docs;
  workspace-shape checker; generation README vs card Status.
- **Required validation:** merge closeout does not treat worktree branch
  delete failure as merge failure (script or documented `gh` flags);
  a check inventories retired top-level package paths and prints safe
  cleanup; optional cheap currentness check if it stays small.
- **PR URL:** pending
- **Merge authorisation:** absent; do not merge

## Boundaries

- Do not delete operator files from existing checkouts. Inventory and
  commands only.
- Do not merge.

## Important Context

- PR13 merge already hit this worktree-delete failure.
- **Report to:** the operator.

## Suggested Next Move

Read this file, run the worktree preflight, then find the merge closeout
command this repo documents.

## Completion Protocol

### Before you start

1. Read this handoff. Run the four git identity commands.
2. Accept a clean dedicated non-`main` registered worktree.
3. Confirm `HEAD == origin/main` and ancestor
   `350da4c27fcbbe1acdb2ac3d8ea5ec3dc5504812`.
4. Confirm this handoff exists in `HEAD`.

### When the assigned runway is complete

1. Close finished papercuts. Push a PR. Do not merge.

### Review and merge path

Awaiting orchestrator review. Merge is operator-authorised only.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; the PR.

### Handoff closeout

If g09 currentness is already aligned, close that entry with evidence
and skip the checker.
