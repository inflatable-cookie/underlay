# g09.044 - Migration And Testing Fleet Closeout

Status: complete
Owner: repo maintainers
Contracts: `021-database-migration-and-schema-workflow.md`,
`022-testing-posture-and-shared-harnesses.md`,
`023-release-and-compatibility-rollout.md`
Found by: `g09.035`, `g09.036`
Depends on: `g09.039`, `g09.040`, `g09.041`, `g09.042`, `g09.043`

## Purpose

Prove the migration/testing repair wave across the fleet, close its upgrade
surface, and return the unresolved whole-app DB harness boundary to an explicit
operator decision.

## Promotion Gate

- [x] `g09.037`–`g09.043` are complete and merged
- [x] every consumer proof names its verified `main` revision
- [x] no consumer PR or destructive-state validation remains in flight

## Scope

- verify all five baseline roots expose state plan/apply and package-owned
  `migration:*` selectors with no retired `db:*` surface
- verify Underlay and both drifted API health routes now meet the cheap baseline
- verify the selected risk-bearing workflow suites and Farmyard managed QA are
  reachable from their advertised root gates
- verify Acowtancy local state application and all fail-closed overlay repairs
- record why remaining minimum-posture packages stay minimum instead of being
  forced into speculative suites
- publish one compact fleet upgrade note and final evidence matrix
- present the `TestDb` whole-app isolation choice to the operator without
  selecting or implementing a design by implication

## Acceptance

- every `g09.035` and `g09.036` finding has a completed repair, an explicit
  contract match, or the named DB-harness operator decision
- task, docs, state, failure-policy, and gate evidence names all six roots and
  affected child packages
- no active guide or consumer task advertises a retired `db:*` selector
- `TestServer` has one reference proof and no forced fleet rewrite
- existing fixed-schema app fixtures remain app-owned pending the DB-harness
  decision
- roadmap, contract index, inventory, logs, specs, and generation front doors
  agree on the next contract-assessment group

## DB Harness Decision

The operator selected app-owned whole-app fixed-schema suites on 2026-08-26.
`TestDb` remains the preferred shared-crate and single-schema seam. Contract
`022` now records that boundary. No multi-schema or database-per-test design is
open by implication.

## Closeout Evidence

- all six consumer roots resolve the expected state, migration, and test plans
- all five baseline consumers expose routed API `migration:*` selectors with no
  retired consumer `db:*` task
- Underlay Reference proves `TestServer`; no forced fleet rewrite followed
- Compli Me, Songsprout, Composer, and Acowtancy expose their named risk suites
  through the repaired merge gates
- merged disposable-state evidence proves from-empty, repeat, and fail-closed
  behavior without repeating destructive operations during this closeout
- remaining minimum-posture packages stay minimum because no unowned product
  risk justifies speculative suites

Full revisions, finding disposition, residuals, and validation are in
[`g09.044 - Migration And Testing Fleet Closeout`](../../logs/2026-08/26-222718-g09-044-migration-testing-fleet-closeout.md).

## Validation

- six-root task and retired-selector inventory
- six-root `effigy test --plan`
- merged per-roadmap validation evidence; do not repeat destructive state applies
  without a new reason
- Underlay `effigy health`
- Underlay `effigy qa:docs`
- Underlay `effigy qa:northstar`
- Underlay `effigy validate`
- `git diff --check`

## Stop Conditions

Stop if a consumer repair is unmerged, a database proof lacks a positively
identified disposable target, or the operator selects a new shared DB lifecycle
that has not been separately designed and bounded.

## Consumer Upgrade Impact

- Impact class: breaking task-interface fleet rollout with compatible proof
  hardening
- Affected consumers: six-consumer family
- Required action: use root state plan/apply, routed API `migration:*` tasks,
  and the repaired root merge gates
- Compatibility window: none for retired `db:*` aliases or fail-open local
  overlays

## Next Task

Execute `g09.045`, the read-only bootstrap, runtime assembly, and access-model
assessment for contracts `024`–`026`.
