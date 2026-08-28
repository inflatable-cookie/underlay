---
title: Papercuts wave 6 vendored skill JSON worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom / papercuts orchestrator
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260828-182030-papercuts-wave6-skill-json.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

Wave 5 closed attention-marker CLI, task-arg `--`, and postgres volume
docs against PATH Effigy PR 48. Skill JSON stayed open because this
repo's vendored skill still queries `.result.payload.tasks[]`. Effigy
PR 49 retargeted the upstream examples to `.result.catalog_tasks[]`.

You are the Underlay implementation worker. Refresh the vendored Effigy
skill from that upstream fix and close the copy. Do not re-implement
task inventory.

## Why It Matters

Agents in this repo still run a jq path that fails before they can
filter task ownership.

## Current State

- **Repository:** `/Users/tom/Dev/projects/underlay`
- **Planning branch:** `main`
- **Planning base commit:** `9f974de2c0c6414f61412e12dafc41fd09237f59`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved
  to that SHA before this handoff was created.
- **Planning checkout:** clean before this handoff file was created.
- **Worker mode:** implementation worker dispatched by the orchestrator.
- **Worker branch:** `worker/papercuts-wave6-skill-json`
- **Worker worktree:** launcher worktree first. `.agents.local.env` was
  absent; ask before creating a manual fallback. Never use `/tmp`.
- **Ready work items, in order:**
  1. Effigy task-inventory JSON example uses a stale payload path —
     update `.agents/skills/effigy/SKILL.md` and the matching
     `references/` jq examples to `.result.catalog_tasks[]` (live field
     is `.task`) from sibling Effigy `552ef1b93283` (PR 49). Close the
     papercut
- **Out of scope:** GitHub Release on execute (keep open); Northstar
  batch-card template in `~/.claude/skills` (installed-skill refresh,
  not this repo); editing Effigy source.
- **Canonical refs:** `PAPERCUTS.md`;
  `.agents/skills/effigy/SKILL.md`; sibling Effigy
  `skills/effigy/SKILL.md` at `552ef1b93283`.
- **Required validation:** vendored examples match live
  `effigy --json tasks` shape. Cite the Effigy SHA you copied from.
- **PR URL:** pending
- **Merge authorisation:** absent; do not merge

## Boundaries

- Copy the upstream skill examples. Do not invent a second JSON schema.
- Do not merge.

## Important Context

- Wave 5 already proved live `effigy --json tasks` returns
  `catalog_tasks`. This lane is the vendored copy.
- **Report to:** the operator.

## Suggested Next Move

Read this file, run the worktree preflight, then retarget the vendored
jq examples from sibling Effigy PR 49.

## Completion Protocol

### Before you start

1. Read this handoff. Run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. Accept a clean dedicated non-`main` registered worktree. Record the
   actual path/branch.
3. Confirm `HEAD == origin/main` and ancestor
   `9f974de2c0c6414f61412e12dafc41fd09237f59`.
4. Confirm this handoff exists in `HEAD`.

### When the assigned runway is complete

1. Update `PAPERCUTS.md`. Push a PR. Do not merge.

### Review and merge path

Awaiting orchestrator review. Merge is operator-authorised only.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; the PR.

### Handoff closeout

Leave GitHub Release and the installed-Northstar-template gap open.
