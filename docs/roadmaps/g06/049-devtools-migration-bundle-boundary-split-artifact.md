# g06.049 Artifact - Devtools Migration-Bundle Boundary Split

## Summary

`underlay-devtools` now has a tighter migration-bundle boundary without
changing the persisted bundle JSON format or making devtools a runtime app
contract.

The batch kept public CLI/serialization option structs raw at the edge, then
centralized the internal store/ref/key validation used by migration bundles and
seed bundles.

## Code Changes

- Added an internal `LocalStoreDir` boundary in
  `migration_bundle/local_store.rs`.
- Centralized local-store digest resolution, ref-path construction, blob-path
  construction, and ref sanitization.
- Reused the migration-bundle local-store helpers from seed-bundle pull instead
  of keeping duplicated raw helper logic.
- Tightened `MigrationBundleRef` and remote digest refs to require
  `sha256:` plus 64 hex characters.
- Changed media-shard generation and validation to use
  `underlay_media::storage::version_object_key` and `BlobObjectKey` parsing.
- Sanitized pulled media-shard output filenames derived from bundle
  annotations before joining them under `media-shards/`.

## Security Boundary

The previous output path accepted the `underlay.shard_id` annotation directly
as part of an output filename. A malicious or malformed bundle annotation could
shape a path-like shard id.

The output path now sanitizes the shard id before writing. The regression test
uses a traversal-shaped annotation and proves the file stays under
`media-shards/`.

## Public API Impact

Public struct fields remain source-compatible:

- `BundlePublishOptions.oci_ref`
- `BundlePullOptions.oci_ref`
- `BundleRunOptions.bundle_ref`
- `local_store_dir: Option<PathBuf>` option fields

Behavior is stricter:

- digest-pinned bundle refs must contain a valid SHA-256 digest
- malformed digest refs fail before local-store or registry IO
- media-shard mapping object keys fail validation if they do not parse as
  canonical blob object keys

Consumer scan:

- the six app roots contain `underlay-devtools` dependencies and wrapper
  scripts, but no direct source use of `MigrationBundleRef`,
  `BundleRunOptions`, `BundlePullOptions`, `BundlePublishOptions`, or
  `migration_bundle_*` APIs
- Farmyard owns substantial seed-bundle tooling, but it does not depend on the
  tightened Rust source APIs directly

Impact classification: no source migration for current consumers.

## Known Backlog

Effigy doctor structural backlog remains known and separate:

- attention markers
- comment ratio
- god-files

This batch reduced duplicated local-store logic but did not attempt a broad
devtools or migration-core file-size campaign.

## Validation

- `cargo test -p underlay-devtools --all-features migration_bundle -- --nocapture`
- `cargo test -p underlay-devtools --all-features -- --nocapture`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
