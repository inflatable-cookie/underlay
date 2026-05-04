# Entity Form Section

**Status:** Implemented (g03.013)

`EntityForm` is the Level 2 form section. It handles form fields, validation,
and submit orchestration.

## When To Use

- Inside a modal dialog for quick edits
- Inside a detail tab for inline editing
- Standalone when you don't need the full page shell

## Usage

```svelte
<EntityForm
  fields={[
    { id: "name", type: "text", label: "Name", required: true },
    { id: "description", type: "textarea", label: "Description", rows: 4 },
    { id: "priority", type: "select", label: "Priority", options: [
      { value: "low", label: "Low" },
      { value: "high", label: "High" }
    ]}
  ]}
  initialValues={{ name: "My Task", priority: "low" }}
  onSubmit={async (values) => {
    await api.updateTask(values);
  }}
/>
```

### With Validation

```svelte
<EntityForm
  fields={[...]}
  validate={(values) => {
    const errors: Record<string, string> = {};
    if (values.name && String(values.name).length < 3) {
      errors.name = "Name must be at least 3 characters";
    }
    return Object.keys(errors).length > 0 ? errors : null;
  }}
  onSubmit={handleSubmit}
/>
```

### Custom Field

```svelte
{#snippet categoryField({ value, onChange, error, disabled })}
  <RelationSelector
    value={value}
    onSelect={(id) => onChange(id)}
    disabled={disabled}
  />
  {#if error}
    <span class="error">{error}</span>
  {/if}
{/snippet}

<EntityForm
  fields={[
    { id: "name", type: "text", label: "Name" },
    { id: "categoryId", type: "custom", label: "Category", render: categoryField }
  ]}
  onSubmit={handleSubmit}
/>
```

### Controlled Mode

```svelte
<script>
  let values = $state({ name: "", priority: "low" });
</script>

<EntityForm
  fields={[...]}
  {values}
  onSubmit={(submittedValues) => {
    // submittedValues === values (controlled)
  }}
/>
```

## Props

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `fields` | `FieldConfig[]` | Yes | Field definitions |
| `initialValues` | `Record` | No | Initial values (uncontrolled) |
| `values` | `Record` | No | Controlled values |
| `fieldErrors` | `Record<string, string>` | No | External field errors (e.g., from API) |
| `error` | `string` | No | Form-level error message |
| `submitting` | `boolean` | No | Whether form is submitting |
| `loading` | `boolean` | No | Whether form is loading initial data |
| `submitLabel` | `string` | No | Default: "Save" |
| `cancelLabel` | `string` | No | Default: "Cancel" |
| `showCancel` | `boolean` | No | Default: true |
| `onSubmit` | `(values) => Promise` | Yes | Submit handler |
| `onCancel` | `() => void` | No | Cancel handler |
| `validate` | `(values) => Record \| null` | No | Custom validation |

## Field Types

| Type | Description | Additional Props |
|------|-------------|------------------|
| `text` | Single-line text input | — |
| `textarea` | Multi-line text area | `rows?: number` |
| `select` | Dropdown | `options`, `loadOptions?: () => Promise` |
| `number` | Numeric input | `min`, `max`, `step` |
| `checkbox` | Boolean toggle | `checkboxLabel` |
| `custom` | Arbitrary content | `render: Snippet<[FieldRenderContext]>` |

## See Also

- [Entity Form Page](./entity-form-page.md) — Full page shell with header and data loading
- [Template API Reference](./template-api-reference.md) — Complete type definitions
