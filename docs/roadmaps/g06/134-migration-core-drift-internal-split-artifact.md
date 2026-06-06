# g06.134 Artifact - Migration-Core Drift Internal Split

## Summary

Migration-core drift detection now lives under a focused `drift/` module
directory instead of one large `drift.rs` file.

Changed files:

- `underlay-migration-core/src/drift.rs` removed
- `underlay-migration-core/src/drift/mod.rs`
- `underlay-migration-core/src/drift/model.rs`
- `underlay-migration-core/src/drift/run.rs`
- `underlay-migration-core/src/drift/lineage.rs`
- `underlay-migration-core/src/drift/summary.rs`

## Module Shape

- `mod.rs`: drift front door, public re-exports, and test module declaration
- `model.rs`: public drift and lineage model types
- `run.rs`: run-report threshold checks and report assembly
- `lineage.rs`: decision index/journal lineage checks
- `summary.rs`: category summary aggregation

The root `underlay_migration_core::{...}` exports remain backed by
`crate::drift::{...}`.

## Behavior Preserved

The split keeps existing drift behavior:

- unresolved decision threshold issues
- governance threshold issues
- verify-stage failure issues
- decision index validation
- expected bundle digest checks
- index entry bundle digest checks
- index-to-journal fingerprint and decision ID checks
- journal-to-index missing fingerprint warnings
- lineage mismatch threshold issue
- category summary aggregation

## Validation

Passed:

- `cargo test -p underlay-migration-core drift --all-features`
  - 2 focused drift tests passed
- `cargo test -p underlay-migration-core --all-features`
  - 43 unit tests passed
  - 0 doc-tests
- `effigy rust:check`

Known backlog:

- `effigy doctor` still fails on the existing structural scan backlog:
  attention markers, comment ratio, and god-files.
- God-file findings dropped from 22 to 21 after this split.
- The next Rust production warning-level target is
  `underlay-media/src/storage.rs`.

## Public API Impact

None.

This was an internal module split. No public drift API, serialized field,
issue code, severity, threshold, behavior, or consumer import path changed.
*** Add File: docs/roadmaps/g06/135-media-storage-modularity-audit.md
# g06.135 - Media Storage Modularity Audit

## Why

After `g06.134`, the next Rust production warning-level file in the god-file
report is `underlay-media/src/storage.rs`.

Media storage key generation is a shared storage boundary. It should be split
from evidence about public API, typed object-key behavior, path safety, and
test coverage, not from file size alone.

## Goal

Classify the media storage surface and decide the safest next structural batch.

## Scope

In scope:

- inspect `underlay-media/src/storage.rs` by responsibility family
- identify public types, key generation, object-key helpers, path validation,
  MIME mapping, and test boundaries
- identify public API or storage behavior that must remain stable
- decide whether the next batch should split internal modules, extract helper
  files, or defer behind a broader media storage checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing media public APIs
- changing stored object key formats
- changing blob adapter behavior
- changing consumer apps

## Acceptance Criteria

- storage responsibilities are grouped by stable behavior family
- public API and object-key behavior boundaries are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a production-code structure audit. If the audit finds storage key
behavior that must change, stop and re-enter planning.

## Current State

`g06.135` is ready.

## Next Task

Execute `g06.135`: media storage modularity audit.
