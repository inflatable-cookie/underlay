# 051 - Poodle Toast Host Runtime Expansion

Status: Complete
Owner: Platform
Created: 2026-03-28
Depends on: 050

## Overview

`g01.050` finished the inline loading migration and retired public Underlay
`PageLoading`.

The only meaningful generic runtime hold left in Underlay is now `ToastHost`.

This wave exists to resolve that final hold cleanly instead of leaving a small
runtime shell stranded in Underlay after the broader generic surface migration
is already complete.

The active question is narrow:

- should Poodle absorb a small store-aware toast host/runtime shell over
  presentational `ToastStack`
- or should app layouts move to explicit local store orchestration over
  `ToastStack` without adding another shared Poodle layer

## Research Basis

- `ts/src/patterns/toasts.ts`
- `../poodle/packages/svelte/composites/src/ToastHost.svelte`
- `../poodle/packages/svelte/composites/src/ToastStack.svelte`
- representative live callers:
  - `../underlay-reference/acme-admin/src/routes/(app)/+layout.svelte`
  - `../contact-patch/cp-admin/src/routes/(app)/+layout.svelte`
- active guide/example residue:
  - `docs/guides/110-admin.md`
  - `docs/guides/code/110-admin/(app)/+layout.svelte`

## Decision Summary

- `ToastHost` was the smallest remaining generic runtime capability gap in the
  Underlay public surface.
- The live caller family is concentrated and honest: root layouts create a
  shared toast store, provide it via context, and render a fixed-position host
  that applies dismiss policy.
- The retained Underlay value is runtime behavior, not just styling:
  - subscribes directly to `ToastStore`
  - manages auto-dismiss timers
  - preserves sticky error toasts
  - wires dismiss back to the store
  - owns fixed-position host layout
- Current Poodle `ToastStack` is already the presentational half of the
  contract, so the next implementation move should stay small and runtime-first
  rather than inventing a parallel toast UI surface.

## Consumer Upgrade Impact

- Consumer app root layouts should expect this wave to migrate from retained
  Underlay `ToastHost` to a direct Poodle runtime-host shape.
- Do not add new Underlay compatibility wrappers during this wave.
- If Poodle expands, the intended outcome is:
  - apps keep their existing `createToastStore()` orchestration
  - apps render a Poodle-owned host over `ToastStack`
  - Underlay drops the public `ToastHost` export entirely

## Current Contract Split

### Underlay `ToastHost`

Former retained value:

- store subscription
- timer lifecycle for auto-dismiss
- sticky danger/error handling
- dismiss wiring
- fixed-position viewport host

Former live shape:

- only 2 live app callers
- both are root layouts
- both already own `createToastStore()` and context setup

### Poodle `ToastStack`

Current owned value:

- presentational stack rendering
- per-item dismiss and action events
- tone, size, and density presentation

Former gap:

- no store-aware host/runtime shell
- no timer policy
- no fixed viewport host wrapper

## Likely Implementation Surface

- Underlay:
  - `ts/src/components/index.ts`
  - `ts/src/components/index.d.ts`
  - `contracts/ui/poodle-adoption-underlay-surface-groups.json`
  - `docs/guides/110-admin.md`
  - `docs/guides/code/110-admin/(app)/+layout.svelte`
- Poodle:
  - `../poodle/packages/svelte/composites/src/ToastStack.svelte`
  - `../poodle/packages/svelte/composites/src/types.ts`

## Batch 51.1 - Runtime Host Contract Reset

- [x] Re-check the retained Underlay `ToastHost` contract against current
      Poodle `ToastStack`.
- [x] Re-check the actual live caller surface in root app layouts.
- [x] Record the focused runtime-host wave in roadmap front doors and the
      durable inventory.
- [x] Fix the first proof family for the later migration batch.

Completed in 51.1:
- The final generic runtime hold is now isolated cleanly: `ToastHost`.
- The live caller family is fixed to the two app-root layouts in
  `acme-admin` and `cp-admin`.
- The next honest move is a focused runtime-host decision over Poodle
  `ToastStack`, not another broad retained-surface reassessment.

## Batch 51.2 - Poodle Runtime Host Landing And Underlay Retirement

- [x] Define the smallest honest Poodle runtime-host expansion over
      `ToastStack`.
- [x] Land store input shape, auto-dismiss policy, sticky error handling,
      dismiss wiring, and viewport positioning in Poodle.
- [x] Migrate the grouped `acme-admin` and `cp-admin` root-layout callers.
- [x] Update active guides/examples to teach direct Poodle `ToastHost`.
- [x] Retire the public Underlay `ToastHost` export.

Completed in 51.2:
- Poodle now owns a store-aware `ToastHost` over presentational `ToastStack`.
- The new runtime shell accepts the existing app toast-store shape, applies
  auto-dismiss policy, preserves sticky danger/error treatment, and owns the
  fixed-position viewport host.
- The grouped `acme-admin` and `cp-admin` root-layout proof family is migrated
  and green.
- Active guidance now teaches direct Poodle `ToastHost` in the admin shell
  pattern.
- Public Underlay `ToastHost` is retired.

## Completion

`g01.051` is complete.

There are no meaningful generic Underlay holds left after this wave.
