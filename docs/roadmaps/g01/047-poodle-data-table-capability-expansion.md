# 047 - Poodle DataTable Capability Expansion

Status: Complete
Owner: Platform
Created: 2026-03-27
Depends on: 046

## Overview

`g01.046` proved that the remaining generic card/action residue belonged in
Poodle, not Underlay. `DataTable` is the next substantial surface in the same
category: the current Underlay implementation is still serving as a richer
generic table runtime rather than a domain-specific workflow shell.

Poodle already owns a `DataTable`, but it is still materially narrower than
the active Underlay contract used across admin, operational, and Dairy views.
The goal of this roadmap is not to copy the Underlay API mechanically. It is
to define the smallest honest Poodle table expansion, migrate the real caller
families in grouped batches, and retire Underlay `DataTable` once the residue
is clean.

## Research Basis

- `ts/src/components/DataTable.svelte`
- `../poodle/packages/svelte/composites/src/DataTable.svelte`
- `../poodle/packages/svelte/composites/src/types.ts`
- representative Underlay callers:
  - `../underlay-reference/acme-admin/src/routes/(app)/system/errors/+page.svelte`
  - `../contact-patch/cp-admin/src/routes/(app)/system/errors/+page.svelte`
  - `../underlay-reference/acme-admin/src/routes/(app)/users/+page.svelte`
  - `../contact-patch/cp-admin/src/routes/(app)/users/+page.svelte`
  - `../underlay-reference/acme-admin/src/routes/(app)/system/jobs/+page.svelte`
  - `../acowtancy/dairy/src/routes/(app)/system/errors/+page.svelte`
  - `../acowtancy/dairy/src/routes/(app)/users/+page.svelte`

## Decision Summary

- `DataTable` should move into Poodle. The missing behavior is still generic
  design-system table/runtime capability, not app-specific workflow logic.
- The current gap is too large for a one-step migration. The right move is a
  staged Poodle expansion with proof callers after each meaningful slice.
- The first proof caller should be the error-log tables, because they exercise:
  - custom cell rendering
  - richer row actions
  - extended-row rendering
  - row-local loading/detail state
- The second broad caller family should be users/jobs tables, because they
  exercise:
  - host-owned pagination
  - limit selector
  - filter and sort integration
  - operational empty/loading states

## Consumer Upgrade Impact

- Consumer apps should expect a staged migration from
  `@inflatable-cookie/underlay/components` `DataTable` to Poodle `DataTable`.
- The migration shape should be:
  - expand the Poodle contract first
  - migrate grouped caller families
  - remove the Underlay public surface only after residue is clean
- Do not add new Underlay compatibility wrappers during this wave.

## Capability Gap

Current Underlay `DataTable` behavior that still needs a Poodle home:

- host-owned pagination footer
- limit selector
- richer column definitions
- custom cell rendering
- extended rows
- richer row actions
- loading rows / loading state
- row click
- compact / striped / sticky-header presentation

## Likely Implementation Surface

- Poodle:
  - `../poodle/packages/svelte/composites/src/DataTable.svelte`
  - `../poodle/packages/svelte/composites/src/types.ts`
  - Poodle docs / specimen files for table usage
- Underlay:
  - `ts/src/components/DataTable.svelte`
  - `contracts/ui/poodle-adoption-underlay-surface-groups.json`
  - `docs/guides/090-ui-kit.md`
  - `docs/guides/190-upgrade-compatibility.md`
- proof callers:
  - error-log tables in `underlay-reference`, `contact-patch`, and `dairy`
  - users/jobs tables in `underlay-reference`, `contact-patch`, and `dairy`

## Batch 47.1 - Contract and Proof-Caller Reset

- [x] Confirm the actual Underlay-vs-Poodle `DataTable` contract gap from code.
- [x] Record the smallest honest first-stage Poodle expansion.
- [x] Pick the first proof caller family and the second follow-on family.
- [x] Refresh the roadmap front doors so the active next wave is explicit.

Completed in 47.1:
- The gap is now explicit rather than implied by old roadmap prose:
  - Poodle already owns table shell, selection, visibility, export, and simple
    row actions
  - Underlay still owns richer column metadata, host-owned pagination state,
    filters, row-action menus, custom cells, extended rows, loading rows, row
    click handling, and table presentation options
- The first proof caller family is now fixed:
  - `acme-admin` system errors
  - `cp-admin` system errors
  - `dairy` system errors
- The second grouped caller family is now fixed:
  - users tables
  - jobs tables
  - scheduled-task detail job tables

## Batch 47.2 - Poodle DataTable Stage 1

- [x] Expand Poodle `DataTable` for the first generic capability slice:
  - custom cell rendering
  - extended rows
  - richer row actions
  - row click
- [x] Update the Poodle contract docs and specimen.
- [~] Migrate the first proof caller family:
  - `acme-admin` system errors
  - `cp-admin` system errors
  - `dairy` system errors
- [x] Re-snapshot what remains before adding pagination/footer behavior.

Completed in 47.2:
- Poodle `DataTable` now owns the first honest generic runtime slice:
  - custom cell slot rendering
  - expanded-row slot rendering
  - richer row action arrays
  - row click handling
  - wider generic column metadata needed by the first proof family
- The stage-1 contract is now documented and demonstrated in the Poodle
  specimen instead of being implied by the app migrations.
- The first proof family is green in:
  - `acme-admin` system errors
  - `cp-admin` system errors
- The `dairy` system error page itself is migrated to Poodle `DataTable`, but
  the full `dairy` repo snapshot still has unrelated stale imports to deleted
  Underlay pattern exports (`EntityActionsMenu` route imports and
  restore-resolution surfaces) that should be cleared in one follow-on batch
  before treating the whole repo as green again.
- The next honest runtime slice is clearer now: pagination/footer, limit
  selector, loading rows, and presentation density should move together as
  Batch `47.3`, then the users/jobs family can migrate in grouped passes.

## Batch 47.3 - Poodle DataTable Stage 2

- [x] Expand Poodle `DataTable` for the next generic runtime slice:
  - host-owned pagination footer
  - limit selector
  - loading rows / loading state
  - compact / striped / sticky-header presentation
- [x] Migrate the users/jobs caller family across the active apps.
- [x] Update upgrade guidance and inventory.

Completed in 47.3:
- Poodle `DataTable` now owns the next honest runtime slice:
  - host-owned filter row
  - host-owned pagination footer
  - limit selector
  - loading rows
  - compact / striped / sticky-header presentation
- The grouped users family is now migrated and green in:
  - `acme-admin`
  - `cp-admin`
  - `dairy`
- The grouped jobs family is now migrated and green in:
  - `acme-admin`
  - `cp-admin`
  - `dairy`
- The Poodle contract, specimen, and preview docs now describe the broadened
  table surface directly instead of leaving the new behavior implicit in app
  code.

## Batch 47.4 - Underlay Retirement

- [x] Sweep remaining Underlay `DataTable` residue across apps, guides, and
      retained Underlay shells.
- [x] Remove the public Underlay `DataTable` export and dead helpers/tests.
- [x] Close the wave with validation and updated retained-surface inventory.

Completed in 47.4:
- The remaining operational and retained-shell table family is now migrated and
  green in:
  - `dairy` AI suggestions, marking, transform diagnostics, and learning-transform
    overview/detail tables
  - retained Underlay `AiRoutingAdmin`
- The public Underlay `DataTable` surface is now gone:
  - `ts/src/components/DataTable.svelte` deleted
  - `ts/src/components/data-table/*` helpers deleted
  - public exports removed from `ts/src/components/index.ts`,
    `ts/src/components/index.d.ts`, and `ts/src/svelte.d.ts`
  - dead table tests/fixtures deleted
- Active guidance now teaches direct Poodle `DataTable` instead of the old
  Underlay wrapper.

## Complete

`g01.047` is complete. The richer generic table runtime now lives in Poodle,
and Underlay no longer exports the old `DataTable` wrapper.
