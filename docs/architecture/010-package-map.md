# Package Map

Underlay is split by layer, mirroring how consuming apps are structured.

## Rust

- `rust/crates/underlay-core`
  - Cross-cutting primitives used by all Rust crates and the API boundary.
  - IDs (`Uuid` UUIDv7), error type (`AppError`), and DTO envelopes (`ListResponse`, `SingleResponse`, `ErrorEnvelope`).

Future crates (intended; add when extracted):

- `underlay-api` (axum helpers, error → response mapping, version header handling)
- `underlay-db` (sqlx patterns, migration conventions, common queries)
- `underlay-auth` (auth boundary types, roles/capabilities abstractions)

## TypeScript + Svelte

- `ts/` exports a single package (`@decodelabs/underlay`) today.
  - `components/`: low-level UI primitives.
  - `patterns/`: higher-level building blocks (forms, list pages, error banners).
  - `client/`: typed HTTP + envelope helpers for building API clients.
  - `nightfire/`: reserved namespace for structured content renderers.

## Contracts

- `contracts/openapi/underlay.openapi.yaml` contains the shared envelope schemas.

Apps can embed their own OpenAPI docs and import/inline these shapes as needed.
