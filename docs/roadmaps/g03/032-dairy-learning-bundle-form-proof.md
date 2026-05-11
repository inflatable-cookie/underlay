# 032 - Dairy Learning Bundle Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.031` proved `EntityFormPage` against the first clearly larger learning
hierarchy pair. The next smallest remaining learning family is bundles: still
smaller than pathways, areas, outcomes, and modules, but large enough to keep
the learning-specific route posture honest.

## Targets

1. `/learning/bundles/new` — Create bundle form (211 lines)
2. `/learning/bundles/[bundleId]/edit` — Edit bundle form (299 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation and bundle loading
- preserve edit-page metadata, hierarchy context, and contextual back links

## Non-Goals

- changing learning bundle workflow behavior
- widening templates for learning hierarchy logic
- tackling pathways, areas, outcomes, modules, or activity families in the same batch

## Exit Criteria

- [x] Dairy learning bundle create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation and bundle loading still work correctly
- [x] edit-page metadata, hierarchy context, and back links remain correct

## Results

- migrated Dairy learning bundle create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned module prefill, bundle loading, module-membership diff
  application, and etag conflict handling
- preserved edit-page hierarchy context and contextual back-link posture
- proved the shell on the next smallest remaining learning family after
  sections

## Next Task

Execute `g03.033`.
