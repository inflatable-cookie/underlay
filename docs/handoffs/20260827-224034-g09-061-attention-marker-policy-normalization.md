---
title: g09.061 attention-marker policy normalization worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260827-224034-g09-061-attention-marker-policy-normalization.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g09, effigy, doctor, scan]
---

## What This Thread Was Doing

The orchestrator assessed Underlay's red `effigy doctor` surface after
`g09.060` closeout. The operator chose a green-doctor finish line: clear error
checks without forcing threshold-driven warning cleanup. This lane owns the
attention-marker half of that decision.

You are the implementation worker, not the planning authority. Commit the
settled Underlay marker policy and prove it removes false doctor errors without
changing public deprecations or hiding unrelated scan debt.

## Why It Matters

The stock attention-marker policy currently calls ordinary security prose,
four intentional Rust deprecation attributes, and one explanatory test note
deferred work. That makes doctor red while obscuring actual action markers.

Underlay needs an explicit repo-owned policy that catches real `TODO`, `FIXME`,
`HACK`, `BUG:`, and `SECURITY:` labels while leaving compatibility metadata to
Contract `023` and compiler tooling.

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
- **Worker branch label:** `worker/g09-061-attention-marker-policy`.
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
  `docs/roadmaps/g09/061-attention-marker-policy-normalization.md` only.
- **Allowed runway:** `g09.061` only.
- **Remaining roadmap budget:** one roadmap.
- **Dispatch topology:** parallel with `g09.062`.
- **Parallel safety:** this lane owns `effigy.toml`, its roadmap, and its own
  execution log. `g09.062` owns workspace-shape source/tests and separate
  evidence. Neither worker edits shared front doors or `docs/logs/README.md`.
- **Canonical refs:** `AGENTS.md`, `PAPERCUTS.md`, `effigy.toml`,
  `docs/architecture/product-guardrails.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/023-release-and-compatibility-rollout.md`, and `g09.061`.
- **Planning evidence:**
  `docs/triage/20260827-223450-underlay-doctor-scan-backlog.md` and
  `docs/logs/2026-08/27-223823-g09-061-062-doctor-error-promotion.md`.
- **Model capability profile:** capable tooling/config worker with medium
  reasoning. Pause rather than inventing a compatibility or scanner policy.
- **Tool/runtime restrictions:** Effigy first; no Effigy source edits, public
  API deletion, consumer edits, release/version work, or workflow changes.
- **Required validation:** effective scan config, attention-marker JSON scan,
  expected partial doctor evidence, `effigy health`, docs QA, Northstar QA, and
  `git diff --check`.
- **PR base/head:** `main` ← selected worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation PR.
- **Merge authorisation:** not granted. The worker must not merge.

## Boundaries

Keep this run inside `g09.061`:

- Add `[scan.attention_markers]` to `effigy.toml` with exactly:
  - warning: `TODO`, `REVIEW`, `placeholder`
  - high: `FIXME`, `HACK`, `workaround`, `tech debt`
  - critical: `BUG:`, `SECURITY:`, `remove before release`
- Keep `doctor = true`, `fail_on_findings = false`, and
  `respect_gitignore = true`. Do not disable scanning or exclude broad source
  trees.
- Confirm the effective manifest and scan JSON expose the committed lists.
- Preserve all Rust `#[deprecated]` and TypeScript `@deprecated` surfaces.
- Do not reword useful prose to evade the scanner.
- Do not edit `/Users/tom/Dev/projects/effigy`; the ignored CLI marker-override
  bug is recorded in `PAPERCUTS.md` and is not this worker's runway.
- Do not edit consumer repos. Compli Me still uses both deprecated pagination
  aliases; retirement needs a separate compatibility and release roadmap.
- Do not edit workspace-shape files, god-file policy, comment-ratio policy,
  shared roadmap front doors, or `docs/logs/README.md`.
- Work only in the selected clean worker worktree. Never clean, reset, stash,
  or discard another checkout's state.
- Do not merge the PR.

## Important Context

- Exact discovery baseline: attention markers report five errors and one
  warning on `main` commit `60ff292b`.
- The critical hit is the phrase “security shapes” in
  `scripts/check-consumer-conformance.sh`, not a `SECURITY:` action label.
- The four high hits are compatibility attributes in `underlay-config`,
  `underlay-db`, `underlay-http`, and `underlay-query`.
- The warning is a normal “Note:” explaining Postgres integration coverage.
- The installed Effigy manifest schema supports project-owned marker lists.
  Per-run CLI marker flags are currently ignored, so validate the committed
  manifest rather than relying on those flags.
- Full `effigy doctor` is expected to remain red only on
  `ts/src/tools/workspace-shape.ts` until `g09.062` merges. Record that as the
  other lane's baseline, not as failure of this lane.
- **Report after:** effective policy plus targeted scan proof; then final QA and
  PR creation.
- **Report to:** the operator, who relays progress to the orchestrator.

## Suggested Next Move

Read this handoff from the top. Run the worktree preflight below before broad
repo reads. Then read `AGENTS.md`, `PAPERCUTS.md`, `effigy.toml`, `g09.061`,
the promoted triage note, and the promotion log. Run `effigy tasks` and capture
the baseline attention-marker scan before editing.

Apply the settled manifest policy as one coherent change. Validate the effective
config and scan JSON before broader repo QA.

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
5. Read the milestone, `g09.061`, `AGENTS.md`, `PAPERCUTS.md`, canonical refs,
   promotion log, and triage note named above.
6. Run `effigy tasks` and the baseline attention-marker scan. Use
   `effigy doctor --verbose` to record the two-lane baseline.

### While you work

- Execute only `g09.061`.
- Use `apply_patch` for edits and Effigy for supported validation.
- Keep the manifest policy, roadmap evidence, and one lane log as the only
  changed surfaces.
- After the policy proof, tell the operator what changed, the effective marker
  lists, the remaining doctor error, and whether any planning issue appeared.
- Stop on any scope expansion or policy choice named in the roadmap or handoff.

### When the assigned runway is complete

1. Run `effigy config --inspect --path scan.attention_markers`,
   `effigy scan attention-markers --json`, `effigy doctor --verbose`,
   `effigy health`, `effigy qa:docs`, `effigy qa:northstar`, and
   `git diff --check`.
2. Set `g09.061` to `in review`, append actual evidence, and create one execution
   log under `docs/logs/2026-08/`. Do not edit shared front doors or
   `docs/logs/README.md`.
3. Push the selected worker branch.
4. Open a reviewable PR against current pushed `main`. The planning base above
   predates the handoff commit and is intentionally not self-referential.
5. Link `g09.061`, the triage note, promotion log, changed manifest, evidence,
   exact partial-doctor state, and validation in the PR body.
6. Report the PR URL and exact head to the operator. Do not merge.

### Review and merge path

The orchestrator will inspect metadata, commits, diff, checks, and changed files
against `g09.061`, the settled marker policy, and the other parallel lane. If
the shared GitHub identity prevents formal self-approval, the orchestrator posts
its exact-head verdict as a PR comment.

Merge remains operator-authorized. Closeout refs are `g09.061`, its execution
log, the doctor triage note, `g09/README.md`, and the active roadmap front doors.

### Handoff closeout

Leave the roadmap and lane log honest. If blocked, record the blocker and stop.
Do not edit the other lane or invent the next roadmap.
