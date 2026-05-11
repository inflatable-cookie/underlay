# 044 - Dairy Pre-Seen Activity Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.043` proved `EntityFormPage` against digital-exam-question activities.
The next smallest remaining create/edit pair in the activity-authoring lane is
pre-seen activities.

## Targets

1. `/learning/activities/pre-seen/new` — Create pre-seen activity form (370 lines)
2. `/learning/activities/pre-seen/[activityId]/edit` — Edit pre-seen activity form (424 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation, activity loading, and release wiring
- preserve edit-page metadata, hierarchy context, and contextual back links

## Non-Goals

- changing pre-seen activity workflow behavior
- widening templates for activity-authoring workflow logic
- tackling the broader copy/move workflow lane in the same batch

## Exit Criteria

- [x] Dairy pre-seen activity create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation, activity loading, and release wiring still work correctly
- [x] edit-page metadata, hierarchy context, and back links remain correct

## Results

- migrated Dairy pre-seen activity create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned release and area selector wiring, material handling,
  activity loading, and edit-page live-status banner behavior
- preserved edit-page hierarchy context and contextual back-link posture
- kept the activity-authoring lane moving in size order while leaving the
  broader copy/move workflow lane separate

## Next Task

Execute `g03.045`.
