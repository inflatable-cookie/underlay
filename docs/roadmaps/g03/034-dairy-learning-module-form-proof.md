# 034 - Dairy Learning Module Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.033` proved `EntityFormPage` against pathways. The next smallest remaining
learning hierarchy pair is modules: still smaller than areas and outcomes, but
substantially richer than the earlier learning form proofs.

## Targets

1. `/learning/modules/new` — Create module form (318 lines)
2. `/learning/modules/[moduleId]/edit` — Edit module form (401 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation and module loading
- preserve edit-page metadata, hierarchy context, and contextual back links

## Non-Goals

- changing learning module workflow behavior
- widening templates for learning hierarchy logic
- tackling areas, outcomes, or activity families in the same batch

## Exit Criteria

- [x] Dairy learning module create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation and module loading still work correctly
- [x] edit-page metadata, hierarchy context, and back links remain correct

## Results

- migrated Dairy learning module create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned Nightfire handling, slug validation, inline level
  creation support, module loading, and etag conflict handling
- preserved edit-page metadata, live-status banner, and hierarchy back-link
  posture
- proved the shell on the next richer learning hierarchy pair after pathways

## Next Task

Execute `g03.035`.
