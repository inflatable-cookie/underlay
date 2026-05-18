# Entity List Section

Status: active

`EntityList` is the Level 2 list engine under `EntityListPage`.

## When To Use

- Inside a dialog that shows a picker list
- Standalone when you don't need the full page shell
- Rare inline list sections where page-shell chrome would be artificial

For real detail-tab child collections, the preferred target shape is now
`EntityListPage`, not raw `EntityList`.

Use raw `EntityList` when the surface is truly narrower:

- picker/dialog content
- inline utility embeds
- subordinate lists that are not acting like their own browse surface

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

If a tab needs filters, pagination summary, batch actions, reorder, header
actions, or add flows, prefer `EntityListPage` instead.

`EntityList` also accepts `queryVariants`, `defaultVariantId`, and
`capabilitiesLoader`. Variants are named baseline queries rendered above
`FilterToolbar`; filters and sort controls then refine the active variant.

Use `capabilitiesLoader` when the API publishes `profile=list-config`
capabilities for the list surface.

## Props

Same as `EntityListPage` minus the page-shell props (`title`, `backHref`, etc.).

Use the shared exported list-template types from
`@decodelabs/underlay/templates` when you need explicit config typing.

## See Also

- [Entity List Page](./entity-list-page.md)
