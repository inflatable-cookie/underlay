# Entity Detail Page

Status: active

`EntityDetailPage` is the Level 1 page shell for read-only detail pages. It
combines `PageHeader`, `MetaBar`, `Tabs`, and `EntityDetail` sections into a
complete detail view.

It supports two normal loading modes:

- caller-provided `dataLoader` for template-owned fetch/loading/error posture
- caller-provided `item` for route-preloaded data, where the template still
  owns the detail shell and tab system without refetch flicker

It supports two normal body modes:

- `tabs={[...]}`
- `content={...}` for single-surface detail pages that do not need top-level
  tabs

## Usage

```svelte
<script lang="ts">
  import {
    EntityDetail,
    EntityDetailPage,
    EntityListPage
  } from "@inflatable-cookie/underlay/templates";

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
  title={project.code}
  section="Projects"
  subtitle={project.name}
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

## Preloaded detail routes

When the route already owns the main fetch and context stitching, pass
`item={...}` instead of forcing the template to refetch:

```svelte
<EntityDetailPage
  item={pageData.data}
  title={outcome.code}
  section="Outcomes"
  subtitle={outcome.label}
  breadcrumbs={breadcrumbs}
  breadcrumbsMarkLastCurrent={false}
  meta={detailMeta}
  tabs={detailTabs}
  headerActions={detailHeaderActions}
/>
```

Use this mode when the route already needs:

- stitched parent context
- custom not-found/error gating
- one authenticated fetch that should not flicker or duplicate

## Single-surface detail pages

When the page is still a real entity detail but does not need top-level tabs,
use `content={...}` instead of inventing a fake one-tab layout:

```svelte
{#snippet jobContent(job)}
  <EntityDetail title="Details">
    <!-- detail modules -->
  </EntityDetail>
{/snippet}

<EntityDetailPage
  title={jobTitle}
  section="Job"
  backHref="/system/jobs"
  dataLoader={loadJob}
  meta={headerMeta}
  headerActions={headerActions}
  content={jobContent}
/>
```

Use this mode when:

- the route is still a repeated entity detail shell
- the body is one continuous detail surface
- adding top-level tabs would be artificial

## Route ownership policy

Unlike list pages, detail routes do not need an extra app-local wrapper by
default.

Normal posture:

- route owns the entity-specific loader and workflow glue
- route mounts `EntityDetailPage` directly
- shared child collections inside tabs reuse app-local list wrappers over
  `EntityListPage` when they are real browse/manage surfaces

Create an extra app-local detail wrapper only when the same detail shell is
truly reused across more than one caller.

## Header posture

- `section` names the resource family, usually the same plural family used by
  the browse page, for example `Modules`, `Documents`, `Blog articles`, or
  `Quiz questions`
- `title` is the main record label shown in the large heading
- prefer a short record label for `title` when the entity has one, for example
  `ACCA`, `Area A1`, or `Q4`
- when the entity only has a long human title, either:
  - use that as `title`, or
  - use a static details title such as `Blog details` and place the longer
    record label in `subtitle`
- use `subtitle` for the longer descriptive title when the main heading should
  stay short
- use `breadcrumbs` for the parent path in nested routes
- when the main `title` already names the current entity, omit that entity from
  the breadcrumb trail and pass `breadcrumbsMarkLastCurrent={false}`
- page actions render as a single ellipsis menu in the header action slot
- do not restate the page identity inside the active details tab with another
  nested `PageHeader`; put slug, keys, and similar secondary identifiers back
  into the detail modules/items

## Nested browse tabs

Use `EntityListPage` inside tabs for real child-collection browse/manage
surfaces, and lower the header level so the tab content stays subordinate to
the detail page:

```svelte
<EntityListPage
  title="Tasks"
  headerLevel={3}
  presentation="table"
  dataLoader={loadProjectTasks}
/>
```

Only fall back to raw `EntityList` when the tab content is genuinely narrower
than a real browse surface, for example a picker-like or inline utility list:

```svelte
<EntityList
  presentation="table"
  dataLoader={loadProjectTasks}
/>
```

When the child collection lives inside the detail grid rather than as a real
top-level browse tab, prefer `EntityInlineListModule` instead of embedding a
full `EntityListPage` shell:

```svelte
<EntityInlineListModule
  title="Levels"
  dataLoader={loadPathwayLevels}
  addDialog={{ title: "Add level", content: addLevelDialog }}
  itemActions={levelActions}
  item={levelRow}
/>
```

Use the same bridge as root list pages:

- client command returns `PagedListResponse<T>`
- tab loader maps it with `toPagedListResult(...)`

Normal tab differences should stay small:

- parent filter clause
- lower header level
- optional reorder or batch-action mode differences
- contextual actions/add behavior

Those are expected `EntityListPage` modes, not a reason to keep a separate tab
list pattern.

## Tab modes

`EntityDetailPage` supports the normal repeated tab variations directly:

- `tabsVariant="underline" | "card"`
- `tabsSize="sm" | "md" | "lg"`
- `keepMountedTabs`

Retained default posture:

- use underline tabs by default
- only pass `tabsVariant="card"` when the route has a proved workflow reason to
  diverge from the normal detail-shell posture

Use `keepMountedTabs` when tab contents should stay mounted after first visit,
for example when:

- nested tab state should persist
- editor or list state should not reset on every tab switch
- the route already paid for the data and should keep the local UI warm

Do not use tabs just to satisfy the template. If the page does not have a real
top-level section split, use `content`.

Reference recipe:

- [entity-detail-tab-paged-list.ts](../../guides/code/073-api-profiles-and-query-contract/entity-detail-tab-paged-list.ts)

## See Also

- [Entity Detail Section](./entity-detail-section.md)
- [Template System Overview](./000-template-system-overview.md)

## Public types

The shared detail-template config types are exported from
`@inflatable-cookie/underlay/templates`:

- `DetailMetaItemConfig`
- `DetailTabConfig`
- `DetailActionConfig`
