# g06.123 Artifact - Media Nightfire Walk Modularity Audit

## Summary

`underlay-media/src/nightfire/walk.rs` is the next Rust production
warning-level file after `g06.122`. It owns traversal and locator construction
for media references inside `NightfireValue` content.

The current file groups:

- `BlockAnchor` block-id/path locator fallback behavior
- field-name matcher traversal for arbitrary JSON block data
- registry-backed block handler traversal
- declared nested Nightfire value resolution
- implicit nested `BlockData` collection
- JSON pointer normalization, joining, and escaping helpers
- nested block/Nightfire shape detection helpers

## Boundary Evidence

The module is crate-internal through `nightfire.rs`:

- `mod walk;`

Current cross-module users:

- `nightfire/context.rs` uses `normalize_relative_pointer`.
- `nightfire/extractor.rs` uses `BlockAnchor` and calls walk methods through
  `NightfireMediaUsageExtractor` and `NightfireBlockMediaUsageExtractor`.

The split can preserve `super::walk::BlockAnchor` and
`super::walk::normalize_relative_pointer` by replacing `walk.rs` with a
`walk/` module front door that re-exports those crate-internal helpers.

## Behavior Evidence

Existing Nightfire media tests cover:

- block-id locators for top-level and nested block IDs
- ancestor block-id fallback when a nested block lacks an ID
- rooted path fallback when the top-level block lacks an ID
- registry handler traversal
- declared nested Nightfire value traversal
- outer-anchor fallback for nested child values without IDs
- resolver behavior for block-id and path locators
- sync integration over the shared media-usage sync path

Baseline validation:

- `cargo test -p underlay-media --features nightfire`
- 52 unit tests passed
- 5 doc-tests passed
- 5 doc-tests ignored

## Decision

Queue `g06.124` as a media Nightfire walk internal split.

Suggested module shape:

- `nightfire/walk/mod.rs`: module front door, crate-internal re-exports, and
  shared imports
- `nightfire/walk/anchor.rs`: `BlockAnchor` and locator construction
- `nightfire/walk/pointer.rs`: pointer normalization, joining, and escaping
- `nightfire/walk/nested.rs`: nested block/Nightfire detection and collection
- `nightfire/walk/field_matcher.rs`: `NightfireMediaUsageExtractor` traversal
- `nightfire/walk/registry.rs`: registry-backed handler traversal and declared
  nested value resolution

This split keeps the two traversal strategies readable without changing the
public `nightfire` module surface.

## Public API Impact

Expected impact: none.

The walk module is internal. If preserving behavior requires changing exported
Nightfire media extraction APIs, locator formats, or resolver behavior, stop
and re-enter planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-media --features nightfire`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
*** Add File: docs/roadmaps/g06/124-media-nightfire-walk-internal-split.md
# g06.124 - Media Nightfire Walk Internal Split

## Why

`g06.123` found that `underlay-media/src/nightfire/walk.rs` mixes locator
anchor construction, JSON pointer helpers, nested content discovery, field-name
matcher traversal, registry handler traversal, and declared nested Nightfire
value traversal in one module.

The next split should make traversal behavior easier to reason about while
preserving media locator formats and Nightfire extraction behavior.

## Goal

Split the media Nightfire walk module into focused internal modules without
changing the public Nightfire media extraction surface.

## Scope

In scope:

- replace `nightfire/walk.rs` with a `nightfire/walk/` module directory
- keep `BlockAnchor` and `normalize_relative_pointer` available to existing
  crate-internal callers
- move locator anchor behavior into `anchor.rs`
- move pointer normalization, joining, and escaping helpers into `pointer.rs`
- move nested block/Nightfire detection and collection into `nested.rs`
- move field-name matcher traversal into `field_matcher.rs`
- move registry-backed handler traversal and declared nested value traversal
  into `registry.rs`
- preserve existing Nightfire media tests

Out of scope:

- changing media public APIs
- changing Nightfire locator formats
- changing traversal semantics
- changing storage, rendition, or sync behavior
- changing consumer apps

## Acceptance Criteria

- the old oversized walk file is replaced by focused internal modules
- public `nightfire` exports remain stable
- existing Nightfire media behavior coverage remains intact
- media Nightfire tests pass
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal module split. If locator formats or public extraction APIs
must change, stop and re-enter planning.

## Current State

`g06.124` is ready.

## Next Task

Execute `g06.124`: media Nightfire walk internal split.
