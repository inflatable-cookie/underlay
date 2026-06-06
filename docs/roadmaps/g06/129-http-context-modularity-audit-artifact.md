# g06.129 Artifact - HTTP Context Modularity Audit

## Summary

`underlay-http/src/context.rs` is the next Rust production warning-level file
after `g06.128`. It owns shared Axum request context extraction and related
tracing helpers.

The current file groups:

- request context public model
- authenticated context public model
- authenticated user extension type
- context extraction error type and response mapping
- public request header constants
- Axum `FromRequestParts` implementation for `RequestContext`
- Axum `FromRequestParts` implementation for `AuthenticatedContext`
- request ID extraction and generation
- client IP extraction from proxy headers
- feature-gated OpenTelemetry context behavior
- feature-gated tracing span helpers
- crate-local context tests

## Boundary Evidence

The public surface is exported from `src/lib.rs`:

- `headers`
- `AuthenticatedContext`
- `AuthenticatedUser`
- `ContextError`
- `RequestContext`

Feature-gated public functions also live in the context module:

- `make_request_span`
- `RequestContext::record_to_span`
- OpenTelemetry helpers on `RequestContext` and `AuthenticatedContext`

The split must preserve `underlay_http::context::{...}` and root
`underlay_http::{...}` imports.

## Behavior Evidence

Existing focused tests cover:

- request ID extraction from `x-request-id`
- request ID generation when the header is absent
- client IP extraction from `cf-connecting-ip`
- client IP extraction from `x-real-ip`
- client IP extraction from first `x-forwarded-for` value
- IP priority order
- missing IP behavior
- request context getters and authentication check
- OpenTelemetry trace extraction and injection when the feature is enabled

Baseline validation:

- `cargo test -p underlay-http context --all-features`
- 15 tests passed
- 1 Docker-backed error-logging test ignored

## Decision

Queue `g06.130` as an HTTP context internal split.

Suggested module shape:

- `context/mod.rs`: public module front door, re-exports, and test module
  declaration
- `context/headers.rs`: public header constants
- `context/model.rs`: `RequestContext`, `AuthenticatedContext`, and
  `AuthenticatedUser`
- `context/error.rs`: `ContextError` and response mapping
- `context/extractors.rs`: Axum extractors for request and authenticated
  context
- `context/parse.rs`: request ID, user agent, and IP extraction helpers
- `context/tracing.rs`: feature-gated span helpers

This keeps public names stable while separating runtime extraction from model
and helper behavior.

## Public API Impact

Expected impact: none.

If preserving the split requires changing exported names, header constants,
status codes, extraction semantics, generated request IDs, IP priority order,
or tracing/OpenTelemetry behavior, stop and re-enter planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-http context --all-features`
- `cargo test -p underlay-http --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
