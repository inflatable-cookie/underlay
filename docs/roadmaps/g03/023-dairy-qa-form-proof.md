# 023 - Dairy QA Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.021` and `g03.022` proved `EntityFormPage` against richer Dairy content
families that keep Nightfire state and preparation logic in the route. The next
honest content proof is the QA create/edit pair, which adds dual Nightfire body
handling and AI-prefill integration.

## Targets

1. `/content/qa/new` — Create QA form (308 lines)
2. `/content/qa/[qaItemId]/edit` — Edit QA form (306 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading, error, and not-found behavior on edit
- preserve question/answer Nightfire handling and AI-prefill wiring
- preserve edit-page metadata and contextual back links

## Non-Goals

- changing QA workflow behavior
- widening templates for QA-specific dual-body behavior
- tackling other content authoring families in the same batch

## Exit Criteria

- [x] Dairy QA create/edit pages use `EntityFormPage`
- [x] loading, error, and not-found states still render correctly
- [x] dual Nightfire body handling and AI-prefill wiring still work correctly
- [x] edit-page metadata and back links remain correct

## Results

- migrated Dairy QA create/edit pages from `SpaFormShell` to `EntityFormPage`
- preserved dual Nightfire question/answer body handling
- preserved AI-prefill and suggestion-audit wiring on create
- preserved edit-page loading, error, and not-found handling
- proved the shell against a stronger content authoring family than the earlier
  single-body content forms

## Next Task

Execute `g03.024`.
