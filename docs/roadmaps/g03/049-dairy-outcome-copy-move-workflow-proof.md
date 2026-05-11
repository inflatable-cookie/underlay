# 049 - Dairy Outcome Copy/Move Workflow Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.048` proved `EntityFormPage` against area copy/move workflows. The next
smallest remaining family in the copy/move lane is outcomes.

## Targets

1. `/learning/outcomes/copy` — Outcome copy workflow (683 lines)
2. `/learning/outcomes/move` — Outcome move workflow (768 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned copy/move workflow logic, staged selection state, and validation
- preserve hierarchy context, workflow warnings, and contextual back links

## Non-Goals

- changing outcome copy/move workflow behavior
- widening templates for copy/move workflow logic
- tackling module copy/move flows in the same batch

## Exit Criteria

- [x] Dairy outcome copy/move pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned copy/move workflow logic, staged selection state, and validation still work correctly
- [x] hierarchy context, workflow warnings, and back links remain correct

## Results

- migrated Dairy outcome copy/move workflow pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned staged selection state, target-area preload logic,
  preview/order/submit workflow, inline operation status rendering, and
  validation behavior
- preserved hierarchy context and contextual back-link posture
- narrowed the remaining copy/move lane to the final modules family

## Next Task

Execute `g03.050`.
