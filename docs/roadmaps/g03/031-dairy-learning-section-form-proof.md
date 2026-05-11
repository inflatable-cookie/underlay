# 031 - Dairy Learning Section Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.030` proved `EntityFormPage` against the next adjacent learning hierarchy
pair after levels. The next smallest remaining real learning family is
sections: still manageable, but clearly larger than levels and pre-seen
releases.

## Targets

1. `/learning/sections/new` — Create section form (213 lines)
2. `/learning/sections/[sectionId]/edit` — Edit section form (409 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation and section loading
- preserve edit-page metadata, hierarchy context, and contextual back links

## Non-Goals

- changing learning section workflow behavior
- widening templates for learning hierarchy logic
- tackling areas, outcomes, modules, or activity families in the same batch

## Exit Criteria

- [x] Dairy learning section create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation and section loading still work correctly
- [x] edit-page metadata, hierarchy context, and back links remain correct

## Results

- migrated Dairy learning section create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned module lookup, validation, section loading, and AI
  suggestion wiring
- preserved edit-page stitched preview, etag conflict handling, and hierarchy
  back-link posture
- proved the shell against the first clearly larger learning hierarchy pair
  after levels and pre-seen releases

## Next Task

Execute `g03.032`.
