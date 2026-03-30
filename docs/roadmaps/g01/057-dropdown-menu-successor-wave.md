# 057 - DropdownMenu Successor Wave

Status: Complete
Owner: Platform
Created: 2026-03-28
Depends on: 052, 056

## Overview

`g01.056` finished the `MediaPicker` successor wave and retired public
Underlay `MediaPicker`.

That leaves a smaller obvious-equivalent residue queue again. The strongest
next focused successor surface is `DropdownMenu`:

- Underlay `DropdownMenu` is a thin interaction wrapper over Bits UI
- Poodle already owns a generic `Menu` primitive
- the remaining Underlay gap looks much smaller than `AutonomousList`, which
  is still a broader list/runtime composition boundary

This wave exists to compare the grouped live `DropdownMenu` caller family
against current Poodle `Menu`, add only the smallest honest generic capability
if needed, migrate the grouped callers, and retire public Underlay
`DropdownMenu`.

## Research Basis

- Underlay:
  - `ts/src/components/DropdownMenu.svelte`
  - `ts/src/patterns/CopyActionsMenu.svelte`
- Poodle:
  - `../poodle/packages/svelte/primitives/src/Menu.svelte`
  - `../poodle/docs/contracts/foundation/menu.md`
- caller sweep:
  - `contact-patch/cp-admin/src/routes/(app)/users/[userId]/+page.svelte`
  - `contact-patch/cp-admin/src/routes/(app)/system/emails/[id]/+page.svelte`
  - `contact-patch/cp-admin/src/routes/(app)/system/scheduled-tasks/[id]/+page.svelte`
  - retained `ts/src/patterns/CopyActionsMenu.svelte`

## Decision Summary

- `DropdownMenu` is the next strongest clean successor wave after
  `MediaPicker`.
- The grouped live caller family is small and concrete.
- The strict comparison shows the likely end state is direct Poodle `Menu`
  composition with one small ergonomic expansion rather than a retained Underlay
  wrapper.
- Current Poodle `Menu` already covers:
  - trigger slot composition
  - open state
  - placement
  - separators
  - disabled items
- The remaining generic gaps are small and explicit:
  - destructive item tone or styling
  - trigger aria-label ergonomics when the trigger is icon-only
  - optional lightweight callback-item mapping for `label` / `onSelect`
    consumers, if keeping that mapping in callers proves too repetitive
- `AutonomousList` is explicitly not the next wave because it remains a
  broader structural and runtime composition question even after the generic
  card/header/batch/pagination/reorder pieces moved into Poodle.

## Consumer Upgrade Impact

- Consumers should expect `DropdownMenu` to collapse onto direct Poodle
  `Menu` composition rather than preserving another long-lived Underlay
  interaction wrapper.
- If Poodle needs a small ergonomic addition for trigger/content composition or
  callback-driven item arrays, add it there instead of retaining the Underlay
  wrapper.
- Do not move route-specific or destructive confirmation workflow into a new
  shared menu shell just to preserve the old API mechanically.

## Planned Batches

## Batch 57.1 - Queue Reset

- [x] Reassess the remaining obvious-equivalent queue after `MediaPicker`
      retirement.
- [x] Confirm `DropdownMenu` is the strongest next focused successor wave.
- [x] Open the focused roadmap and update front doors / durable inventory.

Completed in 57.1:
- `DropdownMenu` is now the active successor wave.
- `AutonomousList` remains a later composition boundary question.

## Batch 57.2 - Poodle Menu Review

- [x] Compare Underlay `DropdownMenu` against current Poodle `Menu`.
- [x] Define the smallest honest generic capability gap, if any.
- [x] Keep route-specific workflow and destructive confirmation outside the
      menu primitive.

Completed in 57.2:
- The grouped live caller family only uses the thin wrapper shape:
  - callback-oriented `items`
  - icon or snippet trigger composition
  - simple end-aligned overlay actions on detail routes
- Poodle `Menu` already owns most of the contract. The remaining reusable gap
  is small enough for one broad implementation wave instead of a longer
  reassessment:
  - destructive item tone/styling
  - icon-trigger aria-label ergonomics
  - optional caller-friendly mapping from callback item arrays into `MenuItem`
    plus `action` event handling
- Route-specific action behavior, delete confirmation, navigation, and
  clipboard side effects remain caller responsibilities and should not be moved
  into the primitive.

## Batch 57.3 - Grouped Migration And Retirement

- [x] Migrate the grouped `cp-admin` detail-route caller family and retained
      `CopyActionsMenu` onto direct Poodle `Menu` composition.
- [x] Retire public Underlay `DropdownMenu` and update docs/inventory once the
      residue scan is honestly clean.

Completed in 57.3:
- Poodle `Menu` now owns the small reusable ergonomics gap:
  - destructive item tone
  - icon-trigger aria-label handling
- Retained `CopyActionsMenu` now composes direct Poodle `Menu`.
- The grouped `cp-admin` detail-route caller family moved onto direct Poodle
  `Menu`.
- The broader Dairy cleanup is now green too after the bounded recovery on
  mixed slot/snippet fallout in the remaining header and shell callers.
- Public Underlay `DropdownMenu` is retired, and live residue scans are clean
  across the active app family.

## Next Task

Open the next focused obvious-equivalent successor wave on
`AutonomousList`, then compare the retained Underlay shell against the now
migrated Poodle list/header/batch/pagination/reorder pieces so the next batch
can decide what still belongs in shared Underlay composition versus direct app
composition over Poodle.
