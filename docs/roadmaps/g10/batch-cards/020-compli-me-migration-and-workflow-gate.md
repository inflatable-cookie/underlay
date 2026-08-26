# g10.020 - Compli Me Migration And Workflow Gate

Status: planned
Owner: Compli Me maintainers
Contracts: `021-database-migration-and-schema-workflow.md`,
`022-testing-posture-and-shared-harnesses.md`
Found by: `g10.015`, `g10.016`
Depends on: `g10.018`

## Purpose

Adopt the baseline migration workflow and connect Compli Me's existing admin
workflow proof to the normal merge gate.

## Promotion Gate

- `g10.018` is complete
- Compli Me `main` is current
- a disposable local PostgreSQL target is available and positively identified

## Scope

- add the root local state stack
- rename `apps/api` DB selectors to package-owned `migration:*` selectors
- remove retired `db:*` selectors and active docs references without aliases
- preserve fail-closed structural/dev-overlay replay
- add the existing admin reorder workflow test to package `validate` and root
  merge-gate orchestration
- retain the API DB integration suite outside mandatory gates in this card; its
  environment is not part of the declared baseline

## Acceptance

- root state apply and API reset/replay reach the same local baseline
- overlay failure fails closed
- no active task or guide advertises a retired `db:*` selector
- the admin reorder workflow test runs from the advertised validate/QA path
- health stays cheap; no DB-backed suite moves into health
- the API remains correctly classified as minimum; this card does not invent a
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

Close this lane independently; `g10.019` and `g10.021`–`g10.023` may run in
parallel.
