---
title: Papercuts wave 5 Effigy and Northstar closeout worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: awaiting-review
owner: Tom / papercuts orchestrator
created: 2026-08-28
updated: 2026-08-28
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260828-164801-papercuts-wave5-effigy-closeout.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

Effigy PR 48 and Northstar PR 6 merged on origin. This repo still lists
the consumer copies: attention-marker CLI overrides, task-arg `--`
widening, a stale skill JSON path, a missing batch-card template, GitHub
Release on execute, and Underlay Reference postgres volume docs.

Worker proved those copies against PATH Effigy / installed Northstar and
closed what matched. Did not re-implement Effigy or Northstar here.

## Why It Matters

Open copies send the next worker into Effigy work that already shipped,
or into a docs hunt that README already answered.

## Current State

- **Repository:** `/Users/tom/Dev/projects/underlay`
- **Planning branch:** `main`
- **Planning base commit:** `4f4beda796cf22bca7177a23f41966a9cf721ad3`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved
  to that SHA before this handoff was created.
- **Planning checkout:** clean before this handoff file was created.
- **Worker mode:** implementation worker dispatched by the orchestrator.
- **Worker branch:** `t3code/complete-papercuts-wave5-effigy` (accepted;
  differs from placeholder `worker/papercuts-wave5-effigy-closeout`)
- **Worker worktree:** `/Users/tom/.t3/worktrees/underlay/t3code-7be03fd0`
  (launcher worktree). `.agents.local.env` absent; no fallback created.
- **Proof binary / pins:** PATH `effigy v0.12.1+local.834a4bd` (older than
  PR 48). Effigy source HEAD includes `02100eefd` but is not on PATH.
  Northstar package HEAD `35a706d91bcb` (PR 6); `~/.claude/skills/northstar`
  not refreshed.
- **Work items:**
  1. Attention-marker CLI overrides — **open**. PATH ignores
     `--warning-marker CUSTOMONLY` (stock patterns unchanged).
  2. Task-arg `--` widening — **closed**. PATH forwards the path after
     `--` (`bun x vitest run '--' '<path>'`).
  3. Northstar batch-card template — **open**. Package ships the file;
     installed `~/.claude/skills/northstar` does not.
  4. Effigy skill JSON path — **open**. Live schema is
     `.result.catalog_tasks[]`; skill still documents `.result.payload.tasks[]`.
  5. Reference postgres volume docs — **closed**. README names
     `underlay-reference-dev-postgres-data`.
- **Out of scope kept open:** Effigy release execute omitting the GitHub
  Release.
- **Canonical refs:** `PAPERCUTS.md`; sibling Effigy
  `02100eefdde17db64652b2b26317bb284c504d8e` (PR 48); sibling Northstar
  `35a706d91bcb` (PR 6); Underlay Reference README runtime notes.
- **PR URL:** pending
- **Merge authorisation:** absent; do not merge

## Boundaries

- Prove against the current pin. Do not re-implement Effigy or Northstar.
- Do not skip catalog members. Do not merge.

## Important Context

- Wave 4 already landed merge-closeout and retired-tree inventory here.
- The postgres papercut is filed here; the live wording lives in Underlay
  Reference. README on that origin already names the named volume.
- **Report to:** the operator.

## Suggested Next Move

Orchestrator: review the PR, then refresh PATH Effigy past PR 48 and
reinstall the Northstar skill so the remaining open copies can close.

## Completion Protocol

### Before you start

1. Read this handoff. Run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. Accept a clean dedicated non-`main` registered worktree. Record the
   actual path/branch. Do not create a second worktree because they
   differ from the placeholder.
3. Confirm `HEAD == origin/main` and ancestor
   `4f4beda796cf22bca7177a23f41966a9cf721ad3`.
4. Confirm this handoff exists in `HEAD`.

### When the assigned runway is complete

1. Update `PAPERCUTS.md`. Push a PR. Do not merge.

### Review and merge path

Awaiting orchestrator review. Merge is operator-authorised only.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; the PR.

### Handoff closeout

If PATH `effigy` is older than PR 48, keep the copies open and report
the version. Do not vendor a local shim.
