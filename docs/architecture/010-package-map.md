# Package Map

Underlay is split by layer, mirroring how consuming apps are structured.

## Rust Crates (29)

### Core

| Crate | Purpose |
|-------|---------|
| `underlay-core` | Cross-cutting primitives: `Uuid` (v7), `AppError`, DTO envelopes (`ListResponse`, `SingleResponse`, `ErrorEnvelope`), slug validation |
| `underlay-http` | Axum HTTP utilities: response helpers, CORS, cookies, pagination, query builders, error logging |
| `underlay-observability` | Tracing bootstrap, `request_id_layer()`, `trace_layer()` |
| `underlay-metrics` | Prometheus registry wrapper + Axum `/metrics` handler |
| `underlay-openapi` | OpenAPI schema types for API documentation |
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
| `underlay-auth-state` | Authentication state storage and flow management |

### Data & Storage

| Crate | Purpose |
|-------|---------|
| `underlay-db` | SQLx pool setup, migration runner, dev reset helpers |
| `underlay-soft-delete` | Soft-delete column conventions, `include_deleted()` semantics |
| `underlay-blob` | Blob storage with pluggable backends (S3, local filesystem) |
| `underlay-image` | Image processing: thumbnails, renditions, format detection |
| `underlay-media` | Media library: file management, storage orchestration, usage tracking |
| `underlay-nightfire` | Block-based structured content protocol (JSON document format) |

### Infrastructure

| Crate | Purpose |
|-------|---------|
| `underlay-events` | Outbox/event record types + writer boundary |
| `underlay-jobs` | Background job queue with optional PostgreSQL persistence and cron scheduling |
| `underlay-email` | Email infrastructure with pluggable backends (SMTP, SES) and templates |
| `underlay-ratelimit` | Rate limiting with pluggable backends |
| `underlay-audit` | Audit logging for admin actions and security-relevant events |
| `underlay-suggestions` | Server-side suggestion query building for `RelationSelector` components |

### Developer Tools

| Crate | Purpose |
|-------|---------|
| `underlay-testing` | `TestDb`, `TestServer`, common fixtures for integration testing |
| `underlay-devtools` | Database migration synchronisation and developer utilities |

### Feature Flags

Several crates use feature flags to keep optional dependencies out of the default build:

| Crate | Flags | Notes |
|-------|-------|-------|
| `underlay-blob` | `s3`, `local`, `dev-server` | Backend selection |
| `underlay-email` | `smtp`, `ses`, `templates` | Backend + template engine |
| `underlay-http` | `tracing`, `validation`, `nightfire`, `error-logging`, `embed` | Modular HTTP features |
| `underlay-jobs` | `postgres`, `scheduler`, `outbox`, `full` | Persistence + scheduling |
| `underlay-media` | `postgres`, `renditions`, `full` | Storage + processing |
| `underlay-testing` | `db`, `server`, `full` | Test infrastructure scope |
| `underlay-validation` | `derive`, `axum` | Macro + framework integration |
| `underlay-auth-password` | `hibp` | Have I Been Pwned breach check |
| `underlay-auth-webauthn` | `attestation` | Attested passkey verification |

## TypeScript + Svelte

- `ts/` exports a single package (`@decodelabs/underlay`) today.
  - `components/`: low-level UI primitives.
  - `patterns/`: higher-level building blocks (forms, list pages, error banners).
  - `client/`: typed HTTP + envelope helpers for building API clients.
  - `nightfire/`: structured content renderers.

## Contracts

- `contracts/openapi/underlay.openapi.yaml` contains the shared envelope schemas.

Apps can embed their own OpenAPI docs and import/inline these shapes as needed.
