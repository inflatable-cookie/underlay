# Entity Form Page

**Status:** Implemented (g03.013)

`EntityFormPage` is a page shell for create and edit forms. It handles the boring
parts:

- header and back-link framing
- loading, success, error, and field-error states
- SPA submit wiring and redirects when you provide `onSubmit`
- the actual `form` wrapper

You still bring your own field layout and custom form controls.

**Why no declarative `EntityForm`?** Real forms have arbitrary layout, custom
fields, conditional logic, complex validation, file uploads, rich text editors,
multi-step flows, etc. A declarative field array is too restrictive and ends up
fighting the consumer. Use Poodle primitives (`Field`, `TextInput`, `Select`,
etc.) directly for the form itself.

## Usage

```svelte
<script>
  import { EntityFormPage } from "@decodelabs/underlay/templates";
  import type { SpaFormResult } from "@decodelabs/underlay/patterns";
  import { Field, TextInput, Select, Button } from "@poodle/svelte";
  import { ProjectCategorySelector } from "$lib/forms";

  let { data } = $props();
  let project = $state(data.project);
  let saving = $state(false);
  let error = $state(null);
  let success = $state(false);

  async function handleSubmit(formData): Promise<SpaFormResult> {
    try {
      await adminCommands.updateProject(data.projectId, {
        name: String(formData.get("name")),
        description: String(formData.get("description"))
      });
      return { success: true };
    } catch (e) {
      return { success: false, error: e.message };
    }
  }
</script>

<EntityFormPage
  section="Edit Project"
  subtitle={project?.name}
  backHref={`/projects/${data.projectId}`}
  loading={!project}
  {error}
  {success}
  onSubmit={handleSubmit}
>
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
    <Button type="submit" variant="primary">Save</Button>
    <Button type="button" variant="secondary"
      onclick={() => goto(`/projects/${data.projectId}`)}>Cancel</Button>
  </div>
</EntityFormPage>
```

## Props

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `title` | `string` | No | Page title |
| `section` | `string` | No | Section label |
| `subtitle` | `string` | No | Secondary line under title/section |
| `backHref` | `string` | No | Back link URL |
| `backLabel` | `string` | No | Back link label |
| `backIsContextual` | `boolean` | No | Marks the back link as contextual |
| `bannerMessage` | `string` | No | Banner warning/info |
| `bannerTone` | `"warning" \| "info" \| "danger"` | No | Banner tone |
| `loading` | `boolean` | No | Show loading spinner |
| `loadingMessage` | `string` | No | Loading message |
| `error` | `string` | No | Form-level error to display |
| `fieldErrors` | `Record<string, string>` | No | Field-level errors for summary callout |
| `success` | `boolean` | No | Show success message |
| `successMessage` | `string` | No | Success message text |
| `prepare` | `(formData) => void` | No | Modify form data before submit |
| `onSubmit` | `SpaSubmitHandler` | No | SPA submit handler |
| `onResult` | `(result) => void` | No | Submission result callback |
| `navigate` | `SpaNavigateFn` | No | Redirect handler after success |
| `headerMeta` | `Snippet` | No | Metadata block below the header |
| `headerActions` | `Snippet` | No | Additional header actions |
| `children` | `Snippet` | Yes | The form content |

## What It Provides

- **PageHeader** with title, section, subtitle, back link, banner
- **Header meta** block below the header when needed
- **Loading state** — shows spinner while data loads
- **Success/error display** — shows save feedback callouts
- **Field-error summary** — shows a field-error list callout
- **Internal `<form>` wrapper** — you provide fields, not the outer form tag
- **SPA submit wiring** — handles client-side submit/redirect when `onSubmit` is provided
- **Consistent spacing** — `entity-form-page` class with gap

## What You Bring

- All form fields using Poodle primitives
- Validation logic
- Submit handler logic
- Any custom components (RelationSelector, file upload, etc.)

## See Also

- [Template System Overview](./000-template-system-overview.md)
- Poodle form primitives: `Field`, `TextInput`, `Select`, `Textarea`, `Button`
