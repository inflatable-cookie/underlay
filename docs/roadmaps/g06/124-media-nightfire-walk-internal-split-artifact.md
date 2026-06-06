# g06.124 Artifact - Media Nightfire Walk Internal Split

## Summary

The Nightfire media walk implementation now lives under a focused
`nightfire/walk/` module directory instead of one large `walk.rs` file.

Changed files:

- `underlay-media/src/nightfire/walk.rs` removed
- `underlay-media/src/nightfire/walk/mod.rs`
- `underlay-media/src/nightfire/walk/anchor.rs`
- `underlay-media/src/nightfire/walk/pointer.rs`
- `underlay-media/src/nightfire/walk/nested.rs`
- `underlay-media/src/nightfire/walk/field_matcher.rs`
- `underlay-media/src/nightfire/walk/registry.rs`

## Module Shape

- `mod.rs`: private module front door and crate-internal re-exports
- `anchor.rs`: `BlockAnchor` and locator construction
- `pointer.rs`: pointer escaping, normalization, and rooted pointer joining
- `nested.rs`: nested `BlockData` and nested `NightfireValue` detection
- `field_matcher.rs`: field-name matcher traversal
- `registry.rs`: registry-backed handler traversal and declared nested value
  traversal

The existing crate-internal surface remains available to `nightfire/context.rs`
and `nightfire/extractor.rs`:

- `BlockAnchor`
- `normalize_relative_pointer`

Moved helpers use `pub(in crate::nightfire)` only where sibling Nightfire
modules need access.

## Behavior Preserved

The split keeps existing Nightfire media behavior:

- block-id locators for top-level and nested block IDs
- ancestor block-id fallback for nested blocks without IDs
- path fallback for top-level blocks without IDs
- registry handler extraction
- declared nested Nightfire value traversal
- implicit nested block traversal
- resolver compatibility for stored locator keys

## Validation

Passed:

- `cargo test -p underlay-media --features nightfire`
  - 52 unit tests passed
  - 5 doc-tests passed
  - 5 doc-tests ignored
- `effigy rust:check`

Known backlog:

- `effigy doctor` still fails on the existing structural scan backlog:
  attention markers, comment ratio, and god-files.
- God-file findings dropped from 27 to 26 after this split.
- The next Rust production warning-level target is
  `underlay-migration-core/src/verification.rs`.

## Public API Impact

None.

This was an internal module split. No public Nightfire media extraction API,
locator format, resolver behavior, storage behavior, or consumer import path
changed.
*** Add File: docs/roadmaps/g06/125-migration-core-verification-modularity-audit.md
# g06.125 - Migration-Core Verification Modularity Audit

## Why

After `g06.124`, the next Rust production warning-level file in the god-file
report is `underlay-migration-core/src/verification.rs`.

Migration verification is part of the migration safety boundary. It should be
split from evidence about verification rule families, execution behavior, error
reporting, and public model impact, not from file size alone.

## Goal

Classify the migration-core verification surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-migration-core/src/verification.rs` by responsibility family
- identify verification models, rule execution, result/error reporting, helper,
  and test boundaries
- identify public API or migration behavior that must remain stable
- decide whether the next batch should split internal modules, extract helper
  files, or defer behind a broader migration verification checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing migration public APIs
- changing migration verification semantics
- changing bundle or pipeline behavior
- changing consumer apps

## Acceptance Criteria

- verification responsibilities are grouped by stable behavior family
- public API and migration behavior boundaries are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a production-code structure audit. If the audit finds verification
behavior that must change, stop and re-enter planning.

## Current State

`g06.125` is ready.

## Next Task

Execute `g06.125`: migration-core verification modularity audit.
