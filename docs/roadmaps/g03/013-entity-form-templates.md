# 013 - EntityForm and EntityFormPage

Status: not started
Owner: repo maintainers
Updated: 2026-05-04

## Context

The template system has list (`EntityListPage`) and detail (`EntityDetailPage`) page
shells. The third page shape is forms: create, edit, and modal dialogs. These are
currently hand-rolled in every consumer app.

## Goals

- build `EntityForm` (Level 2) — self-contained form section for use in pages, tabs, dialogs
- build `EntityFormPage` (Level 1) — full page shell with header, form, and actions
- support declarative field configuration (text, textarea, select, number, date, etc.)
- support validation integration (Zod schema or custom validate function)
- support both page and dialog contexts

## Planned API

### EntityFormPage

```svelte
<EntityFormPage
  title="New Project"
  backHref="/projects"
  
  dataLoader={async (fetch, token) => 
    id ? adminCommands.getProject(id, fetch, token) : null
  }
  
  fields={[
    { id: "name", type: "text", label: "Name", required: true },
    { id: "description", type: "textarea", label: "Description" },
    { id: "status", type: "select", label: "Status", options: statusOptions }
  ]}
  
  schema={projectSchema}
  
  onSubmit={async (values) => 
    id 
      ? adminCommands.updateProject(id, values)
      : adminCommands.createProject(values)
  }
/>
```

### EntityForm (Level 2)

```svelte
<EntityForm
  {fields}
  {schema}
  initialValues={existingData}
  onSubmit={handleSubmit}
/>
```

## Field Types

- `text` — single-line text input
- `textarea` — multi-line text input
- `select` — dropdown with options
- `number` — numeric input
- `date` — date picker
- `datetime` — date/time picker
- `checkbox` — boolean toggle
- `custom` — render a Snippet for arbitrary content

## Non-Goals

- Complex nested forms (use custom fields)
- File upload (use custom fields)
- WYSIWYG editors (use custom fields)
- Multi-step wizards

## Exit Criteria

- `EntityForm` and `EntityFormPage` implemented and exported
- Basic field types work: text, textarea, select, number
- Validation works with Zod schemas
- Form can be used standalone or inside a dialog
- Docs updated with examples
- At least one consumer page migrated as proof

## Next Task

Execute `g03.014`: migrate acme-admin project create/edit pages to `EntityFormPage`.
