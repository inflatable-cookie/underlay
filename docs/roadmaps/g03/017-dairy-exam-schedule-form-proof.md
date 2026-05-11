# 017 - Dairy Exam Schedule Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.014` through `g03.016` proved `EntityFormPage` across the full
acme-admin form family. The next honest consumer proof is a small non-reference
app pair in Dairy that still uses `SpaFormShell`.

## Targets

1. `/exams/schedules/new` — Create exam schedule form (149 lines)
2. `/exams/schedules/[scheduleId]/edit` — Edit exam schedule form (185 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and option-fetch behavior
- preserve form-level errors, redirects, and contextual back links
- preserve edit-page metadata and delete action behavior

## Non-Goals

- changing workflow behavior
- widening templates for Dairy-only form behavior
- tackling broader learning or content form families in the same batch

## Exit Criteria

- [x] Dairy exam schedule create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] edit-page metadata and delete action remain intact
- [x] redirects and contextual back links remain correct

## Results

- migrated Dairy exam schedule create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved option-loading behavior on create
- preserved edit-page metadata and delete action wiring
- proved the shell against a second consumer app beyond `underlay-reference`

## Next Task

Execute `g03.018`.
