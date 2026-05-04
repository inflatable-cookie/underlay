# 005 - EntityListPage Shell Component

Status: complete
Owner: repo maintainers
Updated: 2026-05-04

## Goals

- build `EntityListPage.svelte` as a thin page shell around `EntityList`
- add `PageHeader` with title, count, back link, and action buttons
- support selection-mode toggle, reorder-mode toggle, and add button
- pass all `EntityList` props through transparently

## API Design

```svelte
<EntityListPage
  title="Projects"
  backHref="/"
  backLabel="Back to dashboard"
  
  dataLoader={...}
  presentation="cards"
  filters={...}
  renderItem={...}
  batchActions={...}
  reorder={...}
  
  onAdd={() => gotoWithContext("/projects/new", context)}
  addLabel="Add project"
/>
```

This should render as:
- `PageHeader` with title, count, back link, and action buttons
- `EntityList` with all props passed through

## Execution Plan

### Batch 5.1 - Shell Structure

- [x] create `EntityListPage.svelte`
- [x] integrate `PageHeader` with title, count, back link
- [x] action buttons: selection toggle, reorder toggle, add button

### Batch 5.2 - Prop Pass-Through

- [x] pass all `EntityList` props through transparently
- [x] ensure type definitions flow correctly

### Batch 5.3 - State Coordination

- [x] coordinate selection mode and reorder mode (mutually exclusive)
- [x] handle Escape key to exit modes

## Exit Criteria

- `EntityListPage.svelte` compiles without errors
- renders a complete list page with header, filters, list, pagination, batch bar
- acme-admin `/projects/+page.svelte` can be reduced from 506 lines to ~50 lines

## Next Task

Execute `g03.006`: migrate acme-admin `/projects/+page.svelte` to use
`EntityListPage` as the first proof.
