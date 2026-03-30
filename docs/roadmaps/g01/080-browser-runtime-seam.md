---
title: Browser Runtime Seam
owner: Codex
status: complete
updated: 2026-03-30
---

# Browser Runtime Seam

## Goal

Confirm the retained browser-facing runtime boundary after the `runtime`
namespace split, and remove any helpers that do not actually belong in a pure
browser bucket.

## Outcome

The retained browser/runtime boundary is now explicit:

- `@decodelabs/underlay/runtime/browser` keeps browser-pure helpers:
  - storage
  - DOM helpers
  - keyboard shortcuts
  - timezone detection/state
- toast-coupled clipboard workflow no longer lives under `runtime/browser`
- `copyToClipboard()` and `copyTextToClipboard()` now belong to
  `@decodelabs/underlay/runtime/feedback`

## Judgment

The main seam was `clipboard.ts`.

`copyTextToClipboard()` is browser-facing, but the higher-level
`copyToClipboard()` helper is not a browser primitive. It bundles clipboard
write plus toast success/error orchestration and therefore belongs with the
retained feedback runtime family rather than the browser bucket.

The remaining `runtime/browser` exports still earn retained Underlay ownership:

- `storage.ts`
  - SSR-safe browser storage wrappers
  - draft persistence and state hydration support used by retained runtime
    helpers
- `timezone.svelte.ts`
  - browser timezone detection and conflict state
- `keyboard-shortcuts.svelte.ts`
  - reusable shortcut registration/orchestration
- `dom.ts`
  - generic DOM/browser helpers

These are runtime/browser helpers, not client transport concerns, so they stay
in Underlay rather than moving to `client`.

## Changes

- removed clipboard exports from `runtime/browser`
- added clipboard exports to `runtime/feedback`
- updated live shared callers and guides to use
  `@decodelabs/underlay/runtime/feedback` for clipboard helpers
- updated docs/front doors so the runtime seam is recorded explicitly

## Next Task

Take the next retained-runtime audit on the data/media seam by reviewing the
current `runtime/data`, `runtime/media`, and `runtime/relations` buckets, then
decide which exports are deliberate retained runtime APIs and which should move
to a more specific package or sub-namespace later.
