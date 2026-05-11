# 036 - Dairy Learning Outcome Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.035` proved `EntityFormPage` against areas. The remaining learning
hierarchy pair is outcomes. Finishing it closes the main Dairy learning form
family under the shared form-page shell.

## Targets

1. `/learning/outcomes/new` — Create outcome form (490 lines)
2. `/learning/outcomes/[outcomeId]/edit` — Edit outcome form (482 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation and outcome loading
- preserve edit-page metadata, hierarchy context, and contextual back links

## Non-Goals

- changing learning outcome workflow behavior
- widening templates for learning hierarchy logic
- tackling activity families or deeper workflow lanes in the same batch

## Exit Criteria

- [x] Dairy learning outcome create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation and outcome loading still work correctly
- [x] edit-page metadata, hierarchy context, and back links remain correct

## Results

- migrated Dairy learning outcome create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned area selection, inline area creation, outcome loading,
  AI suggestion wiring, and etag conflict handling
- preserved edit-page metadata and hierarchy back-link posture
- closed the main Dairy learning hierarchy form family under the shared
  `EntityFormPage` shell

## Next Task

Execute `g03.037`.
