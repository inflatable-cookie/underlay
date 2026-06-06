# g06.160 - RelationSelector Source God-File Split

## Why

`g06.159` classified the RelationSelector context pair as the highest-value
TypeScript source split:

- `ts/src/patterns/RelationSelector/context.svelte.ts`
- `ts/src/patterns/RelationSelector/drilldown-context.svelte.ts`

Both files mix state construction, derived values, async loading, action
handlers, and export wiring.

## Goal

Split RelationSelector source responsibilities into smaller internal modules
while preserving the public context API and runtime behavior.

## Scope

In scope:

- split selected-item resolution and initial-selection sync out of
  `context.svelte.ts`
- split search/suggestion action helpers where doing so reduces context size
- split drill-down derivation/loading/navigation helpers from
  `drilldown-context.svelte.ts`
- preserve existing exports and import paths
- run RelationSelector and drill-down focused tests
- run `effigy doctor`

Out of scope:

- changing RelationSelector public props or behavior
- redesigning RelationSelector UI
- splitting unrelated god-files
- consumer-app changes

## Acceptance Criteria

- [x] RelationSelector source files are smaller and responsibility-focused
- [x] public imports remain stable
- [x] focused RelationSelector/drill-down tests pass
- [x] `effigy doctor` state is recorded

## Consumer Upgrade Impact

Expected impact: none.

This should be an internal split with stable exports.

## Evidence

- `bun x vitest run ts/tests/patterns/relation-selector-context.test.ts ts/tests/patterns/relation-drilldown-context.test.ts`
  passed: 2 files, 6 tests.
- `effigy doctor` passed with `ok:15`, `warn:1`, `err:0`.
- `scan.god-files` now reports 12 warnings, down from 14 before this split.
- `ts/src/patterns/RelationSelector/context.svelte.ts` no longer reports.
- `ts/src/patterns/RelationSelector/drilldown-context.svelte.ts` no longer
  reports.

## Current State

`g06.160` is complete.

## Next Task

Execute `g06.161`: HTTP client source god-file split.
