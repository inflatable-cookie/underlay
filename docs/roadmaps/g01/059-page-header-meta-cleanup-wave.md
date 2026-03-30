# 059 - PageHeaderMeta Cleanup Wave

Status: Complete
Owner: Platform
Created: 2026-03-28
Depends on: 055, 058

## Overview

`g01.058` finished the `AutonomousList` successor wave and retired the dead
public list shell.

The next clean residue family is the old `PageHeaderMeta` helper set:

- `PageHeaderMeta`
- `PageHeaderMetaRow`
- `PageHeaderMetaItem`
- `PageHeaderMetaSeparator`

The broader `PageHeader` successor wave already moved active callers onto
Poodle `PageHeader` and `DetailMeta` composition. If the residue scan remains
clean, this family should be retired in one pass rather than left as stale
public exports.

## Research Basis

- Underlay:
  - `ts/src/patterns/PageHeaderMeta.svelte`
  - `ts/src/patterns/PageHeaderMetaRow.svelte`
  - `ts/src/patterns/PageHeaderMetaItem.svelte`
  - `ts/src/patterns/PageHeaderMetaSeparator.svelte`
  - `ts/src/patterns/index.ts`
- caller sweep:
  - `ts/src`
  - `underlay-reference/acme-admin/src`
  - `contact-patch/cp-admin/src`
  - `acowtancy/dairy/src`
- active guides:
  - `docs/guides/090-ui-kit.md`
  - `docs/guides/098-shared-admin-patterns.md`

## Decision Summary

- `PageHeaderMeta` looks like dead public residue rather than a live retained
  structural family.
- The first batch should prove that with a strict residue sweep.
- If live callers are gone, retire the whole helper family in one broad batch
  and update the active guide/import surface at the same time.

## Consumer Upgrade Impact

- Consumers should not build new header metadata composition on the retired
  `PageHeaderMeta*` family.
- Use `DetailMeta*` helpers or caller-owned Poodle composition instead.

## Planned Batches

## Batch 59.1 - Strict Residue Sweep

- [x] Audit the live caller surface for `PageHeaderMeta`, `PageHeaderMetaRow`,
      `PageHeaderMetaItem`, and `PageHeaderMetaSeparator`.
- [x] Decide whether the family is dead public residue or still has a live
      retained caller contract.
- [x] If dead, queue one broad retirement pass instead of another review wave.

Completed in 59.1:
- The residue sweep is clean: there are no live source callers in Underlay,
  `acme-admin`, `cp-admin`, or `dairy`.
- The helper family was only still present as public export residue after the
  broader `PageHeader` migration.
- Public Underlay `PageHeaderMeta`, `PageHeaderMetaRow`,
  `PageHeaderMetaItem`, and `PageHeaderMetaSeparator` are now retired.

## Next Task

Open the next focused successor wave on `MediaActionsMenu`, then compare the
shared Underlay workflow helper against the local wrappers in `acme-admin`,
`cp-admin`, and `dairy` to decide whether it still earns a public shared
surface or can collapse into direct Poodle menu/dialog composition plus local
workflow helpers.
