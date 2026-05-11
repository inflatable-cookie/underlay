# 026 - Dairy Quiz Question Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.025` proved `EntityFormPage` against the written exam question family.
The next honest adjacent proof is the quiz question pair, which is smaller but
still keeps route-owned Nightfire authoring state and outcome handling.

## Targets

1. `/content/quiz-questions/new` — Create quiz question form (244 lines)
2. `/content/quiz-questions/[quizQuestionId]/edit` — Edit quiz question form (323 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned Nightfire and outcome handling
- preserve edit-page metadata and contextual back links

## Non-Goals

- changing quiz question workflow behavior
- widening templates for quiz-specific authoring logic
- tackling other content question families in the same batch

## Exit Criteria

- [x] Dairy quiz question create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned Nightfire and outcome handling still work correctly
- [x] edit-page metadata and back links remain correct

## Results

- migrated Dairy quiz question create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned Nightfire and outcome handling
- preserved edit-page loading, error, and not-found handling
- proved the shell across the remaining adjacent question-authoring pair after
  the digital and written exam question proofs

## Next Task

Execute `g03.027`.
