# 051 - Dairy Template Proof Line Closeout

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.050` closes the final Dairy route family that still used
`SpaFormShell`. The next task is not another blind migration card. It is to
record the final proof posture, verify the remaining surface honestly, and
decide whether `g03` should stop or turn into a new lane.

## Goals

- confirm the Dairy route-level `SpaFormShell` normalization line is complete
- record what remains as retained shared Underlay surface versus consumer-local workflow logic
- update the queue so `g03` stops pretending there is still a migration tail if there is not

## Non-Goals

- opening a new speculative template lane without evidence
- broadening the batch into unrelated contract or pattern redesign
- changing the retained `SpaFormShell` API in the same batch

## Exit Criteria

- [x] Dairy route-level `SpaFormShell` usage is rechecked and classified
- [x] `g03` README and roadmap front doors reflect the real stop point
- [x] next move is explicit: stop, close, or open a new evidence-backed lane

## Results

- Dairy route-level `SpaFormShell` normalization is complete
- `rg -n "SpaFormShell" dairy/src/routes/(app)` now returns no matches
- retained shared Underlay surface:
  - `EntityFormPage` as the route shell for create/edit and copy/move workflows
  - `SpaFormShell` remains a retained surface, but it is no longer the live
    Dairy route-level pattern
- retained consumer-local workflow logic:
  - staged copy/move state
  - preview/order/submit flows
  - inline status panels
  - domain-specific validation and selector wiring
  - delete flows, etag conflict handling, and contextual back-link derivation
- the proof line now covers:
  - CRUD/admin forms
  - richer content authoring forms
  - question authoring forms
  - learning hierarchy forms
  - activity authoring forms
  - copy/move workflow routes

## Closeout

`g03` stops here as a completed template-system generation.

The next honest lane is not another blind Dairy migration card. It is either:

- a new evidence-backed generation for the next retained shared template gap
- or a separate consumer adoption lane if another site exposes new pressure on
  the retained shell/controller boundary

## Next Task

None. `g03` is complete.
