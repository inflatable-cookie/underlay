# g06.083 Artifact - Auth Password Service Tests Modularity Audit

## Summary

`underlay-auth-password/src/tests/service_tests.rs` is now the only remaining
Rust high-error god-file. It is test-only, but it covers security-sensitive
password login, lockout, rate limiting, compromised-password checks, password
change/reset, and email normalization behavior in one file.

The file currently groups:

- shared in-memory password repository fixture
- shared user fixture helper
- successful login and failed-login reset behavior
- failed-login lockout behavior
- login rate-limit behavior
- compromised password rejection behavior
- password change rejection and success behavior
- password reset success behavior
- login email normalization behavior

## Behavior Evidence

The test file covers these stable contracts:

- successful login resets failed-login counters and lockout state
- repeated wrong passwords trigger account lockout
- contextual login attempts respect rate limiting by caller context
- compromised-password checks reject locally blocked passwords when enabled
- password change rejects wrong current passwords
- password change rejects reuse of the current password
- password change updates the hash and allows login with the new password only
- password reset updates the hash and allows login with the reset password
- login normalizes email input before lookup

## Decision

Queue `g06.084` as an auth password service tests internal split.

The split should preserve:

- all test names or comparably searchable behavior names
- in-memory repository lockout/rate-limit fixture behavior
- password hashing/verifier setup behavior
- login, lockout, rate-limit, change, reset, and normalization coverage
- existing production code and public APIs

Suggested test module shape:

- `service_tests.rs`: test module front door
- `service_tests/support.rs`: `MemoryRepo`, `make_user`, and service setup
  helpers
- `service_tests/login.rs`
- `service_tests/lockout.rs`
- `service_tests/password_policy.rs`
- `service_tests/change_password.rs`
- `service_tests/reset_password.rs`

## Public API Impact

Expected impact: none.

This should be a test-only split. If production auth password APIs or security
semantics must change, stop and re-enter planning.

## Validation

- `cargo test -p underlay-auth-password --all-features`

Next code batch validation:

- `cargo test -p underlay-auth-password --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
