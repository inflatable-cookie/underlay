# g06.163 - Pagination Source God-File Split

## Why

`g06.162` cleared the storage source god-file warning. The only remaining source
warning is `ts/src/patterns/pagination.svelte.ts`, which mixes server
cursor-pagination and client local-pagination controllers.

## Goal

Split pagination internals into smaller modules while preserving the public
pagination API.

## Scope

In scope:

- split server cursor pagination and client local pagination responsibilities
- preserve exported controller functions and types
- run pagination focused tests
- run `effigy doctor`

Out of scope:

- changing pagination behavior
- changing public APIs
- changing auth refresh semantics
- splitting test-only god-file warnings
- consumer-app changes

## Acceptance Criteria

- [x] `ts/src/patterns/pagination.svelte.ts` no longer reports as a source god-file,
  or retained size is justified by artifact evidence
- [x] public imports remain stable
- [x] focused pagination tests pass
- [x] `effigy doctor` state is recorded

## Consumer Upgrade Impact

Expected impact: none.

This should be an internal split with stable exports.

## Evidence

- `bun x vitest run ts/tests/patterns/pagination.test.ts` passed: 1 file,
  3 tests.
- `effigy doctor` passed with `ok:15`, `warn:1`, `err:0`.
- `scan.god-files` now reports 9 warnings.
- `ts/src/patterns/pagination.svelte.ts` no longer reports.
- Remaining god-file warnings are test files only.

## Current State

`g06.163` is complete.

## Next Task

Execute `g06.164`: TypeScript test god-file closeout decision.
