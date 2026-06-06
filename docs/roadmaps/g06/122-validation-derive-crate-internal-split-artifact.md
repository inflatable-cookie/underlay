# g06.122 Artifact - Validation Derive Crate Internal Split

## Summary

The validation derive crate now keeps the public proc-macro front door separate
from its private compile-time helpers.

Changed files:

- `underlay-validation-derive/src/lib.rs`
- `underlay-validation-derive/src/derive.rs`
- `underlay-validation-derive/src/field.rs`
- `underlay-validation-derive/src/rules.rs`
- `underlay-validation/Cargo.toml`

The `underlay-validation` dev dependency on `tokio` now enables
`rt-multi-thread` instead of `rt`, because the all-features validation test
suite uses `tokio::runtime::Runtime::new()` in the Axum integration test.

## Module Shape

- `lib.rs`: public derive docs, private module declarations, proc macro entry,
  and crate-local tests
- `derive.rs`: named-struct shape validation and generated `Validate` impl
  construction
- `field.rs`: named field extraction and `#[validate]` attribute collection
- `rules.rs`: `#[validate(...)]` parsing and validator token generation

All helper functions remain crate-private. The exported macro surface is still
only:

- `#[proc_macro_derive(Validate, attributes(validate))]`

## Behavior Preserved

The split keeps the existing derive contract:

- named structs only
- stable generated `::underlay_validation::Validate` paths
- existing validator attribute names and argument syntax
- nested validation error merging
- custom validator string-literal parsing
- current compile-time rejection behavior for unsupported inputs and unknown
  validators

## Validation

Passed:

- `cargo test -p underlay-validation-derive --all-features`
  - 5 unit tests passed
  - 1 doc-test ignored
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
- God-file findings dropped from 28 to 27 after this split.
- The next Rust production warning-level target is
  `underlay-media/src/nightfire/walk.rs`.

## Public API Impact

None.

This was an internal proc-macro implementation split plus a test dependency
feature correction. No macro syntax, generated behavior, or consumer import
path changed.
*** Add File: docs/roadmaps/g06/123-media-nightfire-walk-modularity-audit.md
# g06.123 - Media Nightfire Walk Modularity Audit

## Why

After `g06.122`, the next Rust production warning-level file in the god-file
report is `underlay-media/src/nightfire/walk.rs`.

Nightfire walking code is traversal logic over media content. It should be
split from evidence about traversal responsibilities, mutation boundaries, and
error behavior, not from file size alone.

## Goal

Classify the media Nightfire walk surface and decide the safest next structural
batch.

## Scope

In scope:

- inspect `underlay-media/src/nightfire/walk.rs` by responsibility family
- identify traversal, collection, mutation, error, and helper boundaries
- identify any public or crate-visible behavior that must remain stable
- decide whether the next batch should split internal modules, extract helper
  files, or defer behind a broader media/Nightfire checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing media public APIs
- changing Nightfire content behavior
- changing storage or rendition behavior
- changing consumer apps

## Acceptance Criteria

- walk responsibilities are grouped by stable behavior family
- public or crate-visible behavior boundaries are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a production-code structure audit. If the audit finds traversal or
content behavior that must change, stop and re-enter planning.

## Current State

`g06.123` is ready.

## Next Task

Execute `g06.123`: media Nightfire walk modularity audit.
