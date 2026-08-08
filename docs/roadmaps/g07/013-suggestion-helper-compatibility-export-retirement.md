# g07.013 - Suggestion Helper Compatibility Export Retirement

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.012` deferred retirement of suggestion request helper re-exports from
`patterns/selection-history.ts` and the inherited `runtime/data` surface until a
bounded compatibility card existed.

The six-consumer proof from `g07.010` and the follow-up scan for this card show
that known consumers already use `@inflatable-cookie/underlay/client/suggestions` or
app-local helpers for suggestion request params.

## Goals

- [x] remove compatibility-only suggestion helper re-exports from
  `patterns/selection-history.ts`
- [x] stop `runtime/data` from inheriting those request helper exports
- [x] keep `createSelectionHistory()` public through `runtime/data`
- [x] update tests to prove canonical `client/suggestions` ownership
- [x] update roadmap and upgrade guidance

## Non-Goals

- changing suggestion request vocabulary
- moving selection history out of `runtime/data`
- changing relation selector behavior
- opening a new generation

## Execution Plan

- [x] scan the six consumer roots for deprecated Underlay import paths
- [x] remove the re-export from `ts/src/patterns/selection-history.ts`
- [x] update package compatibility tests
- [x] update closeout and upgrade docs
- [x] run focused TS tests and repo validation

## Acceptance Criteria

- [x] `appendSuggestionParams`, `buildSuggestionParams`, `formatHintsParam`,
  `parseHintsParam`, and `SuggestionRequestOptions` are only taught through
  `@inflatable-cookie/underlay/client/suggestions`
- [x] runtime data still exposes `createSelectionHistory()`
- [x] known consumers do not require the retired re-export paths
- [x] validation passes

## Consumer Upgrade Impact

Impact class: `breaking` for unknown consumers importing suggestion request
helpers from `@inflatable-cookie/underlay/runtime/data` or internal
`patterns/selection-history` paths.

Named six-consumer impact: none pending. The known consumer family already uses
`client/suggestions` or app-local helpers.

Required action for unknown consumers:

1. Replace suggestion request helper imports from `runtime/data` or
   `patterns/selection-history` with
   `@inflatable-cookie/underlay/client/suggestions`.
2. Keep `createSelectionHistory()` imports on
   `@inflatable-cookie/underlay/runtime/data`.

## Validation

- `bun x vitest run ts/tests/client/suggestions.test.ts ts/tests/patterns/selection-history.test.ts ts/tests/package-compatibility.test.ts`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy validate`

## Next Task

No active `g07` task remains. Open a bounded roadmap card before starting
another compatibility-retirement or TS boundary lane.
