---
title: g09.023 workspace-shape conformance worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-25
updated: 2026-08-25
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260825-201515-g09-023-workspace-shape-conformance.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g09, monorepo, conformance]
---

## What This Thread Was Doing

The maintainer chose Acowtancy's single-repository workspace as Underlay's only
supported consumer shape. `g09.021` made that shape normative. `g09.022`
normalized the migration contract and active guides, passed orchestrator review,
and merged as PR #7.

This handoff starts the next bounded batch: encode the consumer workspace shape
as a reusable conformance check with fixture coverage, contract alignment, and a
documented Effigy integration path. This is an Underlay tooling batch. It does
not migrate a consumer.

## Why It Matters

The topology is now clear in contracts and guides, but drift still fails only
when a maintainer notices it. The next five consumer batches need one generic
check that rejects the known fleet defects before directory moves and lockfile
changes become another manual audit cycle.

## Current State

- **Repository:** `git@github.com:inflatable-cookie/underlay.git`
- **Planning branch:** `main`
- **Planning base commit:** `542cb53a6d69c2a5f740809c57e50b79fb77b2ee`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `542cb53a6d69c2a5f740809c57e50b79fb77b2ee` before this handoff was created.
- **Planning checkout:** clean after the `g09.022` closeout push.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** the active strict spec, completed
  `g09.021` and `g09.022` roadmaps and logs, ready `g09.023`, and aligned roadmap,
  contract, spec, architecture, and log front doors.
- **Worker branch label:** `worker/g09-023-workspace-shape-conformance`.
- **Worker worktree:** use the clean dedicated non-`main` worktree supplied by
  the launcher. This handoff does not select a manual fallback path.
- **Manual fallback command:** only after the operator supplies an absolute
  `AGENTS_WORKTREE_CONTAINER_DIR`, create a unique worktree there from
  `origin/main`; do not copy this label blindly over an existing path.
- **Active spec lane:**
  `docs/specs/monorepo-consumer-workspace-rollout.md`.
- **Roadmap milestone:** `docs/roadmaps/g09/README.md`.
- **Ready roadmaps, in order:**
  `docs/roadmaps/g09/023-workspace-shape-conformance.md` only.
- **Allowed runway:** `g09.023` only.
- **Remaining roadmap budget:** one roadmap.
- **Dispatch topology:** serial. `g09.024` remains blocked pending review and
  operator-authorized merge of this PR.
- **Parallel safety check:** no other ready lane exists. `g09.024` consumes the
  check produced here and must not start from an unreviewed interface.
- **Canonical refs:** `AGENTS.md`,
  `docs/architecture/product-guardrails.md`,
  `docs/architecture/070-consumer-drift-prevention.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/024-new-app-bootstrap-and-bring-up.md`,
  `docs/contracts/120-tooling-testing-and-contract-artifacts.md`,
  `docs/contracts/121-underlay-app-review-checklist-and-audit-artifact.md`,
  `docs/contracts/app-review/underlay-app-review-checklist.json`,
  `docs/guides/020-project-structure.md`, and
  `docs/guides/030-underlay-integration.md`.
- **Existing comparison boundary:** `scripts/check-consumer-conformance.sh` is
  the security conformance check. Inspect it for reporting conventions only;
  do not add workspace-shape rules to it.
- **Live proof, read-only:** Acowtancy root `package.json`, `bun.lock`,
  `effigy.toml`, `infra/tasks.toml`, and child package manifests under
  `/Users/tom/Dev/projects/acowtancy`.
- **Model capability profile:** capable coding worker with medium reasoning;
  pause for frontier review if the distributable tooling boundary or contract
  ownership becomes ambiguous.
- **Tool/runtime restrictions:** use Effigy-first validation; no release
  mutations, `.github/workflows/` edits, or consumer-repository edits.
- **Required validation:** inspect `effigy test --plan`, run the focused
  script/fixture tests added by this roadmap, prove the check accepts Acowtancy,
  then run `effigy qa:docs`, `effigy qa:northstar`, `effigy validate`, and
  `git diff --check`.
- **PR base/head:** `main` ← worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation PR.
- **Merge authorisation:** not granted. The worker must not merge.

## Boundaries

Please keep this run inside `g09.023`:

- **In scope:** one generic consumer workspace-shape checker; its distributable
  package/export or command wiring; deterministic diagnostics and exit status;
  compliant and failing fixtures; focused tests; contracts `120` and `121`;
  `docs/contracts/app-review/underlay-app-review-checklist.json`; consumer
  Effigy integration guidance; and direct roadmap/log/front-door evidence updates.
- **Out of scope:** security-policy rules in
  `scripts/check-consumer-conformance.sh`; changes to Effigy's schema; Acowtancy
  README repair (`g09.024`); Underlay Reference or other consumer normalization
  (`g09.025`–`g09.029`); workflow edits; release work; and unrelated contract,
  guardrail, template, runtime, or package refactors.
- Keep the checker product-agnostic. Discover workspace package names and
  dependency edges from manifests; do not encode Acowtancy or fleet package
  names.
- Enforce the ready roadmap and contract `024`: one Git root with no nested
  repositories, root `private: true`, a fully pinned Bun `packageManager`,
  explicit workspace paths that resolve to manifests, one root `bun.lock`, no
  child lockfiles, no internal `file:` edges, and `workspace:*` for discovered
  internal JavaScript edges.
- Accept a root `.git` directory or worktree `.git` file. Exclude generated and
  installed trees from recursive checks. Sort diagnostics so fixture and CI
  output is stable.
- Keep the check separate from security policy in code, task naming, docs, and
  diagnostics.
- The check must be callable from a consumer's Effigy `health` or `validate`
  surface. Underlay publishes only its `ts/` tree today, so do not leave the only
  consumable implementation in an unpublished repo-local script. Update the
  existing package/tooling boundary only as far as required to distribute and
  invoke this check.
- Do not wire the consumer-shape check against Underlay's own root: Underlay is
  the foundation repository, not a normal consumer workspace. Wire focused
  self-tests into Underlay validation and document the consumer task pattern.
- Update contract `121` and its JSON artifact together. Keep the artifact small,
  stable, and contract-shaped rather than embedding app-specific results.
- Do not invent architecture, widen the roadmap, or choose an unresolved
  product, API, persistence, security, or deployment decision.
- Work only in the selected clean worker worktree. Prefer the current
  launcher-provided worktree and record its actual path/branch. If it is
  unusable, stop for operator configuration rather than guessing a path.
- Do not merge the PR.

## Important Context

- **Planning lineage:** the strict rollout is serial through `g09.025`.
  `g09.021` owns the topology, `g09.022` owns the active narrative, and this
  roadmap owns the mechanical shape gate. Consumer edits start after this gate is
  reviewed.
- **Why the roadmap is ready:** contract `024` and the strict spec settle every
  shape rule; Acowtancy supplies a live compliant proof; the fleet baseline in
  the spec supplies failing cases; and the roadmap names acceptance, validation,
  and stop boundaries.
- **Existing tooling seam:** contract `120` owns reusable TS tooling and
  rule-pack behavior. Follow the current `ts/src/tools` and package-export
  patterns unless repository evidence identifies a stronger existing generic
  seam. Do not bolt the rule onto the security shell because it happens to scan
  consumers already.
- **Internal-edge interpretation:** enumerate declared workspace manifests and
  their package names. An edge to one of those names is internal and must use
  `workspace:*`; arbitrary external dependencies do not need that protocol.
- **Fixture minimum:** one compliant Acowtancy-shaped root plus isolated failing
  cases for nested Git metadata, missing or invalid root manifest fields,
  wildcard or unresolved workspace entries, missing/root-duplicate/child locks,
  internal `file:` edges, and internal edges that do not use `workspace:*`.
- **Diagnostics:** each failure must identify a stable rule id or label and the
  offending repo-relative path/value. Multiple failures should be returned in a
  deterministic order with a non-zero process exit.
- **Effigy integration:** document a consumer-owned task that invokes the
  distributed checker and composes into `health` or `validate`. Do not require a
  new Effigy config key or mirror the task into `package.json` scripts.
- **Acowtancy proof:** its root manifest is private, pins `bun@1.3.14`, lists
  four explicit JavaScript workspaces, owns one root lock, and uses
  `workspace:*` for its discovered internal edges. Its existing
  `qa:security` task is a naming/boundary example, not the destination for this
  check.
- **Historical boundary:** do not rewrite closed roadmaps, earlier logs,
  handoffs, or research notes merely because they preserve prior conformance or
  workspace language.
- **Pre-existing health debt:** `effigy doctor` reports the unsupported
  `[isolation]` key plus attention-marker, comment-ratio, and god-file findings
  already logged in `PAPERCUTS.md`. Do not attribute or silently fix them in
  this roadmap.
- **Report after:** first, checker interface plus fixture/test proof; second,
  distribution/Effigy guidance plus contract/artifact alignment and final
  validation.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Read this handoff from the top. Before broad repository reads, run the quick
worktree-safety preflight below. If the current context is a clean, dedicated,
non-`main` registered worktree, use it immediately regardless of its generated
path or branch name. Record the actual path and branch; do not create another
worktree.

Then read `AGENTS.md`, the strict spec, `g09/README.md`, the `g09.023` roadmap,
contracts `024`, `120`, and `121`, the app-review JSON artifact, and the two
architecture refs named above. Use `effigy graph` for implementation ownership
and `rg` for exact path/token proof. Inspect Acowtancy read-only. Start with the
checker interface and fixtures; do not update contract prose before the
executable behavior is settled.

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
   absolute `AGENTS_WORKTREE_CONTAINER_DIR`. Ask the operator if the file or key
   is absent before creating the ignored file or any worktree. Never use `/tmp`,
   `TMPDIR`, or a guessed path; never clean, reset, stash over, or discard
   another checkout's state.
4. From the selected worktree, run `git fetch origin`. Confirm `HEAD` equals
   `origin/main`, confirm
   `git merge-base --is-ancestor 542cb53a6d69c2a5f740809c57e50b79fb77b2ee HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read the active spec, milestone, assigned roadmap, `AGENTS.md`, completed
   `g09.022` log, and canonical refs.
6. Run `effigy tasks` and `effigy doctor`. Record the known Doctor debt without
   widening this roadmap. Run `effigy graph status --json`; refresh the index before
   using graph results when it reports stale or degraded state.

### While you work

- Execute only `g09.023`.
- Keep executable behavior, fixtures, and tests in the first coherent chunk.
  Keep distribution/task guidance and contract/artifact alignment in the second.
- Use `effigy test --plan` before selecting a concrete focused test runner.
- Prove failures with fixtures, not by mutating live consumer repositories.
- Run the finished check against Acowtancy read-only as the compliant live proof.
- After each report point, tell the operator which files changed, what
  validation ran, what remains, and whether a planning decision is needed.
- Stop on missing authority, app-specific assumptions, a required new Effigy
  schema surface, an undistributable interface, or validation that changes the
  plan. Do not turn an open question into new architecture.

### When the assigned runway is complete

1. Inspect `effigy test --plan`, run the focused checker/fixture tests, run the
   checker against Acowtancy, then run `effigy qa:docs`,
   `effigy qa:northstar`, `effigy validate`, and `git diff --check`.
2. Update `g09.023` with actual evidence and create one batch log under
   `docs/logs/2026-08/`. Set the roadmap/front doors to `in review`; keep
   `g09.024` blocked because the orchestrator owns promotion after review and
   merge.
3. Push the selected worker branch.
4. Open a reviewable PR against the current pushed `main`. The planning base
   above predates the handoff commit and is intentionally not self-referential.
5. In the PR body, link the strict spec, milestone, roadmap, changed surfaces,
   fixture and Acowtancy proof, validation, and unresolved items.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will independently review PR metadata, commits, diff, checks,
and changed files against the strict spec, contract `024`, and `g09.023`. If the
same GitHub identity prevents formal approval, the orchestrator will post the
verdict as a PR comment. Make only requested changes on this worker branch and
report back through the operator.

Merge remains operator-authorized. Closeout refs are the `g09.023` roadmap, its
execution log, `g09/README.md`, the strict spec, contracts `120` and `121`, the
app-review artifact, and the roadmap/currentness front doors.

### Handoff closeout

Leave the roadmap, roadmap, log, and next-task state honest. If blocked, record the
blocker and stop. Do not make `g09.024` look ready before orchestrator review
and operator-authorized merge.
