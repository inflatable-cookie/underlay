# g06.045 Artifact - Media Object-Key Boundary Closeout Audit

## Summary

Closed the media object-key boundary lane.

The audit found one remaining Underlay repair candidate:
`underlay-media::renditions` still used raw strings inside rendition generation
and deletion helpers. That service now validates source and target keys before
storage access, exposes typed helper methods, and returns `BlobObjectKey` from
`RenditionResult`.

## Repair

Updated `underlay-media::renditions`:

- `RenditionResult.object_key` is now `BlobObjectKey`
- added typed helpers:
  - `generate_thumbnail_object_key`
  - `generate_preview_object_key`
  - `generate_from_bytes_object_key`
  - `delete_rendition_blob_object_key`
- raw-string public wrappers remain, but parse once before storage access
- generated rendition keys use typed storage-key helpers
- blob calls use `BlobAdapterObjectKeyExt`

## Remaining Raw Strings

Accepted Underlay raw string edges:

- `BlobAdapter` trait methods and backend implementations
- `underlay-media-postgres` private raw SQLx rows
- blob metadata and tests/examples

Accepted consumer raw string edges:

- private raw SQLx rows in app-local media DB modules
- JSON/API DTO object-key fields
- SQL bind parameters and selected text columns
- Farmyard media job raw SQL rows, because they parse before blob access
- Farmyard migration/replay tooling, which is historical and operational
  tooling rather than live media DTO/public URL/delete/download flow
- Farmyard PDF output-object-key flows, which are non-media blob paths

## Result

No known live media DTO/public URL/delete/download path accepts an unparsed
stored media object key.

## Compatibility

Impact: breaking public Rust source change for callers that directly use
`underlay_media::renditions::RenditionResult.object_key` as `String`.

Observed six-consumer impact: no source changes required.

Persisted object-key values and database column types did not change.

## Validation

Validation passed:

- `effigy rust:check`
- `underlay-reference/acme-api`: `cargo check --workspace`
- `contact-patch/cp-api`: `cargo check --workspace`
- `compli-me/api`: `cargo check --workspace`
- `loophole/composer/composer-api`: `cargo check --workspace`
- `songsprout/nursery`: `cargo check --workspace`
- `acowtancy/farmyard`: `cargo check --workspace`

Farmyard still reports its pre-existing unused-function warning in
`farmyard-migration`.

`effigy doctor` still fails on the existing structural backlog:

- `scan.attention-markers`
- `scan.comment-ratio`
- `scan.god-files`

Those findings are outside this lane.

## Residual Gap

Non-media blob paths now need a separate policy decision. Farmyard PDF output
object keys and migration/replay storage paths are examples of real blob-key
flows that are intentionally outside the media-library boundary.
