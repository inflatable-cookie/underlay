# g06.162 Artifact - Storage Source God-File Split

## Summary

Split storage internals into focused modules while preserving the public storage
API and persistence format.

## Changes

- Added `storage-types.ts` for public storage types and envelope types.
- Added `storage-availability.ts` for SSR/browser storage probing.
- Added `storage-envelope.ts` for expiration and envelope parse/serialize
  helpers.
- Added `storage-wrapper.ts` for storage wrapper and reactive store behavior.
- Reduced `storage.ts` to the stable public facade:
  - `storage`
  - `createPersistedStore`
  - `createSessionStore`
  - public type re-exports

## Validation

- `bun x vitest run ts/tests/patterns/storage.test.ts`
  - 1 file passed
  - 11 tests passed
- `effigy doctor`
  - `ok:15`
  - `warn:1`
  - `err:0`

## Doctor State

`scan.god-files` now reports 10 warnings.

Cleared source finding:

- `ts/src/patterns/storage.ts`

Remaining source finding:

- `ts/src/patterns/pagination.svelte.ts`

## Consumer Impact

None expected. This was an internal split with stable public exports and the
same persisted envelope format.
