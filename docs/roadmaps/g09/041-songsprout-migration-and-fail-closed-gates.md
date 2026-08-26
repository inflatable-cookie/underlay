# g09.041 - Songsprout Migration And Fail-Closed Gates

Status: planned
Owner: Songsprout maintainers
Contracts: `021-database-migration-and-schema-workflow.md`,
`022-testing-posture-and-shared-harnesses.md`
Found by: `g09.035`, `g09.036`
Depends on: `g09.038`

## Purpose

Adopt the baseline migration workflow, make Nursery dev overlays fail closed,
and connect the existing Greenhouse and Bloom workflow suites to merge proof.

## Promotion Gate

- `g09.038` is complete
- Songsprout `main` is current
- a disposable local PostgreSQL target is available and positively identified

## Scope

- add the root local state stack
- rename `apps/nursery` DB selectors to package-owned `migration:*` selectors
- remove retired `db:*` selectors and active docs references without aliases
- propagate committed dev-overlay failures instead of logging and succeeding
- add the existing Greenhouse and Bloom workflow suites to their package
  validate/QA paths and root merge-gate orchestration
- leave the Nursery DB integration test out of mandatory gates in this roadmap;
  its environment and teardown are not part of the declared baseline

## Acceptance

- root state apply and Nursery reset/replay apply structural migrations and the
  intentional dev overlay
- a dev-overlay error fails the package task and root state apply
- no active task or guide advertises a retired `db:*` selector
- Greenhouse and Bloom workflow suites run through the advertised merge gate
- health remains check/build focused and starts no DB-backed suite
- test failure in either shell makes root validate/QA fail

## Validation

- `effigy tasks`
- `effigy test --plan`
- `effigy state plan`
- `effigy state apply local --yes` against the disposable database
- routed `nursery/migration:*` reset/replay and forced-overlay-failure proof
- targeted Greenhouse and Bloom suites
- package validate/QA selectors
- `effigy health`
- `effigy validate`
- `effigy qa`
- retired-selector/docs search

## Stop Conditions

Stop before mutation if the database is not disposable. Stop if fail-closed
behavior would affect production startup rather than the declared local overlay
lane. Do not pull DB integration setup into health.

## Consumer Upgrade Impact

- Impact class: breaking local task-interface cutover and local failure-policy
  correction
- Affected consumer: Songsprout
- Required action: replace `db:*` calls with routed `migration:*` selectors;
  fix any local workflow that depended on dev-overlay failure being ignored
- Compatibility window: none

## Next Task

Close this lane independently; `g09.039`, `g09.040`, `g09.042`, and `g09.043`
may run in parallel.
