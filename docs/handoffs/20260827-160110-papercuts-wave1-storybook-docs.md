---
title: Papercuts wave 1 Storybook and docs cleanup worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom / papercuts orchestrator
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260827-160110-papercuts-wave1-storybook-docs.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts]
---

## What This Thread Was Doing

A cross-repo papercuts sweep found Underlay still advertising retired
Storybook tasks, `TestDb` docs promising drop cleanup that never runs, auth
architecture links pointing at removed crate paths, and no
`check:agent-instructions` selector.

The operator approved wave 1 and asked for one orchestrator handoff per
repo. You are the Underlay implementation worker for this docs/tooling
lane only.

This is not a generation batch card. Do not invent a milestone or reopen
g09 currentness work.

## Why It Matters

Agents still discover `storybook` from `effigy tasks` after the surface
was removed. `TestDb` authors leak schemas because the docs lie. Auth
link checks fail on retired paths before they can isolate real drift. A
Northstar AGENTS review has no mechanical selector here.

## Current State

- **Repository:** `/Users/tom/Dev/projects/underlay`
- **Planning branch:** `main`
- **Planning base commit:** `eecedd9bea097e32f805b4479f931b459fe68ebf`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved
  to that SHA before this handoff was created.
- **Planning checkout:** clean before this handoff file was created.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** `PAPERCUTS.md`; this handoff.
- **Worker branch:** `worker/papercuts-wave1-storybook-docs`
- **Worker worktree:** use the clean dedicated non-`main` worktree supplied
  by the launcher. This handoff does not select a manual fallback path.
- **Manual fallback command:** only after the operator supplies an absolute
  `AGENTS_WORKTREE_CONTAINER_DIR`. `.agents.local.env` was absent in the
  planning checkout, so do not create a manual fallback without asking.
- **Active spec lane:** none. Numbered g09 roadmaps remain the queue
  authority; do not create batch cards.
- **Roadmap milestone:** none for this lane.
- **Ready work items, in order:**
  1. Effigy still advertises retired Storybook tasks
  2. `TestDb` docs promise automatic drop cleanup that does not run
  3. Auth architecture links target retired crate paths
  4. No `check:agent-instructions` task in underlay `effigy.toml`
- **Allowed runway:** those four items only, one PR.
- **Remaining card budget:** four papercuts.
- **Dispatch topology:** serial inside this repo; parallel with the other
  wave-1 repos.
- **Parallel safety check:** no shared files with other wave-1 workers.
- **Canonical refs:** `AGENTS.md`; `PAPERCUTS.md`;
  `docs/contracts/001-working-rules.md`;
  `docs/guides/130-testing.md`;
  `docs/architecture/050-auth-database-schema.md`;
  `docs/architecture/010-package-map.md`.
- **Model capability profile:** capable coding model, medium reasoning.
- **Tool/runtime restrictions:** use Effigy selectors; do not edit
  consumer repos; do not start another worker.
- **Required validation:** `effigy tasks` no longer lists Storybook;
  docs QA for the touched TestDb and auth files; prove the new or
  documented agent-instructions selector. Prefer `effigy qa:docs` over a
  full board unless you changed Rust.
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting orchestrator review after worker completion
- **Merge authorisation:** absent; do not merge

## Boundaries

- **In scope:** the four papercuts named above, plus closing those entries
  in `PAPERCUTS.md`.
- **Out of scope:** Svelte `:global(...)` CSS emission; Effigy release
  GitHub-publish behavior; the whole-contract absolute-path sweep; splitting
  `context_tests.rs`; workspace-shape leftover trees; Northstar
  compile-roadmaps missing template; stale Effigy skill JSON paths.
- Storybook: remove the stale task selectors and leftover config or
  dependency residue. Do not restore Storybook.
- `TestDb`: prefer making the docs require explicit `cleanup()` for
  external databases unless you can prove an owned async teardown. Do not
  invent a hidden Drop that cannot run async cleanup.
- Auth links: repoint `docs/architecture/050-auth-database-schema.md`
  (and any still-live callers) at current auth crate owners, or convert
  them to historical prose. Do not resurrect `underlay-auth`.
- Agent-instructions: add `check:agent-instructions` if the Northstar
  bundled audit task is available to this repo; otherwise document
  `qa:docs:agent-defaults` as the consumer-safe fallback in `AGENTS.md`
  and the papercut closeout. Do not invent a second audit.
- `.agents.local.env` is absent. Use the launcher worktree. Ask before
  creating a fallback.
- Do not merge the PR.

## Important Context

- **Planning lineage:** operator-authorized papercuts wave 1, 2026-08-27.
  g09 consumer runtime-access work is a different lane; do not join it.
- **Why these items are ready:** bounded docs/tooling, named surfaces, no
  product decision.
- **Known red state:** `effigy doctor` still has scan findings. Do not
  split god-files or retune scan policy.
- **Report after:** Storybook removal; TestDb docs; auth links; the
  agent-instructions selector or documented fallback; then PR.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Read this file from the top. Run the worktree-safety preflight before
broad reads. Use the launcher worktree if it is clean, dedicated, and
not `main`.

Then read `AGENTS.md`, `PAPERCUTS.md`, root `effigy.toml` around the
Storybook tasks, `docs/guides/130-testing.md` for `TestDb`, and
`docs/architecture/050-auth-database-schema.md`. Start with the Storybook
selectors so `effigy tasks` stops lying.

## Completion Protocol

### Before you start

1. Read this handoff. `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad
   reads, run:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, `git worktree list --porcelain`.
2. If the current root is a registered worktree, status is empty, and the
   branch is not `main`, accept it. Record the actual root/branch. Do not
   create another worktree because the name differs.
3. If the launcher supplied a dirty or `main` worktree, stop and report
   it. `.agents.local.env` was absent, so ask the operator before creating
   a manual fallback. Never use `/tmp` or `TMPDIR`.
4. Confirm `HEAD == origin/main`, confirm
   `git merge-base --is-ancestor eecedd9bea097e32f805b4479f931b459fe68ebf HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read `AGENTS.md`, `PAPERCUTS.md`, and the named surfaces.

### While you work

- Commit in meaningful chunks.
- Report through the operator after each item.
- Stop on missing contracts, ambiguous auth owners, or scope expansion.
- Close finished papercuts in `PAPERCUTS.md`.

### When the assigned runway is complete

1. Run `effigy qa:docs` (or the focused docs checks you used) and
   `effigy tasks` proof that Storybook is gone.
2. Push the worker branch.
3. Open a reviewable PR against current pushed `main`.
4. Report the PR URL. Do not merge.

### Review and merge path

Awaiting orchestrator review after the PR exists. Merge is
operator-authorised only.

- **Closeout refs:** `PAPERCUTS.md`; this handoff; the PR.

### Handoff closeout

If auth crate ownership is still ambiguous after reading the package map,
stop and report rather than guessing a new owner path.
