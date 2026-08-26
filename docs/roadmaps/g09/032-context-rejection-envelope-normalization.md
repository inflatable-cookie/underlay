# g09.032 - Context Rejection Envelope Normalization

Status: complete
Completed: 2026-08-26
Owner: repo maintainers
Contracts: `010-foundation-primitives-and-envelopes.md`, `020-http-transport-and-server-boundary.md`
Found by: `g09.031`

## Purpose

Make shared request-context extractor failures obey the canonical transport
error envelope.

## Scope

- serialize `ContextError` through `underlay_core::ErrorEnvelope`
- use stable codes for unauthenticated and missing-context failures
- replace `RequestContext`'s tuple rejection seam with the shared rejection type
- add response-body assertions for status, code, message, content type, and
  envelope field names

## Acceptance

- `AuthenticatedContext` unauthenticated rejection is `401` JSON with
  `error.code = "auth.unauthorized"`
- missing-context rejection is `400` JSON with a stable request-context code
- neither context extractor emits a plain-text error body
- focused `underlay-http` tests cover both rejection variants
- no auth middleware or session semantics move into `underlay-http`

## Validation

- `effigy rust:test`
- `effigy health`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Stop Conditions

Stop if the repair requires choosing auth/session policy beyond the existing
extractor boundary.

## Consumer Upgrade Impact

- Impact class: compatible wire normalization
- Affected consumers: routes using `AuthenticatedContext` and clients that
  inspect non-2xx bodies
- Required action: none for canonical `ErrorEnvelope` clients; callers relying
  on plain-text bodies must use the stable error code instead

## Completion Evidence

`ContextError` now delegates to the canonical `error_response` serializer.
Unauthenticated failures return `401` with `auth.unauthorized`; missing-context
failures return `400` with `request.context_missing`. `RequestContext` now uses
`ContextError` as its rejection type instead of the tuple seam.

Focused `underlay-http` tests assert the status, JSON content type,
`x-error-code`, and exact `ErrorEnvelope` body for both variants. The full Rust
workspace test task passed. No auth middleware or session behavior changed.

## Next Task

Execute `g09.033`, the page-list contract artifact sync.
