---
title: Papercuts wave 7 GitHub Release protocol worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: awaiting-review
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

Worker aligned Underlay's release protocol and post-release checklist
with that split: execute does not create the GitHub Release; the
operator publishes it separately. Did not teach Effigy to create GitHub
Releases. Did not run release mutations.

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
- **Worker branch:** `t3code/papercuts-wave7-release-protocol` (accepted;
  differs from placeholder `worker/papercuts-wave7-release-protocol`)
- **Worker worktree:** `/Users/tom/.t3/worktrees/underlay/t3code-0f634722`
  (launcher worktree). `.agents.local.env` absent; no fallback created.
- **Work items:**
  1. Effigy release execute omits the promised GitHub Release — **closed**.
     Vendored `release-protocol.md` / `footguns.md` now treat execute as
     tag-only and require operator `gh release create` (or equivalent)
     before tagged consumer smoke. Historical-tag triage no longer cites
     the old execute-creates-release claim.
- **Out of scope kept open:** installed-Northstar batch-card template gap
  (`~/.claude/skills`).
- **Canonical refs:** `PAPERCUTS.md`;
  `.agents/skills/effigy/references/release-protocol.md`;
  `.agents/skills/effigy/references/footguns.md`.
- **Validation:** live Underlay protocol no longer says execute creates
  the GitHub Release; `effigy --json papercuts --all` reports non-empty
  friction/impact/possible_fix/resolution for the closed item; no
  release mutation ran.
- **PR URL:** https://github.com/inflatable-cookie/underlay/pull/19
- **Merge authorisation:** absent; do not merge

## Boundaries

- Docs/protocol only. Do not implement provider publication in Effigy.
- Never run release mutations. Do not merge.

## Important Context

- The other possible fix (execute creates the GitHub Release) stayed out
  of scope. This wave picked the checklist split.
- **Report to:** the operator.

## Suggested Next Move

Orchestrator: re-review PR 19. Leave the installed-Northstar batch-card
template gap open.

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

Runway complete. Protocol split landed on PR 19. Leave the
installed-Northstar batch-card template gap open.
