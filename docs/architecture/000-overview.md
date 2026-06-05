# Underlay Architecture Overview

Underlay is a reusable foundation for building full-stack apps with a consistent architecture:

1. **Rust API** implements domain logic and exposes stable HTTP endpoints.
2. **TypeScript API client** provides typed commands and DTOs for the API.
3. **Shared TypeScript and Svelte layer** provides retained workflow shells,
   admin templates, Nightfire editor/runtime surfaces, and app-facing runtime helpers.
4. **SvelteKit apps** (admin + frontend) compose the client and shared UI/runtime
   layer into user-facing products.

## Goals

- Centralise cross-project primitives (IDs, envelopes, error codes, client patterns, UI patterns).
- Preserve flexibility: apps remain free to define their own domain models and UX.
- Provide a referenceable baseline so teams can start a new app with minimal reinvention.
- Move toward reference-grade package boundaries where platform contracts,
  adapters, and tooling are clearly separated.

## Non-Goals

- Underlay is not a product and should not contain app-specific routes, domain nouns, or business rules.
- Underlay should not force a single hosting/runtime setup.

## Source of Truth

- Architectural expectations are defined in `docs/architecture/`.
- Shared wire contracts live in `contracts/openapi/`.

## Rust Crates

Underlay currently provides 31 Rust crates organised into five domains: Core,
Auth, Data & Storage, Infrastructure, and Developer Tools. The auth system
uses an umbrella + provider pattern (`underlay-auth` defines traits;
`underlay-auth-jwt`, `underlay-auth-password`, etc. implement them).

See [010-package-map.md](./010-package-map.md) for the full crate inventory, descriptions, and feature flags.

## Key Docs

- [system-inventory.md](./system-inventory.md) (significant systems and
  contract-planning inventory)
- [010-package-map.md](./010-package-map.md) (full crate inventory and feature flags)
- [020-reference-grade-underlay-architecture.md](./020-reference-grade-underlay-architecture.md) (target reset architecture)
- [015-error-and-envelopes.md](./015-error-and-envelopes.md) (API error and envelope contract)
- [030-ts-api-client.md](./030-ts-api-client.md) (typed client conventions)
- [040-svelte-ui-kit.md](./040-svelte-ui-kit.md) (UI kit scope and constraints)
