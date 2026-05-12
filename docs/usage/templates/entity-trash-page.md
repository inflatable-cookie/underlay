# Entity Trash Page

Status: active

`EntityTrashPage` is the Level 1 page shell for repeated admin trash
workflows.

It is the right answer when the page is mainly:

- restore
- purge
- empty/loading/error shell
- a grid of deleted items or delete batches

It is not a normal browse/manage list page. That is why it exists separately
from `EntityListPage`.

## Usage

```svelte
<script lang="ts">
  import { EntityTrashPage, EntityListCard } from "@decodelabs/underlay/templates";
  import Trash2 from "lucide-svelte/icons/trash-2";

  let loading = false;
  let error: string | null = null;
  let items = [];

  function restore(id: string) {}
  function requestPurge(id: string) {}
</script>

{#snippet trashVisual()}
  <Trash2 />
{/snippet}

{#snippet trashCard(item)}
  <EntityListCard
    title={item.title}
    subtitle={item.slug}
    leadingIcon="trash-2"
    contextMenuItems={[
      { value: "restore", label: "Restore" },
      { value: "separator", label: "", kind: "separator" as const },
      { value: "purge", label: "Delete permanently", tone: "danger" as const }
    ]}
    contextMenuTrigger="leading"
    onContextAction={(value) => {
      if (value === "restore") restore(item.id);
      if (value === "purge") requestPurge(item.id);
    }}
  />
{/snippet}

<EntityTrashPage
  section="Content Trash"
  title="Content Trash"
  backHref="/content"
  backLabel="Back to content"
  {loading}
  loadingMessage="Loading trash..."
  {error}
  {items}
  renderItem={trashCard}
  emptyTitle="No soft-deleted items found"
  emptyMessage="Soft-deleted items will appear here."
  emptyVisual={trashVisual}
/>
```

## Filtered trash usage

When a trash page needs local controls like search or sort, keep them local to
the wrapper and pass them through `beforeItems` instead of falling back to
`EntityListPage`.

```svelte
<script lang="ts">
  import { EntityTrashPage } from "@decodelabs/underlay/templates";
  import { FilterToolbar, OrderBy, TextInput } from "@poodle/svelte";
</script>

{#snippet trashControls()}
  <FilterToolbar ariaLabel="Trash filters" summaryText="Filters" collapsible>
    <TextInput
      id="trash-search"
      type="search"
      placeholder="Search deleted items"
      ariaLabel="Search deleted items"
      value={searchValue}
      on:valueChange={(event) => updateSearch(event.detail.value)}
      on:clear={() => updateSearch("")}
    />
    <OrderBy
      fields={sortFields}
      value={orderByValue}
      onChange={updateSort}
      compact
    />
  </FilterToolbar>
{/snippet}

<EntityTrashPage
  title="Media Trash"
  backHref="/media"
  backLabel="Back to media"
  {loading}
  {error}
  beforeItems={trashControls}
  items={items}
  renderItem={mediaTrashCard}
  emptyTitle="Trash is empty"
  emptyMessage="Deleted media will appear here."
/>
```

## Wrapper policy

Normal reference-grade posture:

- create an app-local wrapper in `src/lib/lists/*`
- put the `EntityTrashPage` composition there
- keep restore/purge/conflict workflow inside that wrapper
- have the route thin-mount the wrapper

Typical wrapper jobs:

- authenticated page loading
- local search or sort controls
- restore and purge command wiring
- item card rendering
- confirmation dialogs or conflict-resolution dialogs

Do not leave repeated trash-page composition in the route when the same trash
shape exists elsewhere in the app or across reference apps.

## Props

### Page shell

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `title` | `string` | Yes | Page title |
| `section` | `string` | No | Optional section label above the title |
| `subtitle` | `string \| null` | No | Optional subtitle below the title |
| `eyebrow` | `string \| null` | No | Optional eyebrow above the header |
| `headerLevel` | `1 \| 2 \| 3 \| 4 \| 5 \| 6` | No | Heading level for nested composition; defaults to `1` |
| `backHref` | `string` | No | Back link URL |
| `backLabel` | `string` | No | Back link label |

### State

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `loading` | `boolean` | No | Show loading state |
| `loadingMessage` | `string` | No | Loading message |
| `error` | `string \| null` | No | Error message |
| `statusMessage` | `string \| null` | No | Optional non-error page message |
| `statusTone` | `"danger" \| "info" \| "success" \| "neutral" \| "warning"` | No | Status callout tone; defaults to `"danger"` |

### Content

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `beforeItems` | `TemplateSurface` | No | Local controls or explanatory content before the list |
| `items` | `T[]` | Yes | Deleted items or delete batches |
| `renderItem` | `TemplateSurface` | Yes | Item card renderer |
| `minItemWidth` | `string` | No | Grid item width; defaults to `"26rem"` |

### Empty state

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `emptyTitle` | `string` | Yes | Empty-state title |
| `emptyMessage` | `string` | Yes | Empty-state message |
| `emptyVisual` | `TemplateSurface` | No | Optional empty-state visual |

## What it provides

- `PageHeader`
- loading state
- error state
- optional status callout
- optional pre-list control surface
- empty state
- card grid

## What you bring

- authenticated data loading
- restore logic
- purge logic
- confirmation dialogs
- conflict-resolution flows
- deleted-item card rendering

## Use it when

- the page is a trash-specific restore/purge workflow
- the repeated part is the outer shell, not the workflow internals
- items are already loaded by route-owned or authenticated-page logic
- the page may need local controls before the grid, like search or sort

## Don’t use it when

- the page is really a normal `EntityListPage`
- the main behavior is generic browse/manage instead of restore/purge
- the page is a one-off workflow with no repeated trash-shell shape

## Current coverage

Current live admin coverage includes:

- `underlay-reference/acme-admin`
  - media trash
- `acowtancy/dairy`
  - content trash
  - learning trash
  - media trash
- `compli-me/admin`
  - compliments trash
- `contact-patch/cp-admin`
  - media trash
  - chapter trash

## See also

- [Template System Overview](./000-template-system-overview.md)
- [Entity List Page](./entity-list-page.md)
