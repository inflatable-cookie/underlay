# g06.190 Artifact - Media Storage File Key Typing

Status: complete
Owner: repo maintainers
Completed: 2026-06-06

## Purpose

Retire the remaining public string-returning media file-key helpers.

Storage object keys are write-path security boundaries. Runtime callers already
used `version_object_key` and `rendition_object_key`, but the public
`underlay_media::storage` surface still exposed raw `String` file-key helpers.

## Result

The public media storage file-key surface now returns validated
`BlobObjectKey` values:

- `version_object_key`
- `rendition_object_key`
- `StorageKeyGenerator::version_object_key`
- `StorageKeyGenerator::version_object_key_typed`
- `StorageKeyGenerator::rendition_object_key`
- `StorageKeyGenerator::rendition_object_key_typed`
- `StorageKeyGenerator::rendition_object_key_for_type`

Public string-returning file-key helpers were retired:

- `version_key`
- `rendition_key`
- `StorageKeyGenerator::version_key`
- `StorageKeyGenerator::version_key_typed`
- `StorageKeyGenerator::rendition_key`
- `StorageKeyGenerator::rendition_key_typed`
- `StorageKeyGenerator::rendition_key_for_type`

Prefix helpers remain string-returning because list/delete prefix operations do
not represent a single blob object key:

- `media_prefix`
- `versions_prefix`
- `renditions_prefix`
- `version_renditions_prefix`

## Consumer Upgrade Impact

Impact class: `breaking`.

The six-consumer scan found one direct consumer use of the retired API:
Farmyard migration replay used `StorageKeyGenerator::version_key`. That path
now calls `underlay_media::storage::version_object_key` and passes
`BlobObjectKey::as_str()` only at the raw `BlobAdapter` trait boundary.

Runtime upload paths across the consumer family were already using typed object
key helpers.

## Validation

- `cargo test -p underlay-media storage --all-features`
- `cargo check -p farmyard-migration`
- six-consumer source scan for retired storage file-key helpers

## Next Task

Continue the `122` candidate-type audit with media storage config validation or
the remaining blob adapter raw trait compatibility boundary.
