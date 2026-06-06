# g06.191 Artifact - Blob Request Object Key Typing

Status: complete
Owner: repo maintainers
Completed: 2026-06-06

## Purpose

Close the remaining raw-string request edge for single-object blob operations.

The `BlobAdapter` trait still keeps raw `&str` methods for compatibility with
database-loaded keys, metadata DTOs, and migration tooling. But
`UploadRequest` and `DownloadRequest` represent a specific object operation, so
they should not be constructible with unchecked raw strings.

## Result

`UploadRequest` now carries `BlobObjectKey` directly.

Retained constructors:

- `UploadRequest::new`
- `UploadRequest::from_object_key`
- `UploadRequest::parse_key`

`DownloadRequest` now carries `BlobObjectKey` directly.

Retained constructors:

- `DownloadRequest::new`
- `DownloadRequest::from_object_key`
- `DownloadRequest::parse_key`

Adapter implementations convert typed keys to strings only at SDK, URL, or
serialized response boundaries.

## Consumer Upgrade Impact

Impact class: `breaking`.

The six-consumer scan found runtime upload paths already used
`UploadRequest::from_object_key`. One Farmyard shared media download route used
`DownloadRequest::new(object_key.as_str())`; it now uses
`DownloadRequest::from_object_key(object_key)`.

Direct callers that build requests from external strings should switch to
`parse_key`. Callers that already hold a validated key should use `new` or
`from_object_key`.

## Validation

- `cargo test -p underlay-blob --all-features`
- `cargo check -p underlay-media --all-features`
- `cargo check -p farmyard-api -p farmyard-migration`
- six-consumer source scan for `UploadRequest::new` and `DownloadRequest::new`

## Next Task

Continue the `122` candidate-type audit with the raw `BlobAdapter` trait
methods, or leave that compatibility boundary in place with an explicit
closeout note if no safer app-facing conversion remains.
