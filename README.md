# Underlay

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
- Rust: `cargo test`
- TS/Svelte: `pnpm check`

## Status

Early scaffold: the goal is to extract the smallest stable layer from reference implementations (e.g. Acowtancy) and harden the boundaries here.
