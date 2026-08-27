---
title: Papercuts wave 2 CSS and test-split worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom / papercuts orchestrator
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260827-181240-papercuts-wave2-css-tests.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

Wave 1 removed Storybook, fixed TestDb docs, auth links, and
`check:agent-instructions`. Remaining Underlay papercuts still spray
`:global(...)` into Lightning CSS and keep `context_tests.rs` over the
god-file warning line.

You are the Underlay implementation worker for this lane. Do not invent a
generation card.

## Why It Matters

Consumer production builds warn that `global` is not a valid
pseudo-class, hiding new diagnostics. Doctor reports an extra structural
warning on a coherent test file.

## Current State

- **Repository:** `/Users/tom/Dev/projects/underlay`
- **Planning branch:** `main`
- **Planning base commit:** `e9d90391145c5578567b71a5cf1bfd8aad73daec`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved
  to that SHA before this handoff was created.
- **Planning checkout:** clean before this handoff file was created.
- **Worker mode:** implementation worker dispatched by the orchestrator.
- **Planning artifacts included at the base:** `PAPERCUTS.md`; this handoff.
- **Worker branch:** `worker/papercuts-wave2-css-tests`
- **Worker worktree:** use the launcher worktree. This handoff does not
  select a manual fallback path.
- **Manual fallback command:** only after the operator supplies
  `AGENTS_WORKTREE_CONTAINER_DIR`. `.agents.local.env` was absent.
- **Active spec lane:** none.
- **Roadmap milestone:** none.
- **Ready work items, in order:**
  1. Emitted Svelte CSS leaves `:global(...)` for Lightning CSS
  2. Context extractor tests crossed the god-file warning threshold
  3. Active contracts retain machine-local evidence links — only if you
     can bound it to a cheap docs QA check plus the worst offenders,
     not a whole-tree rewrite
- **Allowed runway:** items 1–2 required; item 3 only if it stays a
  bounded check plus a small link sweep. One PR.
- **Remaining card budget:** two required papercuts, one optional.
- **Dispatch topology:** serial inside Underlay; parallel with other
  wave-2 repos.
- **Parallel safety check:** no shared files with other wave-2 workers.
- **Canonical refs:** `AGENTS.md`; `PAPERCUTS.md`;
  Underlay detail-card Svelte styles;
  `rust/crates/underlay-http/src/tests/context_tests.rs`.
- **Model capability profile:** capable coding model, medium reasoning.
- **Tool/runtime restrictions:** use Effigy selectors; do not edit
  consumer apps to silence the CSS warning.
- **Required validation:** a consumer-style Vite/Lightning build of the
  detail-card CSS no longer warns on `global`; `context_tests.rs` split
  so doctor god-file warning for that file is gone without dropping
  coverage. `effigy qa:docs` for any contract-link edits.
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting orchestrator review after worker completion
- **Merge authorisation:** absent; do not merge

## Boundaries

- **In scope:** the CSS emission fix; split context tests into extractor /
  proxy-resolution / model modules without changing coverage; optional
  bounded absolute-path check.
- **Out of scope:** Effigy release GitHub-publish; reference runtime
  postgres volume docs (Underlay Reference); Northstar missing
  batch-card template; Effigy skill JSON path; `--` test:unit forwarding
  (Effigy wave 2); workspace leftover package trees; g09 front-door
  currentness.
- CSS: make Svelte consume `:global` before Lightning CSS, or emit a
  standards-valid selector. Do not disable the Lightning warning.
- Tests: split only. Do not rewrite extractor behaviour.
- Absolute paths: if you start item 3 and it becomes a hundred-file
  sweep, stop after adding the QA check and report.
- Do not merge the PR.

## Important Context

- **Planning lineage:** papercuts wave 2 after Underlay PR 10.
- **`context_tests.rs` is 357 lines** on this checkout.
- **Report after:** CSS; test split; optional links; then PR.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Read this file from the top. Run the worktree-safety preflight. Use the
launcher worktree if it is clean, dedicated, and not `main`.

Start by finding the detail-card `:global` selectors and how they emit.

## Completion Protocol

### Before you start

1. Read this handoff. Then run `git rev-parse --show-toplevel`,
   `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and the
   branch is not `main`, accept it.
3. If the launcher supplied a dirty or `main` worktree, stop and report
   it. `.agents.local.env` was absent, so ask before creating a fallback.
   Never use `/tmp`.
4. Confirm `HEAD == origin/main`, confirm
   `git merge-base --is-ancestor e9d90391145c5578567b71a5cf1bfd8aad73daec HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md` and `PAPERCUTS.md`.

### While you work

- Commit in meaningful chunks.
- Report through the operator after each item.

### When the assigned runway is complete

1. Run the validation named above.
2. Close the finished papercuts in `PAPERCUTS.md`.
3. Push the worker branch and open a PR against current pushed `main`.
4. Report the PR URL. Do not merge.

### Review and merge path

Awaiting orchestrator review. Merge is operator-authorised only.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; the PR.

### Handoff closeout

If `:global` is already consumed on this SHA, close that entry with a
Vite warning proof.
