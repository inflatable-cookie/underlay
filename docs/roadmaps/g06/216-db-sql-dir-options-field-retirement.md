# g06.216 - DB SQL-Dir Options Field Retirement

## Status

Complete.

## Scope

Close public-field compatibility boundaries for:

- `underlay_db::SqlDirOptions`

## Change

- Made SQL-dir option fields private.
- Added `new`, builder methods, and read-only accessors.
- Updated SQL-dir execution internals to use accessors.
- Added a focused builder/accessor unit test.

## Compatibility

Impact: coordinated breaking change.

No known consumers used struct literals for this option type. New callers must
use defaults, builders, and accessors instead of direct field reads or struct
literals.

## Validation

- `cargo test -p underlay-db`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
