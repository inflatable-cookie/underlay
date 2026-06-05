# g06.081 Artifact - Auth OAuth Tests Modularity Audit

## Summary

`underlay-auth-oauth/src/tests/lib_tests.rs` is the largest remaining Rust
high-error god-file after the devtools migration-bundle split. It is test-only,
but it covers OAuth config, login URL generation, callback/session behavior,
and disconnect behavior in one file.

The file currently groups:

- env-var mutation helpers guarded by a global lock
- Google OAuth config/env loading tests
- login URL/state/PKCE tests
- stub OAuth provider fixture
- in-memory user/credential repository fixture
- Google callback success test
- callback state mismatch and verified-email rejection tests
- Google disconnect test

## Behavior Evidence

The test file covers these stable contracts:

- `GoogleOAuthService::from_env` fails when required config is missing
- `start_login_with` builds a URL with state, PKCE challenge, S256 method, and
  redirect URI
- `start_login` generates state and PKCE verifier values
- callback success creates a user and OAuth Google credential
- callback state mismatch returns `AuthError::BadRequest`
- verified-email enforcement rejects unverified Google email addresses
- disconnect removes the stored OAuth Google credential

## Decision

Queue `g06.082` as an auth OAuth tests internal split.

The split should preserve:

- all test names or comparably searchable behavior names
- env-var lock and restore behavior
- stub provider behavior
- in-memory user/credential repository behavior
- login URL and PKCE coverage
- callback success/error coverage
- disconnect coverage
- existing production code and public APIs

Suggested test module shape:

- `lib_tests.rs`: test module front door
- `lib_tests/support.rs`: env helpers, `StubProvider`, `MemoryRepo`, and common
  token/user fixtures
- `lib_tests/config.rs`
- `lib_tests/login.rs`
- `lib_tests/callback.rs`
- `lib_tests/disconnect.rs`

## Public API Impact

Expected impact: none.

This should be a test-only split. If production auth OAuth APIs or security
semantics must change, stop and re-enter planning.

## Validation

- `cargo test -p underlay-auth-oauth --all-features`

Next code batch validation:

- `cargo test -p underlay-auth-oauth --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
