# 025 - Dairy Written Exam Question Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.024` proved `EntityFormPage` against the heavy Dairy digital exam question
family. The next honest adjacent proof is the written exam question pair, which
keeps the same broader markable-question posture with its own route-owned
validation and authoring state.

## Targets

1. `/content/written-exam-questions/new` — Create written exam question form (382 lines)
2. `/content/written-exam-questions/[writtenExamQuestionId]/edit` — Edit written exam question form (453 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation and authoring state
- preserve edit-page metadata and contextual back links

## Non-Goals

- changing written exam question workflow behavior
- widening templates for question-family-specific authoring logic
- tackling additional question families in the same batch

## Exit Criteria

- [x] Dairy written exam question create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation and authoring state still work correctly
- [x] edit-page metadata and back links remain correct

## Results

- migrated Dairy written exam question create/edit pages from `SpaFormShell`
  to `EntityFormPage`
- preserved route-owned validation and authoring state
- preserved edit-page loading, error, and not-found handling
- proved the shell across the adjacent written question family after the
  digital question proof

## Next Task

Execute `g03.026`.
