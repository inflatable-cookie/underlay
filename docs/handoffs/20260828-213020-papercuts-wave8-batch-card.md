---
title: Papercuts wave 8 batch-card template closeout worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: awaiting-review
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

Worker closed the Underlay copy with evidence that sibling Northstar
`origin/main` still ships the skill-asset path and compile-roadmaps can
use it. Recorded installed-skill refresh as an operator machine step.
Did not edit `~/.claude/skills`. Did not edit Northstar. Did not vendor
a local template.

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
- **Worker branch:** `t3code/papercuts-wave8` (accepted; differs from
  placeholder `worker/papercuts-wave8-batch-card`)
- **Worker worktree:** `/Users/tom/.t3/worktrees/underlay/t3code-2233ec1d`
  (launcher worktree). `.agents.local.env` absent; no fallback created.
- **Work items:**
  1. Northstar compile-roadmaps references a missing batch-card template
     — **closed**. Northstar `origin/main` has
     `skills/northstar/assets/templates/docs/specs/templates/batch-card-template.md`
     (`35a706d` / PR 6). compile-roadmaps accepts that skill-shipped path
     when the consumer destination is absent. No Underlay
     `docs/specs/templates/` vendoring. Installed-skill refresh
     (`~/.claude/skills/northstar`) is operator follow-up.
- **Out of scope:** editing Northstar; writing into `~/.claude/skills`;
  GitHub Release execute (already closed).
- **Canonical refs:** `PAPERCUTS.md`; sibling Northstar
  `skills/northstar/assets/templates/docs/specs/templates/batch-card-template.md`.
- **Validation:** quoted skill-shipped path present on Northstar
  `origin/main` (`82f493713efd`); Underlay Open section empty after close.
- **PR URL:** pending
- **Merge authorisation:** absent; do not merge

## Boundaries

- Close-with-evidence, or keep open. Do not vendor a second template.
- Do not merge.

## Important Context

- Wave 5 already proved the source package. This lane was the remaining
  consumer copy.
- **Report to:** the operator.

## Suggested Next Move

Orchestrator: review the PR. Operator: refresh `~/.claude/skills/northstar`
when ready so the installed skill matches the source package.

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

Runway complete. Underlay has no remaining open papercuts after this close.
