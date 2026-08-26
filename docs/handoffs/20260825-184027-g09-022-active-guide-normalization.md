---
title: g09.022 migration contract and active guide normalization worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-25
updated: 2026-08-25
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260825-184027-g09-022-active-guide-normalization.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g09, monorepo, docs]
---

## What This Thread Was Doing

The maintainer chose Acowtancy's single-repository workspace as Underlay's only
supported consumer shape. `g09.021` replaced the old polyrepo-compatible
contract and front-door story, passed orchestrator review, and merged as PR #6.

That review exposed one adjacent authority gap: contract `021` still teaches
retired `db:*` selectors. This handoff starts the next bounded batch. Repair
that contract first, then normalize the remaining active guides and bootstrap
pattern against the monorepo contract. This is documentation work only; it does
not add conformance code or migrate a consumer.

## Why It Matters

The normative workspace contract is now strict, but active narrative docs can
still send consumers toward flat packages, `libs/*`, committed `file:`
dependencies, child installs, and obsolete database commands. Those
contradictions must be removed before Underlay encodes the shape in checks or
rolls it through the five drifted consumer roots.

## Current State

- **Repository:** `git@github.com:inflatable-cookie/underlay.git`
- **Planning branch:** `main`
- **Planning base commit:** `09a9f8be4971af1ec02266f004eb9f0dd7950a28`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `09a9f8be4971af1ec02266f004eb9f0dd7950a28` before this handoff was created.
- **Planning checkout:** clean after the closeout-base push.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** the strict spec, the completed
  `g09.021` roadmap and log, the ready `g09.022` roadmap, and refreshed roadmap,
  contract, spec, architecture, and log front doors.
- **Worker branch label:** `worker/g09-022-active-guide-normalization`.
- **Worker worktree:** use the clean dedicated non-`main` worktree supplied by
  the launcher. No manual fallback path is configured.
- **Manual fallback command:** only after the operator supplies an absolute
  `AGENTS_WORKTREE_CONTAINER_DIR`, create a unique worktree there from
  `origin/main`; do not copy this label blindly over an existing path.
- **Active spec lane:**
  `docs/specs/monorepo-consumer-workspace-rollout.md`.
- **Roadmap milestone:** `docs/roadmaps/g09/README.md`.
- **Ready roadmaps, in order:**
  `docs/roadmaps/g09/022-active-guide-normalization.md` only.
- **Allowed runway:** `g09.022` only.
- **Remaining roadmap budget:** one roadmap.
- **Dispatch topology:** serial. `g09.023` remains blocked pending review and
  operator-authorised merge of this PR.
- **Parallel safety check:** no parallel Underlay docs or conformance worker is
  allowed because `g09.023` depends on the normalized authority produced here.
- **Canonical refs:** `docs/contracts/001-working-rules.md`,
  `docs/contracts/021-database-migration-and-schema-workflow.md`,
  `docs/contracts/024-new-app-bootstrap-and-bring-up.md`,
  `docs/contracts/070-nightfire-and-migration-systems.md`,
  `docs/contracts/120-tooling-testing-and-contract-artifacts.md`,
  `docs/architecture/product-guardrails.md`,
  `docs/guides/020-project-structure.md`, and
  `docs/guides/030-underlay-integration.md`.
- **Live proof, read-only:** Acowtancy root `README.md`, `package.json`,
  `effigy.toml`, `infra/tasks.toml`, and
  `docs/planning/migration-execution/migration-operator-command-ladder.md` in
  `/Users/tom/Dev/projects/acowtancy`.
- **Model capability profile:** capable coding/docs worker with medium reasoning;
  pause for frontier review if migration contract ownership becomes ambiguous.
- **Tool/runtime restrictions:** use Effigy-first validation; no release
  mutations and no `.github/workflows/` edits.
- **Required validation:** `effigy qa:docs`, `effigy qa:northstar`,
  `effigy health`, targeted active-doc `rg` proof, and `git diff --check`.
- **PR base/head:** `main` ← worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation PR.
- **Merge authorisation:** not granted. The worker must not merge.

## Boundaries

Please keep this run inside `g09.022`:

- **In scope:** contract
  `docs/contracts/021-database-migration-and-schema-workflow.md`; guides
  `040-rust-backend.md`, `050-database.md`, `080-typescript-client.md`,
  `130-testing.md`, `140-local-development.md`, `150-ci-cd.md`,
  `160-troubleshooting.md`, `170-checklist.md`, `172-agents-files.md`,
  `175-llm-bootstrap-guide.md`, `190-upgrade-compatibility.md`, and
  `200-project-sync.md`; `docs/patterns/llm-project-bootstrap.md`; and only the
  direct index/currentness updates required by those changes.
- **Out of scope:** workspace-shape conformance code (`g09.023`), Acowtancy
  evidence repair (`g09.024`), all consumer repository edits, Underlay Build
  skill distribution, production code, release work, workflow edits, and
  frozen historical evidence.
- Contract `021` may change only enough to replace retired root/package `db:*`
  aliases with the current root state and package-owned `migration:*` routing.
  Preserve its durable migration, dev-overlay, replay, safety, and validation
  semantics.
- Treat the numbered guide list as a bounded inspection set. Change only files
  that contain drift or need a direct consistency repair; do not rewrite
  unrelated domain guidance.
- Keep contract guarantees normative and guides narrative. Link to contracts
  instead of creating competing guarantees.
- Do not invent architecture, widen the roadmap, or choose an unresolved
  package, runtime, persistence, security, or deployment decision.
- Work only in the selected clean worker worktree. Prefer the current
  launcher-provided worktree and record its actual path/branch. If it is
  unusable, stop for operator configuration rather than guessing a path.
- Do not merge the PR.

## Important Context

- **Planning lineage:** the strict monorepo rollout is a ten-roadmap serial runway
  through `g09.025`; consumer-specific roadmaps may fan out only after the
  reference fixture merges. `g09.021` is complete at merge commit
  `ec21e51cc918284fd9b306144c8d44a2ce1cae96`.
- **Why the roadmap is ready:** contract `024` now owns the topology, root manifest,
  dependency, lockfile, and Effigy rules. Review evidence settled the contract
  `021` selector correction. The roadmap names every remaining active-doc surface,
  acceptance condition, validation gate, and stop boundary.
- **Migration operator decision:** root state operations are `effigy state plan`
  and `effigy state apply local --yes`. Schema work stays package-owned behind
  `migration:*` selectors routed from the workspace root. Do not reintroduce
  root or package `db:migrate` / `db:reset` aliases.
- **Workspace decision:** polyrepos are unsupported; use one Git root,
  `apps/*`, `packages/*`, root `docs/`, one root Bun manifest and lockfile,
  explicit workspaces, internal `workspace:*`, released app dependencies, and
  an Effigy-owned frozen root install.
- **Known hot spots:** guide `175` teaches flat paths, committed `file:`
  dependencies, and per-package installs; guide `080-typescript-client.md`
  teaches `libs/client` and multi-repo variants; guides `130`, `150`, and `170`
  contain `libs/*`, child-install, or multi-repo command examples.
- **Historical boundary:** do not rewrite closed roadmaps, earlier logs,
  handoffs, research notes, or frozen migration records merely because they
  preserve the former layout or selector names.
- **Pre-existing health debt:** `effigy doctor` reports the unsupported
  `[isolation]` key plus attention-marker and god-file scan debt already logged
  in `PAPERCUTS.md`. Do not attribute or silently fix it in this roadmap.
- **Report after:** first, contract `021` plus the migration/development guide
  cluster; second, the remaining bootstrap/testing/CI/upgrade/checklist sweep
  and final validation.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Read this handoff from the top. Before broad repository reads, run the quick
worktree-safety preflight below. If the current context is a clean, dedicated,
non-`main` registered worktree, use it immediately regardless of its generated
path or branch name. Record the actual path and branch; do not create another
worktree.

Then read `AGENTS.md`, the strict spec, `g09/README.md`, the `g09.022` roadmap, the
completed `g09.021` log, and the canonical refs named above. Inspect the live
Acowtancy proof read-only. Start with contract `021`, then normalize the scoped
guides in coherent topic clusters. Use exact-token searches to prove each old
shape is active guidance before editing it.

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
   `git merge-base --is-ancestor 09a9f8be4971af1ec02266f004eb9f0dd7950a28 HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read the active spec, milestone, assigned roadmap, `AGENTS.md`, completed
   `g09.021` log, and canonical refs.
6. Run `effigy tasks` and `effigy doctor`. Record the known doctor debt without
   widening this roadmap.

### While you work

- Execute only `g09.022`.
- Repair contract `021` before treating any guide migration command as current.
- Use `rg` to distinguish active guidance from frozen historical evidence and
  to select only the drifted numbered guides.
- Keep commits aligned with the two meaningful report chunks, not arbitrary
  model turns.
- After each report point, tell the operator which files changed, what
  validation ran, what remains, and whether a planning decision is needed.
- Stop on missing authority, ambiguous migration semantics, scope expansion, or
  validation that changes the plan. Do not turn an open question into new
  architecture.

### When the assigned runway is complete

1. Run `effigy qa:docs`, `effigy qa:northstar`, `effigy health`, the targeted
   active-doc `rg` proof required by the roadmap, and `git diff --check`.
2. Update `g09.022` with actual evidence and create one batch log under
   `docs/logs/2026-08/`. Set the roadmap/front doors to `in review`; keep
   `g09.023` blocked because the orchestrator owns promotion after review and
   merge.
3. Push the selected worker branch.
4. Open a reviewable PR against the current pushed `main`. The planning base
   above predates the handoff commit and is intentionally not self-referential.
5. In the PR body, link the strict spec, milestone, roadmap, changed surfaces,
   evidence, validation, and unresolved items.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will independently review the PR metadata, commits, diff,
checks, and changed files against the strict spec and `g09.022`. If the same
GitHub identity prevents formal approval, the orchestrator will post the verdict
as a PR comment. Make only requested changes on this worker branch and report
back through the operator.

Merge remains operator-authorised. Closeout refs are the `g09.022` roadmap, its
execution log, `g09/README.md`, the strict spec, and the roadmap/currentness
front doors.

### Handoff closeout

Leave the roadmap, roadmap, log, and next-task state honest. If blocked, record the
blocker and stop. Do not make `g09.023` look ready before orchestrator review and
operator-authorised merge.
