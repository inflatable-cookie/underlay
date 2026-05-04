# 004 - EntityList Section Component

Status: complete
Owner: repo maintainers
Updated: 2026-05-04

## Context

`EntityList` is the Level 2 section component. It is a self-contained list
surface with filters, pagination, batch actions, empty states, and reorder mode.
It is used inside `EntityListPage` (as the main content) AND inside detail tabs
(as a child collection). This is the core reusable unit.

## Goals

- build `EntityList.svelte` with declarative configuration
- support both `presentation="cards"` and `presentation="table"`
- support filters, pagination, batch selection, reorder, and empty states
- wire Underlay runtime helpers internally (useAuthenticatedData,
  useBatchActions, createReorderController)
- accept render props for domain-specific items (renderItem, renderCard,
  columns)

## Non-Goals

- building the page shell (that's `EntityListPage` in g03.005)
- supporting every possible list variant upfront — start with the acme-admin
  projects list shape, then generalize

## API Design

```svelte
<EntityList
  title="Projects"                    <!-- optional, for inline use -->
  dataLoader={async (fetch, token, query) => ...}
  
  presentation="cards"                <!-- "cards" | "table" -->
  
  filters={[                          <!-- declarative filter config -->
    { id: "name", type: "search", label: "Name" },
    { id: "status", type: "select", label: "Status", options: [...] }
  ]}
  
  columns={tableColumns}              <!-- for presentation="table" -->
  
  renderItem={(item) => <ProjectCard {item} />}   <!-- for presentation="cards" -->
  
  batchActions={[                     <!-- optional -->
    { id: "delete", label: "Delete", tone: "danger", confirm: true, handler: ... }
  ]}
  
  reorder={{ enabled: true, handler: async (orderedIds) => ... }}  <!-- optional -->
  
  onAdd={() => goto("/projects/new")}  <!-- optional -->
  addLabel="Add project"               <!-- optional -->
/>
```

## Execution Plan

### Batch 4.1 - Core Structure

- [x] create `EntityList.svelte` with basic props interface
- [x] integrate `useAuthenticatedData` for data loading
- [x] integrate `FilterToolbar` for filter rendering
- [x] integrate `ListContainer` for page shell (when title is provided)

### Batch 4.2 - Cards Presentation

- [x] implement `presentation="cards"` with `Grid` + `renderItem` prop
- [x] integrate empty state handling
- [x] integrate loading and error states

### Batch 4.3 - Table Presentation

- [x] implement `presentation="table"` with `DataTable`
- [x] integrate column configuration
- [x] integrate row actions

### Batch 4.4 - Batch And Reorder

- [x] integrate `useBatchActions` for batch selection
- [x] integrate `BulkActionBar`
- [x] integrate `createReorderController` and `EditableList` for reorder mode

### Batch 4.5 - URL Sync

- [x] integrate URL query parameter sync for filters and sort
- [x] support `parseQueryParams` from `@decodelabs/underlay/client/query`

## Exit Criteria

- `EntityList.svelte` compiles without errors
- supports both cards and table presentations
- supports filters, pagination, batch actions, and reorder
- URL sync works for filters and sort
- can be rendered standalone (in a tab) or inside `EntityListPage`

## Next Task

Execute `g03.005`: build `EntityListPage` — the Level 1 page shell that wraps
`EntityList` with `PageHeader`, actions, and page-level state.
