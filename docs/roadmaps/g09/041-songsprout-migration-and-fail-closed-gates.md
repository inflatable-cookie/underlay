# g09.041 - Songsprout Migration And Fail-Closed Gates

Status: changes-requested
Owner: Songsprout maintainers
Contracts: `021-database-migration-and-schema-workflow.md`,
`022-testing-posture-and-shared-harnesses.md`
Found by: `g09.035`, `g09.036`
Depends on: `g09.038`

## Purpose

Adopt the baseline migration workflow, make Nursery dev overlays fail closed,
and connect the existing Greenhouse and Bloom workflow suites to merge proof.

## Promotion Gate

- [x] `g09.038` is complete
- [x] Songsprout `main` is current at `618a5323571fcb2db8f4fac82a42a0b469274d4e`
- [x] a disposable local PostgreSQL target is available and positively identified

Readiness checked 2026-08-26: system `songsprout-dev`, container
`songsprout-dev-postgres-1`, database `songsprout`, user `postgres`, host binding
`127.0.0.1:52732`, volume `songsprout-dev-postgres-data`, no shared services,
and the database accepts connections. Re-prove every fact before the first
mutation because the allocated loopback port may change after stack recreation.

## Review Gate

Re-review on 2026-08-26 found the implementation clean in the
[Songsprout PR4 follow-up](https://github.com/inflatable-cookie/songsprout/pull/4#issuecomment-5430970590)
at exact head `713ac411baae6fa1095caa6594dffa7f26e86438`. The PR description still claims
the repaired local-dev fail-open behavior is unresolved. Correct that provider
evidence before the merge verdict.

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
