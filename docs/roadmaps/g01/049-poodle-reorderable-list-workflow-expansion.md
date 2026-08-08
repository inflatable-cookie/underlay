# 049 - Poodle ReorderableList Workflow Expansion

Status: Complete
Owner: Platform
Created: 2026-03-27
Depends on: 046, 048

## Overview

`g01.048` finished the shared sort-builder migration and retired public Underlay
`OrderBy`. The next strongest retained generic hold is `ReorderableList`.

Poodle already owns the low-level reorderable list interaction surface. The
remaining Underlay `ReorderableList` value is higher-order workflow behavior:
batch submit/cancel flow, dirty-state handling, async submit error handling,
long-list warnings, optional windowed reorder mode, and stronger keyboard/live
announcement guidance.

The goal of this roadmap is not to push app-specific reorder flows into Poodle.
It is to define the smallest honest Poodle workflow expansion for reusable
reorder sessions, migrate the active caller family in grouped passes, and
retire public Underlay `ReorderableList` once residue is clean.

## Research Basis

- `ts/src/patterns/ReorderableList.svelte`
- `ts/src/patterns/reorder-controller.svelte.ts`
- `../poodle/packages/svelte/composites/src/ReorderableList.svelte`
- representative callers:
  - `ts/src/patterns/AutonomousList/AutonomousList.svelte`
  - `../underlay-reference/acme-admin/src/routes/(app)/categories/+page.svelte`
  - `../underlay-reference/acme-admin/src/routes/(app)/projects/+page.svelte`
  - `../underlay-reference/acme-admin/src/routes/(app)/projects/[projectId]/+page.svelte`

## Decision Summary

- `ReorderableList` should move further into Poodle. The remaining behavior is
  still reusable interaction/workflow support, not product-specific business
  logic.
- The existing Poodle `ReorderableList` already proves the low-level drag and
  keyboard movement model. The missing slice is a reusable reorder-session
  shell around that primitive.
- The first proof caller family should be the grouped `acme-admin`
  category/project/task reorder flows, because they all already share the same
  Underlay controller and conflict-recovery wiring.

## Consumer Upgrade Impact

- Consumer apps should expect a staged migration from
  `@inflatable-cookie/underlay/patterns` `ReorderableList` to a Poodle reorder
  workflow surface layered over Poodle `ReorderableList`.
- The migration shape should be:
  - expand the Poodle workflow contract first
  - migrate the active admin reorder flows in grouped batches
  - remove the Underlay public pattern only after residue is clean
- Do not add new Underlay compatibility wrappers during this wave.

## Capability Gap

Current Underlay `ReorderableList` behavior that still needs a Poodle home:

- submit/cancel workflow chrome
- dirty-state gating for save
- async submit error presentation
- optional submit-error transformation hook
- long-list warning behavior
- optional windowed reorder mode for large lists
- richer keyboard/live-region guidance over the low-level reorder primitive

## Likely Implementation Surface

- Poodle:
  - `../poodle/packages/svelte/composites/src/ReorderableList.svelte`
  - adjacent types/docs/specimen files
- Underlay:
  - `ts/src/patterns/ReorderableList.svelte`
  - `ts/src/patterns/reorder-controller.svelte.ts`
  - `contracts/ui/poodle-adoption-underlay-surface-groups.json`
  - `docs/guides/090-ui-kit.md`
- proof callers:
  - `acme-admin` categories/projects/project-detail reorder flows
  - retained `AutonomousList` after the first app proof family is green

## Batch 49.1 - Contract Reset

- [x] Re-check the actual Underlay-vs-Poodle `ReorderableList` boundary from
      code.
- [x] Record the smallest honest staged workflow expansion instead of treating
      the whole Underlay shell as a permanent hold.
- [x] Pick the first proof caller family and refresh roadmap front doors.

Completed in 49.1:
- The low-level reorder primitive is already in Poodle.
- The remaining generic gap is now explicit as reorder-session workflow.
- The first proof family is fixed:
  - `acme-admin` categories
  - `acme-admin` projects
  - `acme-admin` project detail

## Batch 49.2 - Poodle Reorder Session Stage 1

- [ ] Expand Poodle reorder support for the first honest workflow slice:
- [x] Expand Poodle reorder support for the first honest workflow slice:
  - submit/cancel shell
  - dirty-state gating
  - async submit + error surface
  - live announcement guidance
- [x] Update the Poodle contract docs and specimen.
- [x] Migrate the first proof caller family.

Completed in 49.2:
- Poodle `ReorderableList` now owns submit/cancel workflow chrome, dirty-state
  gating, async submit error surface, and the richer keyboard/live-announcement
  guidance previously trapped in the Underlay shell.
- The first proof family is migrated and green:
  - `acme-admin` categories
  - `acme-admin` projects
  - `acme-admin` project detail

## Batch 49.3 - Poodle Reorder Session Stage 2

- [x] Add long-list warning and optional windowed reorder mode.
- [x] Sweep remaining public Underlay `ReorderableList` residue across retained
      shells and docs.
- [x] Retire the public Underlay `ReorderableList` pattern once residue is
      clean.

Completed in 49.3:
- Poodle `ReorderableList` now owns long-list guidance and optional page-window
  mode for large reorder sessions.
- Retained `AutonomousList` is migrated onto direct Poodle `ReorderableList`.
- Public Underlay `ReorderableList` is removed, including the old
  implementation and dedicated harness/tests.

## Next Task

`g01.049` is complete. Open the next focused runtime wave on
`ToastHost` and `PageLoading`, starting with a strict contract split between the
store-driven toast host and the inline page-loading shell so the next roadmap
does not mix two different runtime boundaries into one vague migration pass.
