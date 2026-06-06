# g06.144 Artifact - HTTP Client Tests Modularity Audit

## Summary

`ts/tests/client/http-refactored.test.ts` is the largest high-severity
TypeScript god-file. It is test-only and protects the public
`@decodelabs/underlay/client/http` surface.

The current file groups:

- basic request behavior
- auth token header behavior
- refresh-on-401 behavior
- refresh failure behavior
- refresh concurrency behavior
- retry behavior for idempotent requests
- timeout behavior
- HTTP error handling
- network error handling
- debug logging
- response metadata handling
- `MemoryTokenStore` behavior

## Boundary Evidence

The test file imports the public HTTP client surface:

- `createHttpClient`
- `MemoryTokenStore`
- `HttpClientOptions`

It also imports `UnderlayHttpError` from the public client error module and uses
shared test helpers from `ts/tests/utils/http-mocks.ts`.

Behavior boundaries to preserve:

- URL and method construction
- default and explicit headers
- request body serialization
- 204 response handling
- auth header injection
- refresh-on-401 retry behavior
- token clearing when refresh fails
- refresh handler context
- shared in-flight refresh
- idempotent retry rules
- timeout rules
- error envelope mapping
- network error mapping
- response metadata parsing
- accepted status handling
- memory token store state

## Behavior Evidence

Existing focused validation:

- `bun x vitest run ts/tests/client/http-refactored.test.ts`
- 1 test file passed
- 38 tests passed
- 1 test skipped

## Decision

Queue `g06.145` as an HTTP client tests internal split.

Suggested file shape:

- `ts/tests/client/http/requests.test.ts`: basic request, headers, body, and
  helper method behavior
- `ts/tests/client/http/auth.test.ts`: auth headers, refresh behavior, token
  clearing, refresh context, and refresh concurrency
- `ts/tests/client/http/retry-timeout.test.ts`: retry and timeout behavior
- `ts/tests/client/http/errors-metadata.test.ts`: HTTP errors, network errors,
  debug logging, metadata, accepted statuses, and text responses
- `ts/tests/client/http/token-store.test.ts`: `MemoryTokenStore`
- retire `ts/tests/client/http-refactored.test.ts`

This keeps behavior coverage stable while removing the oversized test file.

## Public API Impact

Expected impact: none.

If preserving the split requires changing `client/http`, `client/errors`, or
HTTP client behavior, stop and re-enter planning.

## Validation

Next code batch validation:

- `bun x vitest run ts/tests/client/http`
- `bun x vitest run ts/tests/client/http-refactored.test.ts` should no longer
  be needed after retirement
- `effigy qa:docs`
- `effigy qa:northstar`
