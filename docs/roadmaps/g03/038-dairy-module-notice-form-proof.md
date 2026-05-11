# 038 - Dairy Module Notice Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.037` proved `EntityFormPage` against module syllabus updates. The next
smallest remaining create/edit pair is module notices: another compact
module-local workflow that extends the same proof line without jumping into the
heavier activity families.

## Targets

1. `/learning/modules/[moduleId]/notices/new` — Create notice form (176 lines)
2. `/learning/modules/[moduleId]/notices/[noticeId]/edit` — Edit notice form (240 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation and notice loading
- preserve edit-page metadata, module context, and contextual back links

## Non-Goals

- changing notice workflow behavior
- widening templates for module-local workflow logic
- tackling aliases, variants, bundle topics, or activity families in the same batch

## Exit Criteria

- [x] Dairy module notice create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation and notice loading still work correctly
- [x] edit-page metadata, module context, and back links remain correct

## Results

- migrated Dairy module notice create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned validation, notice loading, delete flow, and etag
  conflict handling
- preserved edit-page module context and contextual back-link posture
- kept the post-learning-family queue moving through the compact module-local
  workflow pairs

## Next Task

Execute `g03.039`.
