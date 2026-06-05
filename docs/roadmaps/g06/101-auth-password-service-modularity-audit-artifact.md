# g06.101 Artifact - Auth Password Service Modularity Audit

## Summary

`underlay-auth-password/src/service.rs` is the largest remaining Rust
warning-level production file. It combines public config, compromised-password
strategy, repository trait, failed-login model, service construction,
password-policy checks, set-password behavior, login/rate-limit/lockout
behavior, password change, and admin reset behavior in one security-sensitive
file.

The current surface groups:

- `PasswordConfig`
- `CompromisedPasswordStrategy`
- `PasswordAuthRepository`
- `FailedLoginAttempt`
- `PasswordAuthService`
- service construction and analyzer setup
- compromised-password policy checks
- `set_password()`
- `verify_login()` and `verify_login_with_context()`
- `change_password()`
- `reset_password()`

## Public Surface Evidence

The crate root re-exports these service items:

- `CompromisedPasswordStrategy`
- `FailedLoginAttempt`
- `PasswordAuthRepository`
- `PasswordAuthService`
- `PasswordConfig`

Docs reference these names and methods:

- `docs/guides/060-authentication.md`
- `docs/guides/066-spa-deployment-and-static-auth.md`
- `docs/guides/code/060-authentication/auth-service-example.rs`

## Behavior Evidence

The focused crate validation covers these stable contracts:

- password errors map to stable auth error codes
- HIBP k-anonymity parser and local-server check behave as expected
- password strength analysis rejects weak/common passwords and accepts strong
  passwords
- compromised passwords are rejected when policy is enabled
- login normalizes email addresses
- successful login resets failures
- rate limiting blocks login attempts
- lockout triggers after failed attempts
- password change rejects wrong current password
- password change rejects same-password updates
- password change updates the hash and allows login with the new password
- reset password updates the hash and allows login with the reset password

Validation result:

- `cargo test -p underlay-auth-password --all-features`
- 32 unit tests passed

## Decision

Queue `g06.102` as an auth password service internal split.

The split should preserve:

- all crate-root re-exports
- all public service method names and signatures
- default config values
- compromised-password strategy behavior and HIBP feature gate
- password strength threshold behavior
- same-password rejection behavior
- login email normalization
- rate-limit key format
- suspended/deleted account handling
- lockout and failed-attempt behavior
- password credential metadata validation

Suggested module shape:

- `service.rs`: module front door, public re-exports, and test module
- `service/config.rs`: `PasswordConfig` and `CompromisedPasswordStrategy`
- `service/repository.rs`: `PasswordAuthRepository` and `FailedLoginAttempt`
- `service/core.rs`: `PasswordAuthService` type and construction
- `service/policy.rs`: compromised-password helper and password policy
  validation helper
- `service/passwords.rs`: `set_password()`, `change_password()`, and
  `reset_password()`
- `service/login.rs`: `verify_login()` and `verify_login_with_context()`

## Public API Impact

Expected impact: none.

This should be an internal split. If preserving crate-root exports or password
auth semantics forces a public API change, stop and re-enter planning.

## Validation

- `cargo test -p underlay-auth-password --all-features`

Next code batch validation:

- `cargo test -p underlay-auth-password --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
