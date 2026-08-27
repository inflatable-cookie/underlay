---
title: g09.062 workspace-shape internal modularization worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260827-224035-g09-062-workspace-shape-internal-modularization.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g09, effigy, doctor, typescript]
---

## What This Thread Was Doing

The orchestrator assessed Underlay's red `effigy doctor` surface after
`g09.060` closeout. The operator chose a green-doctor finish line: clear error
checks without forcing threshold-driven warning cleanup. This lane owns the
high-severity workspace-shape god-file half of that decision.

You are the implementation worker, not the planning authority. Split the
checker behind its existing public facade and prove there is no export,
diagnostic, CLI, or consumer behavior drift.

## Why It Matters

`ts/src/tools/workspace-shape.ts` has grown to 559 code lines and owns model
definitions, filesystem traversal, manifest discovery, topology checks,
dependency checks, report formatting, and CLI dispatch. It is the only
high-severity god-file finding in Underlay.

Those responsibilities have clean internal seams. Extracting them lowers
change risk while preserving the contract `024` checker that all six consumer
roots rely on.

## Current State

- **Repository:** `git@github.com:inflatable-cookie/underlay.git`
- **Planning branch:** `main`
- **Planning base commit:** `049fae4dd5f326bfbb08bc97b5e6ef7bfcd6c8b5`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `049fae4dd5f326bfbb08bc97b5e6ef7bfcd6c8b5` before this handoff was created.
- **Planning checkout:** clean after the `g09.061`/`g09.062` promotion push.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** `g09.061`, `g09.062`, the doctor
  promotion log, updated front doors, and the promoted doctor triage note.
- **Worker branch label:** `worker/g09-062-workspace-shape-modularization`.
- **Worker worktree:** use the clean dedicated non-`main` worktree supplied by
  the launcher. This handoff does not select a manual fallback path.
- **Manual fallback:** `.agents.local.env` was absent in the planning checkout.
  If the launcher worktree is unusable, ask the operator for an absolute
  `AGENTS_WORKTREE_CONTAINER_DIR` before creating a unique worktree there.
  Never use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** none. The numbered roadmap is the execution authority;
  do not create or cycle batch cards.
- **Roadmap milestone:** `docs/roadmaps/g09/README.md`.
- **Ready roadmap:**
  `docs/roadmaps/g09/062-workspace-shape-internal-modularization.md` only.
- **Allowed runway:** `g09.062` only.
- **Remaining roadmap budget:** one roadmap.
- **Dispatch topology:** parallel with `g09.061`.
- **Parallel safety:** this lane owns workspace-shape source/tests, its roadmap,
  and its own execution log. `g09.061` owns only `effigy.toml` and separate
  evidence. Neither worker edits shared front doors or `docs/logs/README.md`.
- **Canonical refs:** `AGENTS.md`, `PAPERCUTS.md`,
  `docs/architecture/product-guardrails.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/024-new-app-bootstrap-and-bring-up.md`,
  `docs/contracts/120-tooling-testing-and-contract-artifacts.md`, `package.json`,
  `ts/src/tools/workspace-shape.ts`, and `g09.062`.
- **Planning evidence:**
  `docs/triage/20260827-223450-underlay-doctor-scan-backlog.md` and
  `docs/logs/2026-08/27-223823-g09-061-062-doctor-error-promotion.md`.
- **Model capability profile:** capable TypeScript/tooling worker with medium
  reasoning. Pause on public behavior or contract ambiguity.
- **Tool/runtime restrictions:** Effigy first; no consumer, release/version,
  package-export, security-checker, env-authority, or workflow changes.
- **Required validation:** focused workspace-shape tests and typecheck,
  god-file JSON scan, expected partial doctor evidence, full Underlay validation,
  docs QA, Northstar QA, and `git diff --check`.
- **PR base/head:** `main` ← selected worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation PR.
- **Merge authorisation:** not granted. The worker must not merge.

## Boundaries

Keep this run inside `g09.062`:

- Preserve exactly:
  - `WORKSPACE_SHAPE_RULE_IDS`
  - `WorkspaceShapeRuleId`
  - `WorkspaceShapeViolation`
  - `checkWorkspaceShape()`
  - `formatWorkspaceShapeReport()`
  - `runWorkspaceShapeCli()`
- Keep `ts/src/tools/workspace-shape.ts` as the public facade and the existing
  package export/bin wiring unchanged.
- Extract cohesive internal groups under `ts/src/tools/workspace-shape/`.
  Natural seams are model/constants, filesystem/manifest discovery, topology
  checks, dependency checks, reporting, and CLI dispatch. Use a small number of
  modules; do not create one file per helper.
- Keep new code files below the 250-code-line advisory threshold where those
  natural seams allow it. Do not retune or suppress the scanner.
- Preserve every rule ID, repo-relative path, detail string, deterministic sort
  order, report string, help string, and exit behavior.
- Retain existing fixtures. Add focused coverage only if the extraction exposes
  a missing seam assertion.
- Do not add new workspace rules or change Contract `024`.
- Do not touch `effigy.toml`, attention-marker policy, env-authority, security
  conformance, consumers, shared roadmap front doors, or `docs/logs/README.md`.
- Work only in the selected clean worker worktree. Never clean, reset, stash,
  or discard another checkout's state.
- Do not merge the PR.

## Important Context

- Exact discovery baseline: `workspace-shape.ts` is 559 code lines and the only
  high-severity god-file finding; fourteen other files are advisory warnings.
- Contract `120` names the current facade as the source of truth and requires
  stable rule IDs, paths, offending values, sort order, and non-zero drift exit.
- `package.json` exports `@inflatable-cookie/underlay/tools/workspace-shape` and
  the `underlay-workspace-shape` bin. No manifest change is expected.
- Existing fixture coverage exercises compliant shape, root manifest failures,
  path containment, membership, locks, internal edges, shared `file:` edges,
  report output, and the published bin.
- Use `effigy graph` for code ownership/navigation, then exact `rg` and file
  inspection before edits.
- Full `effigy doctor` is expected to retain attention-marker errors until
  `g09.061` merges. Record that as the other lane's baseline, not as failure of
  this lane.
- **Report after:** module boundary and focused tests; then full validation and
  PR creation.
- **Report to:** the operator, who relays progress to the orchestrator.

## Suggested Next Move

Read this handoff from the top. Run the worktree preflight below before broad
repo reads. Then read `AGENTS.md`, `PAPERCUTS.md`, `g09.062`, contracts `024`
and `120`, the promoted triage note, and the promotion log. Run `effigy tasks`,
refresh the graph if needed, and use `effigy graph explore` to confirm the
workspace-shape ownership and likely tests.

Design the smallest cohesive internal module split before editing. Keep public
names and output literals anchored in the facade or deliberately re-exported.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Before any
   broad reads, run:

   ```sh
   git rev-parse --show-toplevel
   git branch --show-current
   git status --porcelain
   git worktree list --porcelain
   ```

2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root and branch. Do not create another worktree because its generated
   path or branch differs from the label above.
3. If the launcher supplied a dirty or `main` worktree, stop and report it. If a
   manual fallback is genuinely needed, require an operator-provided absolute
   `AGENTS_WORKTREE_CONTAINER_DIR`. Never use a temporary or guessed path and
   never clean another checkout.
4. Run `git fetch origin`. Confirm `HEAD == origin/main`, confirm
   `git merge-base --is-ancestor 049fae4dd5f326bfbb08bc97b5e6ef7bfcd6c8b5 HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read the milestone, `g09.062`, `AGENTS.md`, `PAPERCUTS.md`, canonical refs,
   promotion log, and triage note named above.
6. Run `effigy tasks`, inspect test shape if needed, and use the fresh code graph
   plus exact source reads for orientation. Record the baseline god-file scan
   and doctor state.

### While you work

- Execute only `g09.062`.
- Use `apply_patch` for edits and Effigy for supported validation.
- Keep the refactor behavior-preserving and commits aligned with meaningful
  module/test chunks.
- After the split and focused proof, tell the operator which modules changed,
  what public behavior stayed fixed, what passed, and what remains.
- Stop on any contract, export, CLI, diagnostic, or scope ambiguity.

### When the assigned runway is complete

1. Run `effigy check:workspace-shape`,
   `effigy test:unit ts/tests/tools/workspace-shape.test.ts`,
   `effigy check:types`, `effigy scan god-files --json`,
   `effigy doctor --verbose`, `effigy health`, `effigy validate`,
   `effigy qa:docs`, `effigy qa:northstar`, and `git diff --check`.
2. Set `g09.062` to `in review`, append actual evidence, and create one execution
   log under `docs/logs/2026-08/`. Do not edit shared front doors or
   `docs/logs/README.md`.
3. Push the selected worker branch.
4. Open a reviewable PR against current pushed `main`. The planning base above
   predates the handoff commit and is intentionally not self-referential.
5. Link `g09.062`, contracts `024`/`120`, the triage note, promotion log,
   changed modules/tests, scan evidence, exact partial-doctor state, and
   validation in the PR body.
6. Report the PR URL and exact head to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect metadata, commits, diff, checks, and changed files
against `g09.062`, contracts `024`/`120`, the public package/bin surfaces, and
the other parallel lane. If the shared GitHub identity prevents formal
self-approval, the orchestrator posts its exact-head verdict as a PR comment.

Merge remains operator-authorized. Closeout refs are `g09.062`, its execution
log, the doctor triage note, `g09/README.md`, and the active roadmap front doors.

### Handoff closeout

Leave the roadmap and lane log honest. If blocked, record the blocker and stop.
Do not edit the other lane or invent the next roadmap.
