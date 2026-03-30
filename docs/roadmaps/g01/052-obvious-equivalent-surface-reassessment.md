# 052 - Obvious Equivalent Surface Reassessment

Status: Complete
Owner: Platform
Created: 2026-03-28
Depends on: 051

## Overview

`g01.051` finished the final generic runtime-host migration and retired public
Underlay `ToastHost`.

That closes the generic hold list, but the remaining public Underlay surface
still includes several components and patterns that already have direct Poodle
equivalents or are now thin composition shells over already-migrated Poodle
parts.

This wave exists to challenge that residue directly instead of treating the
remaining export list as implicitly justified just because the earlier generic
hold queue is complete.

The first focus set is:

- `DropdownMenu`
- `MediaPicker`
- `LogList`
- `PageHeader`
- `AutonomousList`
- `BatchActionBar`

## Research Basis

- Underlay:
- `ts/src/components/DropdownMenu.svelte`
- `ts/src/components/MediaPicker.svelte`
- `ts/src/components/LogList.svelte`
- `ts/src/patterns/PageHeader.svelte`
  - `ts/src/patterns/AutonomousList/AutonomousList.svelte`
- Poodle:
  - `../poodle/packages/svelte/composites/src/MediaPicker.svelte`
  - `../poodle/packages/svelte/composites/src/LogList.svelte`
  - `../poodle/packages/svelte/composites/src/PageHeader.svelte`
  - `../poodle/packages/svelte/composites/src/ListContainer.svelte`
  - `../poodle/packages/svelte/primitives/src/Menu.svelte`
  - `../poodle/packages/svelte/primitives/src/Callout.svelte`
- representative live callers:
  - `../underlay-reference/acme-admin/src/routes/(app)/+page.svelte`
  - `../underlay-reference/acme-admin/src/routes/(app)/system/audit/+page.svelte`
  - `../contact-patch/cp-admin/src/routes/(app)/+page.svelte`
  - `../contact-patch/cp-admin/src/routes/(app)/system/audit/+page.svelte`
  - `../underlay-reference/acme-admin/src/routes/(app)/users/[userId]/+page.svelte`
  - `../underlay-reference/acme-front/src/routes/(app)/dashboard/+page.svelte`

## Decision Summary

- `Banner` is not a real retained surface. It is already just a thin wrapper
  over Poodle `Callout` and should be treated as direct-retirement work.
- `BatchActionBar` is now the strongest next clean successor wave after
  `LogList` because it has a small grouped live caller family and Poodle
  already owns the base primitive as `BulkActionBar`.
- `MediaPicker`, `LogList`, `PageHeader`, and `FormDialog` are obvious
  successor-surface candidates because Poodle already exports direct same-name
  replacements.
- `DropdownMenu` is a smaller but real reassessment target:
  - the current Underlay surface still wraps Bits UI directly
  - the likely end state is direct Poodle `Menu` composition, or a very small
    helper capability in Poodle if the callback-oriented `items` contract still
    earns shared ownership
- `AutonomousList` is the biggest remaining composition question in this set:
  - it is already built mostly from migrated Poodle pieces
  - it may collapse into direct app composition over Poodle `ListContainer`,
    `PageHeader`, `FilterToolbar`, `Pagination`, `ReorderableList`, and local
    state controllers rather than needing a new single Poodle replacement

## Consumer Upgrade Impact

- Consumer apps should expect this wave to remove more public Underlay UI by
  migrating to direct Poodle components or explicit Poodle-based composition.
- Do not add new Underlay compatibility wrappers during this wave.
- If a migration reveals missing generic behavior, extend Poodle rather than
  preserving a second long-lived Underlay wrapper.

## Initial Classification

### Direct Retirement Candidates

- `Banner`

### Direct Same-Name Poodle Successor Candidates

- `MediaPicker`
- `LogList`
- `PageHeader`

### Composition / Small Capability Review Candidates

- `DropdownMenu`
- `AutonomousList`

### Focused Primitive-Successor Wave

- `BatchActionBar`

## Likely Implementation Surface

- Underlay:
  - `ts/src/components/index.ts`
  - `ts/src/components/index.d.ts`
  - `ts/src/patterns/index.ts`
  - `contracts/ui/poodle-adoption-underlay-surface-groups.json`
  - `docs/guides/090-ui-kit.md`
  - `docs/guides/110-admin.md`
- Poodle:
  - `../poodle/packages/svelte/composites/src/MediaPicker.svelte`
  - `../poodle/packages/svelte/composites/src/LogList.svelte`
  - `../poodle/packages/svelte/composites/src/PageHeader.svelte`
  - `../poodle/packages/svelte/composites/src/ListContainer.svelte`
  - `../poodle/packages/svelte/primitives/src/Menu.svelte`
  - `../poodle/packages/svelte/primitives/src/Callout.svelte`

## Batch 52.1 - Reclassification Reset

- [x] Re-snapshot the post-`g01.051` public Underlay surface.
- [x] Identify the obvious-equivalent residue that should not default to
      “retained”.
- [x] Split that residue into direct-retirement, direct-successor, and
      composition/capability buckets.
- [x] Record the next active wave in roadmap front doors and durable inventory.

Completed in 52.1:
- The remaining public Underlay surface is now re-sorted by real migration
  plausibility instead of historical hold language.
- `Banner` is now explicitly a direct-retirement candidate.
- `MediaPicker`, `LogList`, `PageHeader`, and `FormDialog` are now the lead
  same-name successor family.
- `DropdownMenu` and `AutonomousList` remain the narrower later composition
  questions after that first broad obvious-equivalent batch.

## Batch 52.2 - FormDialog Retirement

- [x] Expand Poodle `FormDialog` into a real shell surface with subtitle,
      success, width, and custom action support.
- [x] Migrate the grouped live `FormDialog` caller family in `acme-admin`,
      `cp-admin`, and `acme-front`.
- [x] Retire the public Underlay `FormDialog` export and dead story/demo
      residue.

Completed in 52.2:
- Poodle `FormDialog` now covers both the built-in submit/cancel path and the
  caller-owned modal-form shell path.
- The grouped live caller family is migrated and green across `acme-admin`,
  `cp-admin`, and `acme-front`.
- Underlay no longer exports `FormDialog`, and the old pattern story/demo
  residue is gone.

## Batch 52.3 - Banner Retirement

- [x] Remove the public Underlay `Banner` export.
- [x] Move retained internal `PageHeader` banner rendering onto direct Poodle
      `Callout`.

Completed in 52.3:
- Underlay no longer exports `Banner`.
- The retained `PageHeader` shell now composes Poodle `Callout` directly for
  banner messaging instead of preserving a second wrapper layer.

## Batch 52.4 - Post-LogList Reset

- [x] Reassess the remaining obvious-equivalent surface after `g01.053`.
- [x] Confirm the strongest next focused successor wave.
- [x] Open the next follow-on roadmap and update front doors / durable
      inventory.

Completed in 52.4:
- `LogList` is no longer part of the remaining obvious-equivalent queue.
- `BatchActionBar` is now the strongest next clean successor wave and is
  opened as `g01.054`.
- `PageHeader`, `MediaPicker`, `DropdownMenu`, and `AutonomousList` remain
  later reassessment targets because they still involve broader shell or
  workflow decisions than the next clean batch.

## Final Outcome

The focused successor waves opened from `g01.052` are now all complete:

- `g01.053` retired `LogList`
- `g01.054` retired `BatchActionBar`
- `g01.055` retired `PageHeader`
- `g01.056` retired `MediaPicker`
- `g01.057` retired `DropdownMenu`
- `g01.058` retired `AutonomousList`

That means the obvious-equivalent residue queue is finished. `g01.052` no
longer owns active successor work.

## Next Task

Complete. The obvious-equivalent residue queue is finished; the remaining
public Underlay surface is now retained on purpose rather than waiting on
another direct-successor wave.
