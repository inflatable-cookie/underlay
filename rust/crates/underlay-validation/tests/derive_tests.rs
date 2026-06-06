//! Integration tests for the derive macro.
//!
//! These tests are in an integration test file (rather than in lib.rs) because
//! the derive macro generates code with `::underlay_validation::` paths, which
//! only resolve correctly when importing the crate externally.

#[path = "derive_tests/basic.rs"]
mod basic;
#[path = "derive_tests/collections.rs"]
mod collections;
#[path = "derive_tests/custom.rs"]
mod custom;
#[path = "derive_tests/nested.rs"]
mod nested;
#[path = "derive_tests/simple.rs"]
mod simple;
