---
title: g10.001 monorepo contract authority worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-25
updated: 2026-08-25
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260825-174520-g10-001-monorepo-contract-authority.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g10, monorepo]
---

## What This Thread Was Doing

The maintainer selected Acowtancy's single-repository workspace as the only
supported default for Underlay consumers. The planning thread assessed all six
consumer roots, promoted the decision into a strict spec, and compiled a
ten-card rollout.

This handoff starts the first bounded implementation batch. It replaces the
normative polyrepo-compatible bootstrap story in Underlay. It does not migrate a
consumer repository yet.

## Why It Matters

Underlay currently teaches two incompatible workspace models. New consumers can
still copy child repositories, child lockfiles, `libs/*`, and committed source
dependencies even though Acowtancy has proved the intended shape. Contract
authority must become unambiguous before the reference fixture and five other
consumers move.

## Current State

- **Repository:** `git@github.com:inflatable-cookie/underlay.git`
- **Planning branch:** `main`
- **Planning base commit:** `2dcf06e1090cda18843e6fc34f105b3ee39dead8`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `2dcf06e1090cda18843e6fc34f105b3ee39dead8` before this handoff was created.
- **Planning checkout:** clean after the planning-base push.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:**
  `docs/specs/monorepo-consumer-workspace-rollout.md`,
  `docs/roadmaps/g10/batch-cards/001-monorepo-contract-authority.md`, and
  `docs/logs/2026-08/25-174056-monorepo-rollout-compiled.md`.
- **Worker branch label:** `worker/g10-001-monorepo-contract-authority`.
- **Worker worktree:** use the clean dedicated non-`main` worktree supplied by
  the launcher. No manual fallback path is configured.
- **Manual fallback command:** only after the operator supplies an absolute
  `AGENTS_WORKTREE_CONTAINER_DIR`, create a unique worktree there from
  `origin/main`; do not copy this label blindly over an existing path.
- **Active spec lane:**
  `docs/specs/monorepo-consumer-workspace-rollout.md`.
- **Roadmap milestone:** `docs/roadmaps/g10/README.md`.
- **Ready cards, in order:**
  `docs/roadmaps/g10/batch-cards/001-monorepo-contract-authority.md` only.
- **Allowed runway:** `g10.001` only.
- **Remaining card budget:** one card.
- **Dispatch topology:** serial. `g10.002` remains blocked pending review and
  merge of this PR.
- **Parallel safety check:** no parallel Underlay docs worker is allowed because
  `g10.002` and `g10.003` overlap the same authority surfaces.
- **Canonical refs:** `docs/contracts/001-working-rules.md`,
  `docs/contracts/024-new-app-bootstrap-and-bring-up.md`,
  `docs/architecture/product-guardrails.md`, and
  `docs/architecture/070-consumer-drift-prevention.md`.
- **Model capability profile:** capable coding/docs worker with medium reasoning;
  pause for frontier review if contract ownership becomes ambiguous.
- **Tool/runtime restrictions:** use Effigy-first validation; no release
  mutations and no `.github/workflows/` edits.
- **Required validation:** `effigy qa:docs`, `effigy qa:northstar`,
  `effigy health`, targeted `rg` proof over scoped active files, and
  `git diff --check`.
- **PR base/head:** `main` ← worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation PR.
- **Merge authorisation:** not granted. The worker must not merge.

## Boundaries

Please keep this run inside `g10.001`:

- **In scope:** contract `024`; guide front door; guide architecture overview;
  project-structure and Underlay-integration guides; new-project quickstart;
  new-project bootstrap prompt; only the direct front-door/currentness updates
  required by those changes.
- **Out of scope:** second-tier guide cleanup in `g10.002`, conformance code in
  `g10.003`, Acowtancy or other consumer edits, Underlay Build skill
  distribution, production code, release work, and frozen historical evidence.
- Do not invent architecture, change unrelated contracts, widen the roadmap, or
  choose an unresolved package/runtime/security decision.
- Do not edit another lane's assigned scope. If shared mutable scope or a hidden
  dependency appears, stop and report it through the operator.
- Work only in the selected clean worker worktree. Prefer the current
  launcher-provided worktree and record its actual path/branch. If it is
  unusable, stop for operator configuration rather than guessing a path.
- Do not merge the PR.

## Important Context

- **Planning lineage:** the 2026-08-17 Northstar refresh opened `g10` for
  contract fidelity and fleet convergence. The maintainer then selected the
  Acowtancy monorepo shape as the first execution lane on 2026-08-25.
- **Why the card is ready:** the strict spec records the topology, root manifest,
  dependency, lockfile, migration, acceptance, validation, and stop decisions.
  Acowtancy provides live evidence. The remaining consumer work is deliberately
  sequenced later.
- **Decisions and preferences:** polyrepos are unsupported; use `apps/*` and
  `packages/*`; one root Bun manifest and lock; explicit workspace paths;
  internal `workspace:*`; released app dependencies; sibling mounts are
  QA/tooling only; orchestration stays in Effigy.
- **Open tension:** Acowtancy's README still contains obsolete `file:` prose.
  That is known evidence drift owned by `g10.004`, not permission to weaken the
  new Underlay contract.
- **Historical boundary:** do not rewrite closed roadmaps, logs, handoffs, or
  frozen migration records merely because they mention an older layout.
- **Report after:** first, the contract/front-door authority rewrite; second,
  the remaining scoped guide alignment and validation.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Read this handoff from the top. Before broad repository reads, run the quick
worktree-safety preflight below. If the current context is a clean, dedicated,
non-`main` registered worktree, use it immediately regardless of its generated
path or branch name. Record the actual path and branch; do not create another
worktree.

Then read `AGENTS.md`, the strict spec, `g10/README.md`, the `g10.001` card, and
the canonical refs named above. Inspect the current Acowtancy root manifest and
workspace task only as evidence; do not edit that repository. Start with
contract `024`, then align the smaller front doors against it.

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
   actual root and branch. Do not compare them with the branch label in this
   handoff or create another worktree because they differ.
3. If the launcher supplied a dirty or `main` worktree, stop and report it. If a
   manual fallback is genuinely needed, read `.agents.local.env` and require an
   absolute `AGENTS_WORKTREE_CONTAINER_DIR`. It was not configured when this
   handoff was written. Ask the operator for it before creating the ignored file
   or any worktree. Never use `/tmp`, `TMPDIR`, or a guessed path; never clean,
   reset, stash over, or discard another checkout's state.
4. From the selected worktree, run `git fetch origin`. Confirm `HEAD` equals
   `origin/main`, confirm
   `git merge-base --is-ancestor 2dcf06e1090cda18843e6fc34f105b3ee39dead8 HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read the active spec, milestone, assigned card, `AGENTS.md`, and canonical
   refs.
6. Run `effigy tasks` and `effigy doctor`. The planning base recorded a
   pre-existing unsupported `isolation` key plus attention-marker scan debt in
   `PAPERCUTS.md`; do not silently fix or attribute those findings to this card.

### While you work

- Execute only `g10.001`.
- Keep contract guarantees normative and guides narrative. Link rather than
  creating competing guarantee text.
- Use `rg` to distinguish active guidance from frozen historical evidence.
- Commit meaningful chunks rather than arbitrary turn-sized edits.
- After each named report point, tell the operator which files changed, what
  validation ran, what remains, and whether a planning decision is needed.
- Stop on missing authority, ambiguous intent, scope expansion, or validation
  that changes the plan. Do not turn an open question into new architecture.

### When the assigned runway is complete

1. Run `effigy qa:docs`, `effigy qa:northstar`, `effigy health`, the targeted
   active-file `rg` proof required by the card, and `git diff --check`.
2. Update `g10.001` and the batch log with actual evidence. Keep `g10.002`
   blocked; the orchestrator owns promotion after review and merge.
3. Push the selected worker branch.
4. Open a reviewable PR against current pushed `main`. The planning base above
   predates the handoff commit and is intentionally not self-referential.
5. In the PR body, link the strict spec, milestone, card, changed surfaces,
   evidence, validation, and unresolved items.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will independently review the PR metadata, commits, diff,
checks, and changed files against the strict spec and `g10.001`. If the same
GitHub identity prevents formal approval, the orchestrator will post the verdict
as a PR comment. Make only requested changes on this worker branch and report
back through the operator.

Merge remains operator-authorised. Closeout refs are the `g10.001` card, its
execution log, `g10/README.md`, the strict spec, and the roadmap/currentness
front doors.

### Handoff closeout

Leave the card, roadmap, log, and next-task state honest. If blocked, record the
blocker and stop. Do not make `g10.002` look ready before the orchestrator has
reviewed and the operator has authorised the merge path.
