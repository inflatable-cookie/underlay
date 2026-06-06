# g06.145 Artifact - HTTP Client Tests Internal Split

## Summary

HTTP client tests now live in behavior-focused files under
`ts/tests/client/http/` instead of one large `http-refactored.test.ts` file.

Changed files:

- `ts/tests/client/http-refactored.test.ts` removed
- `ts/tests/client/http/requests.test.ts`
- `ts/tests/client/http/auth.test.ts`
- `ts/tests/client/http/retry-timeout.test.ts`
- `ts/tests/client/http/errors-metadata.test.ts`
- `ts/tests/client/http/token-store.test.ts`

## Test Shape

- `requests.test.ts`: global fetch, URL/method construction, bodies, 204
  responses, default headers, explicit Accept headers, and PUT/PATCH helpers
- `auth.test.ts`: auth headers, refresh-on-401, refresh failure, token
  clearing, refresh context setters, refresh errors, and shared in-flight
  refresh
- `retry-timeout.test.ts`: idempotent retries, retry status configuration,
  max retry limit, GET timeout behavior, and the retained skipped POST timeout
  case
- `errors-metadata.test.ts`: HTTP error envelopes, network errors, fallback
  status messages, debug logging, response metadata, accepted statuses, text
  responses, and empty header objects
- `token-store.test.ts`: `MemoryTokenStore` access token, refresh token, and
  clear behavior

## Behavior Preserved

The split keeps existing coverage:

- 38 passing tests
- 1 skipped test
- no changes to `client/http`
- no changes to `client/errors`
- no changes to `ts/tests/utils/http-mocks.ts`

## Validation

Passed:

- `bun x vitest run ts/tests/client/http`
  - 5 test files passed
  - 38 tests passed
  - 1 test skipped

Doctor status:

- `scan.god-files`: 18 findings to 17 findings
- high-severity god-file errors: 5 to 4
- `ts/tests/client/http-refactored.test.ts` no longer appears in the report

## Public API Impact

None.

This was a test-only split. No HTTP client API, HTTP behavior, error behavior,
token behavior, or consumer import path changed.
