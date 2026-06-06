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

- `ts/src/patterns/pagination.svelte.ts` no longer reports as a source god-file,
  or retained size is justified by artifact evidence
- public imports remain stable
- focused pagination tests pass
- `effigy doctor` state is recorded

## Consumer Upgrade Impact

Expected impact: none.

This should be an internal split with stable exports.

## Current State

`g06.163` is ready.

## Next Task

Execute `g06.163`: pagination source god-file split.
