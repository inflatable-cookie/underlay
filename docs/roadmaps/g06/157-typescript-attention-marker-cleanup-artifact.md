# g06.157 Artifact - TypeScript Attention Marker Cleanup

## Summary

Removed the remaining TypeScript attention-marker warnings with comment-only
wording changes.

## Changes

- Reworded the Web Crypto hashing comment in `ts/src/patterns/blob-upload.ts`.
- Reworded the media upload plan mapping comment in
  `ts/src/patterns/media-types/requests.ts`.
- Reworded the Svelte store listener lifetime comment in
  `ts/src/patterns/storage.ts`.
- Reworded the underscore behavior comment in
  `ts/tests/patterns/slugify.test.ts`.

## Validation

- `bun x vitest run ts/tests/patterns/blob-upload.test.ts ts/tests/patterns/storage.test.ts ts/tests/patterns/slugify.test.ts`
  - 3 files passed
  - 67 tests passed
- `effigy doctor`
  - `ok:14`
  - `warn:2`
  - `err:0`

## Doctor State

`scan.attention-markers` no longer reports.

Remaining warning families:

- `scan.comment-ratio`: 6 warnings, 0 errors
- `scan.god-files`: 14 warnings, 0 errors

## Consumer Impact

None. This batch changed comments only.
