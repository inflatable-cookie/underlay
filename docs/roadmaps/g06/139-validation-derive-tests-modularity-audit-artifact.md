# g06.139 Artifact - Validation Derive Tests Modularity Audit

## Summary

`underlay-validation/tests/derive_tests.rs` is the next Rust warning-level file
after `g06.138`. It is an integration test target for the `Validate` derive
macro because generated code uses external `::underlay_validation::` paths.

The current file groups:

- basic multi-field derive validation for email, length, range, and required
  fields
- simple string validators for URL, UUID, username, and slug
- numeric validators for positive and non-negative values
- skip validator behavior
- custom validator function behavior
- regex pattern validator behavior
- collection validators for non-empty and collection length
- nested validation and nested field error paths
- alphanumeric validator behavior

## Boundary Evidence

The test target must remain an integration test, not a crate-local unit test,
because the derive macro emits `::underlay_validation::` paths.

Behavior boundaries to preserve:

- `Validate` derive compiles for structs using supported field attributes
- valid values pass
- invalid values produce field errors with expected field paths
- multiple invalid fields are all reported
- `skip` suppresses validation for a field
- custom validator functions can return `FieldError`
- nested validation reports child errors as `address.city`

## Behavior Evidence

Existing focused tests cover:

- 3 basic derived request cases
- 2 simple validator cases
- 2 numeric validator cases
- 1 skip case
- 2 custom validator cases
- 2 pattern validator cases
- 2 collection validator cases
- 2 nested validator cases
- 2 alphanumeric validator cases

Baseline validation:

- `cargo test -p underlay-validation --test derive_tests --all-features`
- 18 integration tests passed

## Decision

Queue `g06.140` as a validation derive tests internal split.

Suggested module shape:

- `tests/derive_tests.rs`: integration-test front door and shared imports
- `tests/derive_tests/basic.rs`: multi-field derived request behavior
- `tests/derive_tests/simple.rs`: URL, UUID, username, slug, numeric, and
  alphanumeric validator behavior
- `tests/derive_tests/custom.rs`: skip, custom validator, and pattern behavior
- `tests/derive_tests/collections.rs`: collection validator behavior
- `tests/derive_tests/nested.rs`: nested derive behavior and nested field paths

This keeps the Cargo integration test target stable while making each macro
behavior family easier to read and extend.

## Public API Impact

Expected impact: none.

If preserving the split requires changing derive macro behavior, validation
field paths, error aggregation, public validation APIs, or test target
semantics, stop and re-enter planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-validation --test derive_tests --all-features`
- `cargo test -p underlay-validation --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
