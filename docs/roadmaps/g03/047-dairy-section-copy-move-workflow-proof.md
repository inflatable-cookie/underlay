# 047 - Dairy Section Copy/Move Workflow Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.046` closed the activity-authoring create/edit proof line. The remaining
`SpaFormShell` surface is now the broader copy/move workflow lane. The smallest
remaining family there is the section copy/move pair.

## Targets

1. `/learning/sections/copy` — Section copy workflow (663 lines)
2. `/learning/sections/move` — Section move workflow (657 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned copy/move workflow logic, staged selection state, and validation
- preserve hierarchy context, workflow warnings, and contextual back links

## Non-Goals

- changing section copy/move workflow behavior
- widening templates for copy/move workflow logic
- tackling area, outcome, or module copy/move flows in the same batch

## Exit Criteria

- [x] Dairy section copy/move pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned copy/move workflow logic, staged selection state, and validation still work correctly
- [x] hierarchy context, workflow warnings, and back links remain correct

## Results

- migrated Dairy section copy/move workflow pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned staged selection state, preview/order/submit workflow,
  inline operation status rendering, and validation behavior
- preserved hierarchy context and contextual back-link posture
- started the remaining copy/move workflow lane with the smallest family first

## Next Task

Execute `g03.048`.
