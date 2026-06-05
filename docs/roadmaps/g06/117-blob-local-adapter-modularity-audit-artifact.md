# g06.117 Artifact - Blob Local Adapter Modularity Audit

## Summary

`underlay-blob/src/adapters/local.rs` is the largest remaining Rust
warning-level production file after `g06.116`. It combines local adapter
configuration, adapter construction, path validation, file read/write helpers,
empty-parent cleanup, `BlobAdapter` trait methods, content-type guessing, debug
formatting, and tests in one file.

The public surface is feature-gated and re-exported from `underlay_blob`:

- `LocalConfig`
- `LocalAdapter`

The current behavior groups are:

- `LocalConfig::new()`, `bucket()`, and `upload_url_base()`
- `LocalAdapter::new()`
- `path_for_key()`
- development-only `write_file()` and `read_file()`
- path containment and empty parent cleanup
- upload plan creation and finalization
- public/signed URL generation
- delete/head/get/put/health-check adapter behavior
- `validate_local_object_key()`
- `guess_content_type()`
- `Debug` formatting

## Boundary Evidence

Contracts treat the local adapter as a narrow utility seam:

- `docs/contracts/122-rust-public-api-inventory.md`
- `docs/contracts/040-storage-blob-and-media-systems.md`

Existing tests cover:

- write/read/head/delete flow
- unsafe key rejection
- public URL generation
- content-type guessing
- path traversal rejection
- cleanup preserving the base directory
- cleanup stopping at non-empty parents

## Behavior Evidence

Baseline validation:

- `cargo test -p underlay-blob --all-features`
- 20 unit tests passed
- 3 doc-tests passed
- 1 doc-test ignored

The split must preserve path-safety behavior, key validation error mapping,
development-only read/write helpers, cleanup behavior, metadata behavior,
upload plan fields, URL formatting, health-check behavior, and content-type
guessing.

## Decision

Queue `g06.118` as a blob local adapter internal split.

Suggested module shape:

- `adapters/local.rs`: small front door, public re-exports, and test module
- `adapters/local/config.rs`: `LocalConfig`
- `adapters/local/adapter.rs`: `LocalAdapter`, construction, debug, helper
  methods, and `BlobAdapter` impl
- `adapters/local/path.rs`: local object-key validation, path containment, and
  empty-parent cleanup helper behavior
- `adapters/local/mime.rs`: `guess_content_type()`

## Public API Impact

Expected impact: none.

This should be an internal split. If preserving `LocalAdapter`, `LocalConfig`,
or path-safety behavior forces a public API or behavior change, stop and
re-enter planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-blob --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
