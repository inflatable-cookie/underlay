# g06.147 Artifact - Media Workflow Internal Split

## Summary

`ts/src/patterns/media-workflow.ts` is now a public barrel. The implementation
moved under `ts/src/patterns/media-workflow/` by stable responsibility.

Module shape:

- `media-workflow.ts`: public export front door
- `media-workflow/browse.ts`: browse page loading, merge, and reset helpers
- `media-workflow/upload.ts`: generic duplicate-aware upload workflow
- `media-workflow/pipeline.ts`: create, replace, duplicate-check, and pipeline
  factory helpers
- `media-workflow/plan.ts`: upload-plan defaults and normalization
- `media-workflow/types.ts`: public type barrel
- `media-workflow/browse-types.ts`: browse pagination and state types
- `media-workflow/workflow-types.ts`: upload workflow and upload-plan types
- `media-workflow/pipeline-types.ts`: pipeline helper option/result types

## Public API Impact

None expected.

The retained public front doors still export the same media workflow names:

- `ts/src/patterns/media-workflow.ts`
- `ts/src/runtime/media.ts`

No consumer app import changes are required.

## Behavior Preserved

The split preserves the audited behavior boundaries:

- browse page default limit remains `12`
- missing browse pagination fields normalize to `null` and `false`
- browse merge appends only when a cursor exists
- upload workflow hashes before duplicate checks
- duplicate checks still short-circuit when an existing item is present
- upload sequencing remains create/initiate/upload/finalise
- progress forwarding remains intact
- visibility normalization still maps unknown values to `public`
- replace uploads keep the provided media ID
- upload plan defaults still normalize headers, max bytes, content types, and
  object key

## Validation

Passed:

- `bun x vitest run ts/tests/patterns/media-upload-flow.test.ts ts/tests/patterns/media-workflow.test.ts`
- `effigy check:types`

Doctor:

- `effigy doctor` still fails on standing structural scans
- god-file findings are now `17` total, `3` high
- `ts/src/patterns/media-workflow.ts` is no longer a high-severity god-file
- remaining high god-files:
  - `ts/src/patterns/forms.ts`
  - `ts/src/templates/template.types.ts`
  - `ts/tests/patterns/optimistic.test.ts`

## Decision

Queue `g06.148` as a forms modularity audit. `forms.ts` is the top remaining
high-severity source god-file.
