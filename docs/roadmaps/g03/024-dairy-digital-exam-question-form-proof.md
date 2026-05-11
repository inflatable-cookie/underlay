# 024 - Dairy Digital Exam Question Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.023` proved `EntityFormPage` against Dairy QA, including dual Nightfire
body handling and AI-prefill behavior. The next honest content authoring proof
is the digital exam question pair, which is larger and carries richer
module/edition loading plus multi-surface Nightfire authoring state.

## Targets

1. `/content/digital-exam-questions/new` — Create digital exam question form (467 lines)
2. `/content/digital-exam-questions/[digitalExamQuestionId]/edit` — Edit digital exam question form (545 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading, error, and not-found behavior where present
- preserve module/edition loading and question-type-specific Nightfire handling
- preserve edit-page metadata and contextual back links

## Non-Goals

- changing digital exam question workflow behavior
- widening templates for question-type-specific authoring behavior
- tackling adjacent markable-question families in the same batch

## Exit Criteria

- [x] Dairy digital exam question create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] multi-surface Nightfire and module/edition loading still work correctly
- [x] edit-page metadata and back links remain correct

## Results

- migrated Dairy digital exam question create/edit pages from `SpaFormShell`
  to `EntityFormPage`
- preserved module/edition loading, label validation, and question-type-specific
  Nightfire handling
- preserved edit-page loading, error, and not-found handling
- proved the shell against the first heavy markable-question authoring family
  in Dairy

## Next Task

Execute `g03.025`.
