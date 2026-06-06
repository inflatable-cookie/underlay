# g06.167 - Acowtancy List-Query Consumer Drift Repair

## Why

`g06.166` proved the current Underlay surface against the six known consumers.
The remaining concrete failure is in Acowtancy Cattle Grid list-query handling,
not Underlay source structure.

Acowtancy root health flags offset-style list-query params, and the Cattle Grid
cache test expects a page-shaped payload while `listModulesForListAdmin` returns
cursor-shaped output.

## Goal

Repair Acowtancy Cattle Grid's learning module list-query drift and rerun the
consumer compatibility proof.

## Scope

In scope:

- inspect `cattle-grid/src/commands/learning/modules.ts`
- remove or retire offset-style params in the learning-module list path
- align `tests/learning-modules-admin-cache.test.ts` to the intended payload
  contract
- rerun Acowtancy root health or the smallest equivalent consumer proof
- rerun Underlay compatibility checks if Underlay changes are required

Out of scope:

- unrelated Acowtancy feature work
- broad pagination redesign
- new Underlay public APIs unless the repair proves a shared gap

## Acceptance Criteria

- Acowtancy list-query audit no longer flags learning-module offset params
- the Cattle Grid learning-module admin-cache test passes
- the intended page-vs-cursor contract is recorded
- consumer repos are left clean or their changes are committed separately

## Consumer Upgrade Impact

Expected impact: Acowtancy-only.

No Underlay breaking change is expected.

## Current State

`g06.167` is ready.

## Next Task

Execute `g06.167`: Acowtancy list-query consumer drift repair.
