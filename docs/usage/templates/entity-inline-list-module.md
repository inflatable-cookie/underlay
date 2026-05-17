# Entity Inline List Module

Status: active

`EntityInlineListModule` is the retained Level 2 surface for managed child
collections that live inside an `EntityDetail` grid.

Use it when the collection is subordinate to the detail page and a full
`EntityListPage` shell would be artificial.

Typical fit:

- pathway levels inside a pathway detail page
- related records with small single-line cards
- child collections that need add, per-item actions, or light pagination
- detail modules where the list should stay visually compact

Do not use it for real browse/manage tabs that behave like their own collection
screen. Those should still reuse an app-local wrapper over `EntityListPage`.

## Usage

```svelte
<script lang="ts">
  import {
    EntityInlineListModule,
    type InlineListDialogContext,
    type InlineListItemActionConfig,
    toPagedListResult
  } from "@decodelabs/underlay/templates";
  import { adminCommands } from "$lib/client";

  async function loadLevels(fetchFn: typeof fetch, token: string | null, query) {
    const response = await adminCommands.listPathwayLevels(pathwayId, fetchFn, token, query);
    return toPagedListResult(response);
  }

  function levelActions(level): InlineListItemActionConfig<typeof level>[] {
    return [
      {
        label: "Edit",
        handler: () => openEditLevel(level)
      }
    ];
  }
</script>

{#snippet levelRow(level)}
  <div class="level-row">
    <span>{level.title}</span>
    <span>{level.code}</span>
  </div>
{/snippet}

{#snippet createLevelDialog(context: InlineListDialogContext)}
  <LevelForm
    pathwayId={pathwayId}
    onSaved={async () => {
      await context.refetch();
    }}
    onCancel={context.close}
  />
{/snippet}

<EntityInlineListModule
  title="Levels"
  span="half"
  dataLoader={loadLevels}
  addLabel="Add level"
  addDialog={{
    title: "Add level",
    content: createLevelDialog
  }}
  item={levelRow}
  itemActions={levelActions}
  emptyMessage="No levels yet."
/>
```

## Props

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `title` | `string` | Yes | Module title |
| `item` | `Snippet<[item, context]>` | Yes | Compact item renderer |
| `items` | `T[]` | No | Static items for route-owned content |
| `dataLoader` | `(fetch, token, query) => Promise` | No | Loader for paged inline collections |
| `query` | `QueryParams` | No | External query state |
| `onQueryChange` | `(query) => void` | No | External query-state owner |
| `pageSize` | `number` | No | Default inline page size; defaults to `5` |
| `actions` | `Snippet` | No | Extra header actions |
| `onAdd` | `() => void` | No | Non-dialog add action |
| `addLabel` | `string` | No | Add button label |
| `addDialog` | `InlineListDialogConfig` | No | Built-in modal add flow |
| `itemActions` | `(item) => InlineListItemActionConfig[]` | No | Per-item ellipsis actions |
| `itemDelete` | `InlineListItemDeleteConfig` | No | Optional delete action |
| `emptyMessage` | `string \| null` | No | Empty-state copy |
| `span` | `"half" \| "full"` | No | Detail-grid span; defaults to `"half"` |
| `onDataChange` | `() => void` | No | Callback after inline actions mutate data |
| `hidePaginationSummary` | `boolean` | No | Hides the compact count summary under the title |

## Notes

- The module stays intentionally compact. It does not grow a full
  `FilterToolbar` shell by default.
- `addDialog` receives `{ close, refetch }`. Call `refetch()` after save to
  refresh the list and close the modal.
- `item` receives a second `context` argument with `refetch()` when the row
  needs to trigger its own refresh.

## See Also

- [Entity Detail Section](./entity-detail-section.md)
- [Entity Detail Page](./entity-detail-page.md)
- [Entity List Page](./entity-list-page.md)
