# g06.073 Artifact - Auth JWT Service Tests Modularity Audit

## Summary

`underlay-auth-jwt/src/tests/service_tests.rs` is the largest remaining Rust
high-error god-file. It is test-only, but it covers security-sensitive JWT,
session, refresh-rotation, fingerprint, and error behavior.

The file currently groups:

- shared `MemoryStore` fixture implementing `SessionStore`
- full session lifecycle test: issue, verify, refresh, replay rejection, revoke
- atomic refresh rotation store test:
  `rotate_session_if_current_rejects_stale_refresh_state`
- key generation tests
- config behavior tests
- token issuance tests
- token validation tests
- token fingerprint tests
- JWT error mapping tests

## Behavior Evidence

The test file covers these stable contracts:

- generated key pairs are unique and base64-valid
- private key material is redacted in debug output
- mismatched public/private keys fail service startup
- access and refresh tokens carry required claims and token-use markers
- token IDs are unique
- access/refresh verification accepts the correct token type only
- expired and not-yet-valid tokens respect leeway
- malformed, wrong-signature, wrong-issuer, and missing-audience tokens fail
- token fingerprints are stable, unique by token, and base64url-encoded
- JWT error codes and `AuthError` conversions remain stable
- session refresh rotates refresh token ID/version/fingerprint
- refresh replay is rejected
- revoked sessions reject access tokens
- `SessionStore::rotate_session_if_current` rejects stale refresh state

## Decision

Queue `g06.074` as an auth JWT service tests internal split.

The split should preserve:

- all test names or comparably searchable behavior names
- `MemoryStore` fixture behavior
- async session lifecycle coverage
- refresh replay and stale-rotation coverage
- key/config/token/fingerprint/error behavior coverage
- existing production code and public APIs

Suggested test module shape:

- `service_tests.rs`: test module front door
- `service_tests/support.rs`: `MemoryStore` and shared helpers
- `service_tests/session_lifecycle.rs`
- `service_tests/key_generation.rs`
- `service_tests/config.rs`
- `service_tests/token_issuance.rs`
- `service_tests/token_validation.rs`
- `service_tests/fingerprint.rs`
- `service_tests/errors.rs`

## Public API Impact

Expected impact: none.

This should be a test-only split. If production auth/session API changes are
needed, stop and re-enter planning.

## Validation

- `cargo test -p underlay-auth-jwt --all-features`

Next code batch validation:

- `cargo test -p underlay-auth-jwt --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
