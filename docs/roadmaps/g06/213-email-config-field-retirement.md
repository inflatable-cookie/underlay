# g06.213 - Email Config Field Retirement

## Status

Complete.

## Scope

Close public-field compatibility boundaries for email config structs:

- `underlay_email::EmailManagerConfig`
- `underlay_email::SmtpConfig`
- `underlay_email::SesConfig`
- `underlay_email::DevCaptureConfig`

## Change

- Made email config fields private.
- Added constructors, builder methods, and read-only accessors for retained
  configuration values.
- Updated Underlay email adapters and tests.
- Migrated known SMTP consumer factories in:
  - `underlay-reference`
  - `contact-patch`
  - `compli-me`
  - `acowtancy/farmyard`

## Compatibility

Impact: coordinated breaking change.

Known consumer struct literals were migrated. New apps must use constructors,
builders, and accessors instead of direct field reads or struct literals.

## Validation

- `cargo test -p underlay-email`
- `cargo check -p acme-infra`
- `cargo check -p cp-infra`
- `cargo check -p compli-me-infra`
- `cargo check -p farmyard-infra`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
