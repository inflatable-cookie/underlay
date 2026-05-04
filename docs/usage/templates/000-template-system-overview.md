# Template System Overview

The Underlay Template System provides reusable higher-order Svelte components
for common admin page shapes. It replaces 300–800 line hand-rolled compositions
with ~50–100 line declarative configurations.

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
- `EntityFormPage` — Create/edit page with form shell and actions

### Level 2 — Sections

Reusable components for use inside pages, tabs, or dialogs:

- `EntityList` — Self-contained list with filters, pagination, batch, reorder
- `EntityDetail` — Metadata and detail sections

Sections are public exports. Use them directly when you need a list inside a
detail tab.

**Forms are not templated.** Real forms have arbitrary layout, custom fields,
conditional logic, complex validation, file uploads, etc. Use Poodle primitives
(`Field`, `TextInput`, `Select`, etc.) directly. Use `EntityFormPage` as a page
shell wrapper that handles the header, loading, and error states.

### Level 3 — Primitives

Poodle owns the primitive layer:

- `PageHeader`, `MetaBar`, `Tabs`
- `ListContainer`, `FilterToolbar`, `DataTable`
- `DetailSection`, `DetailItem`
- `FormDialog`, `AlertDialog`

## Installation

```svelte
<script>
  import { EntityListPage, EntityDetailPage } from "@decodelabs/underlay/templates";
</script>
```

## Quick Example

### List Page

```svelte
<EntityListPage
  title="Projects"
  backHref="/"
  
  dataLoader={async (fetch, token, query) => 
    adminCommands.listProjects(fetch, token, query)
  }
  
  presentation="cards"
  
  filters={[
    { id: "name", type: "search", label: "Name" },
    { id: "status", type: "select", label: "Status", options: statusOptions }
  ]}
  
  renderItem={(project) => <ProjectCard {project} />}
  
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
<EntityDetailPage
  title={project.name}
  section="Project"
  backHref="/projects"
  
  dataLoader={async (fetch, token) => 
    adminCommands.getProject(id, fetch, token)
  }
  
  meta={[
    { label: "ID", value: <Code source={project.id} /> },
    { label: "Status", value: <Pill tone={tone}>{status}</Pill> }
  ]}
  
  tabs={[
    {
      id: "details",
      label: "Details",
      content: <EntityDetail sections={detailSections} />
    },
    {
      id: "tasks",
      label: "Tasks",
      count: taskCount,
      content: <EntityList 
        dataLoader={loadTasks}
        presentation="cards"
        renderItem={(task) => <TaskCard {task} />}
      />
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
- You want consistency across admin pages

**Don't use templates when:**
- The page has a unique shape that doesn't fit standard patterns
- You need fine-grained control over every element
- Building public-facing pages (use Poodle primitives directly)

## Next Steps

- [Entity List Page](./entity-list-page.md) — Browse and filter lists
- [Entity Detail Page](./entity-detail-page.md) — Read-only detail with tabs
- [Entity Form Page](./entity-form-page.md) — Create and edit forms
