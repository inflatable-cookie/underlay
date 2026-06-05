# g06.038 Artifact - Blob Object Key Helper Alignment Plan

## Summary

Plan: add typed media storage-key helpers first, then migrate consumers where the
typed path is useful.

Do not change the `BlobAdapter` trait yet. Its raw `&str` methods are broader
than storage-key generation and are used widely for database-loaded object keys,
cleanup, public URLs, and direct bytes operations. Moving the trait itself to
`BlobObjectKey` would be a larger compatibility wave.

## Current Shared Shape

`underlay-blob`:

- exports `BlobObjectKey`
- validates object keys against absolute paths, traversal components,
  backslashes, null bytes, and control characters
- has typed request constructors:
  - `UploadRequest::from_object_key`
  - `DownloadRequest::from_object_key`
- still keeps `BlobAdapter` methods on raw `&str`:
  - `finalise_upload`
  - `public_url`
  - `delete`
  - `head`
  - `get_bytes`
  - `put_bytes`

`underlay-media::storage`:

- always compiles as part of `underlay-media`
- returns `String` from:
  - `StorageKeyGenerator::version_key`
  - `StorageKeyGenerator::version_key_typed`
  - `StorageKeyGenerator::rendition_key`
  - `StorageKeyGenerator::rendition_key_typed`
  - `StorageKeyGenerator::rendition_key_for_type`
  - `version_key`
  - `rendition_key`
- depends on `underlay-blob` only through the `renditions` feature today, so
  adding typed helpers must avoid forcing blob dependencies onto all media users
  unless that is explicit

## Consumer Usage

Current storage-key helper call families:

- `underlay-reference/acme-api`
  - `version_key` in admin media upload
  - `rendition_key` in media jobs
- `contact-patch/cp-api`
  - `version_key` in admin media upload
  - `rendition_key` in media jobs
- `compli-me/api`
  - `version_key` in admin media upload
- `songsprout/nursery`
  - `version_key` in admin media handlers
- `loophole/composer/composer-api`
  - `version_key` in media upload
- `acowtancy/farmyard`
  - `version_key` in admin media upload
  - `StorageKeyGenerator::version_key` in migration/replay code

Current typed object-key request usage:

- no current consumer uses `UploadRequest::from_object_key`
- no current consumer uses `DownloadRequest::from_object_key`
- consumers use `UploadRequest::new`, raw adapter methods, and raw database
  object-key strings

## Decision

Open an additive execution batch:

1. Add a feature-gated typed object-key helper path in `underlay-media`.
2. Keep existing string-returning helpers stable.
3. Add tests proving generated keys parse as `BlobObjectKey`.
4. Migrate active upload call sites to the typed helper plus
   `UploadRequest::from_object_key` where the crate already depends on
   `underlay-blob`.
5. Leave database-loaded object-key strings and `BlobAdapter` trait methods for
   a later, separate compatibility wave.

## Proposed API Shape

Preferred additive shape:

- add an `object-keys` feature to `underlay-media`:
  - `object-keys = ["underlay-blob"]`
  - `renditions = ["object-keys", "tracing"]`
- add `StorageKeyGenerator` methods behind `object-keys`:
  - `version_object_key(...) -> Result<BlobObjectKey, BlobObjectKeyError>`
  - `version_object_key_typed(...) -> Result<BlobObjectKey, BlobObjectKeyError>`
  - `rendition_object_key(...) -> Result<BlobObjectKey, BlobObjectKeyError>`
  - `rendition_object_key_typed(...) -> Result<BlobObjectKey, BlobObjectKeyError>`
  - `rendition_object_key_for_type(...) -> Result<BlobObjectKey, BlobObjectKeyError>`
- add convenience functions behind `object-keys`:
  - `version_object_key(...)`
  - `rendition_object_key(...)`

This keeps the existing no-blob `underlay-media` dependency shape while giving
media/upload crates an explicit typed path.

## Compatibility Classification

Impact for `g06.038`: none, planning only.

Expected impact for `g06.039`: additive in Underlay. Consumer migration should
be small and source-compatible with existing database storage columns because
`BlobObjectKey::as_str()` and `into_string()` preserve the stored string value.

## Contract Updates

Updated:

- `docs/contracts/040-storage-blob-and-media-systems.md`
- `docs/contracts/122-rust-public-api-inventory.md`

## Validation

Validation passed:

- `effigy qa:docs`
