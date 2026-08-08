# 2026-07-17 - g08 Batch 4 (Lane B): correctness bugs and test gate

## Context

Lane B ran after Lane A Batches 1-2. Four cards: the confirmed UX bugs from
the July audit plus the red unit suite and missing test gate.

## Changes

### g08.011 - Form-feedback clobber

- `EntityFormPage` prop-sync `$effect` now tracks only props and assigns
  unconditionally (the local-state reads made it re-run on submit writes and
  clobber feedback). Double-submit guards added to `EntityFormPage` and
  `SpaFormShell`. `EntityFormPage` redirect routed through
  `resolveRedirectTo` (parity with `g08.003`).
- Component tests: feedback survives effects; in-flight submit ignored.

### g08.012 - Google login dead handler

- `LoginGoogleTab` `onclick` -> `onClick`.
- `check:poodle-prop-names` had a double coverage gap: it only matched
  `@inflatable-cookie/poodle-svelte-primitives|composites` imports (plain `@inflatable-cookie/poodle-svelte`
  invisible) and had no event-handler casing rule. Both fixed, plus an
  import-regex overreach that spanned statements.
- Extended check surfaced three more dead handlers, all fixed:
  `EntityDetailPage` Retry button, guide `097` Button examples, guide `186`
  `EditableList` `onsubmit`/`oncancel`.
- Component test: Google button click invokes handler.

### g08.013 - Media validation bypass and upload cancellation

- `media-upload-flow.setFile` checks the `validateFile()` result object (it
  never throws); oversized/disallowed files now set `fileError`.
- `AbortSignal` threaded end-to-end: blob transport <- media-workflow
  pipeline <- flow controller `cancelUpload()` <- `MediaUploadWorkflowPage` /
  `MediaReplaceFileForm` Cancel buttons (now enabled mid-upload; abort
  returns to select, not error).
- Max-size default centralized on `DEFAULT_MEDIA_UPLOAD_MAX_FILE_SIZE`
  (50 MB, matches Rust); 25 MB stragglers removed. Template hints updated
  (SVG out, AVIF in - aligns with `g08.004` server allowlist).
- Tests: oversize rejection, mid-flight cancel.

### g08.014 - Red unit suite and test gate

- 4 navigation tests updated to the intended trimmed-label behavior.
- `effigy.toml`: new `test` task (`bun x vitest run`) wired into `validate`
  before component tests.

## Current State

- `bun x vitest run`: 119 files / 735 tests green.
- `effigy validate`: green end-to-end (health incl. extended prop-name
  guardrail, svelte-check 0 errors, tsc, unit suite, component suite).
- Lane A Batches 1-2 and Lane B complete; g08 acceptance criteria 1, 2, 4
  ticked.

## Consumer Upgrade Notes

Impact class `none` for all four cards (bug fixes; legacy media controller
retained). The `signal` fields on upload handler inputs are additive.

## Next

`g08` Batch 3 (Lane A): `g08.008` distributed rate-limit backend, `g08.009`
http-client SSRF/timeout defaults, `g08.010` auth hardening batch.
