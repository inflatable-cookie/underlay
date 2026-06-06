# g07.023 - Client HTTP Retry Timeout Audit

Status: complete
Owner: repo maintainers
Started: 2026-06-06
Completed: 2026-06-06

## Purpose

Resolve the remaining `020` drift note around TS `client/http` retry and
timeout behavior.

## Governing References

- [020 HTTP transport and server boundary](../../contracts/020-http-transport-and-server-boundary.md)
- [080 TypeScript client guide](../../guides/080-typescript-client.md)

## Decision

Keep the implementation.

`createHttpClient()` intentionally retries only:

- idempotent methods: `GET` and `DELETE`
- configured retryable HTTP statuses: defaults `502`, `503`, and `504`, plus
  caller-provided `retryStatuses`

It does not retry generic network failures or timeout aborts. Those are
normalized to `UnderlayHttpError(0)`.

Timeout protection intentionally applies only to idempotent methods.

## Changes

- [x] Clarify the TypeScript client guide so retry and timeout wording names
  the idempotent-method boundary.
- [x] Remove the stale retry-comment drift note from `020`.
- [x] Unskip the POST no-timeout test so timeout scope is covered.

## Consumer Upgrade Impact

Impact class: `none`.

No public API or behavior changed.

## Validation

- [x] `bun x vitest run ts/tests/client/http/retry-timeout.test.ts`
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`
- [x] `effigy validate`

## Next Task

No active `g07` task remains. The validation-rejection transport-normalization
drift named in `010` and `020` was resolved by `g06.181`.
