# 008 - EntityDetailPage Shell Component

Status: complete
Owner: repo maintainers
Updated: 2026-05-04

## Goals

- build `EntityDetailPage.svelte` as a page shell around `EntityDetail`
- add `PageHeader` with title, back link, and action buttons
- add `Tabs` for section switching
- support child collection tabs using `EntityList`

## API Design

```svelte
<EntityDetailPage
  title={project.name}
  section="Project"
  backHref="/projects"
  backLabel="Back to projects"
  
  dataLoader={...}
  
  meta={...}
  
  tabs={[
    {
      id: "details",
      label: "Details",
      content: <EntityDetail sections={...} />
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

### Batch 8.1 - Shell Structure

- [x] create `EntityDetailPage.svelte`
- [x] integrate `PageHeader` with title, section, back link, actions
- [x] integrate `Tabs` for section switching

### Batch 8.2 - Tab Content

- [x] support detail tabs using `EntityDetail`
- [x] support list tabs using `EntityList`
- [x] support custom tab content via snippets

### Batch 8.3 - Action Handling

- [x] implement action buttons with optional confirmation dialogs
- [x] support danger-tone actions with confirmation

## Exit Criteria

- `EntityDetailPage.svelte` compiles without errors
- renders header, meta, tabs, and tab content
- acme-admin `/projects/[id]/+page.svelte` can be reduced from 800 lines to ~80

## Next Task

Execute `g03.009`: migrate acme-admin project detail page as proof.
