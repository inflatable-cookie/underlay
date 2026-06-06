# g06.128 - Migration-Core Test Support Internal Split

## Why

`g06.127` found that `underlay-migration-core/src/tests/support.rs` mixes
fingerprint helpers, mock migration components, and in-memory run-store support
in one shared test file.

The next split should reduce test fixture reasoning load while preserving
existing test helper imports and behavior.

## Goal

Split migration-core test support into focused internal test helper modules
without changing production migration behavior.

## Scope

In scope:

- replace `tests/support.rs` with a `tests/support/` module directory
- keep current `super::support::{...}` imports working
- move deterministic fingerprinting into `fingerprint.rs`
- move mock source/plugin/decision/asset components into `mocks.rs`
- move `InMemoryRunStore` into `store.rs`
- preserve existing migration-core tests

Out of scope:

- changing migration public APIs
- changing production migration behavior
- changing test assertions unrelated to support structure
- changing consumer apps

## Acceptance Criteria

- the old oversized support file is replaced by focused test helper modules
- current test helper imports remain stable
- migration-core tests pass
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is a test-only split. If production behavior must change, stop and
re-enter planning.

## Current State

`g06.128` is complete.

Artifact:

- [128 artifact](./128-migration-core-test-support-internal-split-artifact.md)

## Next Task

Execute `g06.129`: HTTP context modularity audit.
