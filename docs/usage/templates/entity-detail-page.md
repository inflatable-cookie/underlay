# Entity Detail Page

Status: active

`EntityDetailPage` is the Level 1 page shell for read-only detail pages. It
combines `PageHeader`, `MetaBar`, `Tabs`, and `EntityDetail` sections into a
complete detail view.

## Usage

```svelte
<script lang="ts">
  import {
    EntityDetail,
    EntityDetailPage,
    EntityListPage
  } from "@decodelabs/underlay/templates";

  async function loadProject(fetchFn: typeof fetch, token: string | null) {
    return await adminCommands.getProject(id, fetchFn, token);
  }

  async function loadProjectTasks(fetchFn: typeof fetch, token: string | null, query) {
    return await adminCommands.listProjectTasks(id, fetchFn, token, query);
  }
</script>

{#snippet detailsTab(project)}
  <EntityDetail title="Details">
    <!-- detail modules -->
  </EntityDetail>
{/snippet}

{#snippet tasksTab(project)}
  <EntityListPage
    title="Tasks"
    presentation="table"
    dataLoader={loadProjectTasks}
    headerLevel={3}
  />
{/snippet}

<EntityDetailPage
  title={project.name}
  section="Project"
  backHref="/projects"
  dataLoader={loadProject}
  meta={[...]}
  tabs={[
    {
      id: "details",
      label: "Details",
      content: detailsTab
    },
    {
      id: "tasks",
      label: "Tasks",
      count: taskCount,
      content: tasksTab
    }
  ]}
  actions={[
    { label: "Edit", handler: handleEdit },
    { label: "Delete", tone: "danger", confirm: true, handler: handleDelete }
  ]}
/>
```

## Header posture

- when `section` is set, the page header now treats that as the primary title
- `title` becomes the subtitle underneath, which keeps long entity names out of
  the large heading style and removes the old duplicated title stack
- page actions render as a single ellipsis menu in the header action slot

## Nested browse tabs

Use `EntityListPage` inside tabs when you want the full list-page shell, but
lower the header level so the tab content stays subordinate to the detail page:

```svelte
<EntityListPage
  title="Tasks"
  headerLevel={3}
  presentation="table"
  dataLoader={loadProjectTasks}
/>
```

When the tab does not need a nested page shell, prefer `EntityList` instead of
carrying a custom tab-specific list controller:

```svelte
<EntityList
  presentation="table"
  dataLoader={loadProjectTasks}
/>
```

Use the same bridge as root list pages:

- client command returns `PagedListResponse<T>`
- tab loader maps it with `toPagedListResult(...)`

Reference recipe:

- [entity-detail-tab-paged-list.ts](../../guides/code/073-api-profiles-and-query-contract/entity-detail-tab-paged-list.ts)

## See Also

- [Entity Detail Section](./entity-detail-section.md)
- [Template System Overview](./000-template-system-overview.md)

## Public types

The shared detail-template config types are exported from
`@decodelabs/underlay/templates`:

- `DetailMetaItemConfig`
- `DetailTabConfig`
- `DetailActionConfig`
