# g09.040 - Compli Me Migration And Workflow Gate

Status: changes-requested
Owner: Compli Me maintainers
Contracts: `021-database-migration-and-schema-workflow.md`,
`022-testing-posture-and-shared-harnesses.md`
Found by: `g09.035`, `g09.036`
Depends on: `g09.038`

## Purpose

Adopt the baseline migration workflow and connect Compli Me's existing admin
workflow proof to the normal merge gate.

## Promotion Gate

- [x] `g09.038` is complete
- [x] Compli Me `main` is current at `240dce062ef5f0817b34caffaf7743542337d45a`
- [x] a disposable local PostgreSQL target is available and positively identified

Readiness checked 2026-08-26: system `compli-me-dev`, container
`compli-me-dev-postgres-1`, database `compli_me`, user `postgres`, host binding
`127.0.0.1:22132`, volume `compli-me-dev-postgres-data`, no shared services,
and the database accepts connections. Re-prove every fact before the first
mutation.

## Review Gate

Re-review on 2026-08-26 requested changes in the
[Compli Me PR6 follow-up](https://github.com/double-dip/compli-me/pull/6#issuecomment-5430970599)
at exact head `2040df643501fc03c056c88098c667357c0e4c40`. The published media migration
is restored, but `run_migrations` now creates the durable `media` schema before
SQLx. Move that idempotent repair into a predecessor migration, remove the
runtime DDL, and re-prove empty plus already-applied histories.

## Scope

- add the root local state stack
- rename `apps/api` DB selectors to package-owned `migration:*` selectors
- remove retired `db:*` selectors and active docs references without aliases
- preserve fail-closed structural/dev-overlay replay
- add the existing admin reorder workflow test to package `validate` and root
  merge-gate orchestration
- retain the API DB integration suite outside mandatory gates in this roadmap; its
  environment is not part of the declared baseline

## Acceptance

- root state apply and API reset/replay reach the same local baseline
- overlay failure fails closed
- no active task or guide advertises a retired `db:*` selector
- the admin reorder workflow test runs from the advertised validate/QA path
- health stays cheap; no DB-backed suite moves into health
- the API remains correctly classified as minimum; this roadmap does not invent a
  managed DB-test environment

## Validation

- `effigy tasks`
- `effigy test --plan`
- `effigy state plan`
- `effigy state apply local --yes` against the disposable database
- routed `api/migration:*` reset/replay proof
- targeted admin workflow test
- `effigy admin/validate`
- `effigy health`
- `effigy validate`
- `effigy qa`
- retired-selector/docs search

## Stop Conditions

Stop before mutation if the database is not disposable. Do not make the API DB
suite a mandatory gate until setup, teardown, credentials, and runtime bounds
are explicit.

## Consumer Upgrade Impact

- Impact class: breaking local task-interface cutover plus stronger merge proof
- Affected consumer: Compli Me
- Required action: replace `db:*` calls with routed `migration:*` selectors and
  use root state plan/apply
- Compatibility window: none

## Next Task

Close this lane independently; `g09.039` and `g09.041`–`g09.043` may run in
parallel.
