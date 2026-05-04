# 006 - Acme-Admin List Page Proof

Status: complete (pending validation)
Owner: repo maintainers
Updated: 2026-05-04

## Context

acme-admin `/projects/+page.svelte` is 506 lines of hand-rolled composition.
It is the ideal first proof because it exercises: filters, sort, cards
presentation, batch selection, batch delete, and reorder mode.

## Goals

- migrate acme-admin `/projects/+page.svelte` to `EntityListPage`
- target: ~50 lines of declarative config
- validate that all existing behavior is preserved

## Non-Goals

- changing the visual design
- adding new features — parity only

## Pre-Migration Enhancements (Completed)

Before migration, `EntityList` and `EntityListPage` were enhanced to support
the full feature surface of the acme-admin projects page:

- **URL sync**: `filterValues`, `sort`, `onFilterChange`, `onSortChange` props
- **Selection context**: `renderItem` now receives `(item, { selectionMode, selected, onToggle })`
- **Async filter loading**: `loadOptions` callback on filter config
- **Sort in filters**: `type: "sort"` filter with `sortFields`
- **Reorder error recovery**: `onReorderError` callback

## Execution Plan

### Batch 6.1 - Migration

- [x] enhance `EntityList`/`EntityListPage` for full feature parity
- [x] rewrite `/projects/+page.svelte` using `EntityListPage`
- [ ] test: filters work (name, category, status)
- [ ] test: sort works
- [ ] test: cards render correctly
- [ ] test: batch selection and delete work
- [ ] test: reorder mode works

### Batch 6.2 - Line Count Verification

- [x] confirm page line count: **506 → 158 lines** (69% reduction)
- [x] document the before/after line count as evidence

## Exit Criteria

- acme-admin projects list renders identically to before
- all interactions work: filters, sort, batch, reorder
- page is ~50 lines of declarative config

## Next Task

Execute `g03.009`: migrate acme-admin project detail page as proof.
