# 010 - Dairy Complex Validation

Status: not started
Owner: repo maintainers
Updated: 2026-05-04

## Context

Dairy's admin pages have complex shapes: cascading filters (pathway → module →
section), batch transforms (copy/move to section), stitched data loading. The
templates must handle this without being "bolted on later."

## Goals

- evaluate Dairy's AreasList (535 lines) against template API
- identify gaps between declarative templates and complex real-world patterns
- plan template enhancements needed for full Dairy migration

## Findings

Evaluated `dairy/src/lib/lists/AreasList.svelte` (535 lines). Seven gaps found:

### 1. Cascading Filter Reset
When a parent filter changes (e.g., pathway), child filters (module, section) must reset. Current `FilterConfig` has no `cascade` or `resetChildren` mechanism.

### 2. Client-Side Filtering
EntityList filters are server-side (sent as query params). AreasList loads all data once then filters client-side by pathway/module/section/search. Need a `filterMode: "server" | "client"` option.

### 3. Batch Navigation Links
Copy/move selected areas navigate to form pages (`/learning/areas/copy?ids=...`), not dialogs. Current batch actions only support handlers/dialogs. Need `batchNavActions: [{ id, label, href }]`.

### 4. Conditional Reorder
Reorder only available when: in a section tab, no search filter, >1 items. Current `ReorderConfig` is just `enabled: boolean`. Need `reorder.canEnable?: (items, filters) => boolean` and `reorder.disabledReason?: string`.

### 5. Stitched Data Loading
`dataLoader` returns `T[]`, but AreasList loads `{ sections: Section[], areas: Area[] }`. Need support for stitched/multi-source data.

### 6. Custom Empty States
AreasList shows different messages: "No areas match your filter" vs "No areas defined yet". Current templates have generic empty states.

### 7. Tab Variant
AreasList works as both page (`/learning/areas`) and tab (inside section detail). EntityListPage is always a full page shell. Need `variant: "page" | "tab"` support.

## Exit Criteria

- [x] Dairy AreasList evaluated against template API
- [x] Seven gaps documented with specific enhancement proposals
- [ ] Template enhancements implemented (g03.010a)
- [ ] Dairy page migrated with templates (g03.010b)

## Next Task

Execute `g03.011`: build the `/underlay-template` developer skill.
