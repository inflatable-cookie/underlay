# 058 - AutonomousList Successor Wave

Status: Complete
Owner: Platform
Created: 2026-03-28
Depends on: 052, 054, 055, 057

## Overview

`g01.057` finished the `DropdownMenu` successor wave and retired public
Underlay `DropdownMenu`.

That leaves `AutonomousList` as the strongest remaining obvious-equivalent
shared shell to challenge next:

- the generic list infrastructure it depended on now lives in Poodle
- retained Underlay `AutonomousList` already composes over Poodle
  `ListCard`, `BulkActionBar`, `Pagination`, `ReorderableList`, and
  `PageHeader`
- the remaining question is no longer primitive parity; it is whether the
  shell still owns enough reusable orchestration to justify staying public in
  Underlay

This wave exists to compare the retained `AutonomousList` shell against the
now-expanded Poodle list stack, classify what still belongs in shared Underlay
composition, and identify any smaller follow-on Poodle gaps without preserving
another monolithic generic wrapper by inertia.

## Research Basis

- Underlay:
  - `ts/src/patterns/AutonomousList/AutonomousList.svelte`
  - `ts/src/patterns/AutonomousList/`
- Poodle:
  - `../poodle/packages/svelte/primitives/src/ListCard.svelte`
  - `../poodle/packages/svelte/primitives/src/BulkActionBar.svelte`
  - `../poodle/packages/svelte/primitives/src/Pagination.svelte`
  - `../poodle/packages/svelte/primitives/src/Menu.svelte`
  - `../poodle/packages/svelte/composites/src/ReorderableList.svelte`
  - `../poodle/packages/svelte/composites/src/PageHeader.svelte`
- retained/shared callers:
  - `ts/src/patterns/AiRoutingAdmin.svelte`
  - `ts/src/patterns/DetailPageShell/DetailPageShell.svelte`
- consumer caller families:
  - `underlay-reference/acme-admin/src/routes/(app)/media/+page.svelte`
  - `underlay-reference/acme-admin/src/routes/(app)/projects/+page.svelte`
  - `contact-patch/cp-admin/src/routes/(app)/media/+page.svelte`
  - representative `acowtancy/dairy` list pages still routed through the shell

## Decision Summary

- `AutonomousList` is now the strongest remaining obvious-equivalent
  structural shell after `DropdownMenu`.
- The right question is not whether Poodle needs another monolithic list
  shell. The right question is which remaining behaviors are still genuinely
  reusable shell composition:
  - filter orchestration
  - route/query wiring
  - selection/reorder mode switching
  - empty/loading/error assembly
  - toolbar/header assembly over already-migrated Poodle parts
- The wave should prefer collapsing behavior into direct app composition or
  smaller Poodle additions rather than preserving a broad generic Underlay
  wrapper unchanged.

## Consumer Upgrade Impact

- Consumers should expect `AutonomousList` to be challenged as a shared shell,
  not treated as a settled retained boundary.
- If the live contract reveals only a small amount of reusable orchestration,
  push that into Poodle or local app composition instead of retaining another
  large public Underlay wrapper.
- Keep product-specific list workflow, route refresh behavior, and domain
  mutations outside any new generic successor surface.

## Planned Batches

## Batch 58.1 - Strict Contract Review

- [x] Audit retained `AutonomousList` against the now-expanded Poodle list
      stack and the grouped live caller family.
- [x] Classify the remaining behavior into:
  - direct Poodle composition
  - retained Underlay shell-only concerns
  - smaller follow-on Poodle gaps
- [x] Decide whether the next broad batch is shell retirement,
      shell-narrowing/internalization, or one focused Poodle follow-on.

Completed in 58.1:
- The live caller sweep is much smaller than expected: there are no direct app
  callers of `AutonomousList`, `createAutonomousListState`, or the exported
  `AutonomousList` types across `acme-admin`, `cp-admin`, or `dairy`.
- There are also no retained Underlay shell callers. `AiRoutingAdmin`,
  `DetailPageShell`, and the other remaining public shells do not render
  through `AutonomousList`.
- The current component is already mostly a composition layer over Poodle:
  - `PageHeader`
  - `FilterToolbar`
  - `PageLoading`
  - `EmptyState`
  - `ListCard`
  - `Pagination`
  - `BulkActionBar`
  - `ReorderableList`
  - `AlertDialog`
- The remaining non-Poodle behavior is shell orchestration, not a missing
  generic design-system contract:
  - auth-gated fetch orchestration over `createListController` /
    `createPaginationController`
  - default filter-state wiring
  - selection/reorder mode switching
  - batch-action registration glue
  - reorder success/error lifecycle glue
- No smaller Poodle gap is proven by the current live surface. The strongest
  next batch is public retirement or internalization, not another Poodle
  capability expansion.

## Decision Update

- `AutonomousList` is no longer an active successor wave because it has no live
  consumer-app or retained-shell caller family.
- The remaining question is purely export posture:
  - retire it from the public Underlay surface if docs/examples are the only
    consumers
  - or internalize the helper/state pieces if tests or future local shells
    still need them
- That means the next broad batch should be a clean retirement/internalization
  pass with docs and export cleanup, not another capability review.

## Next Task

Open the next focused cleanup wave on the dead public `PageHeaderMeta` family,
then sweep the export surface and active guides to retire
`PageHeaderMeta`, `PageHeaderMetaRow`, `PageHeaderMetaItem`, and
`PageHeaderMetaSeparator` if the residue scan remains clean.
