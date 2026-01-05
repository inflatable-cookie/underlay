# Underlay AGENTS

Underlay is a small, reusable foundation for building full-stack apps with the same core architecture as Acowtancy:

- Rust API + domain crates
- Typed TypeScript API client
- Shared Svelte UI kit
- SvelteKit Admin UI + SvelteKit Frontend UI (as consumers of the shared layers)

This repository is a *framework*, not an app. It should provide stable, app-agnostic primitives and patterns.

## Source of Truth

- Architecture and conventions live in `docs/architecture/`.
- When extracting code from a reference app (e.g. Acowtancy), prefer documenting the boundary first, then implementing the shared module.

## Structure

- `rust/` – Rust crates that can be used by any Rust API.
- `ts/` – TypeScript + Svelte library code (UI primitives, patterns, and client helpers).
- `contracts/` – API contracts (OpenAPI schemas and shared envelope types).
- `docs/` – Underlay architecture and integration guides.

## Design Principles

- **App-agnostic**: avoid project-specific naming, routes, and domain types.
- **Stable boundaries**: prefer small, well-typed interfaces that apps compose.
- **No forced stack choices**: provide *defaults* (axum/sqlx/SvelteKit) but keep them optional where feasible.
- **Compatibility first**: if a pattern is derived from a reference implementation, keep it compatible unless there’s a clear win.

## Error and Response Conventions

- Use a consistent error envelope and stable error codes (string codes like `auth.forbidden`, `resource.not_found`).
- Keep DTO envelope shapes shared between Rust and TS via `contracts/openapi/`.

## SvelteKit Form Actions (important)

When using SvelteKit form actions, do not wrap `throw redirect(...)` inside a `try`/`catch` that returns `fail(...)`. Perform redirects after successful `await` calls, and only return `fail(...)` for genuine errors.
