# g06.202 - OAuth Config Field Retirement

## Status

Complete.

## Scope

Close the `underlay_auth_oauth::GoogleOAuthConfig` public-field compatibility
boundary and remove the mutable public verified-email flag from
`GoogleOAuthAppService`.

## Change

- Made `GoogleOAuthConfig` fields private, including the Google client secret.
- Added `GoogleOAuthConfig::new`, read-only accessors, and `with_scopes`.
- Added manual `Debug` output that redacts the client secret value.
- Made `GoogleOAuthAppService::require_verified_email` private.
- Added `require_verified_email()` and `with_require_verified_email()`.
- Updated OAuth service internals and crate tests to use constructors,
  accessors, and builders.
- Updated Acowtancy's Google OAuth construction path.

## Compatibility

Impact: coordinated breaking change.

Known consumer struct literals were migrated. New apps must use
`GoogleOAuthConfig::new`, `with_scopes`, and accessors instead of direct field
reads or struct literals. Apps that need to change the verified-email policy
must use `with_require_verified_email`.

## Validation

- `cargo test -p underlay-auth-oauth`
- `effigy rust:check`
- `cargo check -p farmyard-auth`
