# g06.206 - Security Alert Config Field Retirement

## Status

Complete.

## Scope

Close the `underlay_security_alerts::SecurityAlertConfig` public-field
compatibility boundary.

## Change

- Made `SecurityAlertConfig` fields private.
- Added read-only accessors for alert window, cooldown, and thresholds.
- Added builder-style setters for each retained alert policy value.
- Updated Underlay alert detection internals.
- Migrated known consumer auth alert configuration and cooldown/window reads in:
  - `underlay-reference`
  - `contact-patch`
  - `compli-me`
  - `acowtancy`
  - `songsprout`
  - `loophole/composer`

## Compatibility

Impact: coordinated breaking change.

Known consumer struct literals and direct field reads were migrated. New apps
must use `SecurityAlertConfig::default()` with builders and accessors instead
of direct field reads or struct literals.

## Validation

- `cargo test -p underlay-security-alerts`
- `effigy rust:check`
- `cargo check -p acme-auth`
- `cargo check -p cp-auth`
- `cargo check -p compli-me-auth`
- `cargo check -p farmyard-auth`
- `cargo check -p nursery-auth`
- `cargo check -p composer-api`
