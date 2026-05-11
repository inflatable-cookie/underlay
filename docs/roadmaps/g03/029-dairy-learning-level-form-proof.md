# 029 - Dairy Learning Level Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.028` closed the compact Dairy user CRUD proof. The next honest remaining
pair is the learning level family: the smallest real learning create/edit batch
left before the larger sections, areas, and module shells.

## Targets

1. `/learning/levels/new` — Create level form (173 lines)
2. `/learning/levels/[levelId]/edit` — Edit level form (232 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation and level loading
- preserve edit-page metadata and contextual back links

## Non-Goals

- changing learning-level workflow behavior
- widening templates for learning-specific policy or hierarchy logic
- tackling broader learning form families in the same batch

## Exit Criteria

- [x] Dairy learning level create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation and level loading still work correctly
- [x] edit-page metadata and back links remain correct

## Results

- migrated Dairy learning level create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned pathway option loading, validation, and level loading
- preserved edit-page etag conflict handling and contextual back links
- proved the shell on the first real learning form pair before the larger
  sections, areas, and module families

## Next Task

Execute `g03.030`.
