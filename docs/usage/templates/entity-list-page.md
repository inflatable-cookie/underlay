# Entity List Page

Status: active

`EntityListPage` is the Level 1 shell for real browse/manage list surfaces. It
wraps `EntityList` with header, actions, filters, and list-state management.

Use it for:

- root browse pages
- detail-tab browse pages
- other real list surfaces where the user is browsing, filtering, selecting,
  reordering, or batch-operating on a collection

The location does not matter. If the surface is a real list, `EntityListPage`
is the default answer.

## Usage

```svelte
<script lang="ts">
  import { EntityListPage } from "@inflatable-cookie/underlay/templates";
  import { toPagedListResult } from "@inflatable-cookie/underlay/templates";
  import type { PagedListResponse } from "@inflatable-cookie/underlay/client/types";
  import ProjectCard from "$lib/cards/ProjectCard.svelte";
  import { adminCommands } from "$lib/client";

  async function loadProjects(fetchFn: typeof fetch, token: string | null, query) {
    const response: PagedListResponse<ProjectListItem> =
      await adminCommands.listProjects(fetchFn, token, query);
    return toPagedListResult(response);
  }
</script>

{#snippet projectCard(item, context)}
  <ProjectCard
    project={item}
    selectionMode={context.selectionMode}
    reorderMode={context.reorderMode}
    selected={context.selected}
    onSelectionChange={context.onToggle}
  />
{/snippet}

<EntityListPage
  section="Projects"
  title="All projects"
  backHref="/"
  backLabel="Back to dashboard"
  dataLoader={loadProjects}
  presentation="cards"
  filters={[...]}
  renderItem={projectCard}
  batchActions={[...]}
  reorder={{ enabled: true, handler: ... }}
  onAdd={() => goto("/projects/new")}
/>
```

## Tab usage

`EntityListPage` should also be the default answer for list tabs.

Typical tab posture:

```svelte
<EntityListPage
  title="Tasks"
  headerLevel={3}
  dataLoader={loadProjectTasks}
  filters={[...]}
  renderItem={taskCard}
/>
```

Normal tab differences are small:

- lower `headerLevel`
- parent-scoped filter/query clause in the loader
- optional reorder or batch-action mode changes
- contextual add/back/action behavior

Those are list modes, not a reason to switch to a different list template.

## Header posture

For top-level browse pages, prefer the standard Poodle `PageHeader` posture:

- `section` = the resource family, for example `Modules`
- `title` = the view title, for example `All modules`
- `subtitle` = optional extra context, not the normal place for the main label

For nested browse pages, use the same header shape and add `breadcrumbs` when
the route has a real parent path. Example:

- section `Areas`
- title `All areas`
- breadcrumbs `Pathway / Module / Section`

For tab-local list surfaces, keep the header lighter:

- lower `headerLevel`
- usually no `section`
- short local `title` such as `Tasks` or `Areas`
- let the reusable list wrapper own that local title by default; do not repeat
  the same tab title again in every route unless the route is intentionally
  changing the meaning of the surface

## App wrapper policy

When a consumer app implements a real browse/manage collection, the normal
reference-grade posture is:

- create an app-local wrapper in `src/lib/lists/*`
- put the `EntityListPage` composition there
- have the route thin-mount that wrapper
- reuse the same wrapper in detail tabs when the collection semantics are the
  same

Typical wrapper jobs:

- parent scope props
- query mode choice
- app-local add/delete/navigation wiring
- card/table cell rendering
- small workflow glue that still belongs to the collection surface

Route-local `EntityListPage` composition should be treated as temporary or as
an explicit exception, not as the normal long-term answer for reference apps.

For card-mode lists, the normal reference posture is:

- app-local `src/lib/cards/*` card over `EntityListCard`
- app-local `src/lib/lists/*` wrapper over `EntityListPage`
- thin route mount over that wrapper

Do not leave repeated raw `ListCard` composition in the route or list wrapper
when the surface is a normal admin collection. Promote that repeated card shell
into `src/lib/cards/*` over `EntityListCard`.

## Props

### Page Shell

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `title` | `string` | Yes | Page title |
| `section` | `string` | No | Optional section label above the title |
| `subtitle` | `string` | No | Optional subtitle below the title |
| `eyebrow` | `string` | No | Optional eyebrow above the header |
| `breadcrumbs` | `BreadcrumbItem[]` | No | Optional breadcrumb trail above the header title |
| `breadcrumbsMarkLastCurrent` | `boolean` | No | Mark the final breadcrumb as the current page; defaults to `true` |
| `headerLevel` | `1 \| 2 \| 3 \| 4 \| 5 \| 6` | No | Heading level for nested composition; defaults to `2` |
| `backHref` | `string` | No | Back link URL |
| `backLabel` | `string` | No | Back link label |

### Data

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `dataLoader` | `(fetch, token, query) => Promise` | Yes | Data loading function |

### Presentation

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `presentation` | `"cards" \| "table" \| "log"` | Yes | List presentation mode |
| `renderItem` | `Snippet<[item, context]>` | For cards | Card renderer |
| `columns` | `TableColumn[]` | For table | Table column config |

### Filters

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `filters` | `FilterConfig[]` | No | Declarative filter config |
| `queryVariants` | `ListVariantDefinition[]` | No | Named baseline query variants rendered above filters |
| `defaultVariantId` | `string` | No | Fallback variant used when query state does not name one explicitly |
| `capabilitiesLoader` | `(fetch, token) => Promise<ListCapabilities>` | No | Loads API-published variants and filters |

Query variants are not filters. They represent server-understood baseline
queries such as `pending`, `marked`, or `all`. Filter controls then refine the
active variant.

Changing variants resets `page` to `1`.

When `capabilitiesLoader` is provided, loaded `variants`, `filters`, and
`defaultVariantId` override the static props.

Example:

- `underlay-reference/apps/acme-admin/src/lib/lists/TasksListPage.svelte` declares
  `open`, `completed`, and `all` variants for project tasks.
- `open` is the default baseline and maps to pending plus in-progress tasks on
  the API side.
- the normal status, priority, and sort controls stay in `FilterToolbar` and
  refine the active variant.
- reorder is only available on the `all` baseline because the other variants
  are filtered views.

### Actions

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `batchActions` | `BatchActionConfig[]` | No | Batch action config |
| `reorder` | `{ enabled, handler }` | No | Reorder configuration |
| `onAdd` | `() => void` | No | Add button handler |
| `addLabel` | `string` | No | Add button label |

### Batch Actions

Batch actions support three modes:

**1. Immediate execution** — no confirmation, runs immediately when selected:
```svelte
batchActions={[
  { id: "archive", label: "Archive", handler: async (ids) => archiveProjects(ids) }
]}
```

**2. Confirmation dialog** — shows yes/no confirmation before executing:
```svelte
batchActions={[
  { 
    id: "delete", 
    label: "Delete", 
    tone: "danger",
    confirm: true,  // auto-generated message
    handler: async (ids) => deleteProjects(ids)
  }
]}
```

**3. Custom form dialog** — opens a dialog with custom form content:
```svelte
<script>
  let statusValue = $state("");
</script>

{#snippet statusDialog({ ids, onSubmit, onCancel })}
  <Field label="New Status">
    <Select
      value={statusValue}
      items={[
        { value: "active", label: "Active" },
        { value: "paused", label: "Paused" },
        { value: "completed", label: "Completed" }
      ]}
      onchange={(e) => statusValue = e.currentTarget.value}
    />
  </Field>
  <div class="dialog-actions">
    <Button variant="secondary" onclick={onCancel}>Cancel</Button>
    <Button variant="primary" onclick={() => onSubmit({ status: statusValue })}>
      Update {ids.length} projects
    </Button>
  </div>
{/snippet}

<EntityListPage
  ...
  batchActions={[
    {
      id: "status",
      label: "Update Status",
      dialog: {
        title: "Update Project Status",
        content: statusDialog
      },
      handler: async (ids, values) => {
        await updateProjectStatus(ids, values.status);
      }
    }
  ]}
/>
```

The `handler` receives `(ids, values)` where `values` is the object passed to `onSubmit` from the dialog snippet.

## Public types

The shared list-template config types are exported from
`@inflatable-cookie/underlay/templates`:

- `FilterConfig`
- `BatchActionConfig`
- `BatchDialogConfig`
- `BatchDialogContext`
- `ReorderConfig`
- `PagedListResult`
- `EntityListDataLoader`

## Data loader contract

`EntityListPage` expects a loader that returns:

```ts
{
  data: T[];
  total?: number | null;
  hasMore?: boolean;
}
```

Recommended pattern:

- backend route uses the canonical paged wire envelope from `115`
- TS client normalizes `has_more` to `hasMore`
- page loader returns that shaped result directly to the template
- use `page` + `limit` query params for these routes
- do not wire `EntityListPage` through cursor helpers from
  `@inflatable-cookie/underlay/runtime/data` or `@inflatable-cookie/underlay/client/pagination`

Reference recipe:

- [entity-list-page-paged-loader.ts](../../guides/code/073-api-profiles-and-query-contract/entity-list-page-paged-loader.ts)

Use bounded `ListResponse<T>` only for helper collections that are not real
page shells. If a route feeds `EntityListPage`, it should be a page-shaped
paginated resource surface, not a helper list disguised as one.

## Relationship to `EntityList`

`EntityList` is the lower-level engine under `EntityListPage`.

Prefer `EntityListPage` unless the surface is genuinely narrower, for example:

- inline utility lists inside a detail body
- dialog/picker lists
- embedded lists where page-shell chrome would be artificial

Reference implementation examples in `underlay-reference`:

- shared list wrappers:
  `underlay-reference/apps/acme-admin/src/lib/lists/ProjectsListPage.svelte`
  and
  `underlay-reference/apps/acme-admin/src/lib/lists/TasksListPage.svelte`
- query variants:
  `underlay-reference/apps/acme-admin/src/lib/lists/TasksListPage.svelte`

If a reference app still carries route-local list pages, treat those as
remaining convergence work rather than as co-equal examples.

## See Also

- [Entity List Section](./entity-list-section.md) — The underlying Level 2 component
- [Template System Overview](./000-template-system-overview.md)
