# g06.075 Artifact - Media Nightfire Tests Modularity Audit

## Summary

`underlay-media/src/tests/nightfire_tests.rs` is the largest remaining Rust
high-error god-file. It is test-only, but it carries the expected behavior for
Nightfire media extraction, block handler traversal, locator resolution, and
usage sync.

The file currently groups:

- shared `BlockData` fixture construction and field matcher setup
- test block handlers for direct media references and nested popup content
- in-memory `MediaUsageSyncRepository` fixture behavior
- field-name extraction and block locator fallback tests
- registry-backed handler traversal tests
- explicit locator/path media usage resolver tests
- structured content extraction and usage sync tests

## Behavior Evidence

The test file covers these stable contracts:

- media fields in top-level and nested Nightfire blocks resolve to block-id
  locators when block ids exist
- nested blocks without ids fall back to ancestor block pointers
- root-level blocks without ids fall back to rooted JSON paths
- the common field matcher covers default media reference field names
- registry-backed extraction walks declared nested Nightfire values
- nested handler extraction uses the nearest stable outer anchor when child ids
  are absent
- handler maps accept both module-specific and generic block registrations
- `resolve_nightfire_media_usage` reads block-id locators and path fallback
  locators
- `StructuredContentMediaExtractor::extract_and_sync` composes Nightfire
  extraction with the shared media usage sync path
- sync rejects owners without persisted ids

## Decision

Queue `g06.076` as a media Nightfire tests internal split.

The split should preserve:

- all test names or comparably searchable behavior names
- shared handler fixture behavior
- locator fallback assertions
- registry and handler-map coverage
- structured content sync behavior
- existing production code and public APIs

Suggested test module shape:

- `nightfire_tests.rs`: test module front door
- `nightfire_tests/support.rs`: `block`, `matcher`, test handlers, and
  `TestUsageSyncRepository`
- `nightfire_tests/field_extractor.rs`
- `nightfire_tests/registry.rs`
- `nightfire_tests/resolver.rs`
- `nightfire_tests/sync.rs`

## Public API Impact

Expected impact: none.

This should be a test-only split. If production media or Nightfire APIs must
change, stop and re-enter planning.

## Validation

- `cargo test -p underlay-media --all-features nightfire`

Next code batch validation:

- `cargo test -p underlay-media --all-features nightfire`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
