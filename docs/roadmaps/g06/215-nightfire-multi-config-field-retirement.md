# g06.215 - Nightfire Multi Config Field Retirement

## Status

Complete.

## Scope

Close public-field compatibility boundaries for:

- `underlay_nightfire::MultiConfig`

## Change

- Made multi-block config fields private.
- Added `new`, `one_or_more`, `with_max_blocks`, and read-only accessors.
- Updated Nightfire validation and tests.
- Updated the Nightfire README example.
- Migrated known Acowtancy strategy registry construction.

## Compatibility

Impact: coordinated breaking change.

Known consumer struct literals were migrated. New apps must use constructors,
builders, and accessors instead of direct field reads or struct literals.

## Validation

- `cargo test -p underlay-nightfire`
- `cargo check -p nightfire`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
