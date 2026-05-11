# 046 - Dairy Outcome Activity Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.045` proved `EntityFormPage` against bundle activities. The last remaining
create/edit pair in the activity-authoring lane is outcome activities.

## Targets

1. `/learning/activities/outcome/new` — Create outcome activity form (453 lines)
2. `/learning/activities/outcome/[activityId]/edit` — Edit outcome activity form (493 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation, activity loading, and outcome wiring
- preserve edit-page metadata, hierarchy context, and contextual back links

## Non-Goals

- changing outcome activity workflow behavior
- widening templates for activity-authoring workflow logic
- tackling the broader copy/move workflow lane in the same batch

## Exit Criteria

- [x] Dairy outcome activity create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation, activity loading, and outcome wiring still work correctly
- [x] edit-page metadata, hierarchy context, and back links remain correct

## Results

- migrated Dairy outcome activity create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned outcome selector wiring, material handling, activity
  loading, and edit-page live-status banner behavior
- preserved edit-page hierarchy context and contextual back-link posture
- closed the activity-authoring create/edit proof lane, leaving the broader
  copy/move workflow lane as the remaining `SpaFormShell` surface

## Next Task

Execute `g03.047`.
