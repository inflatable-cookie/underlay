# g06.047 Artifact - Consumer Non-Media Blob Object-Key Adoption Proof

## Summary

Migrated Farmyard's live syllabus-notes PDF blob path to `BlobObjectKey`.

The JSON/API fields and job payload remain string edges, but the runtime now
parses output keys before blob storage access.

## Farmyard Changes

Updated:

- `crates/api/src/routes/admin/learning/modules/management/queries.rs`
- `crates/api/src/routes/admin/learning/modules/management/mutations.rs`
- `crates/jobs/src/tasks/syllabus_notes_pdf.rs`

Behavior:

- user-supplied `output_object_key` values parse as `BlobObjectKey`
- default PDF output keys parse as `BlobObjectKey`
- invalid keys fail with a bad-request API error before enqueue/download
- download checks use `exists_object_key`
- signed download requests use `DownloadRequest::from_object_key`
- PDF upload uses `put_object_bytes`

Migration/replay tooling stayed raw as classified in `g06.046`.

## Compatibility

Impact: breaking source change inside Farmyard only.

No persisted object-key values, database columns, or API JSON field names
changed.

## Validation

Validation passed:

- `acowtancy/farmyard`: `cargo check --workspace`

Farmyard still reports its pre-existing unused-function warning in
`farmyard-migration`.

## Residual Gap

The blob object-key lane is now ready for a broader Rust quality checkpoint
instead of more key-specific migration.
