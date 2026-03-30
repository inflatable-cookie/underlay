# 060 - MediaActionsMenu Reassessment Wave

Status: Complete
Owner: Platform
Created: 2026-03-28
Depends on: 056, 057, 059

## Overview

`g01.059` finished the dead `PageHeaderMeta` cleanup wave.

The next meaningful public-surface question is `MediaActionsMenu`:

- it still has live wrappers in `acme-admin`, `cp-admin`, and `dairy`
- it looks closer to a shared workflow helper than a generic design-system
  primitive
- but much of its surface may now be reproducible with direct Poodle `Menu`,
  `AlertDialog`, and local media workflow composition

This wave exists to compare the retained Underlay helper against the local app
wrappers and decide whether it still earns a shared public export, should be
narrowed, or can retire entirely.

## Research Basis

- Underlay:
  - `ts/src/components/MediaActionsMenu.svelte`
  - `ts/src/components/index.ts`
- caller and wrapper sweep:
  - `underlay-reference/acme-admin/src/lib/components/MediaActionsMenu.svelte`
  - `contact-patch/cp-admin/src/lib/components/MediaActionsMenu.svelte`
  - `acowtancy/dairy/src/lib/menus/MediaActionsMenu.svelte`
  - representative media detail routes in each app
- adjacent Poodle surfaces:
  - `@poodle/svelte-primitives/Menu`
  - `@poodle/svelte-primitives/AlertDialog`

## Decision Summary

- `MediaActionsMenu` should be challenged as a shared helper, not assumed to
  be a permanent retained media workflow surface.
- The first batch should prove how much shared behavior is still real after the
  local wrappers and earlier menu/dialog migrations.

## Consumer Upgrade Impact

- Do not add new direct consumers of Underlay `MediaActionsMenu` until the
  reassessment is complete.
- Prefer local wrapper composition over expanding the shared surface by
  default.

## Planned Batches

## Batch 60.1 - Strict Contract Review

- [x] Audit the shared `MediaActionsMenu` contract against the local wrappers
      and live media-detail callers in `acme-admin`, `cp-admin`, and `dairy`.
- [x] Separate generic menu/dialog behavior from media-specific workflow and
      route side effects.
- [x] Decide whether the next broad batch is direct retirement, narrowing, or
      one smaller follow-on extraction.

Completed in 60.1:
- The live caller family is narrow and fully mediated by local wrappers:
  - `underlay-reference/acme-admin/src/lib/components/MediaActionsMenu.svelte`
  - `contact-patch/cp-admin/src/lib/components/MediaActionsMenu.svelte`
  - `acowtancy/dairy/src/lib/menus/MediaActionsMenu.svelte`
- Each wrapper is almost identical. The only meaningful differences are:
  - app-specific media client imports
  - auth-token acquisition
  - route navigation after purge
  - optional `sourceContext` wiring in Dairy
- The retained Underlay helper does not expose a missing generic design-system
  gap. It is a workflow helper that bundles:
  - `CopyActionsMenu`
  - local toast success/error copy
  - three confirmation dialogs
  - media-specific action arrays for edit/replace/delete/restore/purge
- The route-level callers are already local and thin. They do not prove that a
- portfolio-wide shared public Underlay surface is still the right boundary.
- The strongest next batch is direct retirement:
  - keep the wrappers local to each app
  - collapse their shared internals onto direct Poodle `Menu` /
    `AlertDialog` plus local clipboard/toast/media-command wiring
  - remove public Underlay `MediaActionsMenu`

## Decision Update

- `MediaActionsMenu` no longer looks like a durable shared Underlay export.
- No smaller Poodle capability push is justified by the live contract.
- The next honest move is one broad retirement batch across the three local
  wrappers plus Underlay export/docs cleanup.

## Next Task

Open the next focused reassessment wave on `ErrorBoundary`, then compare the
remaining public Underlay boundary shell against current app-root usage and the
available Poodle `Callout` / local error-recovery composition so the next batch
can decide whether it still earns a public shared surface.
