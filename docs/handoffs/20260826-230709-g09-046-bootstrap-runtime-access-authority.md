---
title: g09.046 bootstrap runtime access authority worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-26
updated: 2026-08-26
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260826-230709-g09-046-bootstrap-runtime-access-authority.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g09, bootstrap, runtime, access]
---

## What This Thread Was Doing

The orchestrator closed `g09.045`, a read-only assessment of contracts `024`-
`026` across Underlay and all six supported consumer roots. The supported
monorepo shape held. The assessment found shared-authority and conformance drift
before the app-owned runtime/security repairs can start.

This handoff starts the one ready serial batch: repair Underlay's bootstrap,
runtime, route/access, workspace-checker, and env-authority surfaces. It does
not edit a consumer repository or choose an app's secrets, proxy topology,
CSRF exception, rate-limit failure policy, or route compatibility window.

## Why It Matters

The next six consumer roadmaps depend on one coherent reference contract. Today
guide `070` contradicts the thin-router rule, contracts contain retired paths and
mixed runtime/version wording, the workspace checker misses two normative shape
violations, and the app-review artifact cannot mechanically distinguish a
committed env/secret authority from folklore. Dispatching consumers first would
make each worker settle the same shared questions differently.

## Current State

- **Repository:** `git@github.com:inflatable-cookie/underlay.git`
- **Planning branch:** `main`
- **Planning base commit:** `d9761b213c381363ac670e6ed97d35f22d2865b5`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `d9761b213c381363ac670e6ed97d35f22d2865b5` before this handoff was created.
- **Planning checkout:** clean after the `g09.045` assessment closeout push.
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** completed `g09.045`, its exact
  six-root assessment log, ready `g09.046`, planned `g09.047`-`g09.053`, and
  aligned roadmap, contract, specs, architecture, and Northstar front doors.
- **Worker branch label:** `worker/g09-046-bootstrap-runtime-access-authority`.
- **Worker worktree:** use the clean dedicated non-`main` worktree supplied by
  the launcher. This handoff does not select a manual fallback path.
- **Manual fallback command:** only after the operator supplies an absolute
  `AGENTS_WORKTREE_CONTAINER_DIR`, create a unique worktree there from
  `origin/main`; `.agents.local.env` was absent in the planning checkout, so do
  not create a manual fallback without asking.
- **Active spec lane:** none. `docs/specs/README.md` says the numbered roadmap
  queue is the authority; do not create batch cards.
- **Roadmap milestone:** `docs/roadmaps/g09/README.md`.
- **Ready roadmaps, in order:**
  `docs/roadmaps/g09/046-bootstrap-runtime-access-authority.md` only.
- **Allowed runway:** `g09.046` only.
- **Remaining roadmap budget:** one roadmap.
- **Dispatch topology:** serial. `g09.047` consumes the shared authority produced
  here; consumer roadmaps `g09.048`-`g09.052` stay planned.
- **Parallel safety check:** this batch changes the contracts, guide, tools, test
  fixtures, templates, checklist artifact, and roadmap currentness consumed by
  every later lane. It cannot run in parallel with reference or consumer work.
- **Canonical refs:** `AGENTS.md`, `PAPERCUTS.md`,
  `docs/architecture/product-guardrails.md`,
  `docs/architecture/system-inventory.md`,
  `docs/contracts/001-working-rules.md`,
  `docs/contracts/024-new-app-bootstrap-and-bring-up.md`,
  `docs/contracts/025-rust-app-runtime-assembly-and-router-topology.md`,
  `docs/contracts/026-route-families-and-access-model.md`,
  `docs/contracts/031-config-and-secrets.md`,
  `docs/contracts/120-tooling-testing-and-contract-artifacts.md`,
  `docs/contracts/121-underlay-app-review-checklist-and-audit-artifact.md`,
  `docs/contracts/app-review/underlay-app-review-checklist.json`,
  `docs/guides/070-api-handlers.md`, and the `g09.045` assessment log.
- **Implementation refs:** `ts/src/tools/workspace-shape.ts`,
  `ts/tests/tools/workspace-shape.test.ts`,
  `ts/bin/underlay-workspace-shape.ts`, `scripts/check-env-manifest.sh`,
  `templates/config/env-manifest.example.txt`,
  `templates/config/required-secrets.example.txt`, `package.json`, and
  `effigy.toml`.
- **Live proof, read-only:** Underlay Reference root at
  `/Users/tom/Dev/projects/underlay-reference`. Do not edit it.
- **Model capability profile:** frontier coding worker with high reasoning. This
  lane touches security and public conformance semantics; pause rather than
  guessing when a contract choice is not already settled by the roadmap.
- **Tool/runtime restrictions:** use Effigy first; no consumer edits, dependency
  changes, releases, `.github/workflows/` edits, state/container/database
  mutations, or broad unrelated cleanup.
- **Required validation:** `effigy test --plan`; focused workspace and env-
  authority tests through Effigy; published-bin smoke against Underlay
  Reference; `effigy health`; `effigy validate`; `effigy qa:docs`;
  `effigy qa:northstar`; `git diff --check`.
- **PR base/head:** `main` ← worker branch.
- **PR URL:** pending.
- **Review state:** awaiting implementation PR.
- **Merge authorisation:** not granted. The worker must not merge.

## Boundaries

Please keep this run inside `g09.046`:

- **In scope:** current portable source evidence in contracts `025`/`026`; guide
  `070` thin-binary/router correction; runtime-family and API-version wording;
  contract `024` external-input/tooling-mount clarification; workspace-shape
  rules and focused fixtures; a separate generic static env/secret-authority
  check and distributable invocation; contract `121` plus its JSON artifact;
  templates/guidance; focused tests; roadmap/log/front-door evidence.
- **Out of scope:** any consumer edit; creating real consumer manifests; choosing
  required secret contents; changing cookie/CSRF behavior; changing proxy or
  rate-limit behavior; public route migrations; Effigy schema changes; package
  dependency or version upgrades; releases; unrelated doc cleanup; and
  implementation of `g09.047`-`g09.053`.
- Contract edits are authorized only where `g09.046` names the shared ambiguity
  or stale evidence. Do not redesign contracts `024`-`026` or widen into auth,
  config, runtime-maturity, or compatibility assessment work.
- Keep the monorepo boundary strict: one Git root, runtime apps under `apps/*`,
  reusable packages under `packages/*`, one root lock, released Underlay/Poodle
  edges. External read-only content inputs and sibling dev tooling mounts may be
  named, but cannot become workspace ownership or committed source dependencies.
- Version policy is already settled by the assessment: `/v1/*` path versioning
  is baseline. The header is optional until an app advertises, sends, logs, or
  validates it; once declared, the server applies it consistently across
  business families. Runtime endpoints are exempt.
- Runtime is a distinct operational family. Auth/account remain shared business
  routes. Preserve the allowed lean/rich health, metrics, and OpenAPI profiles.
- Workspace-shape adds stable failures for a declared JS workspace outside
  `apps/*` / `packages/*` and committed `file:` Underlay/Poodle dependencies.
  Do not turn it into the env scanner or a full contract auditor.
- Add env/secret authority as its own mechanical review seam. It must prove the
  required files and syntax/relationship without reading secret values or
  guessing which keys are mandatory. `scripts/check-env-manifest.sh` currently
  validates a live environment; preserve that runtime use or separate static
  audit mode cleanly rather than making CI require secrets.
- Keep tools generic and distributable through the existing published `ts/`
  package/bin pattern. Diagnostics need stable rule IDs, repo-relative paths,
  deterministic ordering, and focused fixtures.
- Update contract `121` and its JSON artifact together. Keep the artifact small
  and contract-shaped; do not embed app-specific results.
- Historical roadmaps/logs/handoffs may retain raw paths. Do not bulk-rewrite
  frozen evidence.
- Work only in the selected clean worker worktree. Prefer the launcher-provided
  worktree and record its actual path/branch. If it is unusable, stop for
  operator configuration rather than guessing a path.
- Do not merge the PR.

## Important Context

- **Planning lineage:** `g09.021`-`g09.030` made the monorepo shape normative and
  proved it across the fleet. `g09.045` confirmed that rollout still holds; this
  is a second-order authority/conformance repair, not another directory move.
- **Why the roadmap is ready:** the six-root evidence is exact and read-only;
  every generic ambiguity has a recorded recommendation; app-owned choices are
  explicitly excluded; acceptance, validation, stop conditions, and serial
  continuation are written in `g09.046`.
- **Existing checker behavior:** `workspace-shape.ts` already checks Git roots,
  nested Git metadata, private/pinned root manifests, explicit/missing/outside
  workspace paths, undeclared packages under the two supported prefixes, root
  and child locks, and internal dependency protocols. Preserve existing rule
  IDs and published CLI behavior.
- **Checker gaps:** discovery scans only `apps` and `packages`, so a declared
  `web/foo` package can pass. `file:` rejection currently recognizes internal
  package names/targets only, so a committed sibling Underlay/Poodle edge can
  pass. Add narrow explicit rules and fixtures; do not reject arbitrary valid
  external `file:` development dependencies unless the contract does.
- **Env check boundary:** all six normal roots lack both canonical authority
  files. Underlay already ships examples and a runtime-oriented shell check.
  The new mechanical audit should be safe in CI without material secret values.
  It should not claim static completeness against every possible dynamic env
  read unless the implementation can prove that generically.
- **Guide contradiction:** `docs/guides/070-api-handlers.md:34-56` recommends
  router/handlers/DTOs inline in `main.rs`; contract `025` requires one obvious
  router builder outside the thin binary. Rewrite the example around
  `main.rs`, `state.rs`, and explicit `routes/{runtime,shared,admin,front?}`
  ownership. Do not force empty product families or crates.
- **Portable evidence:** active contracts must use repo-local links for Underlay
  content and prose/repo-relative references for sibling consumers. Absolute
  `/Users/...` links are forbidden on the active documentation surface.
- **Known red state:** `effigy doctor` has pre-existing attention-marker and
  large-file findings tracked in `PAPERCUTS.md`. Do not attribute or fix them.
- **Report after:** first, shared contract/guide decisions; second,
  workspace/env tool behavior with fixtures and published invocation; third,
  artifact/currentness alignment and final validation/PR.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Read this handoff from the top. Before broad repository reads, run the quick
worktree-safety preflight below. If the current context is a clean, dedicated,
non-`main` registered worktree, use it immediately regardless of generated path
or branch name. Record the actual path and branch; do not create another
worktree.

Then read `AGENTS.md`, `g09/README.md`, `g09.046`, the `g09.045` log, contracts
`024`-`026`, `120`, `121`, guide `070`, and the checklist artifact. Run
`effigy tasks`, inspect `effigy test --plan`, and use `effigy graph` for tooling
ownership before editing. Start with contract/guide semantics, then settle the
two narrow workspace rules and the static env-authority interface before
writing implementation fixtures.

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
   actual root and branch. Do not compare it with the branch label in this
   handoff or create another worktree because it differs.
3. If the launcher supplied a dirty or `main` worktree, stop and report it. If a
   manual fallback is genuinely needed, read `.agents.local.env` and require an
   absolute `AGENTS_WORKTREE_CONTAINER_DIR`. The planning checkout did not have
   that file, so ask the operator before creating it or any worktree. Never use
   `/tmp`, `TMPDIR`, or a guessed path; never clean, reset, stash over, or
   discard another checkout's state.
4. From the selected worktree, run `git fetch origin`. Confirm `HEAD` equals
   `origin/main`, confirm
   `git merge-base --is-ancestor d9761b213c381363ac670e6ed97d35f22d2865b5 HEAD`,
   and confirm this handoff exists in `HEAD`.
5. Read the milestone, assigned roadmap, `AGENTS.md`, `PAPERCUTS.md`, assessment
   log, and canonical refs named above.
6. Run `effigy tasks`, `effigy test --plan`, and `effigy doctor`. Record known
   Doctor debt without widening the roadmap. Run
   `effigy graph status --refresh --json` before using graph evidence.

### While you work

- Execute only `g09.046`.
- Keep contract/guide semantics as one coherent chunk. Keep checker/env tool and
  fixtures as a second coherent chunk. Finish artifact/currentness alignment and
  final validation as the third.
- Use `apply_patch` for file edits and Effigy for supported test/QA work.
- Test new rule failures with fixtures. Run live consumer checks read-only; do
  not make a conforming result depend on local secrets.
- After each report point, tell the operator which files changed, what
  validation ran, what remains, and whether a planning decision is needed.
- Stop on a required app-specific secret choice, deployment policy, CSRF/proxy
  behavior change, public route compatibility question, new Effigy schema
  requirement, or validation result that changes the plan.

### When the assigned runway is complete

1. Run `effigy test --plan`, the focused workspace/env-authority tests through
   Effigy, the published-bin smoke against Underlay Reference, `effigy health`,
   `effigy validate`, `effigy qa:docs`, `effigy qa:northstar`, and
   `git diff --check`.
2. Update `g09.046` with actual evidence and create one execution log under
   `docs/logs/2026-08/`. Set the roadmap/front doors to `in review`; keep
   `g09.047` planned because promotion belongs to orchestrator review/merge.
3. Push the selected worker branch.
4. Open a reviewable PR against current pushed `main`. The planning base above
   predates the handoff commit and is intentionally not self-referential.
5. In the PR body, link `g09.046`, the milestone, contracts, changed tools,
   fixtures, published invocation, live read-only proof, validation, and any
   unresolved items.
6. Report the PR URL and evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will independently review PR metadata, commits, diff, checks,
and changed files against `g09.046`, the assessment log, contracts `024`-`026`,
`120`, `121`, and the app-review artifact. If the same GitHub identity prevents
formal approval, the orchestrator will post the verdict as a PR comment. Make
only requested changes on this worker branch and report back through the
operator.

Merge remains operator-authorized. Closeout refs are the `g09.046` roadmap, its
execution log, `g09/README.md`, contracts `024`-`026`, `120`, `121`, the app-
review artifact, guide `070`, and active roadmap/currentness front doors.

### Handoff closeout

Leave the roadmap, log, and next-task state honest. If blocked, record the
blocker and stop. Do not make `g09.047` look ready before orchestrator review,
operator-authorized merge, exact-main verification, and its own promotion gate.
