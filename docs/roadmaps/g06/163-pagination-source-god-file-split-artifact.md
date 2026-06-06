# g06.163 Artifact - Pagination Source God-File Split

## Summary

Split pagination internals into server and client controller modules while
preserving the public pagination API.

## Changes

- Added `pagination-server.svelte.ts` for server cursor pagination.
- Added `pagination-client.svelte.ts` for client local pagination.
- Reduced `pagination.svelte.ts` to the stable export facade.

## Validation

- `bun x vitest run ts/tests/patterns/pagination.test.ts`
  - 1 file passed
  - 3 tests passed
- `effigy doctor`
  - `ok:15`
  - `warn:1`
  - `err:0`

## Doctor State

`scan.god-files` now reports 9 warnings.

Cleared source finding:

- `ts/src/patterns/pagination.svelte.ts`

Remaining findings are test files only:

- `ts/tests/nightfire/utils.test.ts`
- `ts/tests/client/sveltekit.test.ts`
- `ts/tests/patterns/forms.test.ts`
- `ts/tests/patterns/i18n.test.ts`
- `ts/tests/nightfire/summary-transform.test.ts`
- `ts/tests/server/csp.test.ts`
- `ts/tests/patterns/slugify.test.ts`
- `ts/tests/client/http/auth.test.ts`
- `ts/tests/client/useAuth.test.ts`

## Consumer Impact

None expected. This was an internal split with stable public exports.
