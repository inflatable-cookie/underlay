# Template System Overview

The Underlay Template System provides reusable higher-order Svelte components
for common admin page shapes. It replaces 300–800 line hand-rolled compositions
with ~50–100 line declarative configurations.

Status: active shared surface.

## Philosophy

**Higher-order composition over hand-rolled assembly.**

Instead of assembling `PageHeader`, `FilterToolbar`, `DataTable`, and
`BulkActionBar` manually on every page, declare what you need and let the
template wire it together.

## Three-Level Hierarchy

### Level 1 — Page Shells

Full page components that include header, actions, and content:

- `EntityListPage` — Browse page with filters, list, pagination, batch actions
- `EntityDetailPage` — Detail page with metadata, tabs, child collections
- `EntityFormPage` — Create/edit page with form shell, actions, and optional sidecar content
- `EntityTrashPage` — Trash workflow shell with loading/error/empty and card grid
- `MediaUploadPage` — Media upload workflow shell with header, loading, and upload-level error framing
- `MediaDetailWorkflowPage` — Media detail workflow shell with header, metadata, tabs, and load/error framing
- `SystemIndexPage` — System/operator index shell with header and nav-card grid
- `AdminDashboardPage` — Admin dashboard shell with header and stacked dashboard sections
- `UsersListPage` — Admin user-management browse shell with search/filter/sort, row actions, and extension hooks (`extraRowActions`, `onCustomRowAction`, `searchFilterId`, `showSortFilter`, `reloadKey`) so app-specific columns and actions stay app-owned
- `ErrorLogListPage` — Retained error-log browse shell with status filter, compact table, stats cards, and expandable detail rows
- `ContextActionBar` and `ContextActionDialog` — Route-aware contextual action shell for app-owned AI actions

Reference posture:

- real browse/manage lists should normally be wrapped in reusable app-local
  components over `EntityListPage`
- routes should thin-mount those wrappers instead of declaring the list page in
  place
- detail and form routes may still use `EntityDetailPage` and `EntityFormPage`
  directly unless a repeated app-local wrapper is genuinely shared

Header policy:

- use Poodle `PageHeader` posture directly
- list pages: `section` = resource family, `title` = browse label such as
  `All modules`
- detail pages: `section` = resource family, normally the same plural family
  used by the browse page
- detail pages: `title` = short record label when available
- detail pages: use a static details title plus `subtitle` when the record only
  has a long human-facing name
- nested views: use breadcrumbs for the parent path rather than duplicating the
  current item in the trail
- converged detail tabs should not add another nested `PageHeader` that only
  repeats the page identity

### Level 2 — Sections

Reusable components for use inside pages, tabs, or dialogs:

- `EntityList` — Self-contained list with filters, pagination, batch, reorder
- `EntityDetail` — Metadata and detail sections
- `EntityInlineListModule` — Compact managed child-collection module for detail grids
- `DetailDataTab` — Data-driven detail tab (loader + typed snippet content)
- `EntityActionsMenu` — Shared record/row actions-menu recipe (edit, restore/purge, copy actions); `MediaActionsMenu` is the media re-skin over it

Admin chrome:

- `AdminNavList` — Shared admin nav list (active state, expansion, badges)
- `AdminUserMenu` — Shared admin user menu (identity, role accent, sign-out)

Sections are public exports. Use them directly when you need a narrower inline
surface that is not really a full browse/manage list tab.

There is no separate shared `EntityTabList` wrapper. Real child-collection
tabs should normally reuse the same app-local wrapper over `EntityListPage`
that the root collection uses.

`EntityListCard` is the shared card shell under those list wrappers. It should
normally own the repeated list-card posture instead of each app hand-rolling
raw `ListCard` compositions.

`EntityTrashPage` is the separate shell for repeated restore/purge pages. Use
it when the route still owns trash workflow logic but the outer page shell is
repeated across apps.

For the retained media family, the preferred lower-level shared sections are:

- `MediaEditDialog`
- `MediaFileDetailsCard`
- `MediaPreviewTab`
- `MediaRenditionsSection`
- `MediaVersionActionDialogs`
- `MediaVersionPreviewDialog`
- `MediaVersionsList`
- `MediaUsageList`

For the repeated route-side media-detail logic under those templates, prefer
the retained `@decodelabs/underlay/runtime/media` helper surface rather than
app-local state/predicate modules.

For reference-grade admin apps, repeated raw `ListCard` collection cards should
be treated as drift unless the surface is an explicit exception.

**Forms are not declaratively templated.** Real forms have arbitrary layout,
custom fields, conditional logic, complex validation, file uploads, etc. Use
Poodle primitives (`Field`, `TextInput`, `Select`, etc.) directly. Use
`EntityFormPage` as a page shell wrapper that handles the header, loading, and
error states, and one optional sidecar content region when the page still has
one clear primary form.

The repeated form *chrome* is shared, though — use it instead of re-skinning
Poodle primitives per app:

- `EntityFormActions` — submit/cancel action bar with busy/disabled states
- `EntityRelationField` — relation picker field (search + select entity)
- `EntityMediaField` — media picker form-field chrome
- `useFormFieldErrors` / `setFormFieldErrors` — field-error context so
  server field errors surface on the right inputs without prop drilling;
  `UserForm` is the shared user create/edit form built on this layer

### Level 3 — Primitives

Poodle owns the primitive layer:

- `PageHeader`, `MetaBar`, `Tabs`
- `ListContainer`, `FilterToolbar`, `DataTable`
- `DetailSection`, `DetailItem`
- `FormDialog`, `AlertDialog`

## Installation

```svelte
<script lang="ts">
  import {
    EntityDetail,
    EntityDetailPage,
    ErrorLogListPage,
    EntityInlineListModule,
    EntityList,
    EntityListPage,
    EntityTrashPage
  } from "@decodelabs/underlay/templates";
</script>
```

See `contextual-action-templates.md` for the contextual action bar and
execution dialog shell.

If you are bootstrapping a new app's admin `/system` section, use
[System Section Bootstrap](./system-section-bootstrap.md).

If you are asking an agent to build a normal admin resource family, start with
[Admin Section Agent Protocol](./admin-section-agent-protocol.md). It is the
short, prescriptive route for list/detail/form/action-menu work across
Underlay-based apps.

## Quick Example

### List Page

```svelte
<script lang="ts">
  import { EntityListPage } from "@decodelabs/underlay/templates";
  import ProjectCard from "$lib/cards/ProjectCard.svelte";

  let { data } = $props();

  async function loadProjects(fetchFn: typeof fetch, token: string | null, query) {
    return await adminCommands.listProjects(fetchFn, token, query);
  }
</script>

{#snippet projectCard(project, context)}
  <ProjectCard
    project={project}
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
  dataLoader={loadProjects}
  presentation="cards"
  filters={[
    { id: "name", type: "search", label: "Name" },
    { id: "status", type: "select", label: "Status", options: statusOptions }
  ]}
  renderItem={projectCard}
  batchActions={[
    {
      id: "delete",
      label: "Delete",
      tone: "danger",
      confirm: true,
      handler: async (ids) => batchDeleteProjects(ids)
    }
  ]}
  
  onAdd={() => goto("/projects/new")}
/>
```

### Detail Page

```svelte
<script lang="ts">
  import {
    EntityDetail,
    EntityDetailPage,
    EntityListPage
  } from "@decodelabs/underlay/templates";
  import { Code, Pill } from "@inflatable-cookie/poodle-svelte";

  let { data } = $props();

  async function loadProject(fetchFn: typeof fetch, token: string | null) {
    return await adminCommands.getProject(data.projectId, fetchFn, token);
  }

  async function loadTasks(fetchFn: typeof fetch, token: string | null, query) {
    return await adminCommands.listProjectTasks(data.projectId, fetchFn, token, query);
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
    headerLevel={3}
    presentation="cards"
    dataLoader={loadTasks}
    renderItem={taskCard}
  />
{/snippet}

<EntityDetailPage
  title={data.project.code}
  section="Projects"
  subtitle={data.project.name}
  backHref="/projects"
  dataLoader={loadProject}
  meta={[
    { label: "ID", value: codeMeta },
    { label: "Status", value: statusMeta }
  ]}
  tabs={[
    {
      id: "details",
      label: "Details",
      content: detailsTab
    },
    {
      id: "tasks",
      label: "Tasks",
      count: data.taskCount,
      content: tasksTab
    }
  ]}
  actions={[
    { label: "Edit", handler: handleEdit },
    { label: "Delete", tone: "danger", confirm: true, handler: handleDelete }
  ]}
/>
```

## When To Use Templates

**Use templates when:**
- Building standard admin CRUD pages
- The page shape matches a common pattern (list, detail, form)
- The page is a repeated trash/restore workflow
- You want consistency across admin pages
- You can express the page with declarative config plus snippets
- You are willing to extract a reusable app-local list wrapper when the surface
  is a real admin collection
- You are willing to extract a reusable app-local card over `EntityListCard`
  when the list presentation is cards

**Don't use templates when:**
- The page has a unique shape that doesn't fit standard patterns
- The route is a dashboard, system index, upload flow, planner, or similar
  utility/workflow surface rather than a real entity list/detail/form/trash page
- You need fine-grained control over every element
- Building public-facing pages (use Poodle primitives directly)
- The template needs more escape hatch than leverage

## Next Steps

- [Entity List Page](./entity-list-page.md) — Browse and filter lists
- [Entity Trash Page](./entity-trash-page.md) — Restore and purge trash flows
- [Media Upload Page](./media-upload-page.md) — Media upload workflow shell
- [Media Detail Workflow Page](./media-detail-workflow-page.md) — Media detail workflow shell
- [System Index Page](./system-index-page.md) — System/operator index shell
- [Admin Dashboard Page](./admin-dashboard-page.md) — Admin dashboard shell
- [Entity List Card](./entity-list-card.md) — Shared card shell for list items
- [Entity Detail Page](./entity-detail-page.md) — Read-only detail with tabs
- [Entity Form Page](./entity-form-page.md) — Create and edit forms
