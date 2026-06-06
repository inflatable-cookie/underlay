# g06.154 Artifact - TypeScript Structural Closeout Audit

## Summary

High-severity god-files are cleared after `g06.153`.

`effigy doctor` still fails because two non-god-file scan families report
errors:

- `scan.attention-markers`
- `scan.comment-ratio`

The remaining god-file findings are warning-only.

## Doctor State

`effigy doctor` result:

- ok: `13`
- warn: `1`
- err: `2`

Error scans:

- `scan.attention-markers`
  - findings: `5`
  - warning findings: `4`
  - error findings: `1`
- `scan.comment-ratio`
  - findings: `7`
  - warning findings: `6`
  - error findings: `1`

Warning scan:

- `scan.god-files`
  - findings: `14`
  - warning findings: `14`
  - error findings: `0`

## Attention Markers

High finding:

- `ts/src/client/navigation.ts`
  - deprecated marker on `navigateOnCancel(...)`

Warning findings:

- `ts/src/patterns/blob-upload.ts`
  - streaming/Web Crypto note
- `ts/src/patterns/media-types/requests.ts`
  - upload plan header mapping note
- `ts/src/patterns/storage.ts`
  - Svelte store cleanup note
- `ts/tests/patterns/slugify.test.ts`
  - underscore filtering note

Decision:

- next cleanup should handle the high deprecated marker first
- warning notes can remain unless they become the next policy target

## Comment Ratio

High finding:

- `ts/src/client/route-protection.ts`
  - ratio `2.06`
  - `97` comment lines / `47` code lines

Warning findings:

- `ts/src/utils/sequence.ts`
- `ts/src/patterns/navigation.ts`
- `ts/src/patterns/RelationSelector/drilldown-types.ts`
- `ts/src/server/csp.ts`
- `ts/src/patterns/local-search.ts`
- `ts/src/client/navigation.ts`

Decision:

- next cleanup should trim redundant comments in `route-protection.ts`
- warning-ratio files should remain a later lane unless the operator wants a
  full comment-ratio cleanup wave

## God Files

No high findings remain.

Warning findings remain in tests and source. Top entries:

- `ts/tests/nightfire/utils.test.ts`
- `ts/tests/client/sveltekit.test.ts`
- `ts/tests/patterns/forms.test.ts`
- `ts/src/patterns/RelationSelector/context.svelte.ts`
- `ts/tests/patterns/i18n.test.ts`

Decision:

- do not continue splitting warning-level god-files before clearing current
  doctor errors
- revisit warning-level god-files after `effigy doctor` no longer fails

## Validation

Ran:

- `effigy doctor`

Result:

- fails on attention-marker and comment-ratio errors
- no high-severity god-files remain

## Next Lane

Queue `g06.155` as TypeScript doctor error cleanup.

Scope:

- remove or reclassify the deprecated marker on `navigateOnCancel(...)`
- trim redundant comments in `route-protection.ts`
- run `effigy doctor`
- preserve navigation and route-protection behavior
