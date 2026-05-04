# Entity Form Section

**Status:** Not implemented — forms are too flexible to template meaningfully.

## Why No EntityForm?

Real-world forms have:
- **Arbitrary layout** — side-by-side fields, field groups, tabs, accordions
- **Custom fields** — relation selectors, file uploads, rich text editors, date pickers with custom logic
- **Conditional fields** — show field B only when field A is "other"
- **Complex validation** — cross-field validation, async validation, server-side validation
- **Dynamic fields** — add/remove rows in a table, nested objects
- **Multi-step flows** — wizard-style forms with validation per step
- **Custom submit behavior** — optimistic updates, draft saving, confirmation dialogs

A declarative field array (`[{ id, type, label }]`) fights all of this. Every
consumer ends up needing `type: "custom"` for 80% of fields, at which point the
template is just indirection.

## What To Use Instead

Use **Poodle primitives directly** for form fields:

```svelte
<form onsubmit={handleSubmit}>
  <FieldSet>
    <Field label="Name" required>
      <TextInput name="name" />
    </Field>

    <Field label="Category">
      <RelationSelector
        value={categoryId}
        onSelect={(id) => categoryId = id}
      />
    </Field>

    <!-- Custom layout: side by side -->
    <div class="row">
      <Field label="Start Date">
        <DatePicker name="startDate" />
      </Field>
      <Field label="End Date">
        <DatePicker name="endDate" />
      </Field>
    </div>
  </FieldSet>

  <Button type="submit" variant="primary">Save</Button>
</form>
```

Use **EntityFormPage** as a wrapper for the page shell (header, loading, error
states) and bring your own form content.

## See Also

- [Entity Form Page](./entity-form-page.md) — Page shell wrapper
- [Template System Overview](./000-template-system-overview.md)
