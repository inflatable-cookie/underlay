# g06.162 - Storage Source God-File Split

## Why

`g06.161` cleared the HTTP client source god-file warning. The next source
warning is `ts/src/patterns/storage.ts`, which mixes public storage types,
availability probing, envelope parsing, wrapper methods, reactive store
behavior, and public factory exports.

## Goal

Split storage internals into smaller modules while preserving the public storage
API.

## Scope

In scope:

- split storage public types from implementation helpers
- split envelope parse/serialize helpers
- split availability probing if useful
- preserve `storage`, `createPersistedStore`, and `createSessionStore`
- run storage focused tests
- run `effigy doctor`

Out of scope:

- changing storage behavior
- changing public APIs
- changing persistence format
- consumer-app changes
- splitting pagination

## Acceptance Criteria

- [x] `ts/src/patterns/storage.ts` no longer reports as a source god-file, or
  retained size is justified by artifact evidence
- [x] public imports remain stable
- [x] focused storage tests pass
- [x] `effigy doctor` state is recorded

## Consumer Upgrade Impact

Expected impact: none.

This should be an internal split with stable exports.

## Evidence

- `bun x vitest run ts/tests/patterns/storage.test.ts` passed: 1 file,
  11 tests.
- `effigy doctor` passed with `ok:15`, `warn:1`, `err:0`.
- `scan.god-files` now reports 10 warnings.
- `ts/src/patterns/storage.ts` no longer reports.

## Current State

`g06.162` is complete.

## Next Task

Execute `g06.163`: pagination source god-file split.
