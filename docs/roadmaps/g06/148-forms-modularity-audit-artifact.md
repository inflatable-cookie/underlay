# g06.148 Artifact - Forms Modularity Audit

## Summary

`ts/src/patterns/forms.ts` is the top remaining high-severity TypeScript source
god-file after `g06.147`.

The file currently groups:

- public form state types
- public field-error helper types
- public auto-save options
- internal form draft capture and restore helpers
- storage resolution for draft persistence
- Svelte store-backed form state
- SvelteKit-style enhance submit handling
- action-result handling via `forms-action-result`
- public helper functions for field errors and error messages

## Public Export Evidence

Public paths:

- `ts/src/patterns/forms.ts`
- `ts/src/runtime/forms.ts`

Related public action-result path:

- `ts/src/patterns/forms-action-result.ts`
- `ts/src/runtime/forms.ts`

Public names to preserve from `forms.ts`:

- `FieldErrors`
- `FormStateOptions`
- `FormAutoSaveOptions`
- `FormState`
- `createFormState`
- `hasFieldErrors`
- `extractErrorMessage`
- `mergeFieldErrors`

Related public names to preserve from `forms-action-result.ts`:

- `ActionResult`
- `ActionFailureResult`
- `resolveActionFailureResult`

## In-Repo Consumers

Runtime barrel:

- `ts/src/runtime/forms.ts`

Tests:

- `ts/tests/patterns/forms.test.ts`
- `ts/tests/patterns/forms-action-result.test.ts`

Guides and historical records reference `createFormState`, but there are no
active in-repo Svelte components importing `createFormState` directly.

## Behavior Boundaries

Behavior to preserve:

- initial global and field errors seed state
- `startSubmit()` clears errors, sets submitting, clears success, and calls
  `onSubmit`
- `setSuccess()` clears errors, clears submitting, sets success, calls
  `onSuccess`, clears drafts by default, and honors `resetOnSuccess`
- `setError()` clears submitting, sets global and field errors, clears success,
  and calls `onError`
- `setFieldErrors()` preserves the current global error while replacing field
  errors
- `clearFieldError()` removes one field key
- `reset()` restores initial state
- `enhance()` restores drafts on attach when enabled
- draft writes debounce and skip file inputs
- draft restore supports text, single checkbox, checkbox groups, radio groups,
  and multi-select controls
- draft restore dispatches input/change events for restored controls
- submit enhancement posts `FormData` to the form action with the form method or
  `POST`
- JSON success, failure, redirect, error, unknown, non-JSON, and network-error
  paths keep current behavior
- action failure normalization keeps string-only field errors
- helper functions preserve current fallback and merge semantics

## Split Plan

Suggested module shape:

- `forms.ts`: public barrel
- `forms/types.ts`: `FieldErrors`, public options, public state types, internal
  state type if needed by the state implementation
- `forms/draft.ts`: draft capture, restore, storage resolution, and draft
  control helpers
- `forms/state.ts`: `createFormState(...)` and enhance submit handling
- `forms/helpers.ts`: `hasFieldErrors(...)`, `extractErrorMessage(...)`, and
  `mergeFieldErrors(...)`

Cycle note:

- change `forms-action-result.ts` to import `FieldErrors` from
  `./forms/types`, not from `./forms`
- keep `forms.ts` re-exporting the same public names
- keep `runtime/forms.ts` unchanged

## Validation Evidence

Passed:

- `bun x vitest run ts/tests/patterns/forms.test.ts ts/tests/patterns/forms-action-result.test.ts`
  - 23 tests passed

Doctor:

- `effigy doctor` still fails on standing structural scans
- god-file findings are `17` total, `3` high
- top high source god-file is `ts/src/patterns/forms.ts`

## Public API Impact

Expected impact: none.

If the split requires changing exported names, state semantics, draft behavior,
or runtime forms exports, stop and re-enter planning.

## Decision

Queue `g06.149` as a mechanical forms internal split.
