# g06.042 Artifact - Stored Object Key Parse-Boundary Audit

## Summary

Decision: parse stored media object keys at the shared repository/domain
boundary.

Do not parse repeatedly at DTO builders, adapter calls, or public URL closures.
Do not change database column types or persisted values.

## Audit Result

Underlay media domain fields currently remain stringly:

- `MediaSummary.thumbnail_object_key: Option<String>`
- `MediaVersion.object_key: Option<String>`
- `MediaRendition.object_key: String`
- `FinalizeUploadInput.object_key: String`
- `CreateRenditionInput.object_key: String`

The Postgres adapter maps database strings directly into those fields:

- `MediaVersionRow.object_key: Option<String>`
- `MediaRenditionRow.object_key: String`
- summary rows with `thumbnail_object_key: Option<String>`

Consumer usage falls into four families:

- DTO/public URL closures over thumbnail, version, and rendition keys
- delete/purge flows over stored version and rendition keys
- download/signed URL flows over stored version keys
- job/migration flows that load keys from database rows or app-local staging
  data

Generated-key paths are already typed after `g06.039` and `g06.041`. Stored
database-loaded keys are the remaining unsafe boundary.

## Decision

Make the next implementation batch a controlled breaking change:

- move Underlay media domain/input object-key fields to `BlobObjectKey`
- parse database-loaded object-key strings inside `underlay-media-postgres`
  row mapping
- preserve SQL/database columns as text
- convert to `as_str()` / `into_string()` only at SQL bind, JSON DTO, and public
  URL edges
- keep the raw `BlobAdapter` trait unchanged

This is the cleanest reference-grade boundary because an invalid stored key
becomes a repository/data integrity error immediately instead of reaching a URL,
delete, or download path.

## Rejected Options

Parse-at-DTO:

- too late; invalid keys still pass through repository/domain layers
- duplicates parse logic across apps

Parse-at-adapter:

- too late for public URL and DTO surfaces
- would overload adapter methods with data-integrity responsibility

Retain raw fields:

- preserves compatibility but leaves the core media domain stringly
- conflicts with the reference-grade typed-boundary goal

Change database column types:

- unnecessary; object keys are still stored as text values
- would create migration churn without improving the Rust boundary

## Compatibility

Expected implementation impact: breaking source change.

Affected consumers must update DTO builders, URL closures, delete/purge flows,
downloads, jobs, and app-local media row models where they cross between typed
domain keys and strings.

The persisted object-key values do not change.

## Validation

Validation passed:

- `effigy qa:docs`

`effigy doctor` remains red on the existing structural scan backlog:

- `scan.attention-markers`
- `scan.comment-ratio`
- `scan.god-files`
