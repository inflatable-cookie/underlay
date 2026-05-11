# 050 - Dairy Module Copy/Move Workflow Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.049` proved `EntityFormPage` against outcome copy/move workflows. The last
remaining family in the copy/move lane is modules.

## Targets

1. `/learning/modules/copy` — Module copy workflow (1102 lines)
2. `/learning/modules/move` — Module move workflow (901 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned copy/move workflow logic, staged selection state, and validation
- preserve hierarchy context, workflow warnings, and contextual back links

## Non-Goals

- changing module copy/move workflow behavior
- widening templates for copy/move workflow logic
- broadening this batch into another template-surface redesign

## Exit Criteria

- [x] Dairy module copy/move pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned copy/move workflow logic, staged selection state, and validation still work correctly
- [x] hierarchy context, workflow warnings, and back links remain correct

## Results

- migrated Dairy module copy/move workflow pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned staged selection state, preview/order/submit workflow,
  inline operation status rendering, validation behavior, and copy-specific
  override checks
- preserved hierarchy context and contextual back-link posture
- removed the last remaining `SpaFormShell` route usage from Dairy app routes

## Next Task

Execute `g03.051`.
