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
- `loophole/composer`

Consumer-affecting changes must follow `023` and classify impact as
`additive`, `deprecation`, or `breaking`.

## Primary Crate Inventory

| Crate | Public surface | Classification | Migration note |
| --- | --- | --- | --- |
| `underlay-core` | `Uuid`, `RawUuid`, `IdGenerator`, `SystemIdGenerator`, `SingleResponse`, `ListResponse`, `ErrorEnvelope`, `ErrorBody`, `AppError`, `AppResult` | stable | Foundation primitives. Do not widen with app-local domain behavior. |
| `underlay-db` | pool config, migration helpers, schema helpers, SQL-dir runner, pagination, typed existence checks, media DB enums, identifier helpers | stable + candidate-type | Identifier helpers are the first safety seam. `TypedExistsCheck` and typed value helpers are the retained existence-check surface; raw `ExistsCheck` and raw value helpers were removed in `g06.033`. |
| `underlay-http` | response helpers, `ApiError`, query parsing, pagination, CORS, context, caching, cookie helpers, feature-gated error logging / embed / OpenAPI helpers | stable + candidate-type | Cookie config is stable. Cookie name/path/domain/value internals should become typed construction boundaries. Feature-gated operational modules remain app-facing only when enabled. |
| `underlay-auth` | `AuthProvider`, extractors, principals, role sets, repository traits, auth/session/credential/user domain rows, optional password hashing | stable | App-facing auth contract. Keep provider/repository traits here rather than in provider-specific crates. The Postgres auth-state adapter moved to `underlay-auth-postgres` in `g06.022`. |
| `underlay-auth-postgres` | `AuthStateStore`, `AuthStateError`, `AuthStateRow` | adapter | Concrete Postgres adapter for short-lived auth workflow state. |
| `underlay-auth-jwt` | `JwtService`, claims, config, key pair, token fingerprint, `SessionManager`, `SessionStore`, `SessionState`, `SessionTokens` | stable + breaking-candidate | `SessionStore` now requires atomic refresh compare-and-swap. Treat further trait changes as breaking and prove consumer implementations before landing. |
| `underlay-blob` | `BlobAdapter`, `BlobAdapterObjectKeyExt`, `NoopAdapter`, upload/download/object types, `BlobObjectKey`, `MediaConfig`, S3 adapter, local adapter | stable + adapter + candidate-type | Adapter trait and S3 adapter are stable. `LocalAdapter` is a narrow utility seam. Request constructors and typed adapter extension methods accept `BlobObjectKey`; core adapter trait methods still accept raw `&str` for compatibility. Live runtime callers should prefer the typed request constructors and extension methods once they hold validated keys; raw strings remain acceptable for adapter internals, metadata DTOs, tests, and migration/replay tooling. |
| `underlay-media` | root media domain IDs/entities/inputs, `MediaRepository`, `MediaRepositoryExt`, `MediaError`, `MediaResult`, re-exported `BlobObjectKey`; module-owned `storage`, `sync`, `nightfire`, and `renditions` helper surfaces | stable + candidate-type | Domain and repository traits are stable at root. Stored object-key fields now use `BlobObjectKey`, and `underlay-media-postgres` parses database strings during row mapping. Helper families are module-owned as of `g06.020`. Storage key helpers now expose `BlobObjectKey` generation while retaining string helpers. Rendition generation returns typed result keys and validates raw-string wrapper inputs. `domain.rs` is split into private internal modules as of `g06.053`; `renditions.rs` is split into private config/result/key/processing/service modules as of `g06.054`. Preserve root exports, `renditions::*` exports, and serialized type shapes for future changes. The Postgres adapter moved to `underlay-media-postgres` in `g06.021`. |
| `underlay-media-postgres` | `PostgresMediaRepository`, `PostgresMediaConfig`, private SQL operation modules and row mappings | adapter + candidate-type | Concrete media Postgres adapter. `PostgresMediaConfig` stores typed schema/table identifiers internally; app config should use fallible constructors when values are external. Adapter internals stay private. |
| `underlay-devtools` | migration/reset helpers, sync migrations, seed bundles, migration bundles, migration reports | adapter + candidate-type | Tooling only. `MigrationBundleRef` requires digest-pinned SHA-256 refs for replay, local-store path handling is centralized behind an internal tooling boundary, and media-shard object keys are validated through the shared media/blob key rules. Public option structs still accept raw CLI/serialization values at the edge. No app runtime crate should depend on devtools behavior. |
| `underlay-audit` | audit row/action/filter, typed `AuditTable`, typed query/append helpers | stable | Typed table config is the only public dynamic table path after the `g06.030` removal proof. |
| `underlay-security-alerts` | alert config/types, detector, typed login-attempt and alert-event tables, typed SQL helpers | stable | Typed table config is the only public dynamic table path after the `g06.030` removal proof. |
| `underlay-jobs` | job types, handler/store traits, runner/registry, event hub, dead-letter store, scheduler config | stable | Core app-facing job contract. Concrete backend storage and notification runtime live outside the contract crate. Consumer imports are crate-root oriented in the current proof family. `types.rs` is split into private internal modules as of `g06.056`; preserve `underlay_jobs::types::*`, crate-root exports, handler/store trait signatures, and serialized job/dead-letter/scheduled-task shapes for future changes. |
| `underlay-jobs-postgres` | `JobRepository`, `RepoError`, `PgDeadLetterRepository`, `ScheduledTaskRepository`, `PgJobNotifier`, `Scheduler`, `PostgresJobRunnerExt`, `outbox`, `tasks`, job SQL constants | adapter | Concrete Postgres job adapter extracted in `g06.024`. Apps depend on this crate for SQLx repositories, LISTEN/NOTIFY, outbox processing, scheduled task runtime, maintenance tasks, and migration SQL. |
| `underlay-migration-core` | migration pipeline model, plugin traits, run store, manifest, OCI layout, governance, audit, drift, integrity, recovery, verification rules | stable + adapter + candidate-type | Large public model is intentionally library-facing. Consumer imports should remain crate-root stable. `pipeline.rs` became a small module front door in `g06.051`; orchestration, resume, checkpoint, decision-support, and failure-classification helpers are now internal submodules while root exports stay stable. OCI/bundle references now align with typed devtools bundle refs. |

## Secondary Crates

These crates remain part of the Rust surface, but they are not the first
`g06` migration gate:

| Crate family | Classification | Note |
| --- | --- | --- |
| `underlay-auth-*` provider crates except JWT | stable + adapter | Provider-specific credential flows. Keep app-facing shared traits in `underlay-auth`. |
| `underlay-email`, `underlay-events`, `underlay-ratelimit`, `underlay-observability`, `underlay-metrics` | stable + adapter | Covered by `060`, `020`, and operational contracts. Review only when affected by a gate. |
| `underlay-validation*`, `underlay-config`, `underlay-testing` | stable | Support-layer contracts. Keep validation/test APIs generic. |
| `underlay-ai-runtime`, `underlay-suggestions`, `underlay-nightfire` | stable | Covered by `080` and `070`; not part of the first platform-boundary migration. |
| `underlay-aws`, `underlay-http-client`, `underlay-soft-delete` | stable + adapter | Small focused helper crates. Review if typed safety gates touch storage or HTTP boundaries. |

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
- `acowtancy/farmyard` owns additional migration-bundle reference parsing that
  should be considered during `MigrationBundleRef` work

## Stop Conditions

Stop and re-enter planning if:

- a public trait change affects a consumer-owned implementation not already in
  the migration plan
- a typed safety wrapper would force broad churn without an additive bridge
- a module split exposes new public APIs just to satisfy internal organization
- devtools behavior starts leaking into runtime app contracts

## Next Task

Execute the next `g06` reference-grade architecture reset task.
