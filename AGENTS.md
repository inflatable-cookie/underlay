# Underlay AGENTS

## Scope

Underlay is an app-agnostic shared foundation, not a product app.

## Hard Rules

- Keep APIs and components reusable across consuming apps.
- Prefer stable boundaries and composable primitives over app-specific behavior.
- Keep TypeScript in `ts/` and Rust in `rust/`.
- Do not introduce app-style TS workspaces (`packages/*`) inside Underlay.
- Keep wire JSON conventions aligned with `snake_case` policy.
- Use UUID v7 for new persistent IDs unless there is a clear exception.
- In SQL migrations, do not use `SET search_path`; fully qualify schema/table names.
- In SvelteKit form actions, do not wrap `throw redirect(...)` in `try/catch` that returns `fail(...)`.
- Use `docs/roadmaps/` for active planning and `docs/logs/` for execution evidence.
- Do not leave compatibility shim docs behind when changing doc structure.

## Reference apps in this repo

`reference/` exists to demonstrate patterns. It is not a production app.

- Rust commands in `reference/acme-api/` are fine.
- Avoid TypeScript install/build commands inside `reference/*` unless explicitly requested, due to local `file:` linking constraints.

## Validation

Run checks scoped to changed areas:

```bash
# TypeScript/Svelte
bun check

# Rust
cargo test --all-features
# or targeted crates when iterating
cargo test -p <crate> --all-features
cargo check -p <crate> --all-features
```

## Source of Truth

- Vision: `docs/vision/`
- Architecture: `docs/architecture/`
- Guides: `docs/guides/`
- Roadmaps: `docs/roadmaps/`
- Logs: `docs/logs/`
- AGENTS standardization guide: `docs/guides/172-agents-files.md`
- JSON naming: `docs/guides/071-json-naming.md`
- Error logging: `docs/guides/078-error-logging.md`
