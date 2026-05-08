# Entity Detail Page

**Status:** In development (g03.007–008)

`EntityDetailPage` is the Level 1 page shell for read-only detail pages. It
combines `PageHeader`, `MetaBar`, `Tabs`, and `EntityDetail` sections into a
complete detail view.

## Usage

```svelte
<EntityDetailPage
  title={project.name}
  section="Project"
  backHref="/projects"
  
  dataLoader={async (fetch, token) => 
    adminCommands.getProject(id, fetch, token)
  }
  
  meta={[...]}
  
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
      content: <EntityListPage title="Tasks" headerLevel={3} ... />
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

## See Also

- [Entity Detail Section](./entity-detail-section.md)
- [Template System Overview](./000-template-system-overview.md)
