# 018 - Dairy Exam Edition Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

After the Dairy exam schedule proof, the adjacent exam edition create/edit pair
was the next small, symmetric consumer batch still using `SpaFormShell`.

## Targets

1. `/exams/editions/new` — Create exam edition form (165 lines)
2. `/exams/editions/[editionId]/edit` — Edit exam edition form (199 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve module search and schedule-loading behavior
- preserve form-level errors, redirects, and contextual back links
- preserve edit-page metadata and delete action behavior

## Non-Goals

- changing workflow behavior
- widening templates for Dairy-only exam-edition behavior
- tackling broader content or learning form families in the same batch

## Exit Criteria

- [x] Dairy exam edition create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] edit-page metadata and delete action remain intact
- [x] redirects and contextual back links remain correct

## Results

- migrated Dairy exam edition create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved module relation search and per-module schedule loading
- preserved edit-page metadata and delete action wiring
- extended the Dairy form proof from exam schedules into the adjacent exam
  edition family while the shape was still identical

## Next Task

Execute `g03.019`.
