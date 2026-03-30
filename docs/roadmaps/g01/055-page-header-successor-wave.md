# 055 - PageHeader Successor Wave

Status: Complete
Owner: Platform
Created: 2026-03-28
Depends on: 052, 054

## Overview

`g01.054` finished the `BatchActionBar` successor wave and retired public
Underlay `BatchActionBar`.

That leaves a smaller set of obvious-equivalent residue, but the next honest
batch is no longer a mixed queue. `PageHeader` is now the strongest focused
successor wave because:

- the live caller family is broad and concrete across retained Underlay shells
  plus `acme-admin` and `cp-admin`
- Poodle already owns a strong simpler `PageHeader` surface and is proven in
  many list, media, account, and system pages
- the remaining Underlay gap is now mostly richer structural composition rather
  than a domain-specific workflow

This wave exists to define the smallest honest Poodle `PageHeader` expansion,
migrate the grouped caller family, and retire public Underlay `PageHeader`
plus any low-value same-family residue that no longer needs a second shell.

## Research Basis

- Underlay:
  - `ts/src/patterns/PageHeader.svelte`
  - `ts/src/patterns/DetailPageShell/DetailPageShell.svelte`
  - `ts/src/patterns/AiRoutingAdmin.svelte`
  - `ts/src/patterns/AutonomousList/AutonomousList.svelte`
- Poodle:
  - `../poodle/packages/svelte/composites/src/PageHeader.svelte`
- representative live callers:
  - `../underlay-reference/acme-admin/src/routes/(app)/projects/[projectId]/+page.svelte`
  - `../underlay-reference/acme-admin/src/routes/(app)/projects/[projectId]/tasks/[taskId]/+page.svelte`
  - `../underlay-reference/acme-admin/src/routes/(app)/system/jobs/[id]/+page.svelte`
  - `../contact-patch/cp-admin/src/routes/(app)/system/emails/[id]/+page.svelte`
  - `../contact-patch/cp-admin/src/routes/(app)/system/scheduled-tasks/[id]/+page.svelte`

## Decision Summary

- Poodle `PageHeader` already owns the simple title/back/count/actions cases and
  is proven across many active pages.
- Underlay `PageHeader` is now mainly carrying:
  - section/title split
  - richer breadcrumb handling
  - contextual back behavior
  - banner/callout composition
  - header meta composition in nearby retained shells
- The likely honest end state is:
  - move any reusable structural header capability into Poodle `PageHeader`
  - keep `DetailMeta` and other true shell-level composition outside the header
  - retire public Underlay `PageHeader` once retained shells and live route
    callers are green

## Consumer Upgrade Impact

- Consumer apps should expect to migrate from Underlay `PageHeader` to direct
  Poodle `PageHeader` plus explicit local meta/callout/actions composition.
- Do not preserve Underlay `PageHeader` as a compatibility shim once the live
  caller family is green.
- Prefer keeping detail-shell meta rows and other page-specific structure
  outside the header component unless a smaller reusable Poodle slot contract is
  clearly justified.

## Planned Batches

## Batch 55.1 - Contract Reset

- [x] Reassess the remaining obvious-equivalent public surface after
      `g01.054`.
- [x] Confirm that `PageHeader` is the strongest next focused successor wave.
- [x] Open the focused roadmap and update front doors / durable inventory.

Completed in 55.1:
- `PageHeader` is now the active follow-on successor wave.
- `MediaPicker` and `DropdownMenu` remain later questions because they still
- involve broader workflow or menu-composition decisions than the first clean
  `PageHeader` cut.

## Batch 55.2 - Poodle Header Capability Review

- [x] Compare the live Underlay `PageHeader` caller family against current
      Poodle `PageHeader`.
- [x] Define the smallest honest reusable expansion needed for the retained
      detail/admin header family.
- [x] Avoid preserving shell-specific meta or page workflow inside the Poodle
      header if local composition is enough.

Completed in 55.2:
- The live Underlay `PageHeader` contract is now pinned down more precisely:
  - broad reusable gap:
    - two-level `section` / `title` treatment
    - banner or callout composition at the header level
    - possibly a lighter contextual-back affordance if it proves reused enough
  - do not pull retained detail-shell meta into Poodle `PageHeader`
- `DetailMeta` and other shell-specific header-detail structure should remain
  outside the generic header. The retained shells can compose that content
  beneath Poodle `PageHeader` once the reusable header gap is closed.
- The first honest implementation batch should therefore target Poodle
  `PageHeader` for:
  - section/title hierarchy
  - banner slot or equivalent reusable callout surface
  - only the smallest proven contextual-back behavior needed by the grouped
    live caller family

Capability matrix from the caller review:
- Already covered by current Poodle `PageHeader`:
  - plain `title`
  - `count`
  - simple `backHref` / `backLabel`
  - actions slot
  - breadcrumbs slot
- Still trapped in Underlay but broadly reusable:
  - section/title split used by retained shells and detail routes
  - header-level banner/callout composition
- Not a header capability target:
  - `DetailMeta` rows and status/id blocks
  - page-specific workflow/actions beyond normal slot composition
  - deeper shell structure owned by `DetailPageShell` and related patterns

## Batch 55.3 - Grouped Caller Migration And Retirement

- [x] Migrate the grouped app caller family and retained Underlay shells onto
      direct Poodle `PageHeader`.
- [x] Retire public Underlay `PageHeader` and update docs/inventory once the
      residue scan is honestly clean.

Completed in 55.3:
- Poodle `PageHeader` now owns the reusable section/title hierarchy and
  banner-level composition needed by the retained detail/admin header family.
- Retained Underlay shells now compose Poodle `PageHeader` directly:
  `DetailPageShell`, `AutonomousList`, `AiRoutingAdmin`, and `FormShell`.
- The remaining direct detail-route family in `acme-admin` and `cp-admin`
  moved to direct Poodle `PageHeader`.
- `DetailMeta` stayed outside the generic header. The migration did not keep
  the old wrapper as a compatibility shim.
- Public Underlay `PageHeader` is retired. The component file, dedicated test,
  fixture, and Storybook story are deleted, and live consumer-app residue is
  clean.

## Completion

`g01.055` is complete. The next clean successor wave should target
`MediaPicker` or `DropdownMenu`, depending on whether you want the next
broader workflow wave or the smaller composition cleanup.
