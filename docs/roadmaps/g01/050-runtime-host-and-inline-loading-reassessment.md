# 050 - Runtime Host And Inline Loading Reassessment

Status: Complete
Owner: Platform
Created: 2026-03-27
Depends on: 049

## Overview

`g01.049` finished the reusable reorder-session workflow migration and retired
public Underlay `ReorderableList`.

The remaining explicit generic holds are now purely runtime-facing:

- Underlay `ToastHost`

These should not be treated as one blended migration family. They represent two
different contracts:

- `ToastHost` is a store-driven host/runtime shell around toast orchestration,
  dismissal policy, and layout.

The goal of this roadmap was to split those boundaries explicitly, move the
inline loading contract into Poodle, and leave only the smaller toast-host
runtime question behind instead of reopening another vague “remaining tail”
wave.

## Research Basis

- `ts/src/components/ToastHost.svelte`
- `../poodle/packages/svelte/composites/src/ToastStack.svelte`
- `../poodle/packages/svelte/composites/src/PageLoading.svelte`
- representative callers:
  - `../underlay-reference/acme-admin/src/routes/(app)/+layout.svelte`
  - `../contact-patch/cp-admin/src/routes/(app)/+layout.svelte`
  - `../underlay-reference/acme-front/src/routes/(app)/+layout.svelte`
  - `../underlay-reference/acme-admin/src/routes/(app)/media/+page.svelte`
  - `../contact-patch/cp-admin/src/routes/(app)/system/jobs/+page.svelte`
  - `../acowtancy/dairy/src/routes/(app)/system/jobs/+page.svelte`

## Decision Summary

- `ToastHost` and `PageLoading` should be treated as separate runtime
  capability decisions inside the same reassessment wave.
- `ToastHost` is the smaller boundary. The live caller set is concentrated in
  app-root layouts that pair `createToastStore()` with a fixed-position host
  over presentational toast items.
- `PageLoading` was the stronger first capability target. The live caller set
  was much broader, and the active apps were using an inline loading shell
  while current Poodle `PageLoading` was still a full-viewport modal overlay.
- The smallest honest capability move was not a second loading component in
  Poodle. It was an inline presentation mode on current Poodle `PageLoading`
  so the same surface could cover both overlay and embedded/page-body loading.
- `PageLoading` is now retired from the public Underlay surface. The live app
  caller family and retained Underlay shells now use direct Poodle
  `PageLoading presentation="inline"`.
- `ToastHost` remains the only meaningful generic runtime hold still left in
  Underlay after this wave.

## Consumer Upgrade Impact

- Consumer apps should expect this wave to separate “toast runtime host” from
  “inline loading state” rather than replacing both through one shared recipe.
- If Poodle expands:
  - `ToastHost` migration would likely move app layouts to direct Poodle host +
    stack composition.
  - `PageLoading` migration would likely move route and panel loading states to
    a direct Poodle inline loader surface.
- Do not add new Underlay compatibility wrappers during this wave.

## Capability Split

### ToastHost

Current Underlay value:

- subscribes directly to `ToastStore`
- drives auto-dismiss timers
- preserves sticky danger/error toasts
- renders a fixed-position host layout
- exposes dismiss wiring directly to the store

Current Poodle `ToastStack` already owns:

- presentational toast stack
- dismiss and action events
- size/density presentation

Open question:

- should Poodle own a small store-aware host, or should apps keep local store
  orchestration and just use `ToastStack` directly?

Current live shape:

- concentrated in app-root layouts
- depends on `createToastStore()` + context wiring
- live caller footprint is small compared with `PageLoading`

### PageLoading

Current Underlay value:

- inline centered loading shell
- small message + spinner contract
- safe for page body, detail sections, and embedded panels
- no backdrop
- no progress requirement
- no cancel affordance

Current Poodle `PageLoading` already owns:

- modal overlay
- optional progress
- cancel affordance
- richer presentational shell and spinner/progress treatment

Open question:

- should Poodle absorb an inline variant, or should it expose a second simpler
  loading surface instead of stretching the modal overlay contract?

Current live shape:

- used across route bodies, detail routes, tabs, and embedded panels
- current live caller sweep is much broader than `ToastHost`
- strongest representative families are admin media/account/system pages plus
  front-app authenticated layouts

Decision for 50.2:

- Poodle should absorb an inline variant on the existing `PageLoading` surface.
- Do not create a second parallel loading component unless the inline and modal
  contracts diverge further later.
- The first proof family should be the grouped admin media/account/system routes
  because they cover:
  - plain route-body loading
  - detail-page loading
  - embedded subsection loading
  - a broad enough caller set to prove the inline mode without immediately
    pulling Dairy into the first migration batch

## Likely Implementation Surface

- Underlay:
  - `ts/src/components/ToastHost.svelte`
  - `ts/src/components/PageLoading.svelte`
  - `ts/src/components/index.ts`
  - `ts/src/components/index.d.ts`
  - `contracts/ui/poodle-adoption-underlay-surface-groups.json`
  - `docs/guides/100-frontend-web.md`
  - `docs/guides/110-admin.md`
- Poodle:
  - `../poodle/packages/svelte/composites/src/ToastStack.svelte`
  - `../poodle/packages/svelte/composites/src/PageLoading.svelte`

## Batch 50.1 - Runtime Contract Reset

- [x] Re-check the actual Underlay-vs-Poodle boundary for `ToastHost`.
- [x] Re-check the actual Underlay-vs-Poodle boundary for `PageLoading`.
- [x] Record the split explicitly in roadmap front doors and the durable
      surface inventory.
- [x] Pick the stronger next runtime target after the split is written down.

Completed in 50.1:
- The live caller sweep confirms `ToastHost` and `PageLoading` are different
  runtime contracts and should not be migrated as one blended tail.
- `ToastHost` is now explicitly the smaller later runtime-host decision:
  app-root layout host + store orchestration over presentational toast UI.
- `PageLoading` is now explicitly the lead implementation target because the
  live caller family is far broader and the gap against Poodle is clearer:
  inline centered page/panel loading versus the current modal overlay Poodle
  surface.

## Batch 50.2 - Inline Loading Capability Design

- [x] Compare retained Underlay `PageLoading` against current Poodle
      `PageLoading`.
- [x] Define the smallest honest inline-loading expansion instead of opening a
      second overlapping Poodle loading component.
- [x] Pick the first broad proof caller family.

Completed in 50.2:
- The strict contract comparison confirms that Underlay `PageLoading` is not a
  separate product workflow shell. It is the inline/embedded presentation mode
  missing from current Poodle `PageLoading`.
- The smallest honest expansion is now explicit: add an inline presentation
  mode to Poodle `PageLoading` while preserving the current overlay mode for
  full-screen blocking states.
- The first proof family is fixed:
  - `acme-admin` media routes
  - `acme-admin` account routes
  - `acme-admin` system routes
  - then mirror to `cp-admin` before taking the broader Dairy wave

## Batch 50.3 - Poodle Inline Loading Stage 1

- [x] Expand Poodle `PageLoading` with an inline presentation mode.
- [x] Update the Poodle contract docs and specimen.
- [x] Migrate the grouped `acme-admin` media/account/system proof family.

Completed in 50.3:
- Poodle `PageLoading` now supports `presentation="inline"` alongside the
  existing overlay treatment.
- The first proof family is migrated and green across `acme-admin`:
  - media routes
  - account routes
  - system routes
- The remaining runtime wave is narrower now:
  - mirror the same inline-loading migration into `cp-admin`
  - then take the broader `dairy` sweep
  - keep `ToastHost` as the later smaller runtime-host decision

## Batch 50.4 - Broad Consumer Sweep And Underlay Retirement

- [x] Mirror the inline-loading migration into the grouped `cp-admin`
      media/account/system family.
- [x] Sweep the broader `dairy` caller family and clear stale retirement
      fallout that was obscuring the real runtime signal.
- [x] Migrate the remaining live `acme-admin`, `acme-front`, and `cp-admin`
      `PageLoading` residue.
- [x] Move retained Underlay internal callers onto direct Poodle
      `PageLoading`.
- [x] Retire the public Underlay `PageLoading` surface and its dedicated tests.

Completed in 50.4:
- The grouped `cp-admin` media/account/system family is migrated to direct
  Poodle `PageLoading presentation="inline"`.
- The broad `dairy` runtime sweep is complete and green after clearing stale
  `OrderBy` and `ReorderableList` retirement fallout that surfaced during the
  loading migration.
- The remaining `acme-admin`, `acme-front`, and final `cp-admin` edit-route
  residue is migrated to direct Poodle `PageLoading`.
- Retained Underlay shells like `AutonomousList` and `AiRoutingAdmin` now use
  direct Poodle `PageLoading`.
- Public Underlay `PageLoading` is retired: the component file, export, and
  dedicated tests are gone, and the live residue scan is clean.

## Completion

`g01.050` is complete.

The only meaningful generic runtime hold still left in Underlay after this
wave is `ToastHost`.
