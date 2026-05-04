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
      content: <EntityList ... />
    }
  ]}
  
  actions={[
    { label: "Edit", handler: handleEdit },
    { label: "Delete", tone: "danger", confirm: true, handler: handleDelete }
  ]}
/>
```

## See Also

- [Entity Detail Section](./entity-detail-section.md)
- [Template System Overview](./000-template-system-overview.md)
