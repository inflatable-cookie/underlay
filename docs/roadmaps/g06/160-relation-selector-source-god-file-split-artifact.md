# g06.160 Artifact - RelationSelector Source God-File Split

## Summary

Split the RelationSelector context source pair into smaller internal modules
while preserving public imports and runtime behavior.

## Changes

- Added focused context helpers:
  - `context-key.ts`
  - `context-use.ts`
  - `context-types.ts`
  - `context-filters.ts`
  - `context-selection.ts`
  - `context-state.ts`
  - `context-ui-actions.ts`
  - `context-value.ts`
- Added focused drill-down helpers:
  - `drilldown-actions.ts`
  - `drilldown-state.ts`
- Kept `context.svelte.ts` and `drilldown-context.svelte.ts` as the Svelte
  rune orchestration modules.

## Validation

- `bun x vitest run ts/tests/patterns/relation-selector-context.test.ts ts/tests/patterns/relation-drilldown-context.test.ts`
  - 2 files passed
  - 6 tests passed
- `effigy doctor`
  - `ok:15`
  - `warn:1`
  - `err:0`

## Doctor State

`scan.god-files` now reports 12 warnings.

Cleared source findings:

- `ts/src/patterns/RelationSelector/context.svelte.ts`
- `ts/src/patterns/RelationSelector/drilldown-context.svelte.ts`

Remaining source findings:

- `ts/src/client/http.ts`
- `ts/src/patterns/pagination.svelte.ts`
- `ts/src/patterns/storage.ts`

## Consumer Impact

None expected. This was an internal module split with stable public exports.
