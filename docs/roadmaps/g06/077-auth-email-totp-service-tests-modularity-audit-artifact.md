# g06.077 Artifact - Auth Email TOTP Service Tests Modularity Audit

## Summary

`underlay-auth-email-totp/src/tests/service_tests.rs` is the largest remaining
Rust high-error god-file after the media Nightfire split. It is test-only, but
it covers authentication-sensitive email TOTP request, verification, and
session behavior in one file.

The file currently groups:

- shared mock code repository state and repository implementation
- shared mock verification-session repository state and implementation
- shared mock email sender
- helper functions for Argon2 code hashing, verification sessions, and default
  rate-limit state
- request-code rate-limit and storage/send tests
- verify-code missing, expired, exhausted, invalid, and successful paths
- code-only verification behavior
- consume/get session repository delegation behavior

## Behavior Evidence

The test file covers these stable contracts:

- rate-limited request-code calls return `EmailTotpError::RateLimited`
- successful request-code calls store a hash, increment send count, and send to
  the expected user/email
- missing active code returns `EmailTotpError::NoActiveCode`
- expired active code returns `EmailTotpError::CodeExpired`
- exhausted active code returns `EmailTotpError::TooManyAttempts`
- invalid code increments attempts and returns `EmailTotpError::InvalidCode`
- successful code verification marks the code used and creates a verification
  session
- code-only verification trims input, marks the code used, and does not create
  a session
- consume/get session calls delegate to the session repository

## Decision

Queue `g06.078` as an auth email TOTP service tests internal split.

The split should preserve:

- all test names or comparably searchable behavior names
- mock repository and sender behavior
- Argon2 hashing helper behavior
- request-code rate-limit/storage/send coverage
- verify-code failure and success coverage
- code-only verification coverage
- session delegation coverage
- existing production code and public APIs

Suggested test module shape:

- `service_tests.rs`: test module front door
- `service_tests/support.rs`: mock repositories, mock sender, `hash`,
  `session`, and `default_rate_limit`
- `service_tests/request_code.rs`
- `service_tests/verify_errors.rs`
- `service_tests/verify_success.rs`
- `service_tests/sessions.rs`

## Public API Impact

Expected impact: none.

This should be a test-only split. If production auth email TOTP APIs or
verification semantics must change, stop and re-enter planning.

## Validation

- `cargo test -p underlay-auth-email-totp --all-features`

Next code batch validation:

- `cargo test -p underlay-auth-email-totp --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
