# Underlay

![Rust CI](https://github.com/decodelabs/underlay/actions/workflows/rust.yml/badge.svg)

Underlay is a reusable foundation for building full-stack apps with stable cross-project patterns.

It provides:

- Rust backend primitives and crates
- typed TypeScript client contracts
- shared Svelte UI components and patterns
- auth, observability, storage, and migration foundations
- docs and guidance for integrating the library into real projects

## Repo Layout

- `rust/` - Rust crates
- `ts/` - TypeScript and Svelte exports
- `contracts/` - shared contract artifacts
- `docs/` - documentation authority

## Docs Start Here

- `docs/vision/001-underlay-foundation-vision.md`
- `docs/architecture/000-overview.md`
- `docs/guides/000-overview.md`
- `docs/roadmaps/README.md`
- `docs/logs/README.md`

## Development

- Install JS deps: `bun install`
- Rust workspace tests: `cargo test --workspace`
- TypeScript and Svelte check: `bun check`

### Postgres Integration Tests

Some `underlay-db` tests spin up Postgres via `testcontainers`.

- Install runtime and CLI:
  - `brew install colima docker`
  - `colima start`
  - verify with `docker ps`
- Run the integration suite:
  - `cargo test -p underlay-db --test postgres_integration -- --ignored`

### Coverage

- Install tarpaulin: `cargo install cargo-tarpaulin`
- Run coverage check: `rust/scripts/check-coverage.sh`
