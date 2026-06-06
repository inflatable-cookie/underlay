# g06.158 Artifact - TypeScript Comment-Ratio Cleanup

## Summary

Cleared the TypeScript comment-ratio warning family by trimming redundant
source comments and in-file examples.

## Changes

- Trimmed comment-heavy source docs in:
  - `ts/src/utils/sequence.ts`
  - `ts/src/patterns/navigation.ts`
  - `ts/src/patterns/RelationSelector/drilldown-types.ts`
  - `ts/src/server/csp.ts`
  - `ts/src/patterns/local-search.ts`
  - `ts/src/client/navigation.ts`
- Fixed the navigation back-info helper to match the documented/tested
  contextual label contract: `Back to <label>`, without double-prefixing labels
  that already start with `Back`.

## Validation

- `bun x vitest run ts/tests/utils/sequence.test.ts ts/tests/patterns/navigation-back-info.test.ts ts/tests/patterns/navigation.test.ts ts/tests/client/navigation.test.ts ts/tests/patterns/local-search.test.ts ts/tests/server/csp.test.ts ts/tests/patterns/relation-drilldown-context.test.ts`
  - 7 files passed
  - 109 tests passed
- `effigy doctor`
  - `ok:15`
  - `warn:1`
  - `err:0`

## Doctor State

`scan.comment-ratio` no longer reports.

Remaining warning family:

- `scan.god-files`: 14 warnings, 0 errors

## Consumer Impact

Low but non-zero.

Most changes were comment-only. The navigation helper now returns the contextual
label format that Underlay tests and docs already specified.
