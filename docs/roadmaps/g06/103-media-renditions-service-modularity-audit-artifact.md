# g06.103 Artifact - Media Renditions Service Modularity Audit

## Summary

`underlay-media/src/renditions/service.rs` is the largest remaining Rust
warning-level production file. It combines service construction, config/key
accessors, direct thumbnail/preview generation, raw-byte generation, rendition
blob deletion, legacy key-prefix version rendition generation, standardized
key generation, repository persistence, and clone behavior in one file.

The current service surface groups:

- `RenditionService<B>`
- `new()`
- `with_defaults()`
- `with_key_generator()`
- `config()`
- `key_generator()`
- `generate_thumbnail()` and `generate_thumbnail_object_key()`
- `generate_preview()` and `generate_preview_object_key()`
- `generate_from_bytes()` and `generate_from_bytes_object_key()`
- `delete_version_renditions()`
- `delete_rendition_blob()` and `delete_rendition_blob_object_key()`
- `generate_version_renditions()`
- `generate_renditions_for_version()`
- `Clone`

## Public Surface Evidence

The public export is:

- `underlay_media::renditions::RenditionService`

The module front door also exports:

- `RenditionConfig`
- `RenditionResult`

Docs and call sites reference the service type and methods:

- `underlay-media/src/renditions/service.rs` doc example
- `docs/contracts/040-storage-blob-and-media-systems.md` as the media
  rendition lifecycle seam

## Behavior Evidence

The focused media crate validation covers broad media behavior but does not
directly exercise `RenditionService` generation methods:

- `cargo test -p underlay-media --all-features`
- 56 unit tests passed
- 5 doc-tests passed
- 6 doc-tests ignored, including the `RenditionService` example

The service split should therefore be conservative: preserve method names,
signatures, object-key parsing, generated key semantics, warning behavior when
generation/deletion fails, repository persistence input fields, and storage
provider/bucket metadata.

## Decision

Queue `g06.104` as a media renditions service internal split.

The split should preserve:

- `underlay_media::renditions::RenditionService`
- all public service methods and signatures
- `RenditionService` clone behavior
- string-key wrappers that parse through `parse_rendition_result_key()`
- typed `BlobObjectKey` methods
- thumbnail and preview config values
- legacy `generate_version_renditions()` behavior
- standardized `generate_renditions_for_version()` behavior
- delete warning behavior and delete count behavior
- repository `CreateRenditionInput` field values

Suggested module shape:

- `service.rs`: service front door and public re-exports
- `service/core.rs`: `RenditionService` type, construction, accessors, and
  `Clone`
- `service/generate.rs`: direct thumbnail, preview, and raw-byte generation
- `service/delete.rs`: single-rendition and version-rendition deletion
- `service/version.rs`: legacy and standardized version rendition generation

## Public API Impact

Expected impact: none.

This should be an internal split. If preserving service methods or generated
rendition semantics forces a public API change, stop and re-enter planning.

## Validation

- `cargo test -p underlay-media --all-features`

Next code batch validation:

- `cargo test -p underlay-media --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
