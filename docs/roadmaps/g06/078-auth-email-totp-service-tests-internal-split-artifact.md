# g06.078 Artifact - Auth Email TOTP Service Tests Internal Split

## Summary

`underlay-auth-email-totp/src/tests/service_tests.rs` is now a small test front
door. The previous test monolith was split into focused modules under
`underlay-auth-email-totp/src/tests/service_tests/`.

The split is test-only. It does not change auth email TOTP production code,
verification/session semantics, public APIs, or consumer apps.

## Module Shape

- `service_tests.rs`: explicit test module front door
- `service_tests/support.rs`: mock code repository, mock session repository,
  mock sender, hash/session/rate-limit helpers
- `service_tests/request_code.rs`: rate-limit and storage/send request tests
- `service_tests/verify_errors.rs`: missing, expired, exhausted, and invalid
  verification tests
- `service_tests/verify_success.rs`: successful verification and code-only
  verification tests
- `service_tests/sessions.rs`: consume/get session delegation tests

## Behavior Preserved

- all 12 auth email TOTP crate tests pass
- request-code rate-limit behavior is unchanged
- request-code storage/send behavior is unchanged
- verify-code missing, expired, exhausted, and invalid paths remain covered
- invalid-code attempt increment behavior is unchanged
- successful verification still marks codes used and creates sessions
- code-only verification still trims input and avoids session creation
- session consume/get delegation remains covered

## Public API Impact

None.

This was a Rust test-structure split only.

## Validation

- `cargo test -p underlay-auth-email-totp --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` remains expected-fail on the known backlog; `scan.god-files`
  improved from 50 findings / 9 errors to 49 findings / 8 errors.
