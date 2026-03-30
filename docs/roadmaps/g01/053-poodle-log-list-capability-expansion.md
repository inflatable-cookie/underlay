# 053 - Poodle LogList Capability Expansion

Status: Complete
Owner: Platform
Created: 2026-03-28
Depends on: 052

## Overview

`g01.052` proved that the remaining obvious-equivalent surface is not one
uniform migration problem.

- `PageHeader` still owns a richer structural shell contract across retained
  Underlay patterns and broad Dairy page usage.
- `MediaPicker` still owns a workflow-heavy browse/upload session that is
  larger than a same-name import swap.
- `LogList` is the cleanest next capability target because it has a small live
  caller family and the remaining gap is still generic audit/log list behavior,
  not domain workflow.

This wave exists to move that broader audit-log list contract into Poodle,
migrate the active callers, and retire public Underlay `LogList`.

## Research Basis

- Underlay:
  - `ts/src/components/LogList.svelte`
  - `ts/src/components/log-list/LogEntryItem.svelte`
  - `ts/src/components/log-list/LogListToolbar.svelte`
  - `ts/src/components/log-list/LogListStatus.svelte`
  - `ts/src/components/log-list/LogListPagination.svelte`
- Poodle:
  - `../poodle/packages/svelte/composites/src/LogList.svelte`
  - `../poodle/packages/svelte/composites/src/types.ts`
- representative live callers:
  - `../underlay-reference/acme-admin/src/routes/(app)/+page.svelte`
  - `../underlay-reference/acme-admin/src/routes/(app)/system/audit/+page.svelte`
  - `../contact-patch/cp-admin/src/routes/(app)/+page.svelte`
  - `../contact-patch/cp-admin/src/routes/(app)/system/audit/+page.svelte`
  - `../acowtancy/dairy/src/routes/(app)/system/audit/+page.svelte`

## Decision Summary

- Current Poodle `LogList` is a terminal/runtime log console:
  - level chips
  - text filter
  - auto-scroll
  - streaming log output
- Current Underlay `LogList` is a paginated audit/activity list:
  - actor and resource formatting
  - callback-driven filters
  - refresh/export actions
  - empty/loading/error states
  - pagination controls
  - custom action icon and entry detail snippets
- The Underlay contract is still generic enough to belong in Poodle, but it is
  not a same-name drop-in replacement today.

## Consumer Upgrade Impact

- Consumer apps should expect Poodle `LogList` to widen from a log-console
  composite into a broader log/audit list surface.
- Do not preserve Underlay as a compatibility shim once Poodle covers the live
  contract.
- Migrate the grouped dashboard/audit caller family in one pass once the
  expanded Poodle contract is landed.

## Planned Batches

## Batch 53.1 - Contract Reset

- [x] Reassess live `LogList` callers against current Poodle `LogList`.
- [x] Confirm that `LogList` is the strongest next capability target in the
      remaining obvious-equivalent family.
- [x] Open the focused roadmap and update the durable inventory/front doors.

Completed in 53.1:
- `LogList` is now the active successor-capability wave.
- `PageHeader` remains a later structural-shell reassessment.
- `MediaPicker` remains a later workflow-surface reassessment.

## Batch 53.2 - Poodle Audit Log Expansion

- [x] Expand Poodle `LogList` to cover the generic audit/activity list
      contract:
  - callback-driven filters
  - loading/error/empty states
  - refresh/export controls
  - pagination
  - actor/resource formatting hooks
  - custom detail rendering
- [x] Update Poodle docs/specimens for the widened log-list surface.
- [x] Migrate the grouped dashboard and audit proof family in `acme-admin` and
      `cp-admin`.

Completed in 53.2:
- Poodle `LogList` now supports both the original stream-viewer shape and the
  broader audit/activity list contract.
- The grouped proof family in `acme-admin` and `cp-admin` now imports Poodle
  `LogList` directly for both dashboard activity feeds and full audit pages.
- The remaining `LogList` tail is now down to the Dairy audit page plus public
  Underlay export/story residue.

## Batch 53.3 - Portfolio Sweep And Retirement

- [x] Migrate the remaining Dairy audit caller.
- [x] Retire public Underlay `LogList` and its helper/story residue.
- [x] Update roadmap and inventory state to reflect the finished boundary.

Completed in 53.3:
- The remaining Dairy audit page now imports Poodle `LogList` directly.
- Public Underlay `LogList` is removed from the component index, and the old
  implementation, helper set, and Storybook residue are deleted.
- `g01.053` is now complete; `LogList` is no longer part of the active public
  Underlay surface.

## Next Task

**Complete:** Poodle `LogList` now owns the generic audit/activity list
contract, the active caller family is migrated, and the public Underlay
`LogList` surface is retired.
