# g07.007 - Relation Selector Boundary Audit

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.006` kept `runtime/data` broad and coherent as the lower collection
workflow layer below templates.

The next boundary is `runtime/relations`: relation selector context/types,
local search helpers, drill-down helpers, selection history, and suggestion
client helpers sit near each other but do not all have the same owner.

## Goals

- [x] inventory relation selector, drill-down, local search, selection history,
  and suggestion client surfaces
- [x] classify each surface as runtime-owned, client-owned, template-owned,
  app-owned, or candidate-retire
- [x] decide whether `runtime/relations` remains one coherent public path
- [x] identify stale docs or consumer import drift
- [x] classify consumer impact before changing exports or behavior

## Non-Goals

- rewriting relation selector UI
- changing app-local relation DTOs or command names
- adding selector-only backend routes
- broad consumer migration without a specific affected surface

## Execution Plan

- [x] inspect `runtime/relations`, relation selector implementation files,
  selection history, and `client/suggestions`
- [x] compare with contracts `080`, `090`, `100`, and collection route
  contracts
- [x] scan active docs and the six-consumer family for usage
- [x] write a boundary classification artifact with any bounded follow-on cards

## Acceptance Criteria

- [x] relation selector and suggestion surfaces have clear owners
- [x] `runtime/relations` either stays coherent or has a queued split decision
- [x] active docs teach the retained public paths
- [x] no consumer-visible change lands without same-card proof

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`
- `effigy check:types`
- targeted source and consumer scans

## Consumer Upgrade Impact

None.

No public API or runtime behavior changed. One active guide import was corrected
from `runtime/relations` to `client/suggestions` for client-owned suggestion
request helpers.

## Next Task

Execute `g07.008`: TS testing and guardrail support gap inventory.
