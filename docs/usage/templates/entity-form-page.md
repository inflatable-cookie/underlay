# Entity Form Page

**Status:** Implemented (g03.013)

`EntityFormPage` is a page shell for create and edit forms. It handles the boring
parts (header, loading, error/success states) and lets you bring your own form
content.

**Why no declarative `EntityForm`?** Real forms have arbitrary layout, custom
fields, conditional logic, complex validation, file uploads, rich text editors,
multi-step flows, etc. A declarative field array is too restrictive and ends up
fighting the consumer. Use Poodle primitives (`Field`, `TextInput`, `Select`,
etc.) directly for the form itself.

## Usage

```svelte
<script>
  import { EntityFormPage } from "@decodelabs/underlay/templates";
  import { Field, TextInput, Select, Button } from "@poodle/svelte";
  import { ProjectCategorySelector } from "$lib/forms";

  let { data } = $props();
  let project = $state(data.project);
  let saving = $state(false);
  let error = $state(null);
  let success = $state(false);

  async function handleSubmit(event) {
    event.preventDefault();
    saving = true;
    error = null;

    const formData = new FormData(event.currentTarget);
    try {
      await adminCommands.updateProject(data.projectId, {
        name: String(formData.get("name")),
        description: String(formData.get("description"))
      });
      success = true;
    } catch (e) {
      error = e.message;
    } finally {
      saving = false;
    }
  }
</script>

<EntityFormPage
  title={project?.name ?? "Edit Project"}
  section="Edit Project"
  backHref={`/projects/${data.projectId}`}
  backLabel="Back to project"
  loading={!project}
  {error}
  {success}
>
  <form onsubmit={handleSubmit}>
    <Field label="Name" required>
      <TextInput name="name" value={project?.name} />
    </Field>

    <Field label="Description">
      <textarea name="description" rows={4}>{project?.description}</textarea>
    </Field>

    <ProjectCategorySelector
      value={project?.categoryId}
      onSelect={(id) => categoryId = id}
    />

    <div class="form-actions">
      <Button type="submit" variant="primary" loading={saving}>Save</Button>
      <Button type="button" variant="secondary"
        onclick={() => goto(`/projects/${data.projectId}`)}>Cancel</Button>
    </div>
  </form>
</EntityFormPage>
```

## Props

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `title` | `string` | Yes | Page title |
| `section` | `string` | No | Section label |
| `backHref` | `string` | No | Back link URL |
| `backLabel` | `string` | No | Back link label |
| `bannerMessage` | `string` | No | Banner warning/info |
| `bannerTone` | `"warning" \| "info" \| "danger"` | No | Banner tone |
| `loading` | `boolean` | No | Show loading spinner |
| `loadingMessage` | `string` | No | Loading message |
| `error` | `string` | No | Form-level error to display |
| `success` | `boolean` | No | Show success message |
| `successMessage` | `string` | No | Success message text |
| `headerActions` | `Snippet` | No | Additional header actions |
| `children` | `Snippet` | Yes | The form content |

## What It Provides

- **PageHeader** with title, section, back link, banner
- **Loading state** — shows spinner while data loads
- **Error display** — shows Callout when `error` is set
- **Success display** — shows Callout when `success` is true
- **Consistent spacing** — `entity-form-page` class with gap

## What You Bring

- The actual `<form>` element
- All form fields using Poodle primitives
- Validation logic
- Submit handler
- Any custom components (RelationSelector, file upload, etc.)

## See Also

- [Template System Overview](./000-template-system-overview.md)
- Poodle form primitives: `Field`, `TextInput`, `Select`, `Textarea`, `Button`
