# Svelte UI Kit

Underlay’s UI kit is intentionally minimal and app-agnostic.

Poodle is the canonical home for shared Svelte primitives and generic
composites. Underlay now keeps only the package surfaces that still express
workflow shells, app/runtime orchestration, or retained specialized systems.

## Goals

- Keep foundational UI in Poodle rather than maintaining parallel Underlay
  implementations.
- Keep only the retained Underlay package surfaces that still express real
  workflow, runtime, or specialized-editor value.
- Keep styling token-based so each app can own branding.

## Exports

- `@poodle/svelte-primitives` / `@poodle/svelte-composites`
  - canonical design-system primitives and generic composites
- `@decodelabs/underlay/patterns`
  - retained workflow and page-shell surface
  - auth workflows, `SpaFormShell`, and `DetailMeta*`
- `@decodelabs/underlay/runtime`
  - retained app/runtime helpers, controllers, and framework-agnostic
    browser/state orchestration
  - keep the root barrel stable as a convenience API, but prefer the narrower
    `runtime/*` subpaths for new focused contracts
- `@decodelabs/underlay/utils`
  - small standalone helper surface
  - prefer `utils/webauthn`, `utils/html`, and `utils/sequence` for new code
- `@decodelabs/underlay/client`
  - transport, SvelteKit integration, query parsing, and client-only
    navigation helpers
- `@decodelabs/underlay/nightfire`
  - retained structured-content editor/runtime package
- `@decodelabs/underlay/styles/base.css`
  - minimal CSS variables

## Nightfire namespace

- `@decodelabs/underlay/nightfire` is reserved for structured content renderers extracted from a reference implementation.
