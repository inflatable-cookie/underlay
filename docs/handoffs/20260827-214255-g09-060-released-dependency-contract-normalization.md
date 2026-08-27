---
title: g09.060 released dependency contract normalization worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260827-214255-g09-060-released-dependency-contract-normalization.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g09, release, dependency, contract]
---

## What This Thread Was Doing

The orchestrator closed papercuts wave 3 in Underlay PR12 and promoted
`g09.060` from `planned` to `ready`. This handoff starts the one remaining
serial roadmap: correct Contract `023` so its release and consumer dependency
teaching matches Contract `024`, the live guides, tooling, and all six consumer
roots.

You are the implementation worker, not the planning authority. This is a docs-
only semantic correction. Do not edit a consumer repository, change a version,
cut a release, or alter release tooling.

## Why It Matters

Contract `023` still calls Underlay unpublished because its root package is
private and teaches committed sibling Cargo `path` and JavaScript `file:`
dependencies as the fleet default. That contradicts the current bootstrap
contract and the deployed fleet, where Underlay is released through immutable
Git tags and consumers pin the same tag on both language surfaces.

Leaving the contradiction active can send maintainers back to a dependency
shape the workspace contract and checker already reject.

## Current State

- **Repository:** `git@github.com:inflatable-cookie/underlay.git`
- **Planning branch:** `main`
- **Planning base commit:** `ec67dfbfb2add489e4309f801fdac6fbc953aeb0`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `ec67dfbfb2add489e4309f801fdac6fbc953aeb0` before this handoff was created.
- **Planning checkout:** clean after the PR12 closeout and `g09.060` promotion
  push.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Worker branch label:**
  `worker/g09-060-released-dependency-contract-normalization`.
- **Worker worktree:** use the clean dedicated non-`main` worktree supplied by
  the launcher. This handoff does not select a manual fallback path.
- **Manual fallback:** `.agents.local.env` was absent in the planning checkout.
  If the launcher worktree is unusable, ask the operator for an absolute
  `AGENTS_WORKTREE_CONTAINER_DIR` before creating a unique worktree there.
  Never use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** none. The numbered roadmap queue is the authority; do
  not create or cycle batch cards.
- **Roadmap milestone:** `docs/roadmaps/g09/README.md`.
- **Ready roadmaps:**
  `docs/roadmaps/g09/060-released-dependency-rollout-contract-normalization.md`
  only.
- **Allowed runway:** `g09.060` only.
- **Remaining roadmap budget:** one roadmap.
- **Dispatch topology:** serial. No consumer or release lane runs inside it.
- **Canonical refs:** `AGENTS.md`, `PAPERCUTS.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/023-release-and-compatibility-rollout.md`,
  `docs/contracts/024-new-app-bootstrap-and-bring-up.md`,
  `docs/guides/030-underlay-integration.md`,
  `docs/guides/040-rust-backend.md`,
  `docs/guides/190-upgrade-compatibility.md`,
  `docs/guides/200-project-sync.md`, `effigy.toml`, and `package.json`.
- **Planning evidence:**
  `docs/logs/2026-08/27-205758-g09-060-released-dependency-promotion.md`,
  `docs/logs/2026-08/27-210231-g09-060-contract-link-collision-gate.md`,
  `docs/logs/2026-08/27-214114-papercuts-wave3-closeout-and-g09-060-promotion.md`,
  and
  `docs/triage/20260827-094140-contract-023-released-dependency-drift.md`.
- **Read-only consumer evidence:** the roots `underlay-reference`,
  `contact-patch`, `compli-me`, `acowtancy`, `songsprout`, and
  `loophole/composer`. Inspect current root manifests and locks only as needed;
  do not edit them.
- **Model capability profile:** capable coding/docs worker with medium reasoning.
  Pause rather than inventing a new publishing or versioning policy.
- **Tool/runtime restrictions:** use Effigy first; no consumer edits, dependency
  mutations, version changes, releases, tag operations, registry publishing,
  `.github/workflows/` edits, or broad unrelated cleanup.
- **Required validation:** `effigy health`, `effigy qa:docs`,
  `effigy qa:northstar`, a targeted scan for every retired Contract `023`
  claim, and `git diff --check`.
- **PR base/head:** `main` ← selected worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation PR.
- **Merge authorisation:** not granted. The worker must not merge.

## Boundaries

Keep this run inside `g09.060`:

- Rewrite Contract `023` so private registry posture is distinct from release
  posture. Underlay remains registry-private and is released through immutable
  Git tags.
- Make the only committed JavaScript example
  `git+ssh://git@github.com/inflatable-cookie/underlay.git#vX.Y.Z`.
- Make the only committed Cargo example
  `{ git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "vX.Y.Z" }`.
- State that both language surfaces pin the same released tag. A consumer cannot
  pin an unreleased shared commit.
- Holding back means retaining the previous proven tag. Upgrading means changing
  every declared Underlay tag in the consumer root, regenerating root locks,
  and validating from that root.
- Sibling Underlay checkouts remain read-only QA/tooling inputs or untracked
  local Cargo patches. Committed `path` and `file:` edges are unsupported.
- Versions follow the release process and semantic versioning. Roadmap
  generation numbers never determine package versions.
- Preserve Contract `023` impact classification, compatibility windows,
  upgrade-note rules, caller proof, narrow retirement, and rollback semantics
  except where the retired path/file examples directly contradict them.
- Preserve the repo-relative links and current monorepo evidence paths delivered
  by PR12. Do not reopen the broad contract-link sweep.
- Update the contract index, contracts front door, roadmap/front-door state,
  triage disposition, and one execution log. Inspect guides `030`, `040`, `190`,
  and `200`; edit only a live contradiction exposed by this correction.
- Historical roadmaps, logs, and handoffs remain evidence. Do not bulk-rewrite
  their old terminology.
- Do not edit consumer manifests or locks, Underlay/Poodle versions,
  `package.json` release values, Effigy release behavior, workflows, tags, or
  registry policy. Do not publish anything.
- Stop if the work requires a new registry-publishing policy, release mutation,
  tooling behavior change, consumer edit, or choice between multiple supported
  dependency shapes.
- Work only in the selected clean worker worktree. Never clean, reset, stash,
  or discard another checkout's state.
- Do not merge the PR.

## Important Context

- Contract `024` already settles the supported monorepo and released-dependency
  rule. This worker aligns `023`; it does not reopen `024`.
- Underlay `package.json` is `private: true` and the synchronized Rust/JavaScript
  version is `0.9.5`. The private flag prevents registry publication; it does
  not mean the project is unreleased.
- Underlay `v0.9.5` is an immutable Git release tag. All six consumer roots use
  tagged Git dependencies rather than committed sibling path/file edges.
- PR12 merged reviewed head `d2cb5cd9` as `9e26ba9a`. It added the docs guard
  against machine-local contract paths and corrected contract evidence onto the
  current `apps/*` / `packages/*` shape.
- Contract `023` currently says the Underlay version tracks the roadmap
  generation and describes a Git tag as an optional hold-back mechanism. Both
  claims are retired and must be replaced, not qualified with compatibility
  prose.
- The existing compatibility and rollback framework remains useful. A rollback
  now means retaining or returning to a known-good released tag, not committing
  a local path dependency.
- `effigy doctor` carries known attention-marker and god-file backlog recorded
  in `PAPERCUTS.md`. Do not widen this docs-only roadmap into that debt.
- **Report after:** first, the Contract `023` semantic rewrite; second, current-
  surface/log alignment and final validation.
- **Report to:** the operator, who relays worker progress to the orchestrator.

## Suggested Next Move

Read this handoff from the top. Before broad repository reads, run the worktree
preflight below. If the current context is a clean, dedicated, non-`main`
registered worktree, use it immediately and record its actual root and branch.

Then read `AGENTS.md`, `PAPERCUTS.md`, the milestone, `g09.060`, contracts
`023` and `024`, the four named guides, and the three planning logs. Run
`effigy tasks`; use exact `rg` searches for the retired claims. Start with the
Contract `023` rewrite, then align only the currentness surfaces named by the
roadmap.

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
   `git merge-base --is-ancestor ec67dfbfb2add489e4309f801fdac6fbc953aeb0 HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read the milestone, assigned roadmap, `AGENTS.md`, `PAPERCUTS.md`, canonical
   refs, planning evidence, and triage note named above.
6. Run `effigy tasks`. Use `effigy doctor` only to record the known structural
   backlog; do not repair it. `effigy test --plan` is optional orientation for
   this docs-only lane and does not require a code test suite.

### While you work

- Execute only `g09.060`.
- Keep the Contract `023` semantic rewrite as one coherent chunk. Keep
  currentness/log alignment and final validation as the second chunk.
- Use `apply_patch` for edits and Effigy for supported validation.
- Verify read-only consumer evidence without changing consumer roots.
- After each report point, tell the operator which files changed, what passed,
  what remains, and whether a planning decision is needed.
- Stop on any scope expansion or policy choice named in the roadmap or this
  handoff.

### When the assigned runway is complete

1. Run `effigy health`, `effigy qa:docs`, `effigy qa:northstar`, the targeted
   retired-claim scan, and `git diff --check`.
2. Update `g09.060` with actual evidence and create one execution log under
   `docs/logs/2026-08/`. Set the roadmap and active front doors to `in review`.
   Do not open a new generation or invent a next roadmap.
3. Push the selected worker branch.
4. Open a reviewable PR against current pushed `main`. The planning base above
   predates the handoff commit and is intentionally not self-referential.
5. In the PR body, link `g09.060`, Contract `023`, Contract `024`, the execution
   log, changed currentness surfaces, read-only fleet evidence, and validation.
6. Report the PR URL and exact head to the operator. Do not merge.

### Review and merge path

The orchestrator will independently inspect PR metadata, commits, diff, checks,
and changed files against `g09.060`, contracts `023`/`024`, the active guides,
and this handoff. If the shared GitHub identity prevents formal self-approval,
the orchestrator will post its exact-head verdict as a PR comment.

Merge remains operator-authorized. Closeout refs are `g09.060`, its execution
log, `g09/README.md`, Contract `023`, the contract index/front door, the triage
note, and active roadmap/currentness front doors.

### Handoff closeout

Leave the roadmap, log, and next-task state honest. If blocked, record the
blocker and stop. Do not open another generation or create a new roadmap from
this worker.
