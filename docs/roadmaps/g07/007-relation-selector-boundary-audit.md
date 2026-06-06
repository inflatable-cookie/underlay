# g07.007 - Relation Selector Boundary Audit

Status: ready
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.006` kept `runtime/data` broad and coherent as the lower collection
workflow layer below templates.

The next boundary is `runtime/relations`: relation selector context/types,
local search helpers, drill-down helpers, selection history, and suggestion
client helpers sit near each other but do not all have the same owner.

## Goals

- [ ] inventory relation selector, drill-down, local search, selection history,
  and suggestion client surfaces
- [ ] classify each surface as runtime-owned, client-owned, template-owned,
  app-owned, or candidate-retire
- [ ] decide whether `runtime/relations` remains one coherent public path
- [ ] identify stale docs or consumer import drift
- [ ] classify consumer impact before changing exports or behavior

## Non-Goals

- rewriting relation selector UI
- changing app-local relation DTOs or command names
- adding selector-only backend routes
- broad consumer migration without a specific affected surface

## Execution Plan

- [ ] inspect `runtime/relations`, relation selector implementation files,
  selection history, and `client/suggestions`
- [ ] compare with contracts `080`, `090`, `100`, and collection route
  contracts
- [ ] scan active docs and the six-consumer family for usage
- [ ] write a boundary classification artifact with any bounded follow-on cards

## Acceptance Criteria

- [ ] relation selector and suggestion surfaces have clear owners
- [ ] `runtime/relations` either stays coherent or has a queued split decision
- [ ] active docs teach the retained public paths
- [ ] no consumer-visible change lands without same-card proof

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`
- `effigy check:types`
- targeted source and consumer scans

## Consumer Upgrade Impact

None for the audit posture.

Update this section if the card makes public API, import, or behavior changes.

## Next Task

Execute this relation selector boundary audit.
