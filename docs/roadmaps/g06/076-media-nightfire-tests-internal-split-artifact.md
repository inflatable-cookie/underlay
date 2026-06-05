# g06.076 Artifact - Media Nightfire Tests Internal Split

## Summary

`underlay-media/src/tests/nightfire_tests.rs` is now a small test front door.
The previous monolith was split into focused modules under
`underlay-media/src/tests/nightfire_tests/`.

The split is test-only. It does not change media production code, Nightfire
extraction semantics, public APIs, or consumer apps.

## Module Shape

- `nightfire_tests.rs`: explicit test module front door
- `nightfire_tests/support.rs`: block fixture, field matcher, test block
  handlers, and in-memory usage sync repository
- `nightfire_tests/field_extractor.rs`: field-name extraction and locator
  fallback tests
- `nightfire_tests/registry.rs`: handler registry, nested traversal, and
  handler-map registration tests
- `nightfire_tests/resolver.rs`: block-id and path locator resolution tests
- `nightfire_tests/sync.rs`: structured content extraction and usage sync tests

## Behavior Preserved

- all 12 Nightfire-selected media tests pass
- top-level and nested block-id locator behavior is unchanged
- ancestor and rooted-path fallback behavior is unchanged
- common media field matching coverage is unchanged
- registry-backed nested handler traversal behavior is unchanged
- block-id and path locator resolution behavior is unchanged
- shared `extract_and_sync` behavior and persisted-owner guard remain covered

## Public API Impact

None.

This was a Rust test-structure split only.

## Validation

- `cargo test -p underlay-media --all-features nightfire`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` remains expected-fail on the known backlog; `scan.god-files`
  improved from 51 findings / 10 errors to 50 findings / 9 errors.
