# TypeScript API Client

The TS client layer is the bridge between the Rust API and SvelteKit apps.

## Principles

- Centralise HTTP concerns (base URL, headers, auth token, error parsing).
- Expose app-specific *commands* as small functions grouped by domain.
- Keep DTOs aligned with the API contract (ideally generated from OpenAPI).

## Underlay exports

- `@decodelabs/underlay/client` provides:
  - DTO envelope types (`ListResponse`, `SingleResponse`, `ErrorEnvelope`).
  - A small `HttpClient` abstraction to build command modules on.

## Integration (in an app)

- Create an app-local client package (or folder) that imports Underlay’s `HttpClient`.
- Implement command modules (`learning`, `content`, `assessment`, etc.) on top.
- Keep Svelte components declarative: call commands in `load` functions and form actions, not inline in components.
