# g06.127 Artifact - Migration-Core Test Support Modularity Audit

## Summary

`underlay-migration-core/src/tests/support.rs` is the next Rust warning-level
file after `g06.126`. It is shared test support for migration pipeline,
decision reuse, integrity, and resume tests.

The current file groups:

- deterministic decision fingerprint helper
- `MockSource`
- `MockPlugin`
- `MockDecisionResolver`
- `MockAssetResolver`
- `InMemoryRunStore`
- run-store checkpoint, snapshot, decision journal, and unresolved-decision
  behavior

## Boundary Evidence

Current callers import helpers through `super::support::{...}` from test
modules:

- `pipeline_basic_tests/mod.rs`
- `pipeline_decision_tests/mod.rs`
- `pipeline_integrity_tests.rs`

The nested pipeline test modules use fixture builders defined in their local
`mod.rs` files, not directly from `support.rs`.

The split can preserve all current imports by replacing `support.rs` with a
`support/` module front door that re-exports the same `pub(super)` helpers.

## Behavior Evidence

Existing migration-core tests rely on:

- source extract call counting
- plugin normalize call counting
- transform failure injection
- semantic verification failure injection
- decision invalidation injection
- deterministic decision fingerprinting
- resolver call counting
- in-memory checkpoint ordering
- latest resume checkpoint lookup
- decision journal append/latest/history lookup
- unresolved decision recording
- stage snapshot write/read behavior

Baseline validation:

- `cargo test -p underlay-migration-core --all-features`
- 43 unit tests passed
- 0 doc-tests

## Decision

Queue `g06.128` as a migration-core test support internal split.

Suggested module shape:

- `tests/support/mod.rs`: front door and re-exports
- `tests/support/fingerprint.rs`: deterministic decision fingerprint helper
- `tests/support/mocks.rs`: `MockSource`, `MockPlugin`,
  `MockDecisionResolver`, and `MockAssetResolver`
- `tests/support/store.rs`: `InMemoryRunStore` and `RunStore` implementation

This is enough to remove the warning-level file while keeping fixture behavior
and caller imports stable.

## Public API Impact

Expected impact: none.

This is test-only support. If preserving tests requires changing production
migration behavior, stop and re-enter planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-migration-core --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
