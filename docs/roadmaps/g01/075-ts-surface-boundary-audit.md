# 075 - TS Surface Boundary Audit

Status: Complete
Owner: Platform
Created: 2026-03-30
Depends on: 074

## Overview

The Svelte contraction line is complete, but `ts/src/` is still a broad mixed
surface. This audit classifies what remains across the TypeScript side of
Underlay:

- what is intentionally retained
- what already overlaps Poodle
- what could become future Poodle capability work
- what should become a standalone package rather than stay under the general
  Underlay umbrella
- what cleanup remains even if no extraction happens immediately

## Research Basis

- `ts/src/index.ts`
- `package.json` exports
- namespace barrels:
  - `ts/src/patterns/index.ts`
  - `ts/src/nightfire/index.ts`
  - `ts/src/client/index.ts`
- `ts/src/server/index.ts`
- `ts/src/utils/index.ts`
- representative boundary files:
  - `ts/src/patterns/forms.ts`
  - `ts/src/patterns/storage.ts`
  - `ts/src/patterns/navigation.ts`
  - `ts/src/patterns/authenticated-data.svelte.ts`
  - `ts/src/patterns/toasts.ts`
  - `ts/src/patterns/media-upload-flow.svelte.ts`
  - `ts/src/patterns/passkey.svelte.ts`
  - `ts/src/client/http.ts`
  - `ts/src/client/sveltekit.ts`
  - `ts/src/server/csp.ts`
  - `ts/src/nightfire/index.ts`
- consumer usage signal across `underlay-reference`, `contact-patch`, and
  `acowtancy`

## Surface Snapshot

- `patterns`: 89 files, 19 `.svelte`, 15 `.svelte.ts`
- `nightfire`: 35 files, 12 `.svelte`
- `client`: 12 files
- `embed`: 11 files
- `server`: 7 files
- `utils`: 5 files
- `validation`: 1 file at audit time

Consumer repo import signal:

- `@decodelabs/underlay/patterns`: heavy live usage
- `@decodelabs/underlay/client`: meaningful live usage
- `@decodelabs/underlay/nightfire`: real but concentrated usage
- `@decodelabs/underlay/server`: light but real usage
- `@decodelabs/underlay/utils`: small usage
- `@decodelabs/underlay/embed`: no current consumer usage found
- `@decodelabs/underlay/validation`: no current consumer usage found at audit time

## Findings

### 1. There is no broad hidden “already ported to Poodle” TS tail left

The big migration debt was the Svelte surface. That is now gone. The remaining
TS surface is mostly:

- controller/state helpers
- SvelteKit/browser integration
- auth/navigation/storage utilities
- Nightfire editor/runtime code
- media/embed/CSP helpers

So the next reduction line should not assume that “more of this belongs in
Poodle” by default.

### 2. `patterns` is now two different things at once

At audit time, `patterns/` mixed:

- true retained workflow/shell UI
  - `LoginPage`
  - `ForgotPasswordFlow`
  - `PasswordRequirements`
  - `SpaFormShell`
  - `DetailMeta*`
- app/runtime helpers
  - `forms.ts`
  - `storage.ts`
  - `navigation.ts`
  - `authenticated-data.svelte.ts`
  - `pagination.svelte.ts`
  - `keyboard-shortcuts.svelte.ts`
  - `timezone.svelte.ts`
  - `passkey.svelte.ts`
  - `media-upload-flow.svelte.ts`
  - relation-selector helper layer

That is the main structural smell left in `ts/src`.

### 3. Only a small subset of remaining TS helpers look like future Poodle work

The strongest future Poodle-adjacent candidates are the helpers that are
directly coupled to Poodle-owned runtime surfaces:

- `toasts.ts` / `useToasts.ts`
  - Poodle owns `ToastHost`
  - Underlay still owns the default toast store/context shape
  - this is the clearest remaining helper/runtime split that could move

- possibly small generic list/runtime controllers over Poodle list surfaces:
  - `batch-selection.svelte.ts`
  - `list-controller.svelte.ts`
  - `pagination.svelte.ts`
  - `reorder-controller.svelte.ts`

But even there, the bar should be high. Those helpers are closer to app/runtime
state orchestration than design-system UI. They are not automatic Poodle work.

### 4. The stronger extraction candidates are standalone libraries, not Poodle

The clearest standalone-package candidates are:

- `embed/`
  - generic, framework-light, and not currently tied to retained Underlay UI
  - low current usage, which makes it easier to split without coordinated UI
    churn

- `client/`
  - generic HTTP/auth/query/navigation helpers
  - already exported as its own public sub-surface
  - logically separate from UI concerns

- `server/`
  - CSP/security header utilities
  - clearly server-only
  - could become a focused server package if it grows

- `nightfire/`
  - already behaves like its own package in everything but npm package
    identity
  - editor/runtime concerns are distinct from the rest of Underlay

### 5. Some helpers should likely stay in Underlay, but under a better namespace

These are useful shared helpers, but they are not good Poodle candidates:

- `storage.ts`
- `navigation.ts` and related navigation state helpers
- `authenticated-data.svelte.ts`
- `passkey.svelte.ts`
- `keyboard-shortcuts.svelte.ts`
- `timezone.svelte.ts`
- `selection-history.ts`
- `local-search.ts` / `drilldown-search.ts`

They still look like legitimate shared app/runtime helpers. The problem is
placement, not legitimacy. Over time they would fit better under a dedicated
runtime/helpers package or namespace than under `patterns`.

### 6. `validation/` was too small to justify a public package boundary

`validation/index.ts` was only a handful of auth-form schemas. It did not look
like a real standalone library.

Current judgment:

- do not extract it as-is
- prefer app-owned schemas plus retained orchestration helpers
- the dead validation export can be removed cleanly

### 7. The root barrel is still broader than it should be

`ts/src/index.ts` still re-exports:

- `patterns`
- `nightfire`
- `styles`
- `client`

That makes the root package feel like a catch-all convenience surface rather
than a carefully curated boundary.

The strongest cleanup here is not immediate deletion. It is to stop growing the
root barrel and consider whether `client` and `nightfire` should continue to be
re-exported from root at all.

## Classification

### Deliberate retained Underlay surface

- `patterns` workflow/shell UI
- `patterns` runtime helpers that are still genuinely shared
- `client`
- `server`
- `nightfire`

### Strong future Poodle challenge candidates

  - `patterns/toasts.ts`
  - `patterns/useToasts.ts`

### Better standalone-library candidates than Poodle candidates

- `embed` at audit time
- `client`
- `nightfire`
- possibly `server` if it grows beyond CSP/security headers

### Cleanup / reshaping targets

- split `patterns` conceptually into:
  - workflow/shell UI
  - runtime/browser helpers
- reconsider the broad root barrel
- remove the dead `validation` surface instead of pretending it is already a
  stable package

## Recommended Follow-On Order

1. Separate retained workflow shells from runtime/browser helper exports.
2. Retire the dead `embed` package surface if no live consumers appear.
3. Reassess the retained toast runtime helper boundary.
   - This is mostly package/API hygiene, not a Poodle migration.

## Next Task

If we continue immediately, the strongest next meaningful batch is a focused
toast-runtime helper audit around `patterns/toasts.ts` and `patterns/useToasts.ts`,
because that is the clearest remaining TS helper boundary where Poodle may now
be the more honest owner.
