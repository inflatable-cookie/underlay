# 035 - Dairy Learning Area Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.034` proved `EntityFormPage` against modules. The next remaining learning
hierarchy pair is areas: broader than modules, but still the next honest step
before the larger outcome family.

## Targets

1. `/learning/areas/new` — Create area form (373 lines)
2. `/learning/areas/[areaId]/edit` — Edit area form (530 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation and area loading
- preserve edit-page metadata, hierarchy context, and contextual back links

## Non-Goals

- changing learning area workflow behavior
- widening templates for learning hierarchy logic
- tackling outcomes or activity families in the same batch

## Exit Criteria

- [x] Dairy learning area create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation and area loading still work correctly
- [x] edit-page metadata, hierarchy context, and back links remain correct

## Results

- migrated Dairy learning area create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned section/module lookup, inline section creation, area
  loading, AI suggestion wiring, and etag conflict handling
- preserved edit-page metadata, stitched preview, and hierarchy back-link
  posture
- proved the shell against the largest remaining learning hierarchy pair before
  outcomes

## Next Task

Execute `g03.036`.
