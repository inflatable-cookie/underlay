# g06.136 Artifact - Media Storage Internal Split

## Summary

Media storage key generation now lives under a focused `storage/` module
directory instead of one large `storage.rs` file.

Changed files:

- `underlay-media/src/storage.rs` removed
- `underlay-media/src/storage/mod.rs`
- `underlay-media/src/storage/config.rs`
- `underlay-media/src/storage/generator.rs`
- `underlay-media/src/storage/filename.rs`

## Module Shape

- `mod.rs`: storage front door, public re-exports, default convenience
  functions, module docs, and test module declaration
- `config.rs`: `StorageKeyConfig` and its default/builder methods
- `generator.rs`: `StorageKeyGenerator`, string key generation, typed-ID
  helpers, object-key validation wrappers, rendition type mapping, and prefix
  helpers
- `filename.rs`: `version_filename(...)` and `mime_to_extension(...)`

The public `underlay_media::storage::{...}` path remains stable.

## Behavior Preserved

The split keeps existing storage behavior:

- default base prefix `media`
- default versions directory `versions`
- default renditions directory `renditions`
- default rendition extension `jpg`
- version key format
- rendition key format
- typed media/version ID helpers
- `BlobObjectKey::parse(...)` validation wrappers
- `RenditionType::Thumbnail` to `thumb`
- `RenditionType::Preview` to `preview`
- custom rendition names
- media/version/rendition prefix formats
- original filename passthrough
- MIME fallback filename generation
- MIME type to extension mapping

## Validation

Passed:

- `cargo test -p underlay-media storage --all-features`
  - 12 focused storage/export tests passed
- `cargo test -p underlay-media --all-features`
  - 56 unit tests passed
  - 5 doc-tests passed
  - 6 doc-tests ignored
- `effigy rust:check`

Known backlog:

- `effigy doctor` still fails on the existing structural scan backlog:
  attention markers, comment ratio, and god-files.
- God-file findings dropped from 21 to 20 after this split.
- The next Rust production warning-level target is
  `underlay-migration-core/src/pipeline/orchestrator/decide.rs`.

## Public API Impact

None.

This was an internal module split. No public storage API, generated key format,
directory default, object-key validation behavior, MIME mapping, or consumer
import path changed.
