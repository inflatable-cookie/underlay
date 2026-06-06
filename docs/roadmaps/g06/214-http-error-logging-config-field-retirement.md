# g06.214 - HTTP Error Logging Config Field Retirement

## Status

Complete.

## Scope

Close public-field compatibility boundaries for:

- `underlay_http::ErrorLoggingConfig`

## Change

- Made error logging config fields private.
- Added read-only accessors for source and logging toggles.
- Updated error logging middleware to use accessors.
- Added a focused builder/accessor unit test.

## Compatibility

Impact: coordinated breaking change.

Known consumers already used constructors and builders, so no consumer code
changes were required. New apps must use constructors, builders, and accessors
instead of direct field reads or struct literals.

## Validation

- `cargo test -p underlay-http --features error-logging`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
