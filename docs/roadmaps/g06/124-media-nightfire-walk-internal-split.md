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

`g06.124` is complete.

Artifact:

- [124 artifact](./124-media-nightfire-walk-internal-split-artifact.md)

## Next Task

Execute `g06.125`: migration-core verification modularity audit.
