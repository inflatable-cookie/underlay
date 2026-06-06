# g06.193 Artifact - Blob Adapter Raw Trait Boundary Closeout

Status: complete
Owner: repo maintainers
Completed: 2026-06-06

## Purpose

Close the remaining `BlobAdapter` raw-key question.

Previous `g06` batches moved generated file keys, upload requests, download
requests, and media storage config to typed validation. The remaining raw
surface is the core adapter trait:

- `finalise_upload(&str)`
- `public_url(&str)`
- `delete(&str)`
- `head(&str)`
- `get_bytes(&str)`
- `put_bytes(&str, ...)`
- `exists(&str)`

## Result

The raw trait methods are retained intentionally.

They are the compatibility boundary for:

- adapter implementations that bridge SDK/filesystem APIs
- database-loaded object keys used by media listing/detail DTOs
- serialized metadata and stored-object DTOs
- tests and no-op/in-memory adapters
- migration and replay tooling that works from stored bundle paths

The preferred app-facing typed surfaces remain:

- `BlobObjectKey`
- `UploadRequest::from_object_key`
- `UploadRequest::parse_key`
- `DownloadRequest::from_object_key`
- `DownloadRequest::parse_key`
- `BlobAdapterObjectKeyExt`

## Consumer Upgrade Impact

Impact class: `none`.

No code changed. The six-consumer scan showed public URL rendering still passes
database-loaded strings in every media consumer family, while generated write
paths already use typed request/key surfaces.

## Validation

- source scan for raw `BlobAdapter` method usage across Underlay and the six
  consumer repos
- `effigy qa:docs`
- `effigy qa:northstar`

## Next Task

Continue the `122` audit outside blob/media, starting with remaining
candidate-type config surfaces such as concrete adapter configs or operational
HTTP config.
