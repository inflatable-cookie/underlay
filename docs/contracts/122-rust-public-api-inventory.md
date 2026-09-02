# 122 - Rust Public API Inventory

Status: active
Owner: repo maintainers
Roadmap: `g06.001`
Depends on: `001`, `020`, `021`, `023`, `025`, `030`, `040`, `060`, `070`, `120`

## Purpose

This contract is the first Rust platform-contract inventory for `g06`.

It classifies the public Rust crate surface so future work can move Underlay
toward typed safe extension points without guessing which APIs are stable,
adapter-owned, internal, or migration targets.

## Classification

- `stable`: app-facing contract surface that should stay source-compatible
  unless a roadmap marks a breaking change.
- `adapter`: public because apps need a concrete backend or provider seam.
- `internal`: currently public or feature-public, but not intended as a broad
  app contract.
- `candidate-type`: safe to keep conceptually, but raw strings or loose values
  should move behind typed constructors.
- `candidate-remove`: public residue that needs caller proof before retention.

## Consumer Boundary

The proof family for this inventory is:

- `underlay-reference`
- `contact-patch`
- `compli-me`
- `acowtancy`
- `songsprout`

Consumer-affecting changes must follow `023` and classify impact as
`additive`, `deprecation`, or `breaking`.

## Primary Crate Inventory

| Crate | Public surface | Classification | Migration note |
| --- | --- | --- | --- |
| `underlay-core` | `Uuid`, `RawUuid`, `IdGenerator`, `SystemIdGenerator`, `SingleResponse`, `ListResponse`, `ErrorEnvelope`, `ErrorBody`, `AppError`, `AppResult` | stable | Foundation primitives. Do not widen with app-local domain behavior. |
| `underlay-db` | pool config, migration helpers, schema helpers, SQL-dir runner, pagination, typed existence checks, media DB enums, identifier helpers | stable + candidate-type | Identifier helpers are the first safety seam. `TypedExistsCheck` and typed value helpers are the retained existence-check surface; raw `ExistsCheck` and raw value helpers were removed in `g06.033`. Existence checks are neutral by default; callers use `TypedExistsCheck::active_only()` when a table follows Underlay's `deleted_at` soft-delete convention. `DbConfig` fields are private and `Debug` redacts the database URL as of `g06.204`; consumers must use `new`, builders, and accessors. `SqlDirOptions` fields are private as of `g06.216`; callers must use defaults, builders, and accessors. Pagination is stable app-facing module surface and is split into private modules as of `g06.058`; consumers import `underlay_db::pagination::*` directly, so future changes must preserve the `pagination` front door, exported item names, cursor encoding, response shapes, and SQL helper behavior. |
| `underlay-http` | response helpers, `ApiError`, query parsing, pagination, CORS, context, caching, cookie helpers, feature-gated error logging / embed / OpenAPI helpers | stable + candidate-type | Query parsing is stable app-facing surface used through both `underlay_http::query::*` and crate-root re-exports; it is split into private modules as of `g06.060`, and future changes must preserve sort/filter parsing semantics, `WhereBuilder`, `FieldMapping`, and the exported `field_mapping!` macro path. Cookie config and helpers are stable app-facing auth/CSRF surface used through both `underlay_http::*` and `underlay_http::cookies::*`; they are split into private modules as of `g06.062`, and future changes must preserve typed cookie values, header helper names, validation behavior, checked builders, typed setters, and read-only accessors. Raw string setters and public config fields were retired in `g06.172` and `g06.176`. CORS and HTTP server config expose checked transition paths as of `g06.194`: external CORS origins should use `try_with_origins`, parsed origins should use `with_origin_values`, and env-facing server config should use `try_from_env`. Known consumers moved their CORS layer conversion paths off direct `underlay_http::CorsConfig` field construction in `g06.195`; `CorsConfig` fields are private as of `g06.196`, so new apps must use builders and accessors. `HttpServerConfig` fields are private as of `g06.197`, so new apps must use constructors and accessors. Error logging is stable feature-gated operational surface used by consumers through crate-root exports and `error_logging_middleware`; it is split into private modules as of `g06.064`, and future changes must preserve the `error-logging` feature gate, row/filter shapes, middleware behavior, SQL helper names, and `ERROR_CONTEXT_HEADER`. `ErrorLoggingConfig` fields are private as of `g06.214`; consumers must use constructors, builders, and accessors. Feature-gated operational modules remain app-facing only when enabled. |
| `underlay-auth` | `AuthProvider`, extractors, principals, role sets, repository traits, auth/session/credential/user domain rows, optional password hashing | stable | App-facing auth contract. Keep provider/repository traits here rather than in provider-specific crates. The Postgres auth-state adapter is `underlay-auth-state-postgres` (renamed from `underlay-auth-postgres` in `g08.018`); Underlay ships no Postgres adapter for the user/credential/session/audit repos (consumer schemas diverge). |
| `underlay-auth-state-postgres` | `AuthStateStore`, `AuthStateError`, `AuthStateRow` | adapter | Concrete Postgres adapter for short-lived auth workflow state only; table configurable via `with_table`. Renamed from `underlay-auth-postgres` (g08.018). |
| `underlay-auth-jwt` | `JwtService`, claims, config, key pair, token fingerprint, `SessionManager`, `SessionStore`, `SessionState`, `SessionTokens` | stable + breaking-candidate | `SessionStore` now requires atomic refresh compare-and-swap. `JwtConfig` fields are private as of `g06.199`; consumers must use `from_env`, `from_env_with_defaults`, `from_values`, `generate`, builders, and accessors. `JwtBehaviorDefaults` remains the non-secret typed app-config literal surface. Treat further trait changes as breaking and prove consumer implementations before landing. |
| `underlay-blob` | `BlobAdapter`, `BlobAdapterObjectKeyExt`, `BlobAdapterPromotionExt`, `VerifiedPromotionResult`, `OwnershipToken`, `OwnedDestinationAuthority`, `OwnedPublicationFacts`, `NoopAdapter`, upload/download/object types, `BlobObjectKey`, `BlobUploadConfig`, S3 adapter, local adapter | stable + adapter + retained-compatibility | Adapter trait and S3 adapter are stable. `LocalAdapter` is a narrow utility seam. `BlobUploadConfig` owns upload-size policy only; thumbnail/rendition policy belongs in `underlay_media::renditions::RenditionConfig`. `UploadRequest` and `DownloadRequest` carry `BlobObjectKey` directly as of `g06.191`; callers parse external strings explicitly or use `from_object_key`. Typed adapter extension methods accept `BlobObjectKey`. Core adapter trait methods intentionally keep raw `&str` as the retained compatibility boundary for adapter internals, database-loaded object keys used in media display paths, metadata DTOs, tests, and migration/replay tooling; `g06.193` closes this item as retained rather than a remaining candidate-type gap. Storage config fields are private as of `g06.209`; consumers must use constructors, builders, and accessors. `g11.001` adds additive, fail-closed-by-default `BlobAdapter::get_bytes_bounded`/`put_bytes_create_only` (raw `&str`, matching the existing core-method boundary) plus `BlobAdapterPromotionExt::promote_verified` (typed `BlobObjectKey` staging/destination, matching the typed-extension convention) for immutable verified staging-to-published promotion; built-in S3 and local adapters implement both new primitives, other adapters refuse via `BlobError::Unsupported` until they do. Card 003 adds additive `OwnershipToken` (redacted Debug, never Displayed; minimum 32 bytes, fresh token per publication as operational hygiene), `OwnedDestinationAuthority`, `OwnedPublicationFacts`, fail-closed `BlobAdapter::put_bytes_create_only_owned`, `BlobAdapterPromotionExt::promote_verified_owned`, and `recover_owned_publication`. The v0.9.7 reserved-metadata verifier binds provider, bucket/namespace, and key with the token via length-prefixed SHA-256; existing v0.9.6 methods are unchanged, and oversized local reserved xattrs do not turn `head`/`exists` into I/O errors. |
| `underlay-media` | root media domain IDs/entities/inputs, `MediaRepository`, `MediaRepositoryExt`, `MediaUsageRepository`, `MediaError`, `MediaResult`, re-exported `BlobObjectKey`; module-owned `storage`, `sync`, `nightfire`, and `renditions` helper surfaces | stable + candidate-type | Domain and repository traits are stable at root. `MediaRepository` owns lifecycle operations; `MediaUsageRepository` owns retained simple `MediaUsage` tracking; generalized usage-edge sync remains in `underlay_media::sync::MediaUsageSyncRepository`. Stored object-key fields now use `BlobObjectKey`, and `underlay-media-postgres` parses database strings during row mapping. Helper families are module-owned as of `g06.020`. Storage file-key helpers expose validated `BlobObjectKey` generation; public string-returning file-key helpers were retired in `g06.190`, while prefix helpers still return strings for list/delete prefix operations. `StorageKeyConfig` fields are private and fallible builders validate prefix/component values as of `g06.192`. Rendition generation returns typed result keys and validates raw-string wrapper inputs. `ThumbnailConfig` and `RenditionConfig` fields are private as of `g06.212`; consumers must use constructors, builders, and accessors. `domain.rs` is split into private internal modules as of `g06.053`; `renditions.rs` is split into private config/result/key/processing/service modules as of `g06.054`. Preserve root exports, `renditions::*` exports, and serialized type shapes for future changes. The Postgres adapter moved to `underlay-media-postgres` in `g06.021`. |
| `underlay-media-postgres` | `PostgresMediaRepository`, `PostgresMediaConfig`, private SQL operation modules and row mappings | adapter + candidate-type | Concrete media Postgres adapter. `PostgresMediaConfig` stores typed schema/table identifiers internally; app config should use fallible constructors when values are external. Adapter internals stay private. |
| `underlay-devtools` | migration/reset helpers, sync migrations, seed bundles, migration bundles, migration reports | adapter + candidate-type | Tooling only. Migration-bundle public exports are crate-root option/report types, `MigrationBundleRef`, `MigrationBundleError`, and build/publish/pull/run entry points. `MigrationBundleRef` requires digest-pinned SHA-256 refs for replay, and `BundleRunOptions` carries that typed ref directly as of `g06.189`. Local-store path handling is centralized behind an internal tooling boundary, local/remote publish and pull behavior is part of the tooling contract, and media-shard object keys are validated through the shared media/blob key rules. `migration_bundle.rs` is split into private model/package/build/output/run modules as of `g06.068`; future changes must preserve crate-root exports, package JSON shape, digest validation, local/remote publish and pull behavior, and seed-bundle private reuse. Devtools option fields are private as of `g07.025`; callers use constructors, builders, and accessors. Publish/pull option constructors still accept raw CLI/serialization values at the edge because tag refs remain valid there. No app runtime crate should depend on devtools behavior. |
| `underlay-audit` | audit row/action/filter, typed `AuditTable`, typed query/append helpers | stable | Typed table config is the only public dynamic table path after the `g06.030` removal proof. |
| `underlay-security-alerts` | alert config/types, detector, typed login-attempt and alert-event tables, typed SQL helpers | stable | Typed table config is the only public dynamic table path after the `g06.030` removal proof. `SecurityAlertConfig` fields are private as of `g06.206`; consumers must use builders and accessors. |
| `underlay-jobs` | job types, handler/store traits, runner/registry, event hub, dead-letter store, scheduler config | stable | Core app-facing job contract. Concrete backend storage and notification runtime live outside the contract crate. Consumer imports are crate-root oriented in the current proof family. `types.rs` is split into private internal modules as of `g06.056`; preserve `underlay_jobs::types::*`, crate-root exports, handler/store trait signatures, and serialized job/dead-letter/scheduled-task shapes for future changes. `JobConfig` fields are private as of `g06.217`; consumers must use presets, builders, and accessors. `JobRunnerConfig` fields are private as of `g06.207`; consumers must use builders and accessors. `SchedulerConfig` fields are private as of `g06.208`; consumers must use builders and accessors. |
| `underlay-jobs-postgres` | `JobRepository`, `RepoError`, `PgDeadLetterRepository`, `ScheduledTaskRepository`, `PgJobNotifier`, `Scheduler`, `PostgresJobRunnerExt`, `outbox`, `tasks`, job SQL constants | adapter | Concrete Postgres job adapter extracted in `g06.024`. Apps depend on this crate for SQLx repositories, LISTEN/NOTIFY, outbox processing, scheduled task runtime, maintenance tasks, and migration SQL. Consumer imports are crate-root oriented in the current proof family. `postgres.rs` is split into private repository operation modules as of `g06.072`; future changes must preserve `JobRepository`, `RepoError`, direct repository method names, `JobStore` behavior, dead-letter insertion behavior, SQL semantics, and root exports. `OutboxConfig` fields are private as of `g06.205`; consumers must use builders and accessors. |
| `underlay-migration-core` | migration pipeline model, plugin traits, run store, manifest, OCI layout, governance, audit, drift, integrity, recovery, verification rules | stable + adapter + candidate-type | Large public model is intentionally library-facing. Consumer imports should remain crate-root stable. `pipeline.rs` became a small module front door in `g06.051`; orchestration, resume, checkpoint, decision-support, and failure-classification helpers are now internal submodules while root exports stay stable. `MigrationOrchestrator` is the stable public orchestrator type; its stage helpers are split into private modules as of `g06.066`, and future changes must preserve `new`, `stage_order`, `run`, stage execution behavior, and persisted stage/report shapes. Verification rules are stable crate-root model surface and are split into private model/evaluator/benchmark/constructor modules as of `g06.070`; future changes must preserve rule/result type names, serialized rule shapes, standard-rule constructor names, evaluator behavior, benchmark behavior, and `PipelinePolicy` verification-rule accessors. `PipelinePolicy`, `AiThresholdPolicy`, and `IntegrityPolicy` fields are private as of `g07.024`; consumers must use defaults, builders, and accessors. `OciBundleConfig` fields are private as of `g07.026`; callers must use constructors and accessors. Governance and manifest policy structs remain public serialized document records validated by Underlay evaluators. OCI/bundle references now align with typed devtools bundle refs. |

## Secondary Crates

These crates remain part of the Rust surface, but they are not the first
`g06` migration gate:

| Crate family | Classification | Note |
| --- | --- | --- |
| `underlay-auth-*` provider crates except JWT | stable + adapter | Provider-specific credential flows. Keep app-facing shared traits in `underlay-auth`. `underlay-auth-password::PasswordConfig` fields are private as of `g06.200`; consumers must use defaults, builders, and accessors. `underlay-auth-email-totp::EmailTotpConfig` fields are private as of `g06.201`; consumers must use builders and accessors. `underlay-auth-oauth::GoogleOAuthConfig` fields are private as of `g06.202`; consumers must use constructors, builders, and accessors, and app-service verified-email policy changes use `with_require_verified_email`. `underlay-auth-totp::TotpConfig` and `underlay-auth-webauthn::WebAuthnConfig` fields are private as of `g06.210`; consumers must use constructors, builders, and accessors. |
| `underlay-email`, `underlay-events`, `underlay-ratelimit`, `underlay-observability`, `underlay-metrics` | stable + adapter | Covered by `060`, `020`, and operational contracts. Review only when affected by a gate. Email manager, SMTP, SES, and development-capture config fields are private as of `g06.213`; consumers must use constructors, builders, and accessors. `RateLimitConfig` fields are private as of `g06.198`; consumers must use constructors and accessors. `ObservabilityConfig` fields are private as of `g06.203`; consumers must use builders and accessors. |
| `underlay-validation*`, `underlay-config`, `underlay-testing` | stable | Support-layer contracts. Keep validation/test APIs generic. `underlay-validation::validators` is the stable validator front door; implementation modules are private as of `g06.188`. |
| `underlay-ai-runtime`, `underlay-suggestions`, `underlay-nightfire` | stable | Covered by `080` and `070`; not part of the first platform-boundary migration. `RetryConfig`, `CircuitBreakerConfig`, and `RouteChainConfig` fields are private as of `g06.211`; consumers must use defaults, builders, and accessors. `underlay-nightfire::MultiConfig` fields are private as of `g06.215`; consumers must use constructors, builders, and accessors. |
| `underlay-aws`, `underlay-http-client`, `underlay-soft-delete` | stable + adapter | Small focused helper crates. Review if typed safety gates touch storage or HTTP boundaries. `AwsConfig` fields are private as of `g06.209`; consumers must use constructors, builders, and accessors. |

## First Migration Gates

### Gate 1: Typed Safety Primitives

Introduce or promote typed constructors for:

- `SqlIdentifier`
- `QualifiedTableName`
- `BlobObjectKey`
- `CookieName`
- `CookiePath`
- `CookieDomain`
- `MigrationBundleRef`

Acceptance:

- invalid values fail before SQL, filesystem, HTTP header, or registry IO
- raw-string APIs are classified as compatibility, migration target, or
  internal-only
- new public APIs prefer typed values

Impact: additive first, deprecation later.

### Gate 2: Auth And Session

Freeze session-store expectations around atomic refresh rotation.

Acceptance:

- `SessionStore` compare-and-swap semantics are documented
- consumer-owned `SessionStore` implementations are scanned before any further
  trait movement
- refresh replay tests remain in `underlay-auth-jwt`

Status: complete in `g06.003`.

Impact: no direct code change for current named consumers; breaking if the trait
changes again or if an unknown consumer owns a direct implementation.

### Gate 3: HTTP Safe Builders

Keep auth and CSRF cookie construction centralized.

Acceptance:

- consumers do not hand-build auth or CSRF cookie strings
- invalid SameSite/Secure/domain/path/token combos fail centrally
- typed cookie fields are additive before any raw-field deprecation

Status: complete in `g06.004`.

Impact: additive now; deprecation only for later raw-field retirement.

### Gate 4: DB Identifier Boundary

Move dynamic schema/table/column use to typed identifiers.

Acceptance:

- dynamic identifiers are quoted through `underlay-db`
- values remain bound parameters
- audit, security-alert, existence, media, and test schema helpers are covered

Status: complete through `g06.037`.

Impact: additive first; deprecation for raw table-name helpers after consumer
proof.

### Gate 5: Media And Devtools Modularity

Keep public contracts separate from adapter/tooling internals.

Acceptance:

- media repository traits remain app-facing; Postgres operation modules remain
  internal
- migration bundle refs cannot escape expected local/remote stores
- devtools stays tooling-only

Status: complete in `g06.006`, `g06.007`, and `g06.049`.

Impact: additive/deprecation.

### Gate 6: Consumer Compatibility Closeout

Record current six-consumer proof for the `g06` public Rust surface changes.

Acceptance:

- targeted consumer API checks pass or failures are classified
- additive, internal, breaking, compatibility-fix, and deprecation impact is
  explicit
- remaining structural validation backlog is visible

Status: complete in `g06.008`.

Impact: no new code impact; release-note and compatibility proof only.

### Gate 7: Jobs Postgres Adapter Extraction

Separate the job contract crate from concrete Postgres storage and notification
runtime.

Acceptance:

- `underlay-jobs` remains the app-facing job contract crate
- `underlay-jobs-postgres` owns SQLx repositories, LISTEN/NOTIFY support,
  outbox processing, scheduled task runtime, operational task helpers, and SQL
  migration constants
- consumers depend on `underlay-jobs-postgres` for concrete Postgres usage

Status: complete in `g06.024`.

Impact: breaking for consumers currently importing concrete Postgres job
exports from `underlay-jobs`.

## Consumer Dependency Readout

Current consumer crates depend heavily on:

- `underlay-core`
- `underlay-http`
- `underlay-auth`
- `underlay-auth-jwt`
- `underlay-db`
- `underlay-blob`
- `underlay-media`
- `underlay-devtools`
- `underlay-audit`
- `underlay-jobs`
- `underlay-security-alerts`

Observed proof points:

- no named consumer directly references `SessionStore` today
- `contact-patch` and `underlay-reference` now use Underlay CSRF helpers
- `AuthCookieConfig` is present across the current API family
- `acowtancy/apps/farmyard` owns additional migration-bundle reference parsing that
  should be considered during `MigrationBundleRef` work

## Stop Conditions

Stop and re-enter planning if:

- a public trait change affects a consumer-owned implementation not already in
  the migration plan
- a typed safety wrapper would force broad churn without an additive bridge
- a module split exposes new public APIs just to satisfy internal organization
- devtools behavior starts leaking into runtime app contracts

## Next Task

No active Rust public API retirement task remains. Re-enter planning before
opening another compatibility-retirement lane.
