# AGENTS (Underlay)

## Scope

`underlay` is the shared foundation repo for reusable Rust crates, TypeScript utilities, Svelte components, and cross-project guidance.

## Hard Rules

- Keep shared code generic and project-agnostic.
- Do not move app-specific behavior from consumer repos into Underlay without a clear reusable boundary.
- Preserve the separation between `rust/`, `ts/`, `contracts/`, and `docs/`.
- Prefer extracting stable patterns over adding one-off compatibility shims.

## Effigy-First Execution

- Start with `effigy tasks` to inspect Underlay's local task surface.
- Prefer `effigy health` as the default repo-owned baseline.
- Use `effigy test --plan` before picking a concrete test runner.
- Prefer local Effigy tasks such as `effigy qa:docs`, `effigy qa:northstar`, `effigy validate`, `effigy rust:check`, `effigy rust:test`, and `effigy test:components`.
- Use `effigy doctor` when you want broader repo scans; it currently includes structural scan findings beyond the task surface itself.
- Fall back to raw `cargo`, `bun`, or `vitest` only when the needed operation is not represented in `effigy.toml`.

## Validation

```bash
effigy health
effigy qa:docs
effigy qa:northstar
effigy validate
effigy test --plan
# Use targeted raw tool commands only when Effigy does not cover the path
```

## Source of Truth

- `./README.md`
- `./docs/guides/README.md`
- `./docs/guides/000-overview.md`
- `./docs/guides/172-agents-files.md`
