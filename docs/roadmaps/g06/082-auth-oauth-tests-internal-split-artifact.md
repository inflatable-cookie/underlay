# g06.082 Artifact - Auth OAuth Tests Internal Split

## Summary

`underlay-auth-oauth/src/tests/lib_tests.rs` is now a small test front door.
The previous test monolith was split into focused modules under
`underlay-auth-oauth/src/tests/lib_tests/`.

The split is test-only. It does not change auth OAuth production code,
redirect/callback semantics, token/session security behavior, public APIs, or
consumer apps.

## Module Shape

- `lib_tests.rs`: explicit test module front door
- `lib_tests/support.rs`: env-var helpers, global env lock, stub OAuth
  provider, in-memory user/credential repository, token/userinfo fixtures, and
  callback request/state helpers
- `lib_tests/config.rs`: env/config behavior
- `lib_tests/login.rs`: login URL, state, and PKCE behavior
- `lib_tests/callback.rs`: callback success, state mismatch, and verified-email
  rejection behavior
- `lib_tests/disconnect.rs`: OAuth credential disconnect behavior

## Behavior Preserved

- all 10 auth OAuth crate tests pass
- missing env config still returns an auth error
- login URL generation still includes state, PKCE challenge, S256 method, and
  redirect URI
- generated login starts still include state and PKCE verifier values
- callback success still creates a user and OAuth Google credential
- state mismatch still returns `AuthError::BadRequest`
- verified-email enforcement still rejects unverified Google email addresses
- disconnect still removes the OAuth Google credential

## Public API Impact

None.

This was a Rust test-structure split only.

## Validation

- `cargo test -p underlay-auth-oauth --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` remains expected-fail on the known backlog; `scan.god-files`
  improved from 48 findings / 7 errors to 47 findings / 6 errors.
