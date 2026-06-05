# Contract: Storage Blob and Media Systems

Status: active
Owner: repo maintainers
Depends on: `030-auth-and-session-systems.md`

## Purpose

Define the shared durable-storage boundary Underlay owns for database access,
soft delete, blob storage, media storage orchestration, and media repository
ownership.

This contract covers:

- shared Postgres pool, migration, schema, and DB-diagnostic helpers
- generic value-existence and media-type helpers
- shared soft-delete semantics and trait/macro surface
- blob adapter, upload-plan, download, and object metadata contracts
- shared AWS config used by blob/email backends
- lower-level media repository, storage-key, rendition, and storage lifecycle
  seams

It does not replace the higher-level
[050-media-library-and-usage.md](/Users/tom/Dev/projects/underlay/docs/contracts/050-media-library-and-usage.md).
`050` sits on top of this contract and defines the richer media graph, usage
sync, and migration semantics.

## Sources of Truth

Primary:

- [`rust/crates/underlay-db/src/lib.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-db/src/lib.rs)
- [`rust/crates/underlay-db/src/pool.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-db/src/pool.rs)
- [`rust/crates/underlay-db/src/migrations.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-db/src/migrations.rs)
- [`rust/crates/underlay-db/src/schemas.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-db/src/schemas.rs)
- [`rust/crates/underlay-db/src/existence.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-db/src/existence.rs)
- [`rust/crates/underlay-db/src/media_types.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-db/src/media_types.rs)
- [`rust/crates/underlay-db/src/db_errors.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-db/src/db_errors.rs)
- [`rust/crates/underlay-soft-delete/src/lib.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-soft-delete/src/lib.rs)
- [`rust/crates/underlay-blob/src/lib.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-blob/src/lib.rs)
- [`rust/crates/underlay-blob/src/adapter.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-blob/src/adapter.rs)
- [`rust/crates/underlay-blob/src/types.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-blob/src/types.rs)
- [`rust/crates/underlay-blob/src/config.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-blob/src/config.rs)
- [`rust/crates/underlay-blob/src/adapters/s3.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-blob/src/adapters/s3.rs)
- [`rust/crates/underlay-blob/src/adapters/local.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-blob/src/adapters/local.rs)
- [`rust/crates/underlay-aws/src/lib.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-aws/src/lib.rs)
- [`rust/crates/underlay-media/src/lib.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-media/src/lib.rs)
- [`rust/crates/underlay-media/src/domain.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-media/src/domain.rs)
- [`rust/crates/underlay-media/src/repository.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-media/src/repository.rs)
- [`rust/crates/underlay-media/src/storage.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-media/src/storage.rs)
- [`rust/crates/underlay-media/src/sync.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-media/src/sync.rs)
- [`rust/crates/underlay-media-postgres/src/lib.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-media-postgres/src/lib.rs)
- [`rust/crates/underlay-media/src/error.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-media/src/error.rs)

Supporting:

- [`docs/contracts/050-media-library-and-usage.md`](/Users/tom/Dev/projects/underlay/docs/contracts/050-media-library-and-usage.md)
- [`docs/architecture/010-package-map.md`](/Users/tom/Dev/projects/underlay/docs/architecture/010-package-map.md)

If these diverge, the shared code wins. The older media contract remains a
higher-level authority for the media graph, not for the lower storage seams.

## Contract Goal

Underlay should provide one reusable durable-storage layer with clean seams:

- apps do not re-invent pool, migration, and DB-diagnostic helpers
- storage backends are swappable behind one blob interface
- soft delete has consistent naming and outcome semantics
- media references remain stable even when blobs and versions evolve
- higher-level media graph and migration contracts can depend on a settled lower
  storage boundary

The goal is portability and consistency, not a monolithic storage framework.

## Shared Boundary

### Database bootstrap and diagnostics

`underlay-db` owns the generic Postgres setup and DB helper surface.

Core pieces:

- `DbConfig`
- `create_pool()`
- `run_migrations()`
- `load_migrator_from_dir()`
- `drop_schemas()`
- `DestructiveGuard`
- `validate_schema_name()`
- `describe_db_error()`
- `map_db_error()`
- `map_db_error_ref()`

Rules:

- Underlay owns the reusable sync/type surface and the recommended schema
  template shape for consumers
- consumer apps own the concrete app migrations that instantiate that template
  in their own schema history
- consumer migrations may backfill or reshape local data, but they should not
  invent a different steady-state `media_usage` contract without promoting that
  change back into Underlay

## Consumer template stance

Underlay should provide template-level guidance for system storage/media
sections, even when apps keep ownership of their actual migration files.

For the media lane that means:

- the contract docs define the canonical `media`, `media_version`,
  `media_rendition`, and `media_usage` shape
- Rust and TS shared types define the stable vocabulary
- sync and audit traits define the behavioral contract
- consumer apps translate that into concrete migration files in their own repo

This is the same split used elsewhere in Underlay:

- Underlay owns the shared contract and recommended template
- apps own the actual rollout, migration ordering, and local backfills

The goal is not “copy this SQL file verbatim into every app”. The goal is “all
apps implement the same steady-state storage/media shape unless they are
consciously extending the contract”.

- shared pool creation is Postgres-first and uses explicit pool tuning knobs
- runtime migrator loading is a tooling/dev seam; production binaries should
  usually embed migrations
- destructive schema-drop helpers are guarded and intentionally strict
- database failures map to the stable `infra.db_error` contract instead of
  leaking raw driver text alone

### Generic existence and classification helpers

`underlay-db` also owns generic low-level data helpers:

- `TypedExistsCheck`
- `value_exists_typed` helpers
- `MediaKind`
- `MediaVisibility`
- `MediaVersionState`
- `detect_media_kind_from_mime_type()`

Rules:

- existence checks are shared DB-level validation helpers, not business-rule
  engines
- callers must construct typed schema/table/column identifiers before shared
  helper SQL construction
- media kind/visibility/version-state enums are lower-level shared vocabulary
  reused by the media system

### Soft-delete contract

`underlay-soft-delete` owns shared soft-delete semantics.

Core pieces:

- `DELETED_AT_COLUMN`
- `DELETE_BATCH_ID_COLUMN`
- `DeleteBatchId`
- `SoftDeleteResult`
- `RestoreBatchResult`
- `PurgeBatchResult`
- `RestoreBlocker*` types
- `SoftDeletable`
- `soft_delete*()` and `batch_soft_delete*()`
- restore/purge macros

Rules:

- `deleted_at` is the baseline shared soft-delete naming convention
- `delete_batch_id` is the canonical shared correlation column for entities
  that participate in batch restore/purge semantics through the retained
  soft-delete helpers
- delete batch ids are first-class shared correlation ids
- restore and purge semantics are explicit outcomes, not boolean afterthoughts
- generic helpers cover simple table cases; complex cascades can implement the
  trait manually
- soft delete is a persistence/lifecycle seam, not a UI workflow

Current boundary note:

- the shipped `underlay-media` repository currently only guarantees
  `deleted_at`-based trash semantics and does not yet participate in the full
  retained `delete_batch_id` restore/purge model

### Blob storage adapter contract

`underlay-blob` owns the generic blob-storage boundary.

Core pieces:

- `BlobAdapter`
- `UploadRequest`
- `UploadPlan`
- `DownloadRequest`
- `SignedUrl`
- `ObjectInfo`
- `StoredObject`
- `BlobError`
- `MediaConfig`

Rules:

- client-side uploads begin with `initiate_upload()`
- upload completion is explicit through `finalise_upload()`
- `BlobObjectKey` is the shared validated key type; upload/download request
  constructors accept it, while core adapter methods still accept raw `&str`
  for compatibility with database-loaded keys and app-local generated keys
- typed adapter convenience methods should be additive wrappers over the raw
  trait, not a trait-signature break, until consumer-owned database key loading
  has a typed parse boundary
- public URLs and signed download URLs are separate concepts
- delete is idempotent
- direct `put_bytes()` exists for server-side derived objects and processing
- backend-specific details stay behind the adapter boundary

This contract is the generic storage seam. It does not define media-graph
meaning by itself.

### Backend implementations

Underlay currently ships backend implementations, not just traits:

- `S3Adapter`
- `LocalAdapter`
- `NoopAdapter`
- shared `AwsConfig`

Rules:

- S3-compatible storage is the production-facing backend model
- local filesystem storage is a narrow utility seam and must not be treated as
  the standard browser-facing development backend or as a production backend
- shared AWS config owns region/endpoint setup reused across crates
- public URL construction, presign durations, and path-style mode are backend
  config concerns, not media-graph concerns

### Media repository and lifecycle boundary

`underlay-media` owns the lower-level media repository and storage lifecycle
surface. `underlay-media-postgres` owns the concrete PostgreSQL adapter.

Core pieces:

- `MediaId`, `MediaVersionId`, `MediaRenditionId`
- `Media`, `MediaVersion`, `MediaRendition`, `MediaSummary`
- `CreateMediaInput`, `UpdateMediaInput`, `FinalizeUploadInput`,
  `CreateRenditionInput`, `ListMediaParams`
- `MediaRepository`
- `MediaRepositoryExt`
- `PostgresMediaRepository` in `underlay-media-postgres`
- `PostgresMediaConfig` in `underlay-media-postgres`

Rules:

- `Media` is the stable logical reference
- `MediaVersion` is the immutable content snapshot
- `MediaRendition` is a derived artifact
- soft delete and hard delete are distinct repository operations
- version finalization is explicit and tied to storage metadata
- repository interfaces stay storage-aware but backend-agnostic at the trait
  level
- `MediaRepository` and `MediaRepositoryExt` are the app-facing repository seam
- Postgres operation modules are adapter internals and must stay private in
  `underlay-media-postgres`
- `PostgresMediaConfig` is an adapter surface that stores typed schema/table
  identifiers internally; external config should use `try_with_schema` and
  `try_with_tables`, while `with_schema` is only for known-good literals
- storage-key helpers are stable shared helpers; string helpers remain
  available, and the `object-keys` feature adds additive `BlobObjectKey`
  generation for callers that want typed construction before crossing back into
  raw adapter/database seams
- generated object keys and database-loaded object keys are different
  construction states; callers should parse database-loaded strings before
  using typed convenience methods

This layer owns CRUD and lifecycle mechanics. The richer meaning of usages,
migration bindings, and media graph semantics is defined in `050`.

Current boundary note:

- the shipped `underlay-media-postgres` `PostgresMediaRepository` still
  exposes the older simple usage tracking surface (`track_usage`,
  `sync_usages`) alongside the newer generalized usage-edge traits, so
  higher-level usage-graph behavior is only partially implemented by the
  default backend today

### Storage key contract

`underlay-media::storage` owns the shared object-key-generation seam.

Core pieces:

- `StorageKeyConfig`
- `StorageKeyGenerator`
- `version_key*()`
- `rendition_key*()`
- `version_object_key*()` behind `object-keys`
- `rendition_object_key*()` behind `object-keys`
- prefix helpers

Rules:

- object-key patterns are shared, explicit, and configurable
- typed object-key helpers must produce the same persisted key values as the
  retained string helpers
- typed helper failures are construction failures and should be handled before
  upload initiation
- media ids and version ids are part of the storage path contract
- rendition objects are a sibling derived space, not mixed into original
  version paths
- storage keys should remain provider-independent

### Usage-sync and migration-binding seams

`underlay-media::sync` owns the lower-level sync interfaces that support the
higher-level `050` media contract.

Core pieces:

- `MediaUsageSyncRepository`
- `MigrationAttachmentBindingRepository`
- `StructuredContentMediaExtractor`
- `StructuredContentWalker`
- `MediaUsageAuditSource`
- `sync_media_usages_for_record()`
- `MediaUsageSyncReport`

Rules:

- the owner/provenance sync scope is explicit and validated
- structured-content media extraction stays generic at the seam
- migration attachment binding reuse is a first-class lower-level repository
  concern
- these are support seams for the richer media-graph contract, not a separate
  competing authority

## Relationship To Contract 050

This contract owns the lower storage/media mechanics.

[050-media-library-and-usage.md](/Users/tom/Dev/projects/underlay/docs/contracts/050-media-library-and-usage.md)
owns the higher-level media system semantics:

- usage graph meaning
- migration replay semantics
- owner-field and locator vocabulary
- content-sync expectations

Rule:

- `040` settles the lower durable-storage and repository seams
- `050` should not contradict those seams, but it may add higher-level meaning
  on top of them

## Invariants

- content records should depend on stable media ids, not raw storage URLs
- blob backends are replaceable behind the shared adapter contract
- soft-delete naming and outcome semantics stay stable across apps
- media versions are immutable snapshots; replacement happens by new version
- local filesystem storage remains a development seam only
- higher-level media graph work must build on these lower seams instead of
  bypassing them

## Extension Points

Allowed:

- app-local repository implementations over the shared traits
- backend-specific adapter implementations beyond S3/local/noop
- app-local Postgres schema/table naming through config
- narrower app-local media policies on top of shared repository operations
- higher-level media graph and upload UX work on top of this storage boundary

Not allowed:

- storing raw provider URLs as the durable shared media reference
- treating local-dev storage endpoints as production infrastructure
- inventing parallel soft-delete conventions for shared entities
- bypassing storage-key and repository seams with app-scattered blob logic

## Known Drift And Assessment Hooks

Current drift worth assessing later:

- [050-media-library-and-usage.md](/Users/tom/Dev/projects/underlay/docs/contracts/050-media-library-and-usage.md)
  is active but still labeled `Proposed shared contract`, so the lower/higher
  media authority chain is not yet expressed cleanly in file state
- `underlay-db::TypedExistsCheck` assumes `deleted_at` unless callers opt out,
  which is convenient but may over-assume soft-delete semantics for some tables
- `underlay-media::MediaRepository` still mixes older simple usage methods
  (`track_usage`, `sync_usages`) with the newer generalized usage-edge sync
  surface, so the repository boundary should be reassessed against the richer
  `050` contract
- `underlay_blob::MediaConfig` sits in the blob crate even though it spans file
  limits and thumbnail concerns that touch media/rendition ownership

These are assessment hooks, not reasons to widen the contract.

## Assessment Questions

Use this contract to judge later implementation work:

- does a storage concern belong at the blob/media seam or at the higher media
  graph layer
- are shared repository and adapter seams still generic enough for multiple
  apps and backends
- do shared helpers preserve stable media references across provider changes
- does a proposed abstraction settle real shared storage mechanics or merely
  encode one app’s workflow
- do the lower storage seams and the higher `050` media contract still align

## Next Task

Execute `g04.007`: write `060-jobs-events-and-operator-systems.md`.
