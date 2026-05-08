# Entity List Page

**Status:** In development (g03.004–005)

`EntityListPage` is the Level 1 page shell for browse/list pages. It wraps
`EntityList` with a `PageHeader`, action buttons, and page-level state
management.

## Usage

```svelte
<EntityListPage
  title="Projects"
  backHref="/"
  backLabel="Back to dashboard"
  
  dataLoader={async (fetch, token, query) => 
    adminCommands.listProjects(fetch, token, query)
  }
  
  presentation="cards"
  
  filters={[...]}
  renderItem={(item) => <ProjectCard {item} />}
  
  batchActions={[...]}
  reorder={{ enabled: true, handler: ... }}
  
  onAdd={() => goto("/projects/new")}
/>
```

## Props

### Page Shell

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `title` | `string` | Yes | Page title |
| `section` | `string` | No | Optional section label above the title |
| `subtitle` | `string` | No | Optional subtitle below the title |
| `eyebrow` | `string` | No | Optional eyebrow above the header |
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
| `presentation` | `"cards" | "table"` | Yes | List presentation mode |
| `renderItem` | `(item) => Snippet` | For cards | Card renderer |
| `columns` | `TableColumn[]` | For table | Table column config |

### Filters

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `filters` | `FilterConfig[]` | No | Declarative filter config |

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

## See Also

- [Entity List Section](./entity-list-section.md) — The underlying Level 2 component
- [Template System Overview](./000-template-system-overview.md)
