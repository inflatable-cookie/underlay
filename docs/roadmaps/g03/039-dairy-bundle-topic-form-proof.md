# 039 - Dairy Bundle Topic Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.038` proved `EntityFormPage` against module notices. The next smallest
remaining create/edit pair is bundle topics: another compact local workflow
pair that is still narrower than the activity families.

## Targets

1. `/learning/bundles/[bundleId]/topics/new` — Create bundle topic form (173 lines)
2. `/learning/bundles/[bundleId]/topics/[topicId]/edit` — Edit bundle topic form (244 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation and topic loading
- preserve edit-page metadata, bundle context, and contextual back links

## Non-Goals

- changing bundle topic workflow behavior
- widening templates for bundle-local workflow logic
- tackling aliases, variants, or activity families in the same batch

## Exit Criteria

- [x] Dairy bundle topic create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation and topic loading still work correctly
- [x] edit-page metadata, bundle context, and back links remain correct

## Results

- migrated Dairy bundle topic create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned validation, topic loading, delete flow, and etag
  conflict handling
- preserved edit-page bundle context and contextual back-link posture
- kept the remaining post-hierarchy queue moving through the smallest local
  workflow pairs first

## Next Task

Execute `g03.040`.
