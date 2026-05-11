# Entity List Section

Status: active

`EntityList` is the Level 2 section component. It is a self-contained list
surface with filters, pagination, batch actions, and empty states.

## When To Use

- Inside a detail tab that shows a child collection
- Inside a dialog that shows a picker list
- Standalone when you don't need the full page shell

For detail-tab child collections, this is the preferred target shape:

- route returns the same paged envelope as any other page-shaped child list
- client command returns `PagedListResponse<T>`
- loader maps it with `toPagedListResult(...)`

Custom cursor-style tab list components are compatibility posture, not the
shared target pattern.

## Usage

```svelte
<script lang="ts">
  import { EntityList, toPagedListResult } from "@decodelabs/underlay/templates";
  import TaskCard from "$lib/cards/TaskCard.svelte";

  async function loadTasks(fetchFn: typeof fetch, token: string | null, query) {
    const response = await adminCommands.listTasks(fetchFn, token, query);
    return toPagedListResult(response);
  }
</script>

{#snippet taskCard(task, context)}
  <TaskCard
    task={task}
    selectionMode={context.selectionMode}
    reorderMode={context.reorderMode}
    selected={context.selected}
    onSelectionChange={context.onToggle}
  />
{/snippet}

<EntityList
  dataLoader={loadTasks}
  presentation="cards"
  renderItem={taskCard}
/>
```

## Props

Same as `EntityListPage` minus the page shell props (`title`, `backHref`, etc.).

Use the shared exported list-template types from
`@decodelabs/underlay/templates` when you need explicit config typing.

## See Also

- [Entity List Page](./entity-list-page.md)
