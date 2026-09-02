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
[050-media-library-and-usage.md](./050-media-library-and-usage.md).
`050` sits on top of this contract and defines the richer media graph, usage
sync, and migration semantics.

## Sources of Truth

Primary:

- [`rust/crates/underlay-db/src/lib.rs`](../../rust/crates/underlay-db/src/lib.rs)
- [`rust/crates/underlay-db/src/pool.rs`](../../rust/crates/underlay-db/src/pool.rs)
- [`rust/crates/underlay-db/src/migrations.rs`](../../rust/crates/underlay-db/src/migrations.rs)
- [`rust/crates/underlay-db/src/schemas.rs`](../../rust/crates/underlay-db/src/schemas.rs)
- [`rust/crates/underlay-db/src/existence.rs`](../../rust/crates/underlay-db/src/existence.rs)
- [`rust/crates/underlay-media/src/types.rs`](../../rust/crates/underlay-media/src/types.rs)
- [`rust/crates/underlay-db/src/db_errors.rs`](../../rust/crates/underlay-db/src/db_errors.rs)
- [`rust/crates/underlay-soft-delete/src/lib.rs`](../../rust/crates/underlay-soft-delete/src/lib.rs)
- [`rust/crates/underlay-blob/src/lib.rs`](../../rust/crates/underlay-blob/src/lib.rs)
- [`rust/crates/underlay-blob/src/adapter.rs`](../../rust/crates/underlay-blob/src/adapter.rs)
- [`rust/crates/underlay-blob/src/types.rs`](../../rust/crates/underlay-blob/src/types.rs)
- [`rust/crates/underlay-blob/src/config.rs`](../../rust/crates/underlay-blob/src/config.rs)
- [`rust/crates/underlay-blob/src/adapters/s3.rs`](../../rust/crates/underlay-blob/src/adapters/s3.rs)
- [`rust/crates/underlay-blob/src/adapters/local.rs`](../../rust/crates/underlay-blob/src/adapters/local.rs)
- [`rust/crates/underlay-aws/src/lib.rs`](../../rust/crates/underlay-aws/src/lib.rs)
- [`rust/crates/underlay-media/src/lib.rs`](../../rust/crates/underlay-media/src/lib.rs)
- [`rust/crates/underlay-media/src/domain.rs`](../../rust/crates/underlay-media/src/domain.rs)
- [`rust/crates/underlay-media/src/repository.rs`](../../rust/crates/underlay-media/src/repository.rs)
- [`rust/crates/underlay-media/src/storage/mod.rs`](../../rust/crates/underlay-media/src/storage/mod.rs)
- [`rust/crates/underlay-media/src/sync.rs`](../../rust/crates/underlay-media/src/sync.rs)
- [`rust/crates/underlay-media-postgres/src/lib.rs`](../../rust/crates/underlay-media-postgres/src/lib.rs)
- [`rust/crates/underlay-media/src/error.rs`](../../rust/crates/underlay-media/src/error.rs)

Supporting:

- [`050-media-library-and-usage.md`](./050-media-library-and-usage.md)
- [`docs/architecture/010-package-map.md`](../architecture/010-package-map.md)

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

The media domain enums (`MediaKind`, `MediaVisibility`, `MediaVersionState`,
`MediaTypeParseError`, `detect_media_kind_from_mime_type`) live in
`underlay-media`, not `underlay-db`, so consumers wanting media types are not
forced to depend on `sqlx`. `underlay-media` no longer depends on
`underlay-db` for them; Postgres binding is handled by the
`underlay-media-postgres` adapter through the enums' string representations.
`MediaKind` is `#[non_exhaustive]` so adding a media kind (video/audio) for a
future consumer is not a breaking change; external `match` sites carry a
wildcard arm.

Rules:

- existence checks are shared DB-level validation helpers, not business-rule
  engines
- callers must construct typed schema/table/column identifiers before shared
  helper SQL construction
- media kind/visibility/version-state enums are shared media vocabulary owned
  by `underlay-media`; import them from there, not from `underlay-db`

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
- `BlobUploadConfig`
- `BlobAdapterPromotionExt`, `VerifiedPromotionResult` (`g11.001`)

Rules:

- client-side uploads begin with `initiate_upload()`
- upload completion is explicit through `finalise_upload()`
- upload enforcement boundary: declared content type and length are
  client-supplied and untrusted. Serving paths use
  `BlobAdapterUploadExt::initiate_upload_validated` (size cap + MIME
  allowlist before signing) and `finalise_upload_verified` (size, allowlist,
  and magic-byte sniff of stored bytes). Client-side allowlists are UX hints
  only
- the default MIME allowlist (`DEFAULT_ALLOWED_CONTENT_TYPES`) excludes
  active content (`image/svg+xml`, `text/html`, `application/javascript`).
  SVG is explicit opt-in via `BlobUploadConfig::with_allowed_content_types`,
  and opted-in scriptable types must be served as attachments
  (`content_disposition_attachment`, RFC 6266 escaped) or from a sandboxed
  origin, never inline same-origin
- blob upload policy is limited to transport/storage concerns such as file-size
  limits; rendition generation policy belongs in
  `underlay_media::renditions::RenditionConfig`
- `BlobObjectKey` is the shared validated key type; upload/download request
  constructors accept it, while core adapter methods still accept raw `&str`
  for compatibility with database-loaded keys and app-local generated keys
- `BlobAdapterObjectKeyExt` provides additive typed convenience methods over
  the raw trait for callers that already hold `BlobObjectKey`
- typed adapter convenience methods are wrappers over the raw trait, not a
  trait-signature break, until consumer-owned database key loading has a typed
  parse boundary
- live runtime paths that create or consume generic non-media blob keys should
  parse to `BlobObjectKey` at their domain/request boundary before calling
  storage
- stored media object keys should parse at the shared repository/domain
  boundary, not repeatedly at DTO or adapter call sites
- shared media domain rows and inputs carry `BlobObjectKey`; JSON DTOs and SQL
  binds convert with `as_str()` or `into_string()` at the edge
- media rendition generation returns typed object keys and validates raw-string
  compatibility wrapper inputs before storage access
- public URLs and signed download URLs are separate concepts
- delete is idempotent
- direct `put_bytes()` exists for server-side derived objects and processing
- raw object-key strings remain acceptable for adapter implementations,
  metadata DTOs, JSON/SQL edges, tests/examples, and historical migration or
  replay tooling where the raw value itself is the artifact under inspection
- backend-specific details stay behind the adapter boundary

### Immutable verified promotion (`g11.001`)

`BlobAdapter` carries two additive, fail-closed-by-default methods:
`get_bytes_bounded()` (reads at most `max_bytes + 1` bytes, never a full
unbounded buffer of an oversized source) and `put_bytes_create_only()`
(creates a destination only if absent, typed `BlobError::DestinationExists`
on collision, never an unconditional overwrite fallback). Existing
implementors keep compiling unchanged; an adapter that does not override
these refuses via `BlobError::Unsupported` rather than silently degrading to
mutable read/write.

`BlobAdapterPromotionExt::promote_verified()` composes those two primitives:
it captures a staging object once under a `BlobUploadConfig` size bound,
validates the captured bytes' size, MIME allowlist membership, and magic
bytes, derives their lowercase SHA-256 server-side, and publishes that exact
vector to a distinct destination key through exclusive create. It returns a
`VerifiedPromotionResult` (destination `StoredObject` plus the derived
SHA-256). Staging is preserved; the caller owns cleanup/recovery policy. No
client-supplied digest enters this path.

Rules:

- this is the immutable-publication seam: it binds bytes actually inspected
  by the server to the object identity an application later marks
  ready/current, closing the same-key mutable-overwrite gap that
  `finalise_upload_verified` does not close
- `finalise_upload_verified` remains available and unchanged; it validates a
  mutable object in place (same key can still be silently replaced between
  inspection and use) and does not establish immutable publication.
  Consumers with a live upload-finalisation path should move to
  `promote_verified` rather than treat the two as interchangeable
- built-in S3 and local adapters implement both primitives; S3 uses one
  conditional `PutObject` (`If-None-Match: *`) and maps every
  precondition/conflict response to the typed collision; local pins one
  owned descriptor to the base directory at adapter construction by walking
  its canonical absolute path one component at a time from an owned root
  descriptor with `openat(O_DIRECTORY | O_NOFOLLOW)` (never a single `open`
  call on the canonicalized path string, which would let a component
  replaced with a symlink between `canonicalize()` and the pinning open be
  silently followed) and descends from a duplicate of that pinned
  descriptor with the same descriptor-relative `openat(..., O_NOFOLLOW)`
  traversal for every key, so containment holds for every path component at
  every stage — construction and per-call alike — never a check-then-act
  resolution of a lexical path; it refuses symlink/non-regular sources
  without blocking (`O_NONBLOCK` plus a post-open regular-file check);
  non-Unix platforms fail closed rather than fall back to a weaker
  resolution
- local exclusive create publishes atomically: bytes are written and
  `fsync`ed to an owned, unguessable same-directory temp file first, then
  published to the final name with `linkat`, then the parent directory is
  `fsync`ed so the new name itself is durable, not only the bytes behind
  it. A concurrent reader never observes partial content, and a write
  failure or a destination collision before `linkat` leaves only the
  caller-owned temp behind, never a poisoned final name that would block
  every retry. Once `linkat` reports success the call cannot fail: the
  parent `fsync` and the temp-file removal that follow are both
  best-effort — their outcome is logged, never returned — so a caller can
  never see an error for a destination that may already be committed, and
  a leftover temp file from either failing never affects destination
  correctness, collision detection, or future retries (both are keyed on
  the final name only). This is a narrow local-filesystem dev/utility seam:
  the parent-directory `fsync` is a best-effort local-filesystem durability
  improvement, not a cross-filesystem (network/overlay) crash guarantee
- S3 non-collision transport failures are redacted before crossing the
  public boundary: full provider detail is logged for operators, but the
  returned error carries only a stable operation label and, when available,
  the HTTP status code — no raw backend error or credential-shaped provider
  text reaches the caller
- `promote_verified` does not trust an adapter's returned destination
  identity: it verifies the returned key and size match what was requested
  and captured before returning success, refusing with a typed internal
  error otherwise
- a destination collision is never retried as an unconditional write;
  ordinary promotion always refuses it, including when destination bytes
  match. Only the owned-recovery surface below may accept an incumbent, and
  only through positive token-bound ownership proof
- the backend ETag is supplemental metadata only; the SHA-256 in
  `VerifiedPromotionResult` is the cross-adapter byte identity

This contract is the generic storage seam. It does not define media-graph
meaning by itself.

### Owned promotion recovery (`g11.001`, v0.9.7 follow-up)

Byte equality, destination-key secrecy, publication intent, ETag, and ordinary
object metadata do not prove which application version created an incumbent.
An application may recover a destination after process loss only when the
exclusive create atomically attached positive ownership evidence that matches a
token persisted by that version before publication.

The additive owned-promotion surface must:

- accept an opaque, high-entropy caller token without rendering it in Debug,
  Display, public errors, logs, URLs, or returned DTOs;
- write a one-way token verifier plus the server-derived SHA-256, size, and
  validated MIME as reserved object metadata in the same backend commit that
  exclusively creates the destination;
- let S3 attach that metadata to the conditional PutObject and let local
  storage attach equivalent metadata to the unpublished temp inode before its
  atomic link publishes the final name;
- let recovery use only the durable token, destination, provider/bucket
  authority, and object head metadata, never staging or mutable media fields;
- compare the token verifier without timing-dependent early exit and refuse
  absent, malformed, incomplete, or mismatched metadata;
- keep every ordinary or unproven collision as
  `BlobError::DestinationExists`, preserving the incumbent unchanged;
- keep existing promotion and mutable methods plus third-party adapter
  implementations source-compatible. Unsupported adapters fail closed.

Persisting intent before create does not authorize adoption by itself. A
pre-create crash followed by a foreign incumbent must refuse. Consumer delete
and purge retain database recovery identity until required blob cleanup
succeeds.

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
- `MediaUsageRepository`
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
- `MediaRepository` and `MediaRepositoryExt` are the app-facing lifecycle
  repository seam
- `MediaUsageRepository` is the retained older simple usage repository seam
  for `MediaUsage`, `track_usage`, `sync_usages`, and usage-count operations
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
- default media repository adapters parse row strings from storage-backed
  tables into Underlay media domain types during row mapping
- the current five-consumer family has adopted the same app-local parse boundary
  where apps own media row/domain models; local DTO conversion is not the
  primary validation point

This layer owns CRUD and lifecycle mechanics. The richer meaning of usages,
migration bindings, and media graph semantics is defined in `050`.

Current boundary note:

- the shipped `underlay-media-postgres` `PostgresMediaRepository` implements
  the older simple usage model through `MediaUsageRepository`
- the generalized usage-edge sync model remains a separate
  `underlay_media::sync::MediaUsageSyncRepository` seam; default backend
  support is still partial and should be assessed against `050` before apps
  assume full usage-graph persistence

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

[050-media-library-and-usage.md](./050-media-library-and-usage.md)
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

- None currently named.

Resolved assessment:

- `g06.187` made `underlay_db::TypedExistsCheck` neutral by default. Callers
  must opt into `deleted_at IS NULL` filtering with `.active_only()` when the
  table uses Underlay's soft-delete convention.
- `g06.186` split the former blob-owned `MediaConfig` into
  `underlay_blob::BlobUploadConfig` for upload-size policy and
  `underlay_media::renditions::RenditionConfig` for thumbnail/rendition policy.
- `g06.183` confirmed
  [050-media-library-and-usage.md](./050-media-library-and-usage.md)
  is now an active contract. The lower/higher media authority stack is explicit
  in file state: `040` owns blob, storage, repository, and lower media
  mechanics; `050` owns usage graph, structured-content sync, migration
  binding, and media-linked content semantics.
- `g06.185` split the older simple `MediaUsage` repository methods from
  `MediaRepository` into `MediaUsageRepository`. `MediaRepository` now owns
  media lifecycle operations, `MediaUsageRepository` owns retained simple
  usage tracking, and `underlay_media::sync::MediaUsageSyncRepository` remains
  the generalized usage-edge sync seam.

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
