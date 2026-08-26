# g09.038 - Underlay Reference Migration And Test Proof

Status: complete
Owner: Underlay Reference maintainers
Contracts: `021-database-migration-and-schema-workflow.md`,
`022-testing-posture-and-shared-harnesses.md`,
`023-release-and-compatibility-rollout.md`
Found by: `g09.035`, `g09.036`
Depends on: `g09.037`
Completed: 2026-08-26

## Purpose

Make Underlay Reference the proof anchor for the baseline migration workflow,
API health minimum, and shared in-memory server harness.

## Promotion Gate

- [x] `g09.037` is complete
- [x] the target branch is current with Underlay Reference `main`
- [x] a disposable local PostgreSQL target is available and positively identified
- [x] no shared, UAT, staging, or production database can be reached by the proof

Readiness checked 2026-08-26: the target checkout is clean and exactly aligned
with `origin/main`. Effigy identifies database `acme` in
`underlay-reference-dev-postgres-1`, bound only to `127.0.0.1:19932` and backed
by the repo-scoped `underlay-reference-dev-postgres-data` volume. The server is
accepting connections and the system reports no shared services.

The proof must use the Effigy-owned `underlay-reference-dev` system and database
`acme`. Stop if any command resolves to another host, project, database, or
volume.

## Goals

- [x] establish the baseline root-state and package migration interface
- [x] prove from-empty application, replay, and fail-closed dev overlays
- [x] restore the API health minimum and prove one shared `TestServer` slice

## Execution Plan

- [x] cut root state orchestration and package migration selectors over as one
  task-interface batch
- [x] run the from-empty state apply and package replay proof against the
  positively identified local database
- [x] add the API health baseline and one bounded `TestServer` route-test proof
- [x] update workflow docs, run the full reference gates, and record closeout

## Scope

- add the root local state stack behind `effigy state plan` and
  `effigy state apply local --yes`
- route schema and reset/replay work through `apps/acme-api` package-owned
  `migration:*` tasks
- remove the retired `db:*` selectors and active docs references without
  compatibility aliases
- preserve separate structural migrations and intentional dev overlays
- add an API build/check baseline to package `health`
- prove `underlay_testing::TestServer` in one existing direct-Axum route-test
  slice while keeping app state, auth, and fixtures app-owned
- retain the admin and front packages at their accepted minimum posture; do not
  promote unrelated suites in the reference proof
- update root and package workflow docs atomically

## Acceptance Criteria

- [x] root state plan names the schema and dev-overlay work in application order
- [x] local state apply from empty reaches the same declared baseline as the
  package reset/replay task
- [x] dev-overlay failure makes reset/state apply fail
- [x] no active task or guide advertises `db:migrate`, `db:reset`, or `db:drop`
- [x] API health includes a cheap Cargo build/check baseline
- [x] one representative route test uses `TestServer`; no app-local state/auth
  fixture moves into Underlay

## Validation

- `effigy tasks`
- `effigy state plan`
- `effigy state apply local --yes` against the disposable database
- routed `acme-api/migration:*` reset/replay proof
- `effigy acme-api/health`
- targeted API route test
- `effigy health`
- `effigy validate`
- `effigy qa`
- retired-selector/docs search

## Stop Conditions

Stop before database mutation if the target cannot be proved disposable. Stop
the harness slice if it requires shared ownership of app state, auth, fixtures,
or fixed-schema isolation.

## Consumer Upgrade Impact

- Impact class: breaking local task-interface cutover
- Affected consumer: Underlay Reference
- Required action: use root state plan/apply and package `migration:*`
  selectors; remove any local automation that calls `db:*`
- Compatibility window: none; retired aliases must not survive the cutover

## Completion Evidence

- Underlay Reference PR
  [#4](https://github.com/inflatable-cookie/underlay-reference/pull/4)
  merged as `854e5ad2f9d4a7c62277447b6686bacb166516e7` from reviewed head
  `fc83785244b1635a2d620f3ed0344bea37869079`.
- Exact clean-shell `effigy state apply local --yes` completed the ordered
  `reset -> structure -> dev-overlay` lineage in the PR worktree.
- From-empty, replay, and forced-overlay-failure proof stayed inside database
  `acme` on the loopback-only `underlay-reference-dev` system.
- `effigy test --plan` selected the three real test targets after the empty
  Acme Front Vitest suite became non-default.
- Independent `effigy validate`, `effigy qa`, API Rust tests, admin/client
  Vitest suites, Svelte checks/builds, docs checks, retired-selector search,
  and `git diff --check` passed.
- The merged target execution log is
  `underlay-reference/docs/logs/2026-08/26-184058-g09-038-migration-and-test-proof.md`.

## Next Task

Dispatch ready roadmaps `g09.039`–`g09.043` as five independent consumer lanes.
