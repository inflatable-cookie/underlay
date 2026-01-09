# Underlay

![Rust CI](https://github.com/decodelabs/underlay/actions/workflows/rust.yml/badge.svg)

Underlay is a reusable foundation for building full-stack apps with the same architecture patterns as Acowtancy:

- Rust API + domain crates
- Typed TypeScript API client
- Shared Svelte UI kit
- SvelteKit Admin UI + SvelteKit Frontend UI (as consumers of the shared layers)

This repo is intentionally *app-agnostic*: it centralises stable primitives (errors, envelopes, IDs, client patterns, UI building blocks) without locking projects into one domain model.

## Repo Layout

- `rust/` – Rust crates (start with `underlay-core`).
- `ts/` – TypeScript + Svelte library exports.
- `contracts/` – shared contract artefacts (OpenAPI schemas, shared DTO envelopes).
- `docs/` – architecture + integration docs (source of truth).

Start reading: `docs/architecture/000-overview.md`.

## Development

- Install JS deps: `pnpm install`
- Rust: `cargo test --workspace`
- TS/Svelte: `pnpm check`

### Postgres Integration Tests (Colima)

Some `underlay-db` tests spin up a Postgres container via `testcontainers`.

- Install runtime + CLI:
  - `brew install colima docker`
  - `colima start`
  - verify: `docker ps`

Run the integration tests:
- `cargo test -p underlay-db --test postgres_integration -- --ignored`

### Coverage

- Install tarpaulin: `cargo install cargo-tarpaulin`
- Run check: `rust/scripts/check-coverage.sh`

## Status

Early scaffold: the goal is to extract the smallest stable layer from reference implementations (e.g. Acowtancy) and harden the boundaries here.
