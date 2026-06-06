# g06.158 - TypeScript Comment-Ratio Cleanup

## Why

`g06.157` removed the attention-marker warning family. `effigy doctor` now
passes with two warning families left. The next cleanup target is
comment-heavy TypeScript source files.

The goal is not to strip useful API documentation blindly. The goal is to move
source files toward implementation-first readability and leave examples to
tests or guides.

## Goal

Reduce TypeScript comment-ratio warnings by trimming redundant in-source
examples and repeated explanatory comments without changing runtime behavior.

## Scope

In scope:

- inspect the six current `scan.comment-ratio` findings
- trim comments where they duplicate names, types, tests, or guides
- preserve public exports and behavior
- run focused tests for touched behavior
- run `effigy doctor`

Out of scope:

- source god-file splitting
- changing public APIs
- moving documentation into new guide files unless needed for clarity
- consumer-app changes

## Acceptance Criteria

- [x] comment-ratio warnings are reduced or each retained warning is justified
- [x] touched tests pass
- [x] `effigy doctor` exits successfully
- [x] artifact records remaining doctor warning state

## Consumer Upgrade Impact

Expected impact: none.

This should be comment-only cleanup.

## Evidence

- `bun x vitest run ts/tests/utils/sequence.test.ts ts/tests/patterns/navigation-back-info.test.ts ts/tests/patterns/navigation.test.ts ts/tests/client/navigation.test.ts ts/tests/patterns/local-search.test.ts ts/tests/server/csp.test.ts ts/tests/patterns/relation-drilldown-context.test.ts`
  passed: 7 files, 109 tests.
- `effigy doctor` passed with `ok:15`, `warn:1`, `err:0`.
- `scan.comment-ratio` no longer reports.

## Implementation Note

The nearby navigation tests exposed an existing contract mismatch:
`navigation-back-info.ts` returned raw context labels even though tests and
guides expect contextual labels to render as `Back to <label>`. This batch fixed
that helper while avoiding double-prefixing labels that already begin with
`Back`.

## Current State

`g06.158` is complete.

## Next Task

Execute `g06.159`: TypeScript source god-file audit.
