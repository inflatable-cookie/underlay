# 025 - Dairy Written Exam Question Form Proof

Status: not started
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

- [ ] Dairy written exam question create/edit pages use `EntityFormPage`
- [ ] loading and error states still render correctly
- [ ] route-owned validation and authoring state still work correctly
- [ ] edit-page metadata and back links remain correct

## Next Task

Execute this proof batch.
