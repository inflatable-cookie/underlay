# 027 - Dairy Mock Config Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.026` completed the quiz question pair and closed the adjacent
question-authoring run. The next honest nearby proof is the Dairy mock-config
pair, which is smaller but still exercises route-owned option loading, edit
metadata, and delete-style workflow actions.

## Targets

1. `/exams/mocks/new` — Create mock config form (156 lines)
2. `/exams/mocks/[editionId]/edit` — Edit mock config form (187 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned edition loading and validation
- preserve edit-page metadata, contextual back links, and remove-config action

## Non-Goals

- changing mock-config workflow behavior
- widening templates for mock-specific workflow logic
- tackling other exams or learning form families in the same batch

## Exit Criteria

- [x] Dairy mock-config create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned edition loading and validation still work correctly
- [x] edit-page metadata, back links, and remove-config action remain correct

## Results

- migrated Dairy mock-config create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned edition loading and validation
- preserved edit-page metadata, contextual back links, and remove-config action
- proved the shell against the smaller exams-adjacent config workflow after the
  main schedule and edition form proofs

## Next Task

Execute `g03.028`.
