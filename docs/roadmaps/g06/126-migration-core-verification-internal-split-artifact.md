# g06.126 Artifact - Migration-Core Verification Internal Split

## Summary

Migration-core verification now lives under a focused `verification/` module
directory instead of one large `verification.rs` file.

Changed files:

- `underlay-migration-core/src/verification.rs` removed
- `underlay-migration-core/src/verification/mod.rs`
- `underlay-migration-core/src/verification/model.rs`
- `underlay-migration-core/src/verification/checksum.rs`
- `underlay-migration-core/src/verification/stage.rs`
- `underlay-migration-core/src/verification/artifact.rs`

## Module Shape

- `mod.rs`: verification front door, public re-exports, and test module
  declaration
- `model.rs`: public verification enums and structs
- `checksum.rs`: `transform_checksum`
- `stage.rs`: `verify_stage` and built-in stage checks
- `artifact.rs`: `build_verification_artifact` and promotion blocker assembly

The root `underlay_migration_core::{...}` exports remain backed by
`crate::verification::{...}`.

## Behavior Preserved

The split keeps existing verification behavior:

- serialized verification model field names
- transform checksum generation
- declarative rule evaluation during `verify_stage`
- decision coverage, unresolved decision, governance, and checksum checks
- plugin semantic verification integration
- promotion blocker strings
- referential-integrity blocker detection from verification issues

## Validation

Passed:

- `cargo test -p underlay-migration-core verification --all-features`
  - 3 focused verification tests passed
- `cargo test -p underlay-migration-core --all-features`
  - 43 unit tests passed
  - 0 doc-tests
- `effigy rust:check`

Known backlog:

- `effigy doctor` still fails on the existing structural scan backlog:
  attention markers, comment ratio, and god-files.
- God-file findings dropped from 26 to 25 after this split.
- The next Rust warning-level target is
  `underlay-migration-core/src/tests/support.rs`.

## Public API Impact

None.

This was an internal module split. No public verification API, serialized
verification artifact field, check code, blocker string, rule-engine behavior,
or consumer import path changed.
*** Add File: docs/roadmaps/g06/127-migration-core-test-support-modularity-audit.md
# g06.127 - Migration-Core Test Support Modularity Audit

## Why

After `g06.126`, the next Rust warning-level file in the god-file report is
`underlay-migration-core/src/tests/support.rs`.

Shared test support affects many migration-core behavior tests. It should be
split from evidence about fixture families and test helper contracts, not from
file size alone.

## Goal

Classify the migration-core test support surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-migration-core/src/tests/support.rs` by fixture and helper
  family
- identify plugin fixture, pipeline fixture, checkpoint, decision, verification,
  and assertion helper boundaries
- identify helper behavior that existing tests rely on
- decide whether the next batch should split internal test modules, extract
  helper files, or defer behind a broader migration-core test cleanup checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing migration public APIs
- changing production migration behavior
- changing test assertions unrelated to support structure
- changing consumer apps

## Acceptance Criteria

- test support responsibilities are grouped by stable fixture/helper family
- test behavior dependencies are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a test-support structure audit. If preserving tests requires changing
production migration behavior, stop and re-enter planning.

## Current State

`g06.127` is ready.

## Next Task

Execute `g06.127`: migration-core test support modularity audit.
