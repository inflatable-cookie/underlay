# g06.121 Artifact - Validation Derive Crate Modularity Audit

## Summary

`underlay-validation-derive/src/lib.rs` is the largest remaining Rust
warning-level production file after `g06.120`. It owns the public
`#[derive(Validate)]` macro and every internal step needed to generate
validation code from `#[validate(...)]` field attributes.

The current file groups:

- proc macro entry and public derive documentation
- named-struct shape validation
- generated `Validate` impl construction
- field traversal and `#[validate]` attribute collection
- simple validator token generation
- length, range, and collection-length argument parsing
- pattern and custom validator parsing
- compile-time error reporting for unsupported inputs and unknown validators
- crate-local token-generation tests

## Boundary Evidence

The exported surface is only:

- `#[proc_macro_derive(Validate, attributes(validate))]`

The crate-private helper surface is currently:

- `impl_validate(input: &DeriveInput) -> SynResult<TokenStream2>`
- `generate_field_validation(field: &Field) -> SynResult<Option<TokenStream2>>`
- `parse_validate_attr(...) -> SynResult<Vec<TokenStream2>>`

Because this is a proc-macro crate, helper modules should stay private or
`pub(crate)`. The split must not add public exports beyond the derive macro.

## Behavior Evidence

Existing coverage is split across two layers:

- `underlay-validation-derive/src/tests/lib_tests.rs` checks compile-time token
  generation and rejection paths for enums, tuple structs, unknown validators,
  basic validators, nested validators, and custom validators.
- `underlay-validation/tests/derive_tests.rs` checks generated validation
  behavior through the public `underlay_validation::Validate` import path.

The split should preserve:

- named structs only
- stable generated `::underlay_validation::Validate` paths
- all existing validator attribute names and syntax
- nested error merging
- custom validator identifier parsing from a string literal
- current compile-time error behavior for unsupported inputs and unknown
  validators

## Decision

Queue `g06.122` as a validation derive crate internal split.

Suggested module shape:

- `lib.rs`: macro documentation, private module declarations, proc macro entry,
  and test module declaration
- `derive.rs`: struct-shape validation and generated impl construction
- `field.rs`: named-field extraction and validation attribute collection
- `rules.rs`: `#[validate(...)]` parser and validator token generation

This keeps the first split small. A deeper split of `rules.rs` into validator
families can wait until the parser itself needs behavior changes.

## Public API Impact

Expected impact: none.

The derive macro name, accepted attributes, generated paths, and validation
behavior should stay stable. If the split requires changing macro syntax or
generated behavior, stop and re-enter planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-validation-derive --all-features`
- `cargo test -p underlay-validation --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
