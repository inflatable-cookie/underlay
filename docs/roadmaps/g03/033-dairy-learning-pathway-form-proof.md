# 033 - Dairy Learning Pathway Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.032` proved `EntityFormPage` against the bundle family. The next smallest
remaining learning hierarchy pair is pathways: larger than bundles, but still a
cleaner proof target than modules, areas, or outcomes.

## Targets

1. `/learning/pathways/new` — Create pathway form (280 lines)
2. `/learning/pathways/[pathwayId]/edit` — Edit pathway form (410 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation and pathway loading
- preserve edit-page metadata, hierarchy context, and contextual back links

## Non-Goals

- changing learning pathway workflow behavior
- widening templates for learning hierarchy logic
- tackling modules, areas, outcomes, or activity families in the same batch

## Exit Criteria

- [x] Dairy learning pathway create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation and pathway loading still work correctly
- [x] edit-page metadata, hierarchy context, and back links remain correct

## Results

- migrated Dairy learning pathway create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned Nightfire handling, slug validation, pathway loading,
  delete flow, and etag conflict handling
- preserved edit-page metadata, live-status banner, and hierarchy back-link
  posture
- proved the shell on the next larger learning hierarchy pair after bundles

## Next Task

Execute `g03.034`.
