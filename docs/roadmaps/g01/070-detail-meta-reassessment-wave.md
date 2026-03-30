# 070 - DetailMeta Reassessment Wave

Status: Complete
Owner: Platform
Created: 2026-03-30
Depends on: 069

## Overview

`g01.069` is complete. The public `RelationSelector` UI wrapper family is
gone, and Underlay now keeps only the lower-level relation search, drilldown,
selection-history, and context helper layer.

The strongest next non-auth public helper family to challenge is `DetailMeta`.

Unlike `SpaFormShell`, `LoginPage`, `ForgotPasswordFlow`, and
`PasswordRequirements`, this surface does not obviously own workflow or policy.
It is a structural helper family that survived the retirement of
`PageHeader`, `PageHeaderMeta`, and `DetailPageShell`, so the next honest move
is to verify whether it still earns public Underlay ownership or is now just
stale composition residue.

## Research Basis

- current shared helpers:
  - `ts/src/patterns/DetailPageShell/DetailMeta.svelte`
  - `ts/src/patterns/DetailPageShell/DetailMetaItem.svelte`
  - `ts/src/patterns/DetailPageShell/DetailMetaId.svelte`
  - `ts/src/patterns/DetailPageShell/DetailMetaStatus.svelte`
  - `ts/src/patterns/DetailPageShell/DetailMetaSeparator.svelte`
- representative active callers:
  - `underlay-reference/acme-admin/src/routes/(app)/`
  - `contact-patch/cp-admin/src/routes/(app)/`
  - `acowtancy/dairy/src/routes/(app)/`

## Decision Focus

- Determine whether the public `DetailMeta` helper family still earns shared
  Underlay ownership as a stable compact-detail-row contract
- or whether it has collapsed into:
  - direct Poodle detail-row/stack composition
  - app-local helpers for small repeated metadata layouts

## Consumer Upgrade Impact

- Do not add new public `DetailMeta*` consumers while this wave is in
  progress.

## Planned Batches

## Batch 70.1 - Caller And Contract Matrix

- [x] Sweep the live `DetailMeta*` caller family across `acme-admin`,
      `cp-admin`, `dairy`, and the retained Underlay guide/example surface.
- [x] Compare the retained helper contract against direct Poodle detail
      composition plus small app-local helper alternatives.
- [x] Decide whether the family still earns public Underlay ownership or should
      move toward retirement/internalization.

### Batch 70.1 Findings

The live caller family is still broad and structurally consistent.

Current active-app usage counts are:

- `acme-admin`: 11 files
- `cp-admin`: 6 files
- `dairy`: 44 files

Representative live usage shows the same compact contract repeating across both
detail and edit surfaces:

- detail pages with ID plus inline pills or compact labeled values
- edit headers with copyable ID/email/code values
- nested detail tabs where the metadata row stays outside the main detail grid
- small local view components such as media/detail headers and activity detail
  shells

The retained helper family is still doing one consistent job:

- `DetailMeta` gives a compact wrapping inline row container
- `DetailMetaItem` gives labeled inline values
- `DetailMetaId` standardizes copyable code-style IDs
- `DetailMetaStatus` standardizes compact status pills
- `DetailMetaSeparator` keeps the row punctuation consistent

This is not another collapsed wrapper surface. It is a small but still
genuinely shared helper contract that survives the retirement of `PageHeader`
and `DetailPageShell`.

Direct Poodle composition is possible, but forcing route-local replacement now
would mostly duplicate a stable metadata-row vocabulary across dozens of active
pages.

## Current Judgment

`DetailMeta*` still earns retained public Underlay ownership for now.

The next honest move is not a migration wave. It is a guide and inventory
closeout that records `DetailMeta*` as an explicit retained helper family, then
resets the queue around the next public surface that has actually collapsed
enough to challenge.

## Batch 70.2 - Guide And Inventory Closeout

- [x] Update the active guide surface so `DetailMeta*` is recorded as an
      explicit retained Underlay helper family.
- [x] Update the roadmap front doors and durable inventory so the queue no
      longer treats `DetailMeta*` like the next likely retirement.
- [x] Reset the queue around the next honest public-surface challenge.

### Batch 70.2 Findings

The guide and inventory surface now reflects the real boundary:

- `DetailMeta*` is retained explicitly
- `PageHeader` and `DetailPageShell` stay retired
- the queue no longer treats `DetailMeta*` like the next easy migration target

With that closeout done, `g01.070` is finished.

## Next Task

Execute `g01.071` Batch `71.1` by writing a fresh strict caller and contract
matrix for the remaining retained auth workflow surface:

- `LoginPage`
- `ForgotPasswordFlow`
- `PasswordRequirements`

Recheck whether those still earn shared public Underlay ownership now that the
public surface is much smaller, then decide whether the next honest move is a
new capability/migration wave or an explicit stop point for the remaining auth
family.
