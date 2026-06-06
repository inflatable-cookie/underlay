# g06.204 - DB Config Field Retirement

## Status

Complete.

## Scope

Close the `underlay_db::DbConfig` public-field compatibility boundary and stop
debug output from exposing database URLs.

## Change

- Made `DbConfig` fields private.
- Added read-only accessors for the database URL and pool settings.
- Kept existing builders as the supported mutation path.
- Replaced derived `Debug` with manual output that redacts the database URL.
- Updated pool construction and crate tests to use accessors.

## Compatibility

Impact: coordinated breaking change.

No known consumer in the proof family directly read the Underlay `DbConfig`
fields. New apps must use `DbConfig::new`, builders, and accessors instead of
direct field reads or struct literals.

## Validation

- `cargo test -p underlay-db`
- `effigy rust:check`
