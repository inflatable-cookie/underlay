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

Use Effigy as the default command surface from the Underlay repo root:

```bash
effigy tasks
effigy health
effigy test --plan
```

Common local commands:

```bash
effigy qa:docs
effigy qa:northstar
effigy validate
effigy qa
effigy rust:build
effigy rust:check
effigy rust:test
effigy test:components
```

Package scripts remain convenience wrappers, but direct `effigy ...` is canonical when you are already in this repo.

Use `effigy doctor` when you want broader repo scans. Underlay currently carries structural scan backlog in that surface, so `health` is the better day-to-day baseline.

### Postgres Integration Tests

Some `underlay-db` tests spin up Postgres via `testcontainers`.

- Install runtime and CLI:
  - `brew install colima docker`
  - `colima start`
  - verify with `docker ps`
- Run the integration suite:
  - `effigy rust:test -- -p underlay-db --test postgres_integration -- --ignored`

### Coverage

- Install tarpaulin: `cargo install cargo-tarpaulin`
- Run coverage check: `rust/scripts/check-coverage.sh`
