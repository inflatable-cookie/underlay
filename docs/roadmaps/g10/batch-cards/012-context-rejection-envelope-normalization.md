# g10.012 - Context Rejection Envelope Normalization

Status: ready
Owner: repo maintainers
Contracts: `010-foundation-primitives-and-envelopes.md`, `020-http-transport-and-server-boundary.md`
Found by: `g10.011`

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

## Next Task

Normalize the two context rejection paths and prove their wire bodies.
