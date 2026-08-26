# g10.015 - Database Migration Contract Assessment

Status: complete
Completed: 2026-08-26
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

## Completion Evidence

The assessment matrix is recorded in
[`g10.015 - Database Migration Contract Assessment`](../../../logs/2026-08/26-162845-g10-015-database-migration-assessment.md).

Verdict: `drifting`. Shared SQL layout, naming, explicit migration inputs, and
typed identifier boundaries match. Confirmed repair candidates are:

- roll the contracted root state stack and `migration:*` task surface through
  the five baseline consumers, starting with Underlay Reference
- make Acowtancy's local state artifact layer apply the installed bundle and dev
  overlay to the DB
- make Songsprout and Composer dev-overlay reset/application fail closed

No database, state stack, consumer file, or production source changed. The
findings do not expose a migration-policy split, so the testing assessment gate
is clear. Repair cards stay deferred until `g10.016` closes.

## Next Task

`g10.016` is complete. Re-enter planning and compile the combined
migration/testing repair wave from both assessment records.
