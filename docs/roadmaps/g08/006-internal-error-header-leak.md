# g08.006 - Internal Error-Header Leak

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Stop shipping internal error detail to clients. `ApiError::into_response` writes
the error message into `x-error-message` and full context JSON into
`x-error-context`, and `with_cause` inserts the raw `cause.to_string()` (DB/IO
Display strings, potentially SQL fragments and paths). The error-logging
middleware reads but never strips them, so every 4xx/5xx ships internals to the
browser. Architecturally this also makes internal error text part of the wire
contract for all consumers.

## Evidence

- `rust/crates/underlay-http/src/errors.rs:77-91,94-112`
- `rust/crates/underlay-http/src/error_logging/middleware.rs:53-76`

## Governing References

- [020 HTTP transport and server boundary](../../contracts/020-http-transport-and-server-boundary.md)
- [033 Error codes and operator audit](../../contracts/033-error-codes-and-operator-audit.md)

## Planned Changes

- [x] Carry message/context in request extensions for the logging middleware
  instead of response headers.
- [x] Strip `x-error-message` / `x-error-context` before returning, or gate them
  behind an explicit debug/local config.
- [x] Confirm the sanitized error envelope remains the only client-facing error
  surface.

## Consumer Upgrade Impact

Impact class: `behavioral`. Consumers or tests reading these headers must move
to logs or the error envelope. Requires six-consumer proof per `023`.

## Validation

- [x] test: 4xx and 500 responses carry no internal-detail headers
- [x] `cargo test -p underlay-http`
- [x] `effigy validate`

## Stop Conditions

None expected.

## Completion Notes

Completed 2026-07-17. `ApiError::into_response` no longer writes
`x-error-message`/`x-error-context` headers; detail travels in response
extensions as `ErrorDetail` (never serialized to the wire). The logging
middleware reads extensions and additionally strips both legacy headers from
every response as defense-in-depth. `x-error-code` (stable code only)
remains. Tests assert 4xx/5xx carry no internal-detail headers. Validated:
`cargo test -p underlay-http` green.

## Next Task

`g08.007` CORS mirror-origin gating.
