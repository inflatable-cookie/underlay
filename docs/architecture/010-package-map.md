# Package Map

Underlay is split by layer, mirroring how consuming apps are structured.

## Rust

- `rust/crates/underlay-core`
  - Cross-cutting primitives used by all Rust crates and API boundaries.
  - IDs (`Uuid` UUIDv7), error type (`AppError`), and DTO envelopes (`ListResponse`, `SingleResponse`, `ErrorEnvelope`).

- `rust/crates/underlay-observability`
  - Tracing bootstrap + request identity conventions.
  - Provides `request_id_layer()` and `trace_layer()` helpers.

- `rust/crates/underlay-http`
  - Axum-focused response helpers (envelopes, error responses, CORS).

- `rust/crates/underlay-auth`
  - Auth boundary types + provider abstraction.
  - Includes an Axum extractor (`Authenticated`) that uses an app-supplied provider.

- `rust/crates/underlay-db`
  - SQLx pool setup + migration runner helpers.
  - Includes guarded helpers for dev reset flows.

- `rust/crates/underlay-soft-delete`
  - Standard soft-delete column naming and result semantics.

- `rust/crates/underlay-events`
  - Outbox/event record types + writer boundary.
  - Includes a template SQL schema apps can adopt.

- `rust/crates/underlay-jobs`
  - Background job runner skeleton + handler registry + store boundary.

- `rust/crates/underlay-metrics`
  - Prometheus registry wrapper + Axum `/metrics` handler.

## TypeScript + Svelte

- `ts/` exports a single package (`@decodelabs/underlay`) today.
  - `components/`: low-level UI primitives.
  - `patterns/`: higher-level building blocks (forms, list pages, error banners).
  - `client/`: typed HTTP + envelope helpers for building API clients.
  - `nightfire/`: reserved namespace for structured content renderers.

## Contracts

- `contracts/openapi/underlay.openapi.yaml` contains the shared envelope schemas.

Apps can embed their own OpenAPI docs and import/inline these shapes as needed.
