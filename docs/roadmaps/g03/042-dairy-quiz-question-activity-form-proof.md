# 042 - Dairy Quiz Question Activity Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.041` proved `EntityFormPage` against module variants. The remaining
`SpaFormShell` surface is now split between heavier copy/move workflows and the
activity-authoring family. The smallest remaining create/edit pair in that
authoring lane is quiz-question activities.

## Targets

1. `/learning/activities/quiz-question/new` — Create quiz-question activity form (328 lines)
2. `/learning/activities/quiz-question/[activityId]/edit` — Edit quiz-question activity form (406 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation, activity loading, and question wiring
- preserve edit-page metadata, hierarchy context, and contextual back links

## Non-Goals

- changing quiz-question activity workflow behavior
- widening templates for activity-authoring workflow logic
- tackling the broader copy/move workflow lane in the same batch

## Exit Criteria

- [x] Dairy quiz-question activity create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation, activity loading, and question wiring still work correctly
- [x] edit-page metadata, hierarchy context, and back links remain correct

## Results

- migrated Dairy quiz-question activity create/edit pages from
  `SpaFormShell` to `EntityFormPage`
- preserved route-owned question selector wiring, material handling, activity
  loading, and edit-page live-status banner behavior
- preserved edit-page hierarchy context and contextual back-link posture
- advanced the queue into the remaining activity-authoring family, leaving the
  broader copy/move workflow lane separate

## Next Task

Execute `g03.043`.
