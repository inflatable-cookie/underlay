# g06.135 Artifact - Media Storage Modularity Audit

## Summary

`underlay-media/src/storage.rs` is the next Rust production warning-level file
after `g06.134`. It owns the media storage key contract, object-key validation
wrappers, default convenience helpers, key-prefix helpers, and MIME filename
mapping.

The current file groups:

- `StorageKeyConfig` and builder-style configuration methods
- `StorageKeyGenerator` construction and config access
- version key string generation
- version key typed-ID helpers
- version `BlobObjectKey` validation helpers
- rendition key string generation
- rendition key typed-ID helpers
- rendition `BlobObjectKey` validation helpers
- `RenditionType` to storage-name mapping
- media, versions, renditions, and version-renditions prefix helpers
- default free functions for version and rendition keys
- MIME-based version filename generation
- MIME type to extension mapping
- crate-local storage tests

## Boundary Evidence

The public surface is available through `underlay_media::storage::{...}` and is
also exercised by root module export tests.

Public names to preserve:

- `StorageKeyConfig`
- `StorageKeyGenerator`
- `version_key`
- `version_object_key`
- `rendition_key`
- `rendition_object_key`
- `version_filename`
- `mime_to_extension`

Public method behavior to preserve:

- `StorageKeyConfig::default()`
- `StorageKeyConfig::with_prefix(...)`
- `versions_dir(...)`
- `renditions_dir(...)`
- `rendition_extension(...)`
- `StorageKeyGenerator::new(...)`
- `StorageKeyGenerator::with_defaults()`
- `config()`
- `version_key(...)`
- `version_key_typed(...)`
- `version_object_key(...)`
- `version_object_key_typed(...)`
- `rendition_key(...)`
- `rendition_key_typed(...)`
- `rendition_object_key(...)`
- `rendition_object_key_typed(...)`
- `rendition_key_for_type(...)`
- `rendition_object_key_for_type(...)`
- `media_prefix(...)`
- `versions_prefix(...)`
- `renditions_prefix(...)`
- `version_renditions_prefix(...)`

In-repo consumers:

- `underlay-media/src/renditions/service/core.rs` stores a
  `StorageKeyGenerator`
- `underlay-devtools/src/migration_bundle/media_shards.rs` uses
  `underlay_media::storage::version_object_key`
- tests import both the module-level free functions and generator types

## Behavior Evidence

Existing focused tests cover:

- default version key format
- default rendition key format
- string key and `BlobObjectKey` helper parity
- unsafe path component rejection through `BlobObjectKey::parse(...)`
- custom base prefix, directory names, and rendition extension
- `RenditionType` mapping to `thumb`, `preview`, and custom names
- prefix helper formats
- MIME filename fallback behavior
- MIME type to extension mapping
- root-module storage exports

Baseline validation:

- `cargo test -p underlay-media storage --all-features`
- 12 focused storage/export tests passed

## Decision

Queue `g06.136` as a media storage internal split.

Suggested module shape:

- `storage/mod.rs`: public storage front door, re-exports, convenience
  functions, and test module declaration
- `storage/config.rs`: `StorageKeyConfig` and its defaults/builders
- `storage/generator.rs`: `StorageKeyGenerator`, key generation, object-key
  helpers, typed-ID helpers, and prefix helpers
- `storage/filename.rs`: `version_filename(...)` and `mime_to_extension(...)`

This keeps the public `underlay_media::storage::{...}` path stable while
separating storage key construction from filename/MIME policy.

## Public API Impact

Expected impact: none.

If preserving the split requires changing exported names, default key formats,
directory defaults, object-key validation behavior, MIME mappings, typed-ID
helpers, or consumer import paths, stop and re-enter planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-media storage --all-features`
- `cargo test -p underlay-media --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
