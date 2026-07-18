# g08.013 - Media Validation Bypass And Upload Cancellation

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Two client-side media bugs. `media-upload-flow.setFile` wraps `validateFile()`
in try/catch expecting a throw, but `validateFile` returns `{valid, error}` and
never throws, so oversized/disallowed files are always accepted and `fileError`
is never set. Separately, the transport supports abort but no caller passes an
`AbortSignal` and Cancel buttons are disabled mid-upload, so a large upload
cannot be stopped.

## Evidence

- validation bypass `ts/src/patterns/media-upload-flow.svelte.ts:172-177`,
  contract `ts/src/patterns/blob-upload.ts:279-306`, exported via
  `ts/src/runtime/media.ts:5`
- abort support unused `blob-upload.ts:41,79-83,121-123`; disabled cancel
  `MediaUploadWorkflowPage.svelte:402,505`, `MediaReplaceFileForm.svelte:164`

## Governing References

- [050 Media library and usage](../../contracts/050-media-library-and-usage.md)
- [100 Shared patterns and workflow shells](../../contracts/100-shared-patterns-and-workflow-shells.md)

## Planned Changes

- [x] Check `validateFile()`'s return value (or delete the legacy controller if
  template flows already validate independently).
- [x] Thread an `AbortSignal` from the upload UIs and enable Cancel mid-upload.
- [x] Centralize the max-file-size default (25 MB vs 50 MB disagree across entry
  points).

## Consumer Upgrade Impact

Impact class: `none` (bug fix) unless the legacy controller is removed, in which
case `behavioral` for its importers.

## Validation

- [x] tests: oversized file rejected; in-progress upload cancellable
- [x] `bun x vitest run` (media suite)
- [x] `effigy validate`

## Stop Conditions

Confirm no consumer depends on the legacy controller before deleting it.

## Completion Notes

Completed 2026-07-17. `media-upload-flow.setFile` now checks the
`validateFile()` result object (it never throws) - oversized/disallowed files
are rejected with `fileError` set. Legacy controller retained (exported via
`runtime/media`); consumer-dependency check before deletion deferred.
`AbortSignal` threaded end-to-end: `uploadToBlob` <- media-workflow pipeline
(`createAndUpload`/`replaceUpload`/`uploadMediaVersion`) <- flow controller
(`cancelUpload()`) <- `MediaUploadWorkflowPage` and `MediaReplaceFileForm`
Cancel buttons, which are now enabled mid-upload and abort in flight; abort
returns to select state, not error. Max-size default centralized on
`DEFAULT_MEDIA_UPLOAD_MAX_FILE_SIZE` (50 MB, matches Rust
`BlobUploadConfig`); stale 25 MB default removed; template hints updated
(SVG out, AVIF in, aligning with `g08.004`). Tests: oversize rejection,
mid-flight cancel. Suite green.

## Next Task

`g08.014` red unit suite fix and test gate.
