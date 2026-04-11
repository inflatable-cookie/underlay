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

- `@poodle/svelte` / `@poodle/svelte`
  - canonical design-system primitives and generic composites
- `@decodelabs/underlay/patterns`
  - retained workflow and page-shell surface
  - auth workflows and `SpaFormShell`
- `@decodelabs/underlay/runtime/*`
  - retained app/runtime helpers, controllers, and framework-agnostic
    browser/state orchestration via explicit feature subpaths
- `@decodelabs/underlay/utils/*`
  - small standalone helper surface via focused subpaths
  - use `utils/webauthn`, `utils/html`, `utils/sequence`, and `utils/slug`
  - keep broader app-formatting helpers on `utils/i18n` for now; presentational
    display-format helpers that earn shared UI ownership should move to Poodle
- `@decodelabs/underlay/client/*`
  - transport, SvelteKit integration, query parsing, and client-only
    navigation helpers via explicit feature subpaths
- `@decodelabs/underlay/nightfire/*`
  - retained structured-content editor/runtime package via explicit subpaths
- `@decodelabs/underlay/styles/base.css`
  - minimal CSS variables

## Nightfire namespace

- `@decodelabs/underlay/nightfire` is reserved for structured content renderers extracted from a reference implementation.
