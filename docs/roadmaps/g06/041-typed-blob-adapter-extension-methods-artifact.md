# g06.041 Artifact - Typed Blob Adapter Extension Methods

## Summary

Added an additive typed adapter extension trait in `underlay-blob` and migrated
generated-key consumer call sites where it fit cleanly.

The core `BlobAdapter` trait still accepts raw `&str`.

## Underlay Changes

`underlay-blob` now exports `BlobAdapterObjectKeyExt`.

Typed methods:

- `finalise_upload_object_key(&BlobObjectKey)`
- `public_object_url(&BlobObjectKey)`
- `delete_object_key(&BlobObjectKey)`
- `head_object_key(&BlobObjectKey)`
- `get_object_bytes(&BlobObjectKey)`
- `put_object_bytes(&BlobObjectKey, ...)`
- `exists_object_key(&BlobObjectKey)`

All methods delegate to the existing raw adapter trait methods.

## Consumer Rollout

Migrated generated-key adapter calls in:

- `underlay-reference/acme-api`
- `contact-patch/cp-api`
- `compli-me/api`
- `acowtancy/farmyard`
- `songsprout/nursery`
- `loophole/composer/composer-api`

Examples:

- upload finalise uses `finalise_upload_object_key`
- upload cleanup uses `delete_object_key`
- magic-byte reads use `get_object_bytes`
- generated thumbnails use `put_object_bytes`

SQL/database binds remain on `object_key.as_str()` because those calls persist
the string value. Public URL and cleanup paths using database-loaded keys remain
on raw adapter methods.

## Compatibility

Underlay impact: additive.

Consumer impact: additive source migration. Existing raw adapter calls remain
valid.

No `BlobAdapter` trait signatures changed.

## Remaining Decision

The next useful boundary is database-loaded object keys. Stored media and
rendition rows still expose object keys as strings, so typed adapter use stops
at generated-key paths.

## Validation

Validation passed:

- `cargo test -p underlay-blob adapter`
- `effigy rust:check`
- six consumer `cargo check --workspace` checks
