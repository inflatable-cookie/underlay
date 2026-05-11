# 045 - Dairy Bundle Activity Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.044` proved `EntityFormPage` against pre-seen activities. The next
smallest remaining create/edit pair in the activity-authoring lane is bundle
activities.

## Targets

1. `/learning/activities/bundle/new` — Create bundle activity form (367 lines)
2. `/learning/activities/bundle/[activityId]/edit` — Edit bundle activity form (434 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation, activity loading, and bundle/topic wiring
- preserve edit-page metadata, hierarchy context, and contextual back links

## Non-Goals

- changing bundle activity workflow behavior
- widening templates for activity-authoring workflow logic
- tackling the broader copy/move workflow lane in the same batch

## Exit Criteria

- [x] Dairy bundle activity create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation, activity loading, and bundle/topic wiring still work correctly
- [x] edit-page metadata, hierarchy context, and back links remain correct

## Results

- migrated Dairy bundle activity create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned bundle and topic selector wiring, material handling,
  activity loading, and edit-page live-status banner behavior
- preserved edit-page hierarchy context and contextual back-link posture
- left the remaining activity-authoring tail explicit instead of mixing it with
  the broader copy/move workflow lane

## Next Task

Execute `g03.046`.
