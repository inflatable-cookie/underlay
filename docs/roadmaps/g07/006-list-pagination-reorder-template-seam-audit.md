# g07.006 - List, Pagination, Reorder, And Template Seam Audit

Status: ready
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.005` kept the public auth/data helper split and consolidated duplicated
auth-fetch mechanics internally.

The next ambiguity is the shape of the broader list/data workflow family:
list controllers, pagination controllers, reorder sessions, batch helpers, and
template-owned list shells are all retained, but their seam needs a fresh audit.

## Goals

- [ ] inventory the list, pagination, reorder, batch, and template list seams
- [ ] classify each helper as runtime-owned, template-owned, app-owned, or
  candidate-retire
- [ ] identify duplicated list workflow state or mismatched docs
- [ ] decide whether `runtime/data` should remain broad or queue a bounded
  split
- [ ] classify consumer impact before changing public exports or behavior

## Non-Goals

- template-system rewrite
- forcing all consumer lists onto templates
- moving Poodle primitives into Underlay
- broad consumer migration without a specific affected surface
- Rust API changes

## Execution Plan

- [ ] inspect `runtime/data`, list/pagination/reorder implementation files, and
  template list components
- [ ] compare with contracts `100`, `110`, `116`, and `117`
- [ ] scan active docs and the six-consumer family for usage
- [ ] write a seam classification artifact with any bounded follow-on cards

## Acceptance Criteria

- [ ] each retained list/data helper has a clear owner and public posture
- [ ] template-owned behavior is separated from lower controller behavior
- [ ] duplicated or ambiguous workflow state has a disposition
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

Execute this list, pagination, reorder, and template seam audit.
