# g06.102 Artifact - Auth Password Service Internal Split

## Summary

`underlay-auth-password/src/service.rs` is now a small module front door with
stable public re-exports. The former mixed password service file was split into
focused modules.

New module shape:

- `service.rs`: front door, public re-exports, and test module
- `service/config.rs`: `PasswordConfig` and `CompromisedPasswordStrategy`
- `service/repository.rs`: `PasswordAuthRepository` and `FailedLoginAttempt`
- `service/core.rs`: `PasswordAuthService` type and construction
- `service/policy.rs`: compromised-password and password-policy validation
  helpers
- `service/passwords.rs`: `set_password()`, `change_password()`, and
  `reset_password()`
- `service/login.rs`: `verify_login()` and `verify_login_with_context()`

## Public API Impact

None expected.

The crate-root service exports, public service methods, default config values,
HIBP feature gate, password policy threshold, same-password rejection, login
email normalization, rate-limit key format, account-status handling, lockout
behavior, and credential metadata validation were preserved.

The service test module still has test-only access to `PasswordAuthError` and
`PasswordAuthResult` through the same local path used before the split.

## Validation

- `cargo test -p underlay-auth-password --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`

`cargo test -p underlay-auth-password --all-features` passed with 32 unit
tests passed.

`effigy doctor` still fails on the known scanner backlog:

- `scan.god-files`: 37 findings, 5 TypeScript error-level findings
- `scan.attention-markers`: 11 findings, 2 error-level findings
- `scan.comment-ratio`: 12 findings, 3 error-level findings

The auth password service no longer appears in the god-file report. The next
largest Rust production warning is
`rust/crates/underlay-media/src/renditions/service.rs`.

## Next Target Evidence

Queue `g06.103` as a media renditions service modularity audit before splitting
`underlay-media/src/renditions/service.rs`. Media rendition generation is a
shared processing surface, so the next batch should classify public service
methods, rendition planning, storage/repository behavior, processor calls, and
tests before moving code.
