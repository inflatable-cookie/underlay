# g06.040 Artifact - Blob Adapter Typed Methods Decision

## Summary

Decision: keep `BlobAdapter` method signatures on raw `&str` for now.

Add typed adapter convenience methods in a follow-up batch instead of changing
the core trait.

## Audit Result

`BlobAdapter` currently has raw object-key methods:

- `finalise_upload(&str)`
- `public_url(&str)`
- `delete(&str)`
- `head(&str)`
- `get_bytes(&str)`
- `put_bytes(&str, ...)`
- default `exists(&str)`

Typed request construction already exists:

- `UploadRequest::from_object_key`
- `DownloadRequest::from_object_key`

After `g06.039`, generated upload keys in the six consumers use
`underlay-media::storage::version_object_key` where practical, and reference /
contact thumbnail generation uses `rendition_object_key`.

Remaining raw adapter use falls into three groups:

- generated typed keys that are immediately converted back with `as_str()`
- database-loaded object-key strings used for public URLs, deletion, and stored
  media cleanup
- migration/replay or app-local generated keys, especially in Farmyard

Those groups should not be forced through one breaking trait change.

## Decision

Do not change `BlobAdapter` trait signatures in this generation step.

Add an additive extension trait in `underlay-blob` for typed keys, likely:

- `finalise_upload_object_key(&BlobObjectKey)`
- `public_object_url(&BlobObjectKey)`
- `delete_object_key(&BlobObjectKey)`
- `head_object_key(&BlobObjectKey)`
- `get_object_bytes(&BlobObjectKey)`
- `put_object_bytes(&BlobObjectKey, ...)`
- `exists_object_key(&BlobObjectKey)`

The methods should delegate to the existing raw trait methods. This gives apps a
clear typed path for generated keys without making every database-loaded string
parse immediately.

## Compatibility

Underlay impact for this decision: docs only.

Expected implementation impact: additive.

Changing the core `BlobAdapter` trait would be breaking because current
consumers implement or call the raw methods, and because stored object keys are
still loaded from database columns as strings.

## Consumer Upgrade Shape

The likely follow-up migration is small:

- generated upload finalise paths can replace `object_key.as_str()` adapter
  calls with typed extension methods
- generated rendition write paths can use typed extension methods
- database-loaded keys should remain raw until each repository/DTO path has an
  explicit parse boundary
- migration/replay paths can opt in later where they already build
  `BlobObjectKey`

## Validation

Validation passed:

- `effigy qa:docs`
