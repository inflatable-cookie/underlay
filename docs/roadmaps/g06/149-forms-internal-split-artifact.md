# g06.149 Artifact - Forms Internal Split

## Summary

`ts/src/patterns/forms.ts` is now a public barrel. The implementation moved
under `ts/src/patterns/forms/` by stable responsibility.

Module shape:

- `forms.ts`: public export front door
- `forms/types.ts`: public form state, option, draft, and field-error types
- `forms/draft.ts`: draft capture, restore, control discovery, event dispatch,
  and storage resolution
- `forms/state.ts`: Svelte store-backed `createFormState(...)`
- `forms/enhance.ts`: form submit enhancement and action-result routing
- `forms/helpers.ts`: field-error and error-message helper functions

Related dependency change:

- `forms-action-result.ts` now imports `FieldErrors` from `./forms/types`
  instead of the public `./forms` barrel

## Public API Impact

None expected.

The retained public front doors still export the same forms names:

- `ts/src/patterns/forms.ts`
- `ts/src/runtime/forms.ts`

No consumer app import changes are required.

## Behavior Preserved

The split preserves the audited behavior boundaries:

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
- submit enhancement keeps current success, failure, redirect, error, unknown,
  non-JSON, and network-error behavior
- action failure normalization keeps string-only field errors

## Validation

Passed:

- `bun x vitest run ts/tests/patterns/forms.test.ts ts/tests/patterns/forms-action-result.test.ts`
- `effigy check:types`

Doctor:

- `effigy doctor` still fails on standing structural scans
- god-file findings are now `16` total, `2` high
- `ts/src/patterns/forms.ts` is no longer a high-severity god-file
- no replacement forms module appears in the god-file warning list
- remaining high god-files:
  - `ts/src/templates/template.types.ts`
  - `ts/tests/patterns/optimistic.test.ts`

## Decision

Queue `g06.150` as a template types modularity audit. `template.types.ts` is the
top remaining high-severity source god-file.
