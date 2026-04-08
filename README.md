# Underlay

![Rust CI](https://github.com/decodelabs/underlay/actions/workflows/rust.yml/badge.svg)

Underlay is a reusable foundation for building full-stack apps with stable cross-project patterns.

It provides:

- Rust backend primitives and crates
- typed TypeScript client contracts
- retained Svelte workflow/page shells and Nightfire editor/runtime surfaces
- shared runtime and utility helpers
- auth, observability, storage, and migration foundations
- docs and guidance for integrating the library into real projects

## Repo Layout

- `rust/` - Rust crates
- `ts/` - TypeScript and Svelte exports
- `contracts/` - shared contract artifacts
- `docs/` - documentation authority

## TS Package Surface

Underlay’s current TypeScript/Svelte package boundary is explicit:

- `@decodelabs/underlay/patterns`
  - retained workflow/page-shell UI
- `@decodelabs/underlay/runtime/*`
  - shared app/runtime helpers and controllers via explicit feature subpaths
- `@decodelabs/underlay/utils/*`
  - small standalone helpers via focused subpaths
- `@decodelabs/underlay/client/*`
  - transport and SvelteKit-facing client helpers via explicit feature subpaths
- `@decodelabs/underlay/nightfire/*`
  - structured content editor/runtime package via explicit subpaths

The flat root import surface `@decodelabs/underlay` is retired. Import from
the explicit package subpaths above instead.

Use Poodle directly for foundational primitives and generic composites.

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

First-time bring-up from another directory:

```bash
effigy bootstrap git@github.com:inflatable-cookie/underlay.git
```

Common local commands:

```bash
effigy qa:docs
effigy qa:northstar
effigy validate
effigy qa
effigy storybook
effigy storybook:build
effigy rust:build
effigy rust:check
effigy rust:test
effigy test:components
```

For shared UI discovery, use the local Storybook catalog for retained Underlay
workflow shells and helpers. Use Poodle's own preview/docs for primitives and
generic composites that no longer belong to Underlay.

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
