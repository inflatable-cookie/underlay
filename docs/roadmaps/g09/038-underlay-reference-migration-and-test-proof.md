# g09.038 - Underlay Reference Migration And Test Proof

Status: ready
Owner: Underlay Reference maintainers
Contracts: `021-database-migration-and-schema-workflow.md`,
`022-testing-posture-and-shared-harnesses.md`,
`023-release-and-compatibility-rollout.md`
Found by: `g09.035`, `g09.036`
Depends on: `g09.037`

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

- [ ] establish the baseline root-state and package migration interface
- [ ] prove from-empty application, replay, and fail-closed dev overlays
- [ ] restore the API health minimum and prove one shared `TestServer` slice

## Execution Plan

- [ ] cut root state orchestration and package migration selectors over as one
  task-interface batch
- [ ] run the from-empty state apply and package replay proof against the
  positively identified local database
- [ ] add the API health baseline and one bounded `TestServer` route-test proof
- [ ] update workflow docs, run the full reference gates, and record closeout

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

- [ ] root state plan names the schema and dev-overlay work in application order
- [ ] local state apply from empty reaches the same declared baseline as the
  package reset/replay task
- [ ] dev-overlay failure makes reset/state apply fail
- [ ] no active task or guide advertises `db:migrate`, `db:reset`, or `db:drop`
- [ ] API health includes a cheap Cargo build/check baseline
- [ ] one representative route test uses `TestServer`; no app-local state/auth
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

## Next Task

After this proof merges, promote `g09.039`–`g09.043` as independent consumer
lanes.
