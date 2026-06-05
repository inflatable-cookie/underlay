# g06.006 - Media Repository Contract And Adapter Split Completion

## Why

The Rust audit split the large Postgres media adapter into operation modules and
routed dynamic table construction through the DB identifier boundary.

That improved local reasoning, but the media crate still needs a contract pass
that separates app-facing repository behavior from Postgres adapter internals.

## Goal

Freeze the media repository contract, prove the Postgres module split did not
leak new public internals, and record any remaining adapter cleanup before
release closeout.

## Scope

In scope:

- audit `underlay-media` public exports against `040` and `050`
- verify `MediaRepository` and `MediaRepositoryExt` remain the app-facing seam
- keep Postgres operation modules private
- classify table config and storage-key helpers as stable, adapter, or
  candidate-type surfaces
- run targeted media tests and Rust checks for the media crate

Out of scope:

- redesigning media domain records
- changing consumer media schemas
- changing blob adapter behavior
- moving Nightfire-specific media sync into app-local code

## Contract References

- `040`: storage, blob, and media systems
- `050`: media library and usage
- `023`: release and compatibility rollout
- `122`: Rust public API inventory

## Consumer Upgrade Impact

Impact classification: `additive` / `internal`.

Any public media trait or schema behavior change is `breaking` and needs fresh
consumer proof before landing.

## Acceptance Criteria

- public media exports are inventoried
- Postgres operation modules stay private
- adapter table config uses the DB identifier boundary
- remaining raw table config or storage-key string surfaces are classified
- targeted media crate validation passes

## Public Surface Inventory

Stable app-facing surface:

- domain IDs, entities, inputs, and usage edge types from `domain`
- `MediaRepository`
- `MediaRepositoryExt`
- `MediaUsageSyncRepository`
- `MigrationAttachmentBindingRepository`
- structured content media sync traits and helpers
- storage key helpers: `version_key`, `rendition_key`, `version_filename`,
  `mime_to_extension`, `StorageKeyConfig`, and `StorageKeyGenerator`

Adapter surface:

- `PostgresMediaRepository`
- `PostgresMediaConfig`
- optional rendition service types behind the `renditions` feature
- optional Nightfire media extraction types behind the `nightfire` feature

Private adapter internals:

- `postgres/list_query.rs`
- `postgres/media_ops.rs`
- `postgres/rendition_ops.rs`
- `postgres/usage_ops.rs`
- `postgres/version_ops.rs`
- `postgres_rows.rs`

Classification:

- `MediaRepository` and `MediaRepositoryExt` remain the app-facing seam.
- Postgres operation modules did not become public exports.
- `PostgresMediaConfig` remains a public adapter surface with raw fields for
  compatibility.
- `PostgresMediaConfig::try_with_schema` and `try_with_tables` are the new
  early-validation path for schema/table identifiers.
- string-returning storage key helpers remain stable for consumers today, but
  are a candidate-type surface for later alignment with `BlobObjectKey`.

## Consumer Proof

Current consumer scan found:

- `underlay-reference`, `contact-patch`, `compli-me`, `songsprout`, and
  `loophole/composer` use shared storage key helpers for media upload paths
- `underlay-reference` and `contact-patch` also use `rendition_key` in media job
  handlers
- no scanned consumer imports new Postgres operation modules
- no scanned consumer depends directly on `underlay_media::PostgresMediaConfig`
  or `PostgresMediaRepository`
- `acowtancy/farmyard` owns an app-local `DbMediaRepository`; this batch does
  not migrate that repository

## Code Changes

- Added `PostgresMediaConfig::try_with_schema`.
- Added `PostgresMediaConfig::try_with_tables`.
- Added table-config tests for accepted and rejected schema/table names.
- Extended `040` with the media repository/adapter boundary classification.

## Validation

- `cargo test -p underlay-media --features postgres storage`
- `cargo test -p underlay-media --features postgres try_with`
- `cargo test -p underlay-media --features postgres builds_`
- `cargo test -p underlay-media --features postgres rejects_invalid_configured_table_names`
- `cargo clippy -p underlay-media --all-features --all-targets -- -D warnings`
- `git diff --check`

## Current State

`g06.006` is complete.

## Next Task

Execute `g06.007`: devtools bundle/store boundary isolation.
