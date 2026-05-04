# Entity Form Page

**Status:** Implemented (g03.013)

`EntityFormPage` is the Level 1 page shell for create and edit forms. It wraps
`EntityForm` with a `PageHeader`, data loading, and submit state management.

## Usage

### Create Form

```svelte
<EntityFormPage
  title="New Project"
  section="New Project"
  backHref="/projects"
  backLabel="Back to projects"
  
  fields={[
    { id: "name", type: "text", label: "Name", required: true },
    { id: "description", type: "textarea", label: "Description", rows: 6 },
    { id: "status", type: "select", label: "Status", options: [
      { value: "active", label: "Active" },
      { value: "archived", label: "Archived" }
    ]}
  ]}
  
  onSubmit={async (values) => {
    await adminCommands.createProject(values);
    goto("/projects");
  }}
/>
```

### Edit Form

```svelte
<EntityFormPage
  title={project.name}
  section="Edit Project"
  backHref={`/projects/${id}`}
  backLabel="Back to project"
  
  dataLoader={async (fetch, token) => {
    const project = await adminCommands.getProject(id, fetch, token);
    return { name: project.name, description: project.description, status: project.status };
  }}
  
  fields={[...]}
  
  onSubmit={async (values) => {
    await adminCommands.updateProject(id, values);
  }}
/>
```

## Props

### Page Shell

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `title` | `string` | Yes | Page title |
| `subtitle` | `string` | No | Subtitle or description |
| `section` | `string` | No | Section label (e.g., "Edit Project") |
| `backHref` | `string` | No | Back link URL |
| `backLabel` | `string` | No | Back link label |
| `bannerMessage` | `string` | No | Banner warning/info |
| `bannerTone` | `"warning" \| "info" \| "danger"` | No | Banner tone |

### Data

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `dataLoader` | `(fetch, token) => Promise` | No | Data loader for edit mode |
| `fields` | `FieldConfig[]` | Yes | Declarative field config |

### Validation

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `validate` | `(values) => Record<string, string> \| null` | No | Custom validation function |

### Actions

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `onSubmit` | `(values) => Promise<void>` | Yes | Submit handler |
| `onCancel` | `() => void` | No | Cancel handler |
| `submitLabel` | `string` | No | Submit button label (default: "Save") |
| `cancelLabel` | `string` | No | Cancel button label (default: "Cancel") |
| `showCancel` | `boolean` | No | Show cancel button (default: true) |
| `successMessage` | `string` | No | Message shown after successful submit |

## See Also

- [Entity Form Section](./entity-form-section.md) — The underlying Level 2 component
- [Template System Overview](./000-template-system-overview.md)
- [Template API Reference](./template-api-reference.md) — FieldConfig types
