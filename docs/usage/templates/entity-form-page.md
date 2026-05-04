# Entity Form Page

**Status:** Planned (g03.013)

`EntityFormPage` is the Level 1 page shell for create and edit forms.

## Planned API

```svelte
<EntityFormPage
  title="New Project"
  backHref="/projects"
  
  dataLoader={async (fetch, token) => 
    id ? adminCommands.getProject(id, fetch, token) : null
  }
  
  fields={[
    { id: "name", type: "text", label: "Name", required: true },
    { id: "description", type: "textarea", label: "Description" }
  ]}
  
  onSubmit={async (values) => 
    id 
      ? adminCommands.updateProject(id, values)
      : adminCommands.createProject(values)
  }
/>
```

## See Also

- [Entity Form Section](./entity-form-section.md)
- [Template System Overview](./000-template-system-overview.md)
