# g06.089 Artifact - Auth WebAuthn Service Modularity Audit

## Summary

`underlay-auth-webauthn/src/service.rs` is the largest remaining Rust
warning-level production file after the AI runtime test split. It is
security-sensitive auth code, and the current file mixes WebAuthn protocol
operations, state and passkey serialization, stored-passkey conversion helpers,
metadata extraction, authentication update helpers, and HTTP adapter wrappers
in one implementation block.

The service surface currently groups:

- `WebAuthnService` construction from `WebAuthnConfig`
- passkey registration start/finish behavior
- passkey authentication start/finish behavior
- discoverable authentication start/identify/finish behavior
- passkey and ceremony-state serialization helpers
- stored-passkey conversion and sync metadata helpers
- credential-id lookup and post-authentication update helpers
- HTTP request/response adapter wrappers

## Behavior Evidence

The focused crate validation covers these stable contracts:

- registration starts with a challenge and returns registration state
- authentication can start without allowed credentials for discoverable flows
- HTTP registration start returns a state id and public-key options
- credential ids round-trip through base64url encoding
- stored-passkey JSON exposes counters, transports, and backup sync flags
- credential metadata derives from stored passkey fields
- invalid stored-passkey JSON and passkey encoding fail gracefully
- invalid HTTP allowed credentials are rejected
- invalid registration finish fails as an auth error
- WebAuthn error mapping remains stable

## Decision

Queue `g06.090` as an auth WebAuthn service internal split.

The split should preserve:

- the `WebAuthnService` public type
- all existing public service methods
- crate-root `WebAuthnService` and `types::*` exports
- current feature-gated state serialization behavior
- current WebAuthn challenge, verification, credential, and sync semantics
- current HTTP request/response adapter behavior
- current tests and error-mapping assertions

Suggested production module shape:

- `service.rs`: service front door and construction
- `service/core.rs`: registration, authentication, and discoverable ceremony
  methods
- `service/encoding.rs`: passkey and ceremony-state serialization
- `service/storage.rs`: stored passkey, credential-id, counter, transport,
  sync-info, metadata, lookup, and update helpers
- `service/http.rs`: HTTP adapter wrappers

## Public API Impact

Expected impact: none.

This should be an internal production-code split. If preserving the existing
method surface forces a semantic change, stop and re-enter planning.

## Validation

- `cargo test -p underlay-auth-webauthn --all-features`

Next code batch validation:

- `cargo test -p underlay-auth-webauthn --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
