# g06.074 Artifact - Auth JWT Service Tests Internal Split

## Summary

`underlay-auth-jwt/src/tests/service_tests.rs` is now a small test front door.
Production auth/JWT code is unchanged.

New test modules:

- `service_tests/support.rs`: `MemoryStore` fixture implementing
  `SessionStore`
- `service_tests/session_lifecycle.rs`: session issue, verify, refresh,
  replay rejection, revoke, and stale refresh rotation
- `service_tests/key_generation.rs`: key generation and decode behavior
- `service_tests/config.rs`: config defaults, generated keys, mismatch startup
  failure, and debug redaction
- `service_tests/token_issuance.rs`: access/refresh claim issuance and token ID
  uniqueness
- `service_tests/token_validation_success.rs`: valid access and refresh token
  verification
- `service_tests/token_temporal.rs`: expiry, not-before, and leeway behavior
- `service_tests/token_rejections.rs`: malformed, wrong signature, wrong token
  type, and wrong issuer rejection
- `service_tests/token_audience.rs`: configured audience behavior
- `service_tests/fingerprint.rs`: fingerprint determinism, uniqueness, and
  base64url encoding
- `service_tests/errors.rs`: JWT error codes and `AuthError` conversion

## Preserved Behavior

The split preserved:

- all auth JWT production APIs
- `MemoryStore` fixture behavior
- async session lifecycle coverage
- refresh replay and stale-rotation coverage
- key/config/token/fingerprint/error behavior coverage
- 38 passing auth JWT tests

## Structural Result

`service_tests.rs` moved from a high-error god-file into a front door:

- `service_tests.rs`: 22 lines
- largest child module: `token_temporal.rs` at 109 lines
- no `underlay-auth-jwt/src/tests/service_tests*` entry remains in the
  `scan.god-files` report

`effigy doctor` now reports:

- `scan.god-files`: 51 findings, 10 errors, 41 warnings
- `scan.attention-markers`: 11 findings, 2 errors, 9 warnings
- `scan.comment-ratio`: 12 findings, 3 errors, 9 warnings

The doctor failure remains the known structural backlog.

## Public API Impact

Impact: none.

This was a test-only split. No consumer app update is required.

## Validation

- `cargo test -p underlay-auth-jwt --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` expected failure on known structural scans
