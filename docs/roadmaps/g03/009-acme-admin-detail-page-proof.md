# 009 - Acme-Admin Detail Page Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-04

## Context

acme-admin `/projects/[id]/+page.svelte` is 800 lines. It has: header, meta,
detail sections, tasks tab with filters, batch actions, and reorder. The tasks
tab is essentially an `EntityList` inside a tab.

## Goals

- migrate to `EntityDetailPage` + `EntityList` in the tasks tab
- target: ~80 lines
- preserve all behavior

## Results

- migrated acme-admin `/projects/[projectId]/+page.svelte` to `EntityDetailPage` + `EntityList`
- line count: **800 → 412 lines** (48% reduction)
- all behavior preserved: header, meta, details, tabs, task filters, batch actions, reorder
- custom batch dialog support added to templates for the "update status" action

## Exit Criteria

- [x] page renders identically
- [x] all tabs work
- [x] task list filters, batch, reorder work
- [x] page migrated to templates

## Notes

The ~80 line target was optimistic for a detail page that includes both project
metadata/sections and a full task list with filters, batch actions, and reorder.
The 412-line result is still a significant reduction from 800 lines and validates
the template system for complex detail pages.

Template enhancements required for this migration:
- Custom batch action dialog support (added in g03.009)

## Next Task

Execute `g03.010`: stress-test with Dairy's complex cases (cascading filters,
batch transforms, stitched data).
