# g06.084 Artifact - Auth Password Service Tests Internal Split

## Summary

`underlay-auth-password/src/tests/service_tests.rs` is now a small test front
door. The previous test monolith was split into focused modules under
`underlay-auth-password/src/tests/service_tests/`.

The split is test-only. It does not change auth password production code,
password hashing or verification semantics, reset/session security behavior,
public APIs, or consumer apps.

## Module Shape

- `service_tests.rs`: explicit test module front door
- `service_tests/support.rs`: in-memory password repository, user fixture, and
  service setup helper
- `service_tests/login.rs`: login success and email normalization tests
- `service_tests/lockout.rs`: lockout and rate-limit tests
- `service_tests/password_policy.rs`: compromised-password policy test
- `service_tests/change_password.rs`: password change rejection and success
  tests
- `service_tests/reset_password.rs`: reset password success test

## Behavior Preserved

- all 32 auth password crate tests pass
- successful login still resets failed-login counters and lockout state
- repeated wrong passwords still trigger account lockout
- contextual login attempts still respect rate limiting
- compromised-password checks still reject locally blocked passwords when
  enabled
- password change still rejects wrong current passwords and password reuse
- password change still updates the hash and invalidates the old password
- password reset still updates the hash and allows login with the reset password
- login still normalizes email input before lookup

## Public API Impact

None.

This was a Rust test-structure split only.

## Validation

- `cargo test -p underlay-auth-password --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` remains expected-fail on the known backlog; `scan.god-files`
  improved from 47 findings / 6 errors to 46 findings / 5 errors. The remaining
  high-error god-files are TypeScript-only.
