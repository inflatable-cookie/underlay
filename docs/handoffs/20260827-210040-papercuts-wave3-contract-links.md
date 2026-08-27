---
title: Papercuts wave 3 contract-link worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom / papercuts orchestrator
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260827-210040-papercuts-wave3-contract-links.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

Wave 2 split context tests and fixed detail-card CSS. Remaining Underlay
papercuts include machine-local `/Users/tom/Dev/projects/...` links in
active contracts. The operator approved papercuts wave 3.

You are the Underlay implementation worker. Add a docs QA check for
absolute local paths and convert the worst offenders. Do not start an
Effigy or Northstar skill rewrite.

## Why It Matters

Contract navigation is checkout-specific and the docs boundary already
forbids those links. Agents follow them into the wrong machine.

## Current State

- **Repository:** `/Users/tom/Dev/projects/underlay`
- **Planning branch:** `main`
- **Planning base commit:** `1dc9ddbdec93391e03d78d25be2d652b0e9f7a1c`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved
  to that SHA before this handoff was created.
- **Planning checkout:** clean before this handoff file was created.
- **Worker mode:** implementation worker dispatched by the orchestrator.
- **Worker branch:** `worker/papercuts-wave3-contract-links`
- **Worker worktree:** launcher worktree first. `.agents.local.env` was
  absent; ask before creating a manual fallback. Never use `/tmp`.
- **Ready work items, in order:**
  1. Active contracts retain machine-local evidence links
- **Out of scope:** Effigy release GitHub-publish; `--` test:unit
  forwarding (Effigy); skill JSON path (Effigy skill); Northstar missing
  batch-card template (Northstar repo); reference postgres volume docs
  (Underlay Reference); leftover local package trees; g09 front-door
  currentness.
- **Canonical refs:** `PAPERCUTS.md`; `docs/contracts/`; docs boundary QA.
- **Required validation:** a docs QA check fails on `/Users/` (or other
  absolute local) links in active contracts; converted Underlay targets
  are relative; sibling-repo targets are prose refs. `effigy qa:docs`.
- **PR URL:** pending
- **Merge authorisation:** absent; do not merge

## Boundaries

- Sweep `docs/contracts/` and any still-live architecture files the check
  actually hits. If the sweep explodes past contracts into all of docs,
  stop after the check plus contracts and report.
- Do not merge.

## Important Context

- Wave 1 already repointed some auth architecture links. This lane is the
  remaining absolute-path style.
- **Report to:** the operator.

## Suggested Next Move

Read this file, run the worktree preflight, then add the QA check and
convert contract offenders.

## Completion Protocol

### Before you start

1. Read this handoff. Run the four git identity commands.
2. Accept a clean dedicated non-`main` registered worktree.
3. Confirm `HEAD == origin/main` and ancestor
   `1dc9ddbdec93391e03d78d25be2d652b0e9f7a1c`.
4. Confirm this handoff exists in `HEAD`.

### When the assigned runway is complete

1. Run `effigy qa:docs`. Close the papercut. Push a PR. Do not merge.

### Review and merge path

Awaiting orchestrator review. Merge is operator-authorised only.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; the PR.

### Handoff closeout

If a sibling evidence link has no relative home, convert it to prose
rather than inventing a path.
