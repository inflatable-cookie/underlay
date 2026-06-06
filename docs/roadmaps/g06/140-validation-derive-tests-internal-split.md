# g06.140 - Validation Derive Tests Internal Split

## Why

`g06.139` found that `underlay-validation/tests/derive_tests.rs` mixes several
derive macro behavior families in one integration test file.

The target must remain an integration test because the derive macro emits
external crate paths. The next step is a mechanical test split that keeps the
same test target and validation behavior.

## Goal

Split validation derive integration tests into focused internal modules without
changing validation APIs, derive macro behavior, field paths, or test target
semantics.

## Scope

In scope:

- keep `underlay-validation/tests/derive_tests.rs` as the integration-test
  front door
- move basic multi-field derive tests into `tests/derive_tests/basic.rs`
- move simple, numeric, and alphanumeric validator tests into
  `tests/derive_tests/simple.rs`
- move skip, custom validator, and pattern tests into
  `tests/derive_tests/custom.rs`
- move collection validator tests into `tests/derive_tests/collections.rs`
- move nested validation tests into `tests/derive_tests/nested.rs`
- preserve all 18 current integration test cases

Out of scope:

- changing validation public APIs
- changing derive macro behavior
- changing validation semantics
- changing consumer apps

## Acceptance Criteria

- `cargo test -p underlay-validation --test derive_tests --all-features` passes
- full `underlay-validation` tests pass with all features
- `effigy rust:check` passes
- roadmap artifact records the final module shape and public API impact

## Consumer Upgrade Impact

Expected impact: none.

This should be a test-only split. If validation behavior needs to change, stop
and re-enter planning.

## Current State

`g06.140` is ready.

## Next Task

Execute `g06.140`: validation derive tests internal split.
