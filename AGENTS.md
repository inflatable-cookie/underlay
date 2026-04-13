# AGENTS (Underlay)

## Scope

`underlay` is the shared foundation repo for reusable Rust crates, TypeScript utilities, Svelte components, and cross-project guidance.

## Hard Rules

- Keep shared code generic and project-agnostic.
- Do not move app-specific behavior from consumer repos into Underlay without a clear reusable boundary.
- Preserve the separation between `rust/`, `ts/`, `contracts/`, and `docs/`.
- Prefer extracting stable patterns over adding one-off compatibility shims.
- Treat `docs/roadmaps/README.md` and `docs/roadmaps/g02/README.md` as the
  live queue authority when active shared-surface or consumer-normalization
  work is in flight.
- Treat `docs/architecture/product-guardrails.md`,
  `docs/contracts/001-working-rules.md`, and `docs/specs/` as the strict
  control pack for the active lane.

## Effigy-First Execution

- Start with `effigy tasks` to inspect Underlay's local task surface.
- Prefer `effigy health` as the default repo-owned baseline.
- Use `effigy test --plan` before picking a concrete test runner.
- Prefer local Effigy tasks such as `effigy qa:docs`, `effigy qa:northstar`, `effigy validate`, `effigy rust:check`, `effigy rust:test`, and `effigy test:components`.
- Use `effigy doctor` when you want broader repo scans; it currently includes structural scan findings beyond the task surface itself.
- Fall back to raw `cargo`, `bun`, or `vitest` only when the needed operation is not represented in `effigy.toml`.

For first-time local bring-up from outside this repo:
- use `effigy bootstrap git@github.com:inflatable-cookie/underlay.git`

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
- `./docs/architecture/product-guardrails.md`
- `./docs/contracts/001-working-rules.md`
- `./docs/roadmaps/README.md`
- `./docs/roadmaps/g02/README.md`
- `./docs/guides/README.md`
- `./docs/guides/000-overview.md`
- `./docs/guides/172-agents-files.md`

## Internal Writing Style

Use the repo-local style reference for internal work and normal replies:

- `docs/policy/internal-writing-style.md`
