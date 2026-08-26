# g09.042 - Composer Migration And Fail-Closed Gates

Status: changes-requested
Owner: Composer maintainers
Contracts: `021-database-migration-and-schema-workflow.md`,
`022-testing-posture-and-shared-harnesses.md`
Found by: `g09.035`, `g09.036`
Depends on: `g09.038`

## Purpose

Adopt the baseline migration workflow, include Composer's intentional dev
overlay in replay, and connect its current API/admin proof to the merge gate.

## Promotion Gate

- [x] `g09.038` is complete
- [x] Composer `main` is current at `153b47afa68b61aaaf7e64daa6d79ac0be566343`
- [x] a disposable local PostgreSQL target is available and positively identified

Readiness checked 2026-08-26: system `loophole-composer-dev`, container
`loophole-composer-dev-postgres-1`, database `composer`, user `postgres`, host
binding `127.0.0.1:58832`, volume `loophole-composer-dev-postgres-data`, no
shared services, and the database accepts connections. Re-prove every fact
before the first mutation.

## Review Gate

Review on 2026-08-26 requested changes in the
[Composer PR4 review](https://github.com/inflatable-cookie/loophole-composer/pull/4#issuecomment-5430715131)
at exact head `62160f94b455224696e8e256777261a0b4a37d2a`. Rework three boundaries before
re-review: keep durable prerequisites and migration history on the explicit
forward-only contract, preserve Composer's canonical container runtime instead
of forcing every child task onto the host, and route API/Admin validation to
explicit package-owned tests rather than the workspace-wide built-in board.

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

Close this lane independently; `g09.039`–`g09.041` and `g09.043` may run in
parallel.
