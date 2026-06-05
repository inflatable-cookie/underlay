# g06.048 Artifact - Post-Blob-Key Rust Quality Checkpoint

## Summary

The Rust surface is materially cleaner than it was at the start of `g06`.

The object-key lane closed the highest-value live runtime gap: media storage,
stored media rows, rendition generation, and the one identified non-media
consumer runtime path now parse object keys before blob IO. The remaining raw
object-key strings are mostly compatibility trait inputs, DTO/SQL edges,
tests, metadata payloads, and migration/replay tooling.

The next reference-grade batch should not be another broad blob sweep. It
should tighten the tooling boundary that still mixes public devtools options,
local OCI store path handling, migration bundle refs, and media shard mapping.

## Evidence

- `effigy doctor` still fails only on the known structural scan backlog:
  `scan.attention-markers`, `scan.comment-ratio`, and `scan.god-files`.
- The largest Rust files remain concentrated in migration-core, media,
  devtools, HTTP helpers, and adapter tests.
- `underlay-blob` now exposes `BlobObjectKey`, typed request constructors, and
  `BlobAdapterObjectKeyExt`; the raw `BlobAdapter` trait remains a deliberate
  compatibility boundary.
- `underlay-media` now stores root media object-key fields as `BlobObjectKey`
  and returns typed rendition object keys.
- `underlay-media-postgres` parses stored object-key strings at row-mapping
  edges.
- `underlay-devtools` already has `MigrationBundleRef`, but bundle options,
  local-store paths, media directory paths, and media-shard mapping payloads
  still carry raw strings or `PathBuf` values through the public tooling
  surface.

## Current Quality State

Modularity:

- Runtime contract crates and concrete Postgres adapter crates are now better
  separated: media, jobs, and auth runtime adapters no longer sit inside the
  core contract crates.
- `underlay-media` is still broad, but the remaining breadth is mostly
  domain/helper breadth rather than concrete SQL adapter leakage.
- `underlay-devtools` remains the least clean boundary. It mixes CLI-facing
  option structs, local OCI store mechanics, bundle packaging, media-shard
  payload construction, and replay helpers in one public crate.

Public API shape:

- Typed SQL identifier and blob object-key wrappers are now the preferred
  construction model.
- Raw values remain where they are structurally justified: serialization,
  database rows, adapter compatibility, tests, and historical migration data.
- `underlay-devtools` is the next public API risk because its typed bundle ref
  exists beside looser path/store/ref inputs.

Security boundaries:

- The live blob runtime now rejects invalid object keys before storage access
  in the covered Underlay and consumer paths.
- Dynamic SQL identifier use has already moved behind typed helpers.
- The next security-adjacent concern is filesystem and local-store path
  handling in devtools. Tooling is lower risk than runtime, but it still writes
  bundle blobs, ref mappings, pulled outputs, and media shard artifacts.

Extensibility:

- Consuming apps can now extend runtime storage/media behavior with clearer
  typed seams.
- Adapter crates give apps concrete backends without forcing SQL-specific
  behavior into contract crates.
- Devtools should get the same treatment: keep app runtime independent from
  migration tooling, and make bundle/local-store concepts explicit enough that
  future consumers can use them without copying raw path rules.

## Known Backlog Versus New Risk

Known backlog:

- Effigy doctor structural findings remain: attention markers, comment ratio,
  and god-files.
- Several large Rust modules still exceed the preferred size threshold.
- `underlay-migration-core` is intentionally broad and should be split only
  when the public model can stay coherent.

New or current risk:

- No new runtime object-key security issue was found after `g06.047`.
- The remaining actionable risk is devtools boundary clarity, especially
  local-store paths and migration media shard object-key mapping.

## Decision

Queue `g06.049` as a devtools migration-bundle boundary split.

This is the right next batch because it is:

- security-adjacent without being a production runtime emergency
- directly named by the Rust public API inventory
- aligned with the reference-grade architecture goal
- bounded enough to execute without a generation rollover
- likely to reduce god-file pressure in the same pass

## Validation

Completed for this checkpoint:

- `effigy tasks`
- `effigy doctor` - expected failure on known structural scan backlog
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`

Next batch validation should include:

- `effigy rust:check`
- targeted `cargo test -p underlay-devtools --all-features migration_bundle`
- `effigy qa:docs`
- `effigy qa:northstar`
