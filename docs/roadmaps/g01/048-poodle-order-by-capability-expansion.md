# 048 - Poodle OrderBy Capability Expansion

Status: Complete
Owner: Platform
Created: 2026-03-27
Depends on: 047

## Overview

`g01.047` finished the shared table migration and retired public Underlay
`DataTable`. The next honest retained generic hold is `OrderBy`.

Current Poodle `OrderBy` started as a lightweight single-sort toolbar. The
remaining Underlay `OrderBy` surface was broader: it owned an ordered
multi-field sort builder with add/remove, per-field direction, drag
reordering, compact trigger summaries, and caller-owned URL round-tripping of
ordered sort arrays.

The goal of this roadmap is not to copy the old Underlay API mechanically. It
is to define the smallest honest Poodle expansion for multi-field sort-builder
behavior, migrate the active caller family in grouped passes, and retire public
Underlay `OrderBy` once residue is clean.

## Research Basis

- `ts/src/components/OrderBy/OrderBy.svelte`
- `ts/src/components/OrderBy/types.ts`
- `../poodle/packages/svelte/primitives/src/OrderBy.svelte`
- representative callers:
  - `../underlay-reference/acme-admin/src/routes/(app)/media/+page.svelte`
  - `../underlay-reference/acme-admin/src/routes/(app)/projects/+page.svelte`
  - `../underlay-reference/acme-admin/src/routes/(app)/projects/[projectId]/+page.svelte`
  - `../underlay-reference/acme-admin/src/routes/(app)/categories/+page.svelte`
  - `../contact-patch/cp-admin/src/routes/(app)/media/+page.svelte`

## Decision Summary

- `OrderBy` should move into Poodle. The missing behavior is still generic
  browse/sort-builder capability, not app-specific workflow.
- The current gap is larger than a same-name swap but smaller than the table
  runtime that just moved. It is a good next staged capability wave.
- The first proof family should be the media and category/project list pages,
  because they already isolate the real contract:
  - ordered multi-field sort arrays
  - field add/remove
  - per-field direction
  - drag reordering
  - compact trigger summaries
  - URL round-tripping through caller-owned search params

## Consumer Upgrade Impact

- Consumer apps should expect a staged migration from
  `@inflatable-cookie/underlay/components` `OrderBy` to Poodle `OrderBy`.
- The migration shape should be:
  - expand the Poodle contract first
  - migrate the active admin caller family in grouped batches
  - remove the Underlay public surface only after residue is clean
- Do not add new Underlay compatibility wrappers during this wave.

## Capability Gap

Current Underlay `OrderBy` behavior that still needs a Poodle home:

- ordered multi-field sort arrays
- add/remove fields
- per-field direction
- drag reordering
- compact trigger summaries
- bindable full sort-array state
- caller-friendly change callback for full ordered sort arrays

## Likely Implementation Surface

- Poodle:
  - `../poodle/packages/svelte/primitives/src/OrderBy.svelte`
  - `../poodle/packages/svelte/primitives/src/types.ts`
  - Poodle docs / specimen files for sort-builder usage
- Underlay:
  - `ts/src/components/OrderBy/OrderBy.svelte`
  - `contracts/ui/poodle-adoption-underlay-surface-groups.json`
  - `docs/guides/090-ui-kit.md`
- proof callers:
  - media/category/project list pages in `underlay-reference`
  - media list page in `contact-patch`

## Batch 48.1 - Contract Reset

- [x] Confirm the actual Underlay-vs-Poodle `OrderBy` contract gap from code.
- [x] Record the smallest honest staged expansion instead of preserving the
      older wrapper by default.
- [x] Pick the first proof caller family and refresh roadmap front doors.

Completed in 48.1:
- The gap is now explicit rather than treated as a generic retained hold.
- The first proof family is fixed:
  - `acme-admin` media
  - `acme-admin` categories
  - `acme-admin` projects
  - `acme-admin` project detail
  - `cp-admin` media

## Batch 48.2 - Poodle OrderBy Stage 1

- [x] Expand Poodle `OrderBy` for the first honest capability slice:
  - multi-field ordered sort arrays
  - field add/remove
  - per-field direction
  - compact trigger summaries
- [x] Update the Poodle contract docs and specimen.
- [x] Migrate the first proof caller family.

Completed in 48.2:
- Poodle `OrderBy` now owns ordered multi-field sort arrays, field add/remove,
  per-field direction, compact trigger summaries, and compatibility emission
  for the older single-sort signal.
- The first proof family is migrated and green:
  - `acme-admin` media
  - `acme-admin` categories
  - `acme-admin` projects
  - `acme-admin` project detail
  - `cp-admin` media

## Batch 48.3 - Poodle OrderBy Stage 2

- [x] Add drag-reordering ergonomics and finalize the caller-owned array-change
      contract.
- [x] Sweep remaining public Underlay `OrderBy` residue across active apps and
      docs.
- [x] Retire the public Underlay `OrderBy` surface once residue is clean.

Completed in 48.3:
- Poodle `OrderBy` now owns drag-reordering ergonomics in addition to the
  stage-1 multi-field builder contract.
- The active app caller family is already on direct Poodle `OrderBy`, so the
  remaining residue was only public Underlay export/docs state.
- Public Underlay `OrderBy` is now removed from the component surface and the
  old local implementation files are deleted.

## Next Task

**Complete**
