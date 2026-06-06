# g06.192 Artifact - Media Storage Config Validation

Status: complete
Owner: repo maintainers
Completed: 2026-06-06

## Purpose

Close the remaining unchecked media storage key config edge.

`StorageKeyConfig` controlled object-key prefixes and directory components, but
its fields were public `String`s. That let consumers build invalid key formats
that only failed later during object-key generation.

## Result

`StorageKeyConfig` fields are private.

Validated builders now enforce blob object-key rules at the config boundary:

- `StorageKeyConfig::with_prefix`
- `StorageKeyConfig::versions_dir`
- `StorageKeyConfig::renditions_dir`
- `StorageKeyConfig::rendition_extension`

Read-only accessors expose retained values:

- `base_prefix`
- `versions_dir_name`
- `renditions_dir_name`
- `rendition_extension_name`

`StorageKeyGenerator` continues to accept `StorageKeyConfig`, but generated
file keys and prefixes now come from validated config values.

## Consumer Upgrade Impact

Impact class: `breaking`.

The six-consumer scan found no direct consumer construction or field access for
`StorageKeyConfig`. Existing runtime consumers use default storage helpers and
typed object-key generation.

Direct library callers that use custom storage config must handle builder
`Result`s and use accessors instead of fields.

## Validation

- `cargo test -p underlay-media storage --all-features`
- `cargo check -p underlay-media --all-features`
- six-consumer source scan for `StorageKeyConfig` construction and field access

## Next Task

Reassess the raw `BlobAdapter` trait boundary. If it remains justified, close
it explicitly as retained compatibility rather than continuing to churn the
same surface.
