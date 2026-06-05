# g06.043 Artifact - Typed Media Domain Object-Key Fields

## Summary

Moved the shared Underlay media domain/input object-key fields to
`BlobObjectKey`.

The default Postgres adapter now parses stored object-key strings during row
mapping. SQL/database columns remain text, and persisted values are unchanged.

## Underlay Changes

`underlay-media`:

- made `underlay-blob` a normal dependency
- re-exported `BlobObjectKey` and `BlobObjectKeyError`
- changed shared object-key fields to typed values:
  - `MediaSummary.thumbnail_object_key: Option<BlobObjectKey>`
  - `MediaVersion.object_key: Option<BlobObjectKey>`
  - `MediaRendition.object_key: BlobObjectKey`
  - `FinalizeUploadInput.object_key: BlobObjectKey`
  - `CreateRenditionInput.object_key: BlobObjectKey`
- promoted typed storage-key helpers to always-on because `BlobObjectKey` is now
  part of the normal media surface

`underlay-media-postgres`:

- parses stored object-key strings during row conversion
- returns repository errors for invalid stored keys
- binds typed keys to SQL with `as_str()`

`underlay-media::renditions`:

- uses typed rendition keys when creating repository inputs
- uses typed blob adapter extension methods where practical

## Consumer Rollout

The six consumer workspaces compiled without source changes after the Underlay
domain update.

Reason: the current consumers mostly use app-local media DB row and DTO models
for stored media keys. Those local models still expose object keys as strings.

This is acceptable for `g06.043` because the shared Underlay default repository
boundary is now typed, but it leaves a known consumer-local follow-up.

## Compatibility

Underlay impact: breaking public Rust source change for callers that directly
construct or destructure shared `underlay-media` domain/input rows.

Observed six-consumer impact: no source changes required in this batch.

Persisted object-key values did not change.

## Residual Gap

Consumer app-local media row models still load stored object keys as raw
strings. They should adopt a typed parse boundary in their own DB/domain layer
or move onto the shared Underlay media repository shapes where practical.

## Validation

Validation passed:

- `cargo check -p underlay-media -p underlay-media-postgres --all-features`
- `cargo test -p underlay-media domain`
- `cargo test -p underlay-media storage`
- six consumer `cargo check --workspace` checks
