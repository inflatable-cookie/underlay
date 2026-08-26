# g10.022 - Composer Migration And Fail-Closed Gates

Status: planned
Owner: Composer maintainers
Contracts: `021-database-migration-and-schema-workflow.md`,
`022-testing-posture-and-shared-harnesses.md`
Found by: `g10.015`, `g10.016`
Depends on: `g10.018`

## Purpose

Adopt the baseline migration workflow, include Composer's intentional dev
overlay in replay, and connect its current API/admin proof to the merge gate.

## Promotion Gate

- `g10.018` is complete
- Composer `main` is current
- a disposable local PostgreSQL target is available and positively identified

## Scope

- add the root local state stack
- rename `apps/composer-api` DB selectors to package-owned `migration:*`
  selectors
- remove retired `db:*` selectors and active docs references without aliases
- make reset/replay apply the committed dev overlay and propagate its failure
- make local startup dev-overlay handling fail visibly within the local-only
  policy boundary
- add the existing API module tests and admin freshness test to package
  validate/QA and root merge-gate orchestration

## Acceptance

- root state apply and API reset/replay reach structural plus intentional dev
  state from empty
- reset and local startup do not report success after dev-overlay failure
- no active task or guide advertises a retired `db:*` selector
- existing API module tests and admin freshness proof run through root
  validate/QA
- health remains cheap and does not require a database test environment

## Validation

- `effigy tasks`
- `effigy test --plan`
- `effigy state plan`
- `effigy state apply local --yes` against the disposable database
- routed `composer-api/migration:*` reset/replay and forced-overlay-failure proof
- targeted API module and admin freshness tests
- package validate/QA selectors
- `effigy health`
- `effigy validate`
- `effigy qa`
- retired-selector/docs search

## Stop Conditions

Stop before mutation if the database is not disposable. Stop if startup
failure-policy changes would escape the local/dev overlay path or alter
production availability policy.

## Consumer Upgrade Impact

- Impact class: breaking local task-interface cutover and local failure-policy
  correction
- Affected consumer: Composer
- Required action: replace `db:*` calls with routed `migration:*` selectors;
  fix any local workflow that assumed reset omitted the dev overlay
- Compatibility window: none

## Next Task

Close this lane independently; `g10.019`–`g10.021` and `g10.023` may run in
parallel.
