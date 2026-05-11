# 013 - EntityFormPage

Status: complete (pivoted)
Owner: repo maintainers
Updated: 2026-05-04

## Results

**Pivot:** Declarative `EntityForm` was removed. Real forms have arbitrary layout,
custom fields, conditional logic, complex validation, file uploads, rich text
editors, multi-step flows, etc. A declarative field array fights all of this.

**What was kept:**
- `EntityFormPage` as a **page shell wrapper** — handles header, loading,
  error/success states, field-error summary, and SPA submit wiring
- Consumer brings their own form content using Poodle primitives directly
- Clean separation: templates own the shell, Poodle owns the primitives, consumer owns the form logic

## What EntityFormPage Provides

- `PageHeader` with title, section, back link, banner
- `PageHeader` subtitle and contextual back-link support
- optional header metadata block
- Loading state (spinner while data loads)
- Error display (Callout when `error` is set)
- Success display (Callout when `success` is true)
- field-error summary
- internal form wrapper and SPA submit/redirect handling
- Consistent spacing via `entity-form-page` class

## What The Consumer Brings

- All form fields using Poodle primitives (`Field`, `TextInput`, `Select`, etc.)
- Validation logic
- Submit handler
- Any custom components (RelationSelector, file upload, etc.)

## Example

```svelte
\u003cEntityFormPage
  title={project?.name ?? "Edit Project"}
  section="Edit Project"
  backHref={`/projects/${id}`}
  loading={!project}
  error={submitError}
  success={submitSuccess}
\u003e
    \u003cField label="Name" required\u003e
      \u003cTextInput name="name" value={project?.name} /\u003e
    \u003c/Field\u003e

    \u003cProjectCategorySelector
      value={categoryId}
      onSelect={(id) =\u003e categoryId = id}
    /\u003e

    \u003cButton type="submit" variant="primary"\u003eSave\u003c/Button\u003e
\u003c/EntityFormPage\u003e
```

## Rationale

After implementing a declarative `EntityForm` with field types, it became clear
that every real-world form quickly needs:
- Side-by-side field layouts
- Conditional field visibility
- Custom components (RelationSelector, file upload, rich text)
- Cross-field validation
- Dynamic field arrays
- Custom submit orchestration

The `type: "custom"` escape hatch ended up being needed for 80% of fields. At
that point the template is just indirection. Better to use Poodle primitives
directly and let `EntityFormPage` handle only the page-level concerns.

## Next Task

Execute `g03.014`: migrate acme-admin project create/edit pages to `EntityFormPage`.
