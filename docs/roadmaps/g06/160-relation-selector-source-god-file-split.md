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

- RelationSelector source files are smaller and responsibility-focused
- public imports remain stable
- focused RelationSelector/drill-down tests pass
- `effigy doctor` state is recorded

## Consumer Upgrade Impact

Expected impact: none.

This should be an internal split with stable exports.

## Current State

`g06.160` is ready.

## Next Task

Execute `g06.160`: RelationSelector source god-file split.
