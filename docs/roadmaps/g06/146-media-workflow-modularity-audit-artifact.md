# g06.146 Artifact - Media Workflow Modularity Audit

## Summary

`ts/src/patterns/media-workflow.ts` is the next high-severity TypeScript
source god-file after `g06.145`. It is public shared code exported through
`ts/src/runtime/media.ts`.

The current file groups:

- media browse pagination types and helpers
- media upload workflow step/result types
- duplicate-check result types
- upload plan/init result types
- create/replace upload option types
- generic `uploadMediaWithKnownHash(...)`
- duplicate-aware `runMediaUploadWorkflow(...)`
- `createMediaAndUpload(...)`
- `replaceMediaUpload(...)`
- `checkMediaDuplicateFile(...)`
- `createMediaUploadPipeline(...)`
- internal `uploadMediaVersion(...)`
- internal `normalizeUploadVisibility(...)`
- internal `toUploadPlan(...)`

## Public Export Evidence

Public path:

- `@inflatable-cookie/underlay/runtime/media` re-exports
  `../patterns/media-workflow`

In-repo consumers:

- `ts/src/templates/MediaPickerWorkflow.svelte`
- `ts/src/templates/MediaUploadStatusPanel.svelte`
- runtime media barrel

Public names to preserve include:

- `DEFAULT_MEDIA_UPLOAD_MAX_FILE_SIZE`
- all exported media workflow types
- `loadMediaBrowsePage`
- `mergeMediaBrowseItems`
- `createResetMediaBrowseState`
- `uploadMediaWithKnownHash`
- `runMediaUploadWorkflow`
- `createMediaAndUpload`
- `replaceMediaUpload`
- `checkMediaDuplicateFile`
- `createMediaUploadPipeline`

## Behavior Boundaries

Behavior to preserve:

- browse page defaults to limit `12`
- missing `nextCursor` becomes `null`
- missing `hasMore` becomes `false`
- browse merge appends only when a cursor is present
- reset state returns empty items and false/null pagination state
- upload workflow computes file hash before duplicate check
- duplicate result short-circuits upload when `exists` and `item` are present
- upload path creates/initiates/uploads/finalises in order
- progress from blob upload is forwarded
- `createMediaAndUpload(...)` normalizes visibility
- replace upload keeps the provided media ID
- duplicate-file check returns hash plus duplicate result
- pipeline helpers preserve context passthrough
- upload plan headers, max bytes, content types, and object key defaults remain
  stable

## Validation Evidence

Related tests:

- `bun x vitest run ts/tests/patterns/media-upload-flow.test.ts`
  - 3 tests passed

Coverage gap:

- there is no direct focused test target for `media-workflow.ts`
- the next split should either preserve behavior with existing coverage only or
  add narrow tests around exported pure helpers and upload sequencing

Attempted but not useful:

- `bun x tsc --noEmit --pretty false` prints TypeScript compiler help because
  the repo root does not expose a direct `tsc` project invocation

## Decision

Queue `g06.147` as a media workflow internal split.

Suggested module shape:

- `media-workflow.ts`: public front door and re-export surface
- `media-workflow/types.ts`: exported workflow, upload, pagination, and
  pipeline types
- `media-workflow/browse.ts`: browse page loading, merge, and reset helpers
- `media-workflow/upload.ts`: `uploadMediaWithKnownHash(...)` and
  `runMediaUploadWorkflow(...)`
- `media-workflow/pipeline.ts`: create, replace, duplicate check, and pipeline
  factory helpers
- `media-workflow/plan.ts`: upload plan normalization

This keeps public import paths stable while separating browse state, generic
upload workflow, and app-level media pipeline helpers.

## Public API Impact

Expected impact: none.

If preserving the split requires changing exported names, upload sequencing,
duplicate behavior, upload plan defaults, visibility normalization, or runtime
media imports, stop and re-enter planning.

## Validation

Next code batch validation:

- `bun x vitest run ts/tests/patterns/media-upload-flow.test.ts`
- add or run focused `media-workflow` tests if introduced by the split
- `effigy qa:docs`
- `effigy qa:northstar`
