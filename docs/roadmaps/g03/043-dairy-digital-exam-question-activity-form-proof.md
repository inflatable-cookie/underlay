# 043 - Dairy Digital Exam Question Activity Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.042` proved `EntityFormPage` against quiz-question activities. The next
smallest remaining create/edit pair in the activity-authoring lane is digital
exam question activities.

## Targets

1. `/learning/activities/digital-exam-question/new` — Create digital-exam-question activity form (340 lines)
2. `/learning/activities/digital-exam-question/[activityId]/edit` — Edit digital-exam-question activity form (414 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation, activity loading, and question wiring
- preserve edit-page metadata, hierarchy context, and contextual back links

## Non-Goals

- changing digital-exam-question activity workflow behavior
- widening templates for activity-authoring workflow logic
- tackling the broader copy/move workflow lane in the same batch

## Exit Criteria

- [x] Dairy digital-exam-question activity create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation, activity loading, and question wiring still work correctly
- [x] edit-page metadata, hierarchy context, and back links remain correct

## Results

- migrated Dairy digital-exam-question activity create/edit pages from
  `SpaFormShell` to `EntityFormPage`
- preserved route-owned question selector wiring, material handling, activity
  loading, and edit-page live-status banner behavior
- preserved edit-page hierarchy context and contextual back-link posture
- kept the activity-authoring lane moving in size order while leaving the
  broader copy/move workflow lane separate

## Next Task

Execute `g03.044`.
