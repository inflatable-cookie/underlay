# g06.188 Artifact - Validation Validator Surface Internal Split

Status: complete
Owner: repo maintainers
Completed: 2026-06-06

## Purpose

Address the largest remaining Rust production source file found by the fresh
post-drift sweep: `underlay-validation/src/validators.rs`.

## Result

The public validator surface is unchanged:

- `underlay_validation::validators::email`
- `url`
- `uuid`
- `length`
- `required`
- `range`
- `positive`
- `non_negative`
- `pattern`
- `one_of`
- `not_empty`
- `collection_length`
- `alphanumeric`
- `username`
- `slug`
- `unique_items`
- `unique_items_detailed`

Internally, the implementation is split by responsibility:

- `validators/string.rs`
- `validators/numeric.rs`
- `validators/pattern.rs`
- `validators/collection.rs`

`validators.rs` remains the stable public front door and re-exports the same
function names.

## Consumer Upgrade Impact

Impact class: `internal`.

No current consumer changes are required. The six-consumer scan found no direct
dependency on validator implementation modules; consumers use either derive
validation, `validator`, or the stable `underlay_validation::validators` front
door.

## Validation

- `cargo test -p underlay-validation --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- six-consumer source scan for direct `underlay_validation::validators` usage

## Next Task

Continue the Rust public-surface audit from `122`, or leave structural work
closed until Effigy reports Rust source findings again.
