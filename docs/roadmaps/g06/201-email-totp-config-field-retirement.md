# g06.201 - Email TOTP Config Field Retirement

## Status

Complete.

## Scope

Close the `underlay_auth_email_totp::EmailTotpConfig` public-field
compatibility boundary.

## Change

- Made `EmailTotpConfig` fields private.
- Added read-only accessors for code expiry, send limits, verification attempts,
  session expiry, and code length.
- Kept existing builder-style setters as the supported mutation path.
- Updated email-TOTP service internals and crate tests to use accessors.

## Compatibility

Impact: coordinated breaking change.

No known consumer in the proof family directly read the Underlay
`EmailTotpConfig` fields. New apps must use builders and accessors instead of
direct field reads or struct literals.

## Validation

- `cargo test -p underlay-auth-email-totp`
- `effigy rust:check`
