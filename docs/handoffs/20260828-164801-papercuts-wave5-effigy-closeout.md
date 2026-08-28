---
title: Papercuts wave 5 Effigy and Northstar closeout worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
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

You are the Underlay implementation worker. Prove those upstream fixes
against the current pin and close the copies that now match. Do not
re-implement Effigy or Northstar here.

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
- **Worker branch:** `worker/papercuts-wave5-effigy-closeout`
- **Worker worktree:** launcher worktree first. `.agents.local.env` was
  absent; ask before creating a manual fallback. Never use `/tmp`.
- **Ready work items, in order:**
  1. Attention-marker CLI overrides are ignored — close if current Effigy
     applies `--warning-marker` / `--high-marker` / `--critical-marker`
     and a CLI contract test (or equivalent) shows the rendered pattern
     lists change
  2. Effigy task arguments silently widen when preceded by `--` — close
     if `effigy test:unit -- <paths>` forwards the paths or rejects the
     form instead of running the full suite
  3. Northstar compile-roadmaps references a missing batch-card template
     — close if the installed Northstar skill assets now ship
     `docs/specs/templates/batch-card-template.md` (Northstar PR 6)
  4. Effigy task-inventory JSON example uses a stale payload path —
     close only if the *installed* skill already queries
     `.result.catalog_tasks[]`. If `skills/effigy` still says
     `.result.payload.tasks[]`, keep this copy open and pointed at
     Effigy. Do not edit Effigy from this repo
  5. Reference runtime docs misstate database storage shape — close if
     Underlay Reference README already names
     `underlay-reference-dev-postgres-data` as the live store. Do not
     edit Underlay Reference from this repo
- **Out of scope:** Effigy release execute omitting the GitHub Release
  (keep open; protocol vs create is an Effigy decision); merge-closeout
  and leftover-tree work already on this SHA; editing Effigy or
  Northstar.
- **Canonical refs:** `PAPERCUTS.md`; sibling Effigy
  `02100eefdde17db64652b2b26317bb284c504d8e` (PR 48); sibling Northstar
  `35a706d91bcb` (PR 6); Underlay Reference README runtime notes.
- **Required validation:** cite the Effigy/Northstar SHA or version you
  actually ran. Close only with that proof. Keep GitHub Release open.
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

Read this file, run the worktree preflight, then prove each copy against
current Effigy and Northstar. Close what matches. Leave GitHub Release
open.

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
