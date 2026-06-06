# g07.001 - TS Runtime And Workflow Ownership Inventory

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g06` closed the Rust platform-contract transition and reference-grade reset.
The next reference-grade risk is the TypeScript surface: runtime barrels,
pattern-owned workflow helpers, template shells, and support utilities are
usable, but the contracts still name several ownership drifts.

This card inventories the current surface before any removals or consumer
rollout work.

## Goals

- [x] inventory public TypeScript exports under `runtime/*`, `patterns/*`,
  `client/*`, `templates/*`, `testing/*`, `tools/*`, `utils/*`, and
  `nightfire/*`
- [x] classify each family as retained, candidate-rehome, candidate-remove,
  candidate-consolidate, or support-only
- [x] identify duplicated workflow orchestration, especially auth-aware
  fetch/list/pagination flows
- [x] identify template-vs-pattern overlap that could confuse consuming apps
- [x] record which findings need consumer import proof before implementation

## Non-Goals

- changing exports
- changing package subpaths
- editing consumer apps
- splitting large test files
- resolving the warning-only `scan.god-files` backlog

## Execution Plan

- [x] inspect `package.json` exports, `ts/src/index.ts`, and subpath barrels
- [x] compare the implementation surface to contracts `090`, `100`, `110`, and
  `120`
- [x] scan for stale or surprising export paths across Underlay docs and source
- [x] produce a `g07.001` artifact with the classification table and the next
  bounded cards

## Acceptance Criteria

- [x] the artifact names every major TS public family and its classification
- [x] consumer-affecting candidates are explicitly marked as requiring rollout
  proof under `023`
- [x] no implementation work is started from intuition before the inventory is
  complete
- [x] the roadmap queue advances to `g07.002` only if the runtime subpath audit
  has enough evidence to execute without fresh planning

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`
- targeted `rg` scans for stale import/export guidance

## Consumer Upgrade Impact

None for this inventory card.

Follow-on cards may be `additive`, `deprecation`, or `breaking` depending on
the classified surface and consumer proof.

## Next Task

Move to `g07.002`: runtime subpath public surface audit.
