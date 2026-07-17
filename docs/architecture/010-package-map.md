# Package Map

Underlay is split by layer, mirroring how consuming apps are structured.

## Rust Crates (36)

The current crate map is the live implementation shape. The target
reference-grade reset is documented in
[020-reference-grade-underlay-architecture.md](./020-reference-grade-underlay-architecture.md).
During `g06`, crate roots and adapter/tooling boundaries may change in
controlled breaking batches with six-consumer proof.

### Core

| Crate | Purpose |
|-------|---------|
| `underlay-core` | Cross-cutting primitives: `Uuid` (v7), `AppError`, DTO envelopes (`ListResponse`, `SingleResponse`, `ErrorEnvelope`), slug validation |
| `underlay-config` | Layered TOML configuration helpers (file stacking; apps own typed config) |
| `underlay-http` | Axum HTTP utilities: response helpers, CORS, cookies, pagination, error logging |
| `underlay-http-client` | Shared outbound `reqwest::Client` with SSRF guards and timeout defaults |
| `underlay-query` | Shared query model + SQL `WHERE`/`ORDER` building: filter/sort vocabulary, `WhereBuilder`, `FieldMapping` |
| `underlay-observability` | Tracing bootstrap, `request_id_layer()`, `trace_layer()` |
| `underlay-metrics` | Prometheus registry wrapper + Axum `/metrics` handler |
| `underlay-validation` | Declarative `Validate` trait + built-in validators (email, URL, length, range) |
| `underlay-validation-derive` | `#[derive(Validate)]` proc macro for automatic validation codegen |

### Authentication

| Crate | Purpose |
|-------|---------|
| `underlay-auth` | Auth boundary types + `AuthProvider` trait + Axum `Authenticated` extractor |
| `underlay-auth-jwt` | JWT session management: token creation, validation, refresh |
| `underlay-auth-password` | Password authentication with Argon2id hashing |
| `underlay-auth-totp` | Time-based One-Time Password (TOTP) primitives |
| `underlay-auth-email-totp` | Email-based OTP verification flows |
| `underlay-auth-webauthn` | WebAuthn / Passkey registration and authentication |
| `underlay-auth-oauth` | OAuth2 provider primitives (Google, etc.) |

### Data & Storage

| Crate | Purpose |
|-------|---------|
| `underlay-db` | SQLx pool setup, migration runner, dev reset helpers |
| `underlay-soft-delete` | Soft-delete column conventions, `include_deleted()` semantics |
| `underlay-blob` | Blob storage with pluggable backends (S3, local filesystem) |
| `underlay-media` | Media library: file management, storage orchestration, usage tracking |
| `underlay-media-postgres` | PostgreSQL adapter for `underlay-media` repository contracts |
| `underlay-migration-core` | Deterministic migration contracts: pipeline stages, plugin traits, run-store and decision journal types |
| `underlay-nightfire` | Block-based structured content protocol (JSON document format) |

### Infrastructure

| Crate | Purpose |
|-------|---------|
| `underlay-events` | Outbox/event record types + writer boundary |
| `underlay-jobs` | Background job contracts, runner, registry, dead-letter contracts, event hooks, and scheduler config |
| `underlay-jobs-postgres` | PostgreSQL job adapter: repositories, LISTEN/NOTIFY, scheduler runtime, outbox processor, maintenance tasks, and SQL constants |
| `underlay-email` | Email infrastructure with pluggable backends (SMTP, SES) and templates |
| `underlay-ratelimit` | Rate limiting with pluggable backends |
| `underlay-audit` | Audit logging for admin actions and security-relevant events, with typed table config over app-owned tables |
| `underlay-auth-state-postgres` | PostgreSQL auth-state adapter (`AuthStateStore`) for `underlay-auth` workflow state; configurable table |
| `underlay-security-alerts` | Shared failed-login/lockout security alert thresholds, typed table config, dedupe checks, and event persistence helpers |
| `underlay-suggestions` | Server-side suggestion query building for `RelationSelector` components |
| `underlay-ai-runtime` | Provider-agnostic AI runtime contracts, OpenAI-compatible client, routing candidate selection |
| `underlay-aws` | Shared AWS SDK configuration for Underlay crates (consistent region/credential setup) |

### Developer Tools

| Crate | Purpose |
|-------|---------|
| `underlay-testing` | `TestDb`, `TestServer`, common fixtures for integration testing |
| `underlay-devtools` | Database migration synchronisation and developer utilities |

### Feature Flags

Several crates use feature flags to keep optional dependencies out of the default build:

| Crate | Flags | Notes |
|-------|-------|-------|
| `underlay-blob` | `s3`, `local` | Backend selection |
| `underlay-email` | `smtp`, `ses`, `templates` | Backend + template engine |
| `underlay-http` | `tracing`, `validation`, `nightfire`, `error-logging`, `embed` | Modular HTTP features |
| `underlay-jobs` | none | Core job contracts |
| `underlay-jobs-postgres` | none | PostgreSQL job adapter |
| `underlay-media` | `renditions`, `nightfire`, `full` | Storage + processing |
| `underlay-media-postgres` | none | PostgreSQL media adapter |
| `underlay-testing` | `db`, `server`, `full` | Test infrastructure scope |
| `underlay-validation` | `derive`, `axum` | Macro + framework integration |
| `underlay-auth-password` | `hibp` | Have I Been Pwned breach check |
| `underlay-auth` | `hashing` | Optional password hashing support |
| `underlay-auth-webauthn` | `attestation` | Attested passkey verification |

## TypeScript + Svelte

`ts/src/` is currently organized into these domains:

- `client/`: HTTP, query, pagination, auth, navigation, media, and transport helpers
- `nightfire/`: structured-content editor/runtime, markdown/media blocks, registries, validation
- `patterns/`: retained workflow shells, auth flows, relation selector, optimistic/list/reorder/navigation helpers
- `runtime/`: browser/runtime orchestration helpers and app-facing controllers
- `server/`: CSP and security-header server helpers
- `templates/`: higher-order admin template system (`EntityListPage`, `EntityDetailPage`, `EntityFormPage`, related modules/cards)
- `testing/`: TS test helpers
- `tools/`: repo guardrails and template-surface tooling
- `utils/`: focused standalone helpers

Public package guidance is documented in the repo root [README.md](../../README.md)
and the architecture overview rather than repeated here as an old namespace map.

## Contracts

- `contracts/openapi/underlay.openapi.yaml` contains the shared envelope schemas.

Apps can embed their own OpenAPI docs and import/inline these shapes as needed.
