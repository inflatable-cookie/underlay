# Svelte UI Kit

Underlay’s UI kit is intentionally minimal and app-agnostic.

## Goals

- Provide primitive components (buttons, cards, inputs) with sensible defaults.
- Provide higher-level patterns that repeat across apps (filters, list pages, error banners), but keep them generic.
- Keep styling token-based so each app can own branding.

## Exports

- `@decodelabs/underlay/components` – primitives (e.g. `Button`, `Card`).
- `@decodelabs/underlay/patterns` – higher-level, app-agnostic patterns.
- `@decodelabs/underlay/styles/base.css` – minimal CSS variables.

## Nightfire namespace

- `@decodelabs/underlay/nightfire` is reserved for structured content renderers extracted from a reference implementation.
