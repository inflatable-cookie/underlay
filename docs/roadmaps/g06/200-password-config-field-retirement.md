# g06.200 - Password Config Field Retirement

## Status

Complete.

## Scope

Close the `underlay_auth_password::PasswordConfig` public-field compatibility
boundary.

## Change

- Made `PasswordConfig` fields private.
- Added read-only accessors for password policy, lockout, rate-limit, and
  compromised-password settings.
- Added builder-style setters for each retained policy value.
- Updated password-auth service internals and crate tests to use accessors and
  builders.
- Updated the known Composer local-login policy literal to use builders.

## Compatibility

Impact: coordinated breaking change.

Known consumers that constructed `PasswordConfig` with public fields were
migrated. New apps must use `PasswordConfig::default()` with builders and
accessors instead of direct field reads or struct literals.

## Validation

- `cargo test -p underlay-auth-password`
