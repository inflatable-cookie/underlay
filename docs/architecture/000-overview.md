# Underlay Architecture Overview

Underlay is a reusable foundation for building full-stack apps with a consistent architecture:

1. **Rust API** implements domain logic and exposes stable HTTP endpoints.
2. **TypeScript API client** provides typed commands and DTOs for the API.
3. **Shared Svelte UI kit** provides presentational components and app-agnostic UI patterns.
4. **SvelteKit apps** (admin + frontend) compose the client and UI kit into user-facing products.

## Goals

- Centralise cross-project primitives (IDs, envelopes, error codes, client patterns, UI patterns).
- Preserve flexibility: apps remain free to define their own domain models and UX.
- Provide a referenceable baseline so teams can start a new app with minimal reinvention.

## Non-Goals

- Underlay is not a product and should not contain app-specific routes, domain nouns, or business rules.
- Underlay should not force a single hosting/runtime setup.

## Source of Truth

- Architectural expectations are defined in `docs/architecture/`.
- Shared wire contracts live in `contracts/openapi/`.

## Key Docs

- `docs/architecture/015-error-and-envelopes.md` (API error and envelope contract)
- `docs/architecture/030-ts-api-client.md` (typed client conventions)
- `docs/architecture/040-svelte-ui-kit.md` (UI kit scope and constraints)
