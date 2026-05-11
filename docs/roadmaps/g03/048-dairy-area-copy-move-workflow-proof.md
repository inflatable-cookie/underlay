# 048 - Dairy Area Copy/Move Workflow Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.047` proved `EntityFormPage` against section copy/move workflows. The next
smallest remaining family in the copy/move lane is areas.

## Targets

1. `/learning/areas/copy` — Area copy workflow (711 lines)
2. `/learning/areas/move` — Area move workflow (753 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned copy/move workflow logic, staged selection state, and validation
- preserve hierarchy context, workflow warnings, and contextual back links

## Non-Goals

- changing area copy/move workflow behavior
- widening templates for copy/move workflow logic
- tackling outcome or module copy/move flows in the same batch

## Exit Criteria

- [x] Dairy area copy/move pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned copy/move workflow logic, staged selection state, and validation still work correctly
- [x] hierarchy context, workflow warnings, and back links remain correct

## Results

- migrated Dairy area copy/move workflow pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned staged selection state, target-section preload logic,
  preview/order/submit workflow, inline operation status rendering, and
  validation behavior
- preserved hierarchy context and contextual back-link posture
- kept the remaining copy/move workflow lane moving in size order

## Next Task

Execute `g03.049`.
