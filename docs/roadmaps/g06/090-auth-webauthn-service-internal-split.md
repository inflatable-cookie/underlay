# g06.090 - Auth WebAuthn Service Internal Split

## Why

`g06.089` found that `underlay-auth-webauthn/src/service.rs` mixes protocol
ceremonies, serialization, stored-passkey conversion, metadata extraction,
post-authentication updates, and HTTP adapters in one production auth file.

WebAuthn is security-sensitive. The next split should reduce reasoning load
without changing the public service surface.

## Goal

Split the auth WebAuthn service into focused internal modules while preserving
all public APIs and credential behavior.

## Scope

In scope:

- keep `service.rs` as the small service front door
- move registration, authentication, and discoverable ceremony methods into a
  focused core service module
- move passkey and feature-gated ceremony-state serialization into a focused
  encoding module
- move stored-passkey conversion, credential-id, counter, transport, sync-info,
  metadata, lookup, and update helpers into a focused storage module
- move HTTP request/response adapter wrappers into a focused HTTP module
- preserve existing tests and error behavior

Out of scope:

- changing auth WebAuthn public APIs
- changing challenge, verification, or credential semantics
- changing feature flags
- changing consumer apps

## Acceptance Criteria

- `service.rs` becomes a small front door for `WebAuthnService`
- responsibility groups live in focused internal modules
- crate-root exports and service method names remain stable
- WebAuthn tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal split. If public exports, method names, feature flags, or
credential semantics must change, stop and re-enter planning.

## Current State

`g06.090` is ready.

## Next Task

Execute `g06.090`: auth WebAuthn service internal split.
