# g06.210 - MFA Config Field Retirement

## Status

Complete.

## Scope

Close public-field compatibility boundaries for MFA auth config structs:

- `underlay_auth_totp::TotpConfig`
- `underlay_auth_webauthn::WebAuthnConfig`

## Change

- Made `TotpConfig` fields private.
- Added read-only accessors for TOTP issuer, algorithm, digits, period, and
  skew.
- Made `WebAuthnConfig` fields private.
- Added `WebAuthnConfig::new` plus read-only relying-party accessors.
- Updated Underlay MFA internals and tests.
- Migrated known consumer auth MFA configuration in:
  - `underlay-reference`
  - `contact-patch`
  - `compli-me`
  - `acowtancy`
  - `songsprout`

## Compatibility

Impact: coordinated breaking change.

Known consumer struct literals were migrated. New apps must use constructors,
builders, and accessors instead of direct field reads or struct literals.

## Validation

- `cargo test -p underlay-auth-totp`
- `cargo test -p underlay-auth-webauthn`
- `cargo check -p acme-auth`
- `cargo check -p cp-auth`
- `cargo check -p compli-me-auth`
- `cargo check -p farmyard-auth`
- `cargo check -p nursery-auth`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
