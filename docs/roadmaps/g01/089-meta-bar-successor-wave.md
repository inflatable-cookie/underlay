# 089 - MetaBar Successor Wave

Status: Complete
Owner: Platform
Created: 2026-03-30
Depends on: 088

## Overview

`g01.088` is complete. The retained package boundaries are explicit again, and
the active sibling app surface is clean after the package-boundary recovery.

The next honest retained UI challenge is the `DetailMeta*` helper family. It
still had live callers in `acme-admin` and `cp-admin`, but the actual contract
had collapsed to a tiny generic metadata-ribbon shape:

- one wrapping inline row
- one optional labeled item
- direct composition of IDs, status pills, and other values

That no longer earns five Underlay exports or `Detail*` naming. The right
successor is a smaller generic Poodle surface.

## Research Basis

- current Underlay helper files:
  - `ts/src/patterns/DetailPageShell/DetailMeta.svelte`
  - `ts/src/patterns/DetailPageShell/DetailMetaItem.svelte`
  - `ts/src/patterns/DetailPageShell/DetailMetaId.svelte`
  - `ts/src/patterns/DetailPageShell/DetailMetaStatus.svelte`
  - `ts/src/patterns/DetailPageShell/DetailMetaSeparator.svelte`
- current Poodle adjacent surfaces:
  - `packages/svelte/primitives/src/DetailRow.svelte`
  - `packages/svelte/primitives/src/Code.svelte`
  - `packages/svelte/primitives/src/Pill.svelte`
- live caller family:
  - `underlay-reference/acme-admin/src/routes/(app)/`
  - `contact-patch/cp-admin/src/routes/(app)/`

## Decision Focus

- Replace `DetailMeta*` with a smaller generic Poodle metadata-ribbon surface
- remove the `Detail*` naming and helper-wrapper pile
- migrate active callers onto direct `Code` and `Pill` composition

## Consumer Upgrade Impact

- `@decodelabs/underlay/patterns` no longer exports:
  - `DetailMeta`
  - `DetailMetaItem`
  - `DetailMetaId`
  - `DetailMetaStatus`
  - `DetailMetaSeparator`
- use `MetaBar` and `MetaItem` from `@inflatable-cookie/poodle-svelte-primitives`
- compose copyable IDs with `Code inline source={...} showCopyButton`
- compose state and classification badges directly with `Pill`

## Planned Batches

## Batch 89.1 - Poodle Meta Ribbon Capability

- [x] Define the smallest honest generic successor contract in Poodle
- [x] Add a minimal primitive pair:
  - `MetaBar`
  - `MetaItem`
- [x] Add preview/docs coverage for the new primitive pair

### Batch 89.1 Findings

The old Underlay family was over-split. The real reusable contract is only:

- an inline wrapping metadata ribbon with subtle separation
- a compact optional-label item

IDs and statuses are not separate generic components. They are direct
composition over existing Poodle primitives:

- `Code`
- `Pill`

## Batch 89.2 - Caller Migration And Underlay Retirement

- [x] Migrate the live `acme-admin` and `cp-admin` caller family
- [x] Remove the Underlay `DetailMeta*` exports and implementation files
- [x] Remove the now-dead Underlay test/fixture residue
- [x] Update the live guide, roadmap, and durable inventory surface

### Batch 89.2 Findings

The active caller family migrated cleanly onto:

- Poodle `MetaBar`
- Poodle `MetaItem`
- direct Poodle `Code`
- direct Poodle `Pill`

The retained Underlay helper family no longer proved a unique shared contract
after that move, so the public wrapper family is retired.

## Next Task

This wave is complete. The next honest follow-on is a fresh retained-surface
challenge elsewhere in the remaining Underlay package boundary, not more
cleanup on the retired `DetailMeta*` family.
