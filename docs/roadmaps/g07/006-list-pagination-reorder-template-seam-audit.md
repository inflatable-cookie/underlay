# g07.006 - List, Pagination, Reorder, And Template Seam Audit

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.005` kept the public auth/data helper split and consolidated duplicated
auth-fetch mechanics internally.

The next ambiguity is the shape of the broader list/data workflow family:
list controllers, pagination controllers, reorder sessions, batch helpers, and
template-owned list shells are all retained, but their seam needs a fresh audit.

## Goals

- [x] inventory the list, pagination, reorder, batch, and template list seams
- [x] classify each helper as runtime-owned, template-owned, app-owned, or
  candidate-retire
- [x] identify duplicated list workflow state or mismatched docs
- [x] decide whether `runtime/data` should remain broad or queue a bounded
  split
- [x] classify consumer impact before changing public exports or behavior

## Non-Goals

- template-system rewrite
- forcing all consumer lists onto templates
- moving Poodle primitives into Underlay
- broad consumer migration without a specific affected surface
- Rust API changes

## Execution Plan

- [x] inspect `runtime/data`, list/pagination/reorder implementation files, and
  template list components
- [x] compare with contracts `100`, `110`, `116`, and `117`
- [x] scan active docs and the six-consumer family for usage
- [x] write a seam classification artifact with any bounded follow-on cards

## Acceptance Criteria

- [x] each retained list/data helper has a clear owner and public posture
- [x] template-owned behavior is separated from lower controller behavior
- [x] duplicated or ambiguous workflow state has a disposition
- [x] no consumer-visible change lands without same-card proof

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`
- `effigy check:types`
- targeted source and consumer scans

## Consumer Upgrade Impact

None.

This card did not change public exports, imports, or runtime behavior.

## Next Task

Move to `g07.007`: relation selector boundary audit.
