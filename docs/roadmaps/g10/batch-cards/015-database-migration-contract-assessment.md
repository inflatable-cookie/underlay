# g10.015 - Database Migration Contract Assessment

Status: ready
Owner: repo maintainers
Contract: `021-database-migration-and-schema-workflow.md`
Depends on: `g10.014`

## Purpose

Assess the shared migration boundary and all six consumer workspaces against
contract `021` before opening migration repairs.

## Scope

- inspect Underlay's DB, migration, devtools, state-layout, and task-routing
  authority named by `021`
- inspect each consumer root and every affected API child under the current
  `apps/*` workspace shape
- compare structural migrations, dev overlays, naming, forward-only posture,
  root state routing, package-owned `migration:*` tasks, replay proof, and
  dynamic SQL identifier use with the contract
- classify each consumer as baseline, advanced, drifted, or materially
  ambiguous for every applicable clause
- record documentation drift and compile only bounded repairs supported by the
  evidence
- do not change schema, apply state, reset databases, or alter production code

Consumer roots:

- `underlay-reference`
- `contact-patch`
- `compli-me`
- `acowtancy`
- `songsprout`
- `loophole/composer`

## Acceptance

- one timestamped evidence matrix covers every `021` rule and all six roots
- the matrix names root orchestration and affected API-package evidence
  separately
- every finding has one disposition: contract match, documentation repair,
  bounded implementation card, consumer rollout card, or operator decision
- baseline and advanced migration profiles stay distinct
- retired top-level package paths are absent from active assessment authority
- no production or consumer repository state changes during assessment

## Evidence Requirements

For each contract clause record:

- Underlay contract, guide, crate, or task evidence
- root-workspace routing evidence
- affected API-package migration and task evidence
- verdict and confidence
- repair owner and validation boundary when drift is confirmed

Use read-only task discovery. If replay proof would require credentials,
containers, database reset, or state apply, record the proof gap instead of
performing the mutation.

## Validation

- `effigy health`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Stop Conditions

Stop and return to planning if the fleet exposes a product-policy split between
the baseline and advanced migration profiles, or if contract compliance cannot
be assessed without mutating a consumer database. Do not settle either question
through incidental code or config changes.

## Consumer Upgrade Impact

- Impact class: assessment only
- Affected consumers: six-consumer family
- Required action: none until a finding is promoted into a repair card

## Next Task

Execute this assessment. If it closes without a migration-policy gap, promote
`g10.016` to `ready`; otherwise return to planning with the unresolved boundary.
