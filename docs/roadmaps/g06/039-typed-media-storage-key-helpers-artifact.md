# g06.039 Artifact - Typed Media Storage Key Helpers

## Summary

Added feature-gated typed media object-key generation without changing the blob
adapter trait or persisted database values.

`underlay-media::storage` now has `BlobObjectKey` helpers behind the
`object-keys` feature. Existing string helpers remain stable.

## Underlay Changes

`underlay-media`:

- added `object-keys = ["underlay-blob"]`
- made `renditions` include `object-keys`
- added typed `StorageKeyGenerator` methods:
  - `version_object_key`
  - `version_object_key_typed`
  - `rendition_object_key`
  - `rendition_object_key_typed`
  - `rendition_object_key_for_type`
- added free helpers:
  - `version_object_key`
  - `rendition_object_key`
- added storage tests proving typed keys match string keys
- added rejection tests for unsafe generated key components

## Consumer Rollout

Updated current upload/rendition generation paths in:

- `underlay-reference/acme-api`
- `contact-patch/cp-api`
- `compli-me/api`
- `acowtancy/farmyard`
- `songsprout/nursery`
- `loophole/composer/composer-api`

The rollout moved practical upload initiation paths to
`UploadRequest::from_object_key`. Adapter and database seams continue to receive
`object_key.as_str()` because `BlobAdapter` and repository/database APIs still
accept raw strings.

Reference/contact media job thumbnail key generation now uses the typed
rendition helper before writing derived objects.

## Compatibility

Underlay impact: additive.

Consumer impact: small source update where apps opt into the new typed helper.
Persisted object-key strings are unchanged.

No `BlobAdapter` trait signatures changed in this batch.

## Remaining Decision

The next meaningful question is whether to add typed adapter convenience methods
or change any blob adapter signatures. That requires a separate decision because
raw adapter methods also cover database-loaded keys and non-generated object
paths.

## Validation

Validation passed:

- `cargo test -p underlay-media storage --features object-keys`
- `cargo test -p underlay-media storage`
- `effigy rust:check`
- six consumer `cargo check --workspace` checks
