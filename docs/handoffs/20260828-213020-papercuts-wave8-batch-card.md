---
title: Papercuts wave 8 batch-card template closeout worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom / papercuts orchestrator
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260828-213020-papercuts-wave8-batch-card.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

Wave 7 split GitHub Release out of execute. The last Underlay copy is
the missing batch-card template. Northstar PR 6 already ships
`skills/northstar/assets/templates/docs/specs/templates/batch-card-template.md`.
Wave 5 left this open because `~/.claude/skills/northstar` still lacked
the file.

You are the Underlay implementation worker. Close this copy if compile-roadmaps
in this checkout can use the skill-shipped path, and record that refreshing
the installed skill is an operator machine step. Do not edit
`~/.claude/skills`. Do not edit Northstar.

## Why It Matters

An open copy still sends workers into Northstar asset work that already
landed in the source package.

## Current State

- **Repository:** `/Users/tom/Dev/projects/underlay`
- **Planning branch:** `main`
- **Planning base commit:** `103184aca4dabb9c420ac259993ac4d31d916db4`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved
  to that SHA before this handoff was created.
- **Planning checkout:** clean before this handoff file was created.
- **Worker mode:** implementation worker dispatched by the orchestrator.
- **Worker branch:** `worker/papercuts-wave8-batch-card`
- **Worker worktree:** launcher worktree first. `.agents.local.env` was
  absent; ask before creating a manual fallback. Never use `/tmp`.
- **Ready work items, in order:**
  1. Northstar compile-roadmaps references a missing batch-card template
     — close if sibling Northstar still ships the skill-asset file
     (PR 6 / later). Note the installed-skill refresh as operator
     follow-up, not a repo defect
- **Out of scope:** editing Northstar; writing into `~/.claude/skills`;
  GitHub Release execute (already closed).
- **Canonical refs:** `PAPERCUTS.md`; sibling Northstar
  `skills/northstar/assets/templates/docs/specs/templates/batch-card-template.md`.
- **Required validation:** quote the skill-shipped path on current
  Northstar `origin/main`. If compile-roadmaps in this repo still
  requires the consumer-destination path and that file is absent here,
  keep the copy open with that evidence instead of inventing a local
  template.
- **PR URL:** pending
- **Merge authorisation:** absent; do not merge

## Boundaries

- Close-with-evidence, or keep open. Do not vendor a second template.
- Do not merge.

## Important Context

- Wave 5 already proved the source package. This lane is the remaining
  consumer copy.
- **Report to:** the operator.

## Suggested Next Move

Read this file, run the worktree preflight, then confirm the skill-shipped
template still exists and close or keep the copy honestly.

## Completion Protocol

### Before you start

1. Read this handoff. Run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. Accept a clean dedicated non-`main` registered worktree. Record the
   actual path/branch.
3. Confirm `HEAD == origin/main` and ancestor
   `103184aca4dabb9c420ac259993ac4d31d916db4`.
4. Confirm this handoff exists in `HEAD`.

### When the assigned runway is complete

1. Update `PAPERCUTS.md`. Push a PR. Do not merge.

### Review and merge path

Awaiting orchestrator review. Merge is operator-authorised only.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; the PR.

### Handoff closeout

If this repo has no more open papercuts after the close, say so.
