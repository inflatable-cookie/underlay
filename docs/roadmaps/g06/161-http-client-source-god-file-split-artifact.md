# g06.161 Artifact - HTTP Client Source God-File Split

## Summary

Split HTTP client types, token storage, and header helpers out of
`ts/src/client/http.ts` while preserving the public `client/http` API.

## Changes

- Added `ts/src/client/http-types.ts` for public HTTP/auth types.
- Added `ts/src/client/http-token-store.ts` for `MemoryTokenStore`.
- Added `ts/src/client/http-headers.ts` for case-insensitive header helpers.
- Kept `ts/src/client/http.ts` as the client factory, raw request, retry, and
  auth-refresh flow module.
- Re-exported all public types and `MemoryTokenStore` from `client/http`.

## Validation

- `bun x vitest run ts/tests/client/http/requests.test.ts ts/tests/client/http/auth.test.ts ts/tests/client/http/retry-timeout.test.ts ts/tests/client/http/errors-metadata.test.ts ts/tests/client/http/token-store.test.ts`
  - 5 files passed
  - 38 tests passed
  - 1 skipped
- `effigy doctor`
  - `ok:15`
  - `warn:1`
  - `err:0`

## Doctor State

`scan.god-files` now reports 11 warnings.

Cleared source finding:

- `ts/src/client/http.ts`

Remaining source findings:

- `ts/src/patterns/storage.ts`
- `ts/src/patterns/pagination.svelte.ts`

## Consumer Impact

None expected. This was an internal split with stable public exports.
