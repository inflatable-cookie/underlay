---
title: Papercuts wave 7 GitHub Release protocol worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom / papercuts orchestrator
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260828-201730-papercuts-wave7-release-protocol.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

Wave 6 retargeted the vendored Effigy skill. One remaining Underlay
copy: `effigy release execute --yes` reported a complete release without
a GitHub Release, until the operator ran `gh release create`.

You are the Underlay implementation worker. Align this repo's release
protocol and post-release checklist with that split: execute does not
create the GitHub Release; the operator publishes it separately. Do not
teach Effigy to create GitHub Releases. Do not run release mutations.

## Why It Matters

A successful execute still reads as "the public release exists" when
only the git tag landed.

## Current State

- **Repository:** `/Users/tom/Dev/projects/underlay`
- **Planning branch:** `main`
- **Planning base commit:** `14c97eb894884fa69f0a6991e66da28d54976c79`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved
  to that SHA before this handoff was created.
- **Planning checkout:** clean before this handoff file was created.
- **Worker mode:** implementation worker dispatched by the orchestrator.
- **Worker branch:** `worker/papercuts-wave7-release-protocol`
- **Worker worktree:** launcher worktree first. `.agents.local.env` was
  absent; ask before creating a manual fallback. Never use `/tmp`.
- **Ready work items, in order:**
  1. Effigy release execute omits the promised GitHub Release — update
     Underlay release protocol / post-release checklist so they declare
     `gh release create` (or equivalent) as a separate operator step
     after execute. Close this copy if the local protocol no longer
     claims execute publishes the GitHub Release. If Effigy's own
     protocol still promises that create, keep a pointer at Effigy but
     still fix Underlay's wording
- **Out of scope:** making `effigy release execute` create GitHub
  Releases; editing Effigy; running `release prepare/execute`; the
  installed-Northstar batch-card template gap (`~/.claude/skills`).
- **Canonical refs:** `PAPERCUTS.md`; release protocol / post-release
  checklist (find the live files; do not invent a second protocol).
- **Required validation:** the live Underlay protocol no longer says
  execute creates the GitHub Release. No release mutation ran.
- **PR URL:** pending
- **Merge authorisation:** absent; do not merge

## Boundaries

- Docs/protocol only. Do not implement provider publication in Effigy.
- Never run release mutations. Do not merge.

## Important Context

- The other possible fix (execute creates the GitHub Release) is
  explicitly out of scope. This wave picks the checklist split.
- **Report to:** the operator.

## Suggested Next Move

Read this file, run the worktree preflight, then find the live release
protocol and make the provider-publication step explicit.

## Completion Protocol

### Before you start

1. Read this handoff. Run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. Accept a clean dedicated non-`main` registered worktree. Record the
   actual path/branch.
3. Confirm `HEAD == origin/main` and ancestor
   `14c97eb894884fa69f0a6991e66da28d54976c79`.
4. Confirm this handoff exists in `HEAD`.

### When the assigned runway is complete

1. Update `PAPERCUTS.md`. Push a PR. Do not merge.

### Review and merge path

Awaiting orchestrator review. Merge is operator-authorised only.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; the PR.

### Handoff closeout

Leave the installed-Northstar batch-card template gap open.
