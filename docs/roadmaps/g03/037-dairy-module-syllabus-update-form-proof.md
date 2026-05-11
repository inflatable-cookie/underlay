# 037 - Dairy Module Syllabus Update Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.036` closed the main Dairy learning hierarchy form family. The next
smallest remaining create/edit pair is module syllabus updates: a compact
module-local workflow that still keeps the shared form-page proof moving
without jumping into the heavier activity families.

## Targets

1. `/learning/modules/[moduleId]/syllabus-updates/new` — Create syllabus update form (165 lines)
2. `/learning/modules/[moduleId]/syllabus-updates/[updateId]/edit` — Edit syllabus update form (242 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation and syllabus-update loading
- preserve edit-page metadata, module context, and contextual back links

## Non-Goals

- changing syllabus-update workflow behavior
- widening templates for module-local workflow logic
- tackling aliases, notices, variants, bundle topics, or activity families in the same batch

## Exit Criteria

- [x] Dairy module syllabus update create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation and syllabus-update loading still work correctly
- [x] edit-page metadata, module context, and back links remain correct

## Results

- migrated Dairy module syllabus-update create/edit pages from `SpaFormShell`
  to `EntityFormPage`
- preserved route-owned validation, syllabus-update loading, delete flow, and
  etag conflict handling
- preserved edit-page module context and contextual back-link posture
- kept the post-learning-family queue moving on compact module-local workflow
  pairs

## Next Task

Execute `g03.038`.
