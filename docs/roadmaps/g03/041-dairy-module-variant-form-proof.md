# 041 - Dairy Module Variant Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.040` proved `EntityFormPage` against module aliases. The next smallest
remaining create/edit pair is module variants: another compact module-local
workflow pair that still sits well below the heavier activity families.

## Targets

1. `/learning/modules/[moduleId]/variants/new` — Create module variant form (190 lines)
2. `/learning/modules/[moduleId]/variants/[variantId]/edit` — Edit module variant form (259 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation and variant loading
- preserve edit-page metadata, module context, and contextual back links

## Non-Goals

- changing module variant workflow behavior
- widening templates for module-local workflow logic
- tackling activity families in the same batch

## Exit Criteria

- [x] Dairy module variant create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation and variant loading still work correctly
- [x] edit-page metadata, module context, and back links remain correct

## Results

- migrated Dairy module variant create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned code validation, variant loading, delete flow, and
  etag conflict handling
- preserved edit-page module context and contextual back-link posture
- advanced the queue from compact module-local workflow pairs into the
  smallest remaining activity-authoring proof family

## Next Task

Execute `g03.042`.
