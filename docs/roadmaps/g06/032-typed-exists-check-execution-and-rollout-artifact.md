# g06.032 Artifact - Typed ExistsCheck Execution And Rollout

## Summary

`g06.032` added a typed composite existence-check builder in `underlay-db` and
migrated the current consumer usage.

## Underlay Changes

Added:

- `TypedExistsCheck`
- `TypedExistsCheck::new(QualifiedTableName)`
- `TypedExistsCheck::from_schema_table(schema, table)`
- `TypedExistsCheck::parse_table(table)`
- typed condition methods:
  - `value(column, value)`
  - `value_i32(column, value)`
  - `scope(column, value)`
  - `nullable_value(column, value)`
  - `excluding(id)`
  - `include_deleted()`

`TypedExistsCheck` stores a `QualifiedTableName` and `SqlIdentifier` values
internally. Values remain bound parameters.

Retained:

- `ExistsCheck`
- raw value existence helper functions

Those raw helpers still validate and quote identifiers. They remain
compatibility surface for now.

## Consumer Rollout

Current direct consumer usage was in Farmyard.

Farmyard changes:

- content slug/key checks now use a Farmyard-local wrapper over
  `TypedExistsCheck`
- learning module, area, activity, and pathway uniqueness checks now use
  `TypedExistsCheck`
- stale audit reexports were updated in Acme, Contact Patch, Compli Me, and
  Farmyard after raw audit wrappers were removed in `g06.030`

## Validation

Passed:

- `cargo test -p underlay-db existence --all-features`
- `effigy rust:check`
- `cargo check -p acme-db -p acme-api`
- `cargo check -p cp-db -p cp-api`
- `cargo check -p compli-me-db -p compli-me-api`
- `cargo check -p farmyard-db -p farmyard-api`

## Compatibility Decision

Impact: additive.

Do not remove raw `ExistsCheck` or raw helper functions in this batch. The next
batch should decide whether to deprecate them now that the known consumer usage
has moved.
