# 054 - BatchActionBar Successor Wave

Status: Complete
Owner: Platform
Created: 2026-03-28
Depends on: 052, 053

## Overview

`g01.053` finished the `LogList` successor wave and retired public Underlay
`LogList`.

That leaves a smaller set of obvious-equivalent residue, but the next honest
batch is no longer a broad same-name family. `BatchActionBar` is now the
strongest clean successor target because:

- the live caller family is small and concrete
- Poodle already owns the base primitive as `BulkActionBar`
- the remaining Underlay behavior is narrow enough to either move into Poodle
  or become explicit caller-owned workflow without preserving a second wrapper

This wave exists to move the remaining shared batch-action shell into Poodle or
direct Poodle composition, migrate the live caller family, and retire public
Underlay `BatchActionBar`.

## Research Basis

- Underlay:
  - `ts/src/components/BatchActionBar.svelte`
  - `ts/src/patterns/AutonomousList/AutonomousList.svelte`
- Poodle:
  - `../poodle/packages/svelte/primitives/src/BulkActionBar.svelte`
  - `../poodle/packages/svelte/primitives/src/types.ts`
- representative live callers:
  - `../underlay-reference/acme-admin/src/routes/(app)/media/+page.svelte`
  - `../underlay-reference/acme-admin/src/routes/(app)/projects/+page.svelte`
  - `../underlay-reference/acme-admin/src/routes/(app)/projects/[projectId]/+page.svelte`
  - `../contact-patch/cp-admin/src/routes/(app)/media/+page.svelte`
  - `ts/src/patterns/AutonomousList/AutonomousList.svelte`

## Decision Summary

- Current Underlay `BatchActionBar` still bundles:
  - selection summary
  - clear / select-all toggles
  - dynamic registered actions
  - optional built-in delete confirmation
  - optional built-in status-update dialog
- Current Poodle `BulkActionBar` already owns:
  - selection summary
  - generic action buttons
  - clear action
- The gap is materially smaller than `PageHeader`, `MediaPicker`, or
  `AutonomousList`.
- The likely honest end state is:
  - Poodle `BulkActionBar` widened slightly for select-all behavior and richer
    action metadata if that proves truly generic
  - destructive confirmation and status-update dialogs owned by callers or
    nearby workflow shells instead of preserved as hidden wrapper behavior

## Consumer Upgrade Impact

- Consumer apps should expect to migrate from Underlay `BatchActionBar` to
  direct Poodle `BulkActionBar` or a slightly widened Poodle successor surface.
- Do not keep Underlay `BatchActionBar` as a compatibility shim once the live
  caller family is green.
- Prefer making confirm/status workflows explicit in app code if they do not
  justify primitive-level ownership.

## Planned Batches

## Batch 54.1 - Contract Reset

- [x] Reassess remaining obvious-equivalent residue after `g01.053`.
- [x] Confirm that `BatchActionBar` is the strongest next clean successor wave.
- [x] Open the focused roadmap and update front doors / durable inventory.

Completed in 54.1:
- `BatchActionBar` is now the active follow-on successor wave.
- `PageHeader`, `MediaPicker`, `DropdownMenu`, and `AutonomousList` remain
  later questions because they still involve broader shell or workflow
  decisions.

## Batch 54.2 - Poodle Bulk Action Expansion

- [x] Compare live `BatchActionBar` callers against current Poodle
      `BulkActionBar`.
- [x] Add the smallest honest generic capability needed for the grouped caller
      family.
- [x] Avoid preserving built-in confirm/status workflow unless it proves
      reusable enough to belong in Poodle.

Completed in 54.2:
- Poodle `BulkActionBar` now supports select-all / deselect-all affordance,
  loading and disabled gating, warning-tone actions, and richer icon input so
  retained Underlay `AutonomousList` can pass through registered component
  icons directly.
- The first grouped migration family is now on direct Poodle
  `BulkActionBar`: `acme-admin` media, `acme-admin` projects,
  `acme-admin` project detail, `cp-admin` media, and retained Underlay
  `AutonomousList`.
- The old built-in delete confirmation and status-update dialog behavior is not
  being preserved inside the primitive. Those workflows are now explicit in
  route code or in the surrounding list controller.

## Batch 54.3 - Grouped Caller Migration And Retirement

- [x] Migrate the remaining Dairy `BatchActionBar` caller family and any local
      wrappers onto direct Poodle `BulkActionBar`.
- [x] Retire public Underlay `BatchActionBar` and update docs/inventory once
      the cross-portfolio residue scan is honestly clean.
- [x] Keep destructive confirmation and status workflows explicit in caller or
      nearby workflow-shell code rather than reintroducing them as hidden
      wrapper behavior.

Completed in 54.3:
- The full remaining Dairy caller family now uses an app-local
  `DairyBatchActionBar` successor over Poodle `BulkActionBar`, so the generic
  rendering contract is no longer sourced from Underlay.
- Public Underlay `BatchActionBar` is removed from the component export surface,
  and the old implementation plus Storybook residue is deleted.
- Active guides now teach Poodle `BulkActionBar` with explicit confirmation
  dialogs instead of a retained Underlay batch-workflow wrapper.

## Completion

`g01.054` is complete. Underlay no longer exports `BatchActionBar`.
