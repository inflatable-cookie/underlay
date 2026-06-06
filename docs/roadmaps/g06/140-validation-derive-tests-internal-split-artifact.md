# g06.140 Artifact - Validation Derive Tests Internal Split

## Summary

Validation derive integration tests now use a small front-door test target with
behavior-focused modules instead of one large `derive_tests.rs` file.

Changed files:

- `underlay-validation/tests/derive_tests.rs`
- `underlay-validation/tests/derive_tests/basic.rs`
- `underlay-validation/tests/derive_tests/simple.rs`
- `underlay-validation/tests/derive_tests/custom.rs`
- `underlay-validation/tests/derive_tests/collections.rs`
- `underlay-validation/tests/derive_tests/nested.rs`

## Module Shape

- `derive_tests.rs`: integration-test front door and explicit module paths
- `basic.rs`: multi-field derived request behavior
- `simple.rs`: URL, UUID, username, slug, numeric, and alphanumeric validators
- `custom.rs`: skip, custom validator, and pattern validators
- `collections.rs`: collection validators
- `nested.rs`: nested validation and nested field error paths

The Cargo integration test target remains `derive_tests`.

## Behavior Preserved

The split keeps existing validation behavior:

- `Validate` derives for all current field attributes still compile
- valid values pass
- invalid values report expected field names
- multiple invalid fields are aggregated
- `skip` excludes a field from validation
- custom validator functions return `FieldError`
- regex pattern validation still works
- collection validators still report collection field errors
- nested validation still reports `address.city`

## Validation

Passed:

- `cargo test -p underlay-validation --test derive_tests --all-features`
  - 18 integration tests passed
- `cargo test -p underlay-validation --all-features`
  - 50 unit tests passed
  - 18 integration tests passed
  - 19 doc-tests passed
  - 5 doc-tests ignored
- `effigy rust:check`

Known backlog:

- `effigy doctor` still fails on the existing structural scan backlog:
  attention markers, comment ratio, and god-files.
- God-file findings dropped from 19 to 18 after this split.
- No Rust files remain in the current god-file report.

## Public API Impact

None.

This was a test-only split. No validation API, derive macro behavior,
validation field path, error aggregation behavior, or consumer import path
changed.
