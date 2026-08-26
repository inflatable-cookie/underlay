# g09.043 - Acowtancy State And Test Orchestration Repair

Status: ready
Owner: Acowtancy maintainers
Contracts: `021-database-migration-and-schema-workflow.md`,
`022-testing-posture-and-shared-harnesses.md`
Found by: `g09.035`, `g09.036`
Depends on: `g09.038`

## Purpose

Make Acowtancy's advanced local state stack apply what it stages, restore the
API proof baseline, and remove the live shared-mock compatibility cast.

## Promotion Gate

- [x] `g09.037` and `g09.038` are complete
- [x] Acowtancy `main` is current at `3cdd5efe4f8e58e52618935bb73b419656faa481`
- [x] a disposable local PostgreSQL target and disposable artifact workspace are
  available and positively identified
- [x] the canonical local seed bundle is available through the declared state path

Readiness checked 2026-08-26: system `acowtancy-dev`, container
`acowtancy-dev-postgres-1`, database `acowtancy`, user `postgres`, host binding
`127.0.0.1:22432`, volume `acowtancy-dev-postgres-data`, and no shared services;
the database accepts connections. `effigy state plan local` resolves the pinned
canonical spine digest `sha256:79d0165f4477...`, and an independent registry
manifest fetch succeeds. Re-prove database, artifact workspace, and exact digest
before the first mutation.

## Scope

- make `state apply local` invoke the existing Farmyard bundle application and
  dev-overlay path after artifact installation
- fail the state operation when bundle or dev-overlay application fails
- preserve one app-owned implementation seam; do not move bundle semantics into
  Underlay or Effigy
- add a cheap build/check baseline to Farmyard `health`
- make root QA reach `farmyard/qa`, including its managed DB suite and teardown
- remove Cattle Grid's `as unknown` HTTP-client mock adapter after `g09.037`
  proves structural compatibility
- keep Dairy and Cream at their accepted minimum posture; do not force their
  broad configured suites into this repair

## Acceptance

- from-empty local state apply installs and applies the canonical bundle, then
  applies the dev overlay exactly once per declared lifecycle
- repeated apply proves idempotence or returns an explicit bounded replay error
- bundle/dev-overlay failure makes root state apply fail and leaves evidence
- Farmyard health contains a cheap Cargo build/check baseline
- root QA reaches the package-owned Farmyard managed suite and always tears its
  test database down
- Cattle Grid consumes `createMockHttpClient()` without `as unknown`
- app-local DB, bundle, auth, and router composition stay app-owned

## Validation

- `effigy tasks`
- `effigy test --plan`
- `effigy state plan`
- from-empty and repeated `effigy state apply local --yes` against disposable
  state
- bundle row/invariant and forced-failure proof
- `effigy farmyard/health`
- `effigy farmyard/qa`
- targeted Cattle Grid typecheck/tests
- `effigy health`
- `effigy validate`
- `effigy qa`

## Stop Conditions

Stop before mutation if database or artifact targets are not disposable. Stop
if root QA cannot guarantee Farmyard teardown on failure. Return to Underlay if
removing the mock cast exposes a real shared-interface mismatch.

## Consumer Upgrade Impact

- Impact class: local state correctness and merge-gate hardening
- Affected consumer: Acowtancy
- Required action: local state users should expect bundle and dev-overlay
  application, not artifact staging only; QA now includes the managed Farmyard
  suite
- Compatibility window: none for incomplete state application

## Next Task

Close this lane independently. `g09.044` waits for all five consumer repair
roadmaps.
