# g06.157 - TypeScript Attention Marker Cleanup

## Why

`g06.156` classified the remaining attention-marker findings as scanner noise:
plain explanatory `Note:` wording, not deferred work.

Cleaning these removes one warning family from `effigy doctor` without changing
behavior.

## Goal

Remove the four TypeScript attention-marker warnings by rewording comments only.

## Scope

In scope:

- `ts/src/patterns/blob-upload.ts`
- `ts/src/patterns/media-types/requests.ts`
- `ts/src/patterns/storage.ts`
- `ts/tests/patterns/slugify.test.ts`
- targeted tests where touched behavior has nearby coverage
- `effigy doctor`

Out of scope:

- changing upload, storage, media, or slugify behavior
- broad comment-ratio cleanup
- source god-file splitting
- consumer-app changes

## Acceptance Criteria

- [x] `scan.attention-markers` has no TypeScript findings
- [x] touched tests pass
- [x] `effigy doctor` still exits successfully
- [x] artifact records remaining doctor warning state

## Consumer Upgrade Impact

Expected impact: none.

This should be comment-only cleanup.

## Current State

`g06.157` is complete.

## Next Task

Execute `g06.158`: TypeScript comment-ratio cleanup.
