# 040 - Dairy Module Alias Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.039` proved `EntityFormPage` against bundle topics. The next smallest
remaining create/edit pair is module aliases: another compact module-local
workflow pair that still sits well below the heavier activity families.

## Targets

1. `/learning/modules/[moduleId]/aliases/new` — Create module alias form (184 lines)
2. `/learning/modules/[moduleId]/aliases/[aliasId]/edit` — Edit module alias form (246 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation and alias loading
- preserve edit-page metadata, module context, and contextual back links

## Non-Goals

- changing module alias workflow behavior
- widening templates for module-local workflow logic
- tackling variants or activity families in the same batch

## Exit Criteria

- [x] Dairy module alias create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation and alias loading still work correctly
- [x] edit-page metadata, module context, and back links remain correct

## Results

- migrated Dairy module alias create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned slug validation, alias loading, delete flow, and etag
  conflict handling
- preserved edit-page module context and contextual back-link posture
- kept the post-hierarchy queue moving through the smallest remaining
  module-local workflow pairs first

## Next Task

Execute `g03.041`.
