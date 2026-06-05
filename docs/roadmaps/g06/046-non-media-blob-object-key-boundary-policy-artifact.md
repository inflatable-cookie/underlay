# g06.046 Artifact - Non-Media Blob Object-Key Boundary Policy

## Summary

Defined the policy for blob object keys outside the media library.

The media object-key lane is closed. Generic blob paths now have a separate
rule instead of being silently forced into the media repository contract.

## Policy

Live runtime paths that create or consume generic non-media blob keys should
parse to `BlobObjectKey` at their domain/request boundary before calling
storage.

Accepted raw string edges:

- `BlobAdapter` trait methods
- backend adapter implementations
- blob metadata DTOs such as returned object keys
- JSON/API request and response fields
- SQL bind/row edges
- tests and examples
- historical migration, bundle, replay, or recovery tooling where the raw
  object key is part of the artifact being inspected

## Audit Result

Underlay:

- `underlay-blob` raw adapter methods remain the compatibility floor
- `BlobAdapterObjectKeyExt` remains the typed convenience layer
- `UploadRequest::from_object_key` and `DownloadRequest::from_object_key`
  remain the typed request constructors
- devtools migration-bundle media shard keys remain raw artifact metadata

Consumers:

- media-library runtime paths are already typed after `g06.044` and `g06.045`
- Farmyard PDF output keys are a live non-media runtime path and should move to
  typed request/boundary handling in a follow-up
- Farmyard migration/replay object-key flows remain raw migration tooling

## Compatibility

Impact: policy-only for Underlay in this batch.

No code changes were required for consumers.

## Validation

Validation passed:

- `effigy qa:docs`
- `effigy qa:northstar`

No Rust code changed in this batch.

## Residual Gap

Farmyard PDF output keys are the only live consumer runtime path identified in
this audit that should adopt `BlobObjectKey` outside the media library.
