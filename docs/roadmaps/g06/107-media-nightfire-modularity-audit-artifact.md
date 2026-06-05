# g06.107 Artifact - Media Nightfire Modularity Audit

## Summary

`underlay-media/src/nightfire.rs` is the largest remaining Rust warning-level
production file after `g06.106`. It combines the module front door, public
registry exports, Nightfire media-usage resolution, visit context, field-name
matching, field-rule construction, generic extractor construction, sync
composition, owner-field validation, and root walking entry points.

Existing child modules already cover:

- `nightfire/registry.rs`: block media handler traits, registrations, handler
  map, and block-registration adaptation
- `nightfire/walk.rs`: recursive field and block-handler walking internals,
  anchors, locator creation, pointer helpers, and nested value parsing

The remaining front file groups:

- `NightfireMediaReferenceMatch`
- `NightfireMediaVisitContext`
- `resolve_nightfire_media_usage()`
- `NightfireMediaReferenceMatcher`
- `NightfireMediaFieldRule`
- `NightfireFieldNameMatcher`
- `NightfireMediaUsageExtractor`
- `NightfireBlockMediaUsageExtractor`
- `extract_and_sync()` for both extractor families
- `StructuredContentMediaExtractor<NightfireValue>` implementations
- `StructuredContentWalker<NightfireValue>` implementations

## Public Surface Evidence

The public Nightfire module is feature-gated at `underlay_media::nightfire`.
Tests and consumers can import:

- block registry types re-exported from `registry.rs`
- `NightfireFieldNameMatcher`
- `NightfireMediaUsageExtractor`
- `NightfireBlockMediaUsageExtractor`
- `resolve_nightfire_media_usage()`

Existing tests cover:

- field-name extraction and common field defaults
- block-id and path locator resolution
- block-handler registry extraction
- nested Nightfire walking
- sync through `sync_media_usages_for_record()`
- persisted-owner validation

## Behavior Evidence

Baseline validation:

- `cargo test -p underlay-media --all-features`
- 56 unit tests passed
- 5 doc-tests passed
- 6 doc-tests ignored

The split should preserve module exports, locator behavior, owner-field
mismatch errors, persisted-owner validation, common field names, handler-map
registration behavior, recursive walk semantics, and sync report behavior.

## Decision

Queue `g06.108` as a media Nightfire internal split.

Suggested module shape:

- `nightfire.rs`: small module front door, child module declarations, public
  re-exports, and test module
- `nightfire/context.rs`: `NightfireMediaVisitContext`
- `nightfire/resolver.rs`: `resolve_nightfire_media_usage()`
- `nightfire/matcher.rs`: `NightfireMediaReferenceMatch`,
  `NightfireMediaReferenceMatcher`, `NightfireMediaFieldRule`, and
  `NightfireFieldNameMatcher`
- `nightfire/extractor.rs`: `NightfireMediaUsageExtractor`,
  `NightfireBlockMediaUsageExtractor`, sync methods, extractor trait impls,
  and walker trait impls
- keep `nightfire/registry.rs` and `nightfire/walk.rs` as existing focused
  modules

## Public API Impact

Expected impact: none.

This should be an internal split. If preserving Nightfire exports or extraction
semantics forces a public API change, stop and re-enter planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-media --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
