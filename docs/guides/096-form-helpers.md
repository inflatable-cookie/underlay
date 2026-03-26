# 096 - Form Helpers

This document covers Underlay's form helper patterns for common operations in SPA form pages.

## Overview

Form pages in SPA apps often share similar patterns:
- Submitting forms with specific intents (save, save-close, delete)
- Syncing selection state from multiple sources
- Creating search/suggest functions for RelationSelector
- Auto-generating slugs from titles
- Calculating next values in sequences

These helpers extract these patterns into reusable, tested utilities.

---

## Quick Reference

| Helper | Import | Purpose |
|--------|--------|---------|
| `submitFormWithIntent()` | `@decodelabs/underlay/patterns` | Submit form with intent |
| `useSyncedSelection()` | `@decodelabs/underlay/patterns` | Manage selection state |
| `createLocalSearchFns()` | `@decodelabs/underlay/patterns` | Search/suggest for RelationSelector |
| `slugify` / `validateSlug` | `@decodelabs/underlay/patterns` | Pure slug helpers for app-owned slug fields |
| `useValidatedForm()` | `@decodelabs/underlay/patterns` | Lightweight Zod-backed client-side form orchestration |
| `TabsRoot` + `TabsList` + `TabsTrigger` + `TabsContent` | `@decodelabs/underlay/components` | Multi-section form tabs |
| `getNextLetter()` | `@decodelabs/underlay/utils` | Next letter in sequence |
| `getNextNumber()` | `@decodelabs/underlay/utils` | Next number in sequence |

---

## Zod-Backed Validated Forms

Use `useValidatedForm()` when the form benefits from immediate client-side schema checks but you do not want to replace Underlay's existing field or submit primitives.

```ts
import { useValidatedForm } from "@decodelabs/underlay/patterns";
import { registerRequestSchema } from "@decodelabs/underlay/validation";

const form = useValidatedForm({
  schema: registerRequestSchema,
  initialValues: {
    email: "",
    password: "",
    displayName: "",
  },
  onSubmit: async (values) => {
    await api.auth.register(values);
  },
  validateOnChange: true,
});
```

Use it as the form-state owner rather than alongside hidden field registries:

- `useValidatedForm()` owns schema parsing, submit state, and field-error mapping.
- Derive submit enablement from the field state your form already owns.
- Keep server validation in place for every submission path.

---

## Form Tabs Approach (Large Forms)

Use form tabs when a single form has multiple conceptual sections (for example: Details / Notes / Marking).

### Why this pattern exists

- Keeps long forms scannable without splitting into separate routes
- Preserves one submit surface (typically a Poodle `SplitButton` plus hidden intent field and form action)
- Works with rich editors that break when mounted under `display: none`

### Recommended wiring

The recommended hierarchy is:

1. `TabsRoot variant="form"`
2. `TabsList`
3. `TabsTrigger`
4. `TabsContent`
5. Fields/inputs

```svelte
<TabsRoot bind:value={activeTab} variant="form">
  <TabsList collapsible>
    <TabsTrigger value="details">Details</TabsTrigger>
    <TabsTrigger value="notes">Notes</TabsTrigger>
  </TabsList>

  <TabsContent value="details">
    <div class="underlay-form-grid">
      <!-- details fields -->
    </div>
  </TabsContent>

  <TabsContent value="notes">
    <div class="underlay-form-grid">
      <!-- notes fields -->
    </div>
  </TabsContent>
</TabsRoot>
```

### Section ID/value alignment rule

Use one canonical id per tab section and keep it aligned across:

- `TabsTrigger value="details"`
- `TabsContent value="details"`
If these diverge, the active section and rendered panel will drift apart.

### Collapsible tabs for narrow layouts

Use `TabsList collapsible` so tabs can collapse into a dropdown without losing labels or counts.

```svelte
<TabsList collapsible>
  <TabsTrigger value="details">Details</TabsTrigger>
  <TabsTrigger value="notes" count={3}>Notes</TabsTrigger>
</TabsList>
```

### Rich-editor compatibility

The `form` variant keeps inactive tab panels mounted (hidden without `display: none`) so editors like CodeMirror/EasyMDE can initialize correctly and keep state while switching tabs.

### Recommended composition

- Put top-level sections in form tabs (`details`, `notes`, `marking`, etc.)
- Keep micro-modes (e.g. `Edit` / `Preview`) as nested tabs *inside* a section
- Keep `FormActions` outside tab panels so submit controls remain constant
- Keep section-specific completion logic app-owned if you need it

### Common mistakes

- Mismatched `TabsTrigger.value` / `TabsContent.value`
- Moving `FormActions` inside a tab panel (actions disappear when switching tabs)

---

## Form Intent Submission

### `submitFormWithIntent()`

Submit a form with a specific intent value. Useful for delete buttons that need to submit the main form with `intent="delete"`.

```typescript
import { submitFormWithIntent } from "@decodelabs/underlay/patterns";

function handleDelete() {
  submitFormWithIntent("delete");
}
```

**Parameters:**
- `intent` - The intent value to set (e.g., "delete", "save", "save-close")
- `formSelector` - CSS selector for the form (default: `"form"`)
- `intentFieldName` - Name of the hidden intent input field (default: `"intent"`)

**Example with custom form:**

```typescript
// Submit a specific form with a custom intent field
submitFormWithIntent("archive", "#settings-form", "action");
```

**Usage in form components:**

```svelte
<script lang="ts">
  import { submitFormWithIntent } from "@decodelabs/underlay/patterns";

  function handleDelete() {
    // This sets the hidden "intent" input to "delete" and submits
    submitFormWithIntent("delete");
  }
</script>

<!-- The form must have a hidden intent input -->
<form method="post">
  <input type="hidden" name="intent" value="save-close" />

  <!-- Delete button that changes intent before submitting -->
  <button type="button" onclick={handleDelete}>Delete</button>

  <button type="submit">Save</button>
</form>
```

---

## Selection State Management

### `useSyncedSelection()`

Manage selection state that needs to sync from multiple sources:
1. Initialize once from loaded data or URL parameters
2. Restore from form values after validation failure
3. Allow user changes

**File:** Uses Svelte 5 runes (requires `.svelte.ts` extension or Svelte component)

```typescript
import { useSyncedSelection } from "@decodelabs/underlay/patterns";

// Create selection state
const selection = useSyncedSelection<string>();

// Initialize from loaded data (runs once when data becomes available)
$effect(() => {
  selection.initializeFrom(pageData.data?.areaId);
});

// Sync from form values (after validation failure)
$effect(() => {
  if (typeof formValues?.areaId === "string") {
    selection.syncFrom(formValues.areaId);
  }
});
```

**Interface:**

```typescript
interface SyncedSelectionResult<T> {
  /** Current selected value (reactive, get/set) */
  value: T | null;

  /** Whether the selection has been initialized */
  readonly hasInitialized: boolean;

  /** Initialize from source (one-time, only if not already initialized) */
  initializeFrom: (source: T | null | undefined) => void;

  /** Sync from source (always updates if non-null) */
  syncFrom: (source: T | null | undefined) => void;

  /** Reset state to allow re-initialization */
  reset: () => void;
}
```

**Full example:**

```svelte
<script lang="ts">
  import { useSyncedSelection } from "@decodelabs/underlay/patterns";
  import RelationSelector from "@decodelabs/underlay/patterns/RelationSelector";

  let { data } = $props();
  let formValues = $state<Record<string, unknown> | undefined>(undefined);

  // Create selection state
  const areaSelection = useSyncedSelection<string>();

  // Initialize from URL parameter when data loads
  $effect(() => {
    areaSelection.initializeFrom(data.preselectedAreaId);
  });

  // Sync from form values after validation failure
  $effect(() => {
    if (typeof formValues?.areaId === "string") {
      areaSelection.syncFrom(formValues.areaId);
    }
  });
</script>

<RelationSelector
  name="areaId"
  bind:value={areaSelection.value}
  search={searchAreas}
  suggestions={suggestAreas}
/>
```

**Key behaviors:**

- `initializeFrom()` only sets the value once (when `hasInitialized` is false)
- `syncFrom()` always updates the value (useful for form value restoration)
- Use `bind:value={selection.value}` for two-way binding with form components
- The `value` property has both getter and setter for reactive updates

---

## Local Search Functions

### `createLocalSearchFns()`

Create search and suggest functions for `RelationSelector` when filtering client-side data.

```typescript
import { createLocalSearchFns } from "@decodelabs/underlay/patterns";

const { search, suggest } = createLocalSearchFns(
  () => sections,  // Getter for current items
  {
    toSelectable: (s) => ({
      id: s.sectionId,
      label: s.label,
      description: s.title
    }),
    getSearchText: (s) => [s.label, s.title, s.moduleName ?? ""]
  }
);
```

**Options:**

```typescript
interface LocalSearchOptions<TItem, TSelectable> {
  /** Convert item to SelectableRelation format */
  toSelectable: (item: TItem) => TSelectable;

  /** Extract searchable text fields from item */
  getSearchText: (item: TItem) => string[];

  /** Optional filter function (called before text search) */
  applyFilters?: (
    items: TItem[],
    filters: Record<string, string | undefined> | undefined
  ) => TItem[];

  /** Maximum suggestions to return (default: all) */
  maxSuggestions?: number;
}
```

**With filters:**

```typescript
const { search, suggest } = createLocalSearchFns(
  () => sections,
  {
    toSelectable: (s) => ({
      id: s.sectionId,
      label: `${s.moduleName} ${s.label}`,
      description: s.title
    }),
    getSearchText: (s) => [s.label, s.title, s.moduleName ?? ""],
    applyFilters: (items, filters) =>
      filters?.moduleId
        ? items.filter(s => s.moduleId === filters.moduleId)
        : items
  }
);
```

**In RelationSelector:**

```svelte
<script lang="ts">
  import { createLocalSearchFns } from "@decodelabs/underlay/patterns";
  import RelationSelector from "@decodelabs/underlay/patterns/RelationSelector";

  interface Props {
    sections: Array<{
      sectionId: string;
      label: string;
      title: string;
      moduleId: string;
      moduleName: string;
    }>;
    moduleId?: string;
  }

  let { sections, moduleId }: Props = $props();

  const { search: searchSections, suggest: suggestSections } = createLocalSearchFns(
    () => sections,
    {
      toSelectable: (s) => ({
        id: s.sectionId,
        label: `${s.moduleName} ${s.label}`,
        description: s.title
      }),
      getSearchText: (s) => [s.label, s.title, s.moduleName ?? ""],
      applyFilters: (items, filters) =>
        filters?.moduleId
          ? items.filter(s => s.moduleId === filters.moduleId)
          : items
    }
  );

  // Dynamic filters based on selected module
  const sectionFilters = $derived(
    moduleId ? { moduleId } : undefined
  );
</script>

<RelationSelector
  name="sectionId"
  label="Section"
  search={searchSections}
  suggestions={suggestSections}
  filters={sectionFilters}
/>
```

---

## Slug Field

### App-Owned Slug Fields

Underlay no longer exports a `SlugField` component. Build slug fields with
Poodle `Field` and `TextInput`, keep the generation state in the form, and use
Underlay slug helpers only where you want to share pure formatting rules.

```svelte
<script lang="ts">
  import { Field, TextInput, type InputValidationStatus } from "@poodle/svelte-primitives";
  import { slugify, isReservedSlug, isValidSlugFormat } from "@decodelabs/underlay/patterns";

  let title = $state("");
  let slug = $state("");
  let lastAutoSlug = $state("");
  let slugStatus = $state<InputValidationStatus>("idle");
  let slugError = $state<string | null>(null);

  $effect(() => {
    const nextAutoSlug = slugify(title);
    if (!slug.trim() || slug === lastAutoSlug) {
      slug = nextAutoSlug;
    }
    lastAutoSlug = nextAutoSlug;
  });

  async function validateSlug(value: string) {
    const normalized = value.trim();

    if (!isValidSlugFormat(normalized, 64)) {
      return { valid: false, message: "Use lowercase letters, numbers, and hyphens only." };
    }

    if (isReservedSlug(normalized)) {
      return { valid: false, message: "This slug is reserved." };
    }

    return await api.validateSlug(normalized);
  }
</script>

<Field label="Title">
  <TextInput
    id="title"
    name="title"
    value={title}
    on:valueChange={(event) => {
      title = event.detail.value;
    }}
  />
</Field>

<Field
  id="slug"
  label="Slug"
  error={slugStatus === "invalid" ? slugError : null}
  validationState={slugStatus === "validating" ? "pending" : slugStatus === "invalid" ? "invalid" : slugStatus === "valid" ? "valid" : "none"}
>
  <TextInput
    id="slug"
    name="slug"
    value={slug}
    autocomplete="off"
    required
    pattern="[a-z0-9]+(?:-[a-z0-9]+)*"
    maxLength={64}
    validate={validateSlug}
    on:valueChange={(event) => {
      slug = event.detail.value;
    }}
    on:validationChange={(event) => {
      slugStatus = event.detail.status;
      slugError = event.detail.status === "invalid" ? event.detail.message || null : null;
    }}
    on:blur={() => {
      slug = slugify(slug);
    }}
  />
</Field>
```

Use the same pattern for prefixes, scoped uniqueness, or edit-mode exclusion:
- prepend display-only keys with `TextInput.prefix`
- pass sibling IDs through `validationContext`
- derive submit gating from real values plus `validationChange`

For the reusable cross-app recipe, see Poodle
[Slug Field Recipes](../../../poodle/docs/guides/007-slug-field-recipes.md).

---

## Sequence Helpers

### `getNextLetter()`

Get the next available letter not in the existing set.

```typescript
import { getNextLetter } from "@decodelabs/underlay/utils";

getNextLetter(["A", "B", "C"]);        // "D"
getNextLetter(["A", "C", "D"]);        // "B" (fills gap)
getNextLetter([]);                      // "A"
getNextLetter(["A", "B"], { lowercase: true }); // "c"
```

**Parameters:**
- `existing` - Array of existing letter labels
- `options.lowercase` - Return lowercase letters (default: false)

**Behavior:**
- Tries single letters first (A-Z)
- Then double letters (AA-ZZ)
- Case-insensitive comparison (["a"] blocks "A")
- Fills gaps rather than always appending

### `getNextNumber()`

Get the next available positive integer.

```typescript
import { getNextNumber } from "@decodelabs/underlay/utils";

getNextNumber([1, 2, 3]);   // 4
getNextNumber([1, 5, 3]);   // 6 (max + 1, doesn't fill gaps)
getNextNumber([]);          // 1
```

**Parameters:**
- `existing` - Array of existing numbers

**Behavior:**
- Returns `max(existing) + 1`
- Returns `1` for empty array
- Does NOT fill gaps (unlike `getNextLetter`)

**Example usage in form:**

```svelte
<script lang="ts">
  import { getNextLetter, getNextNumber } from "@decodelabs/underlay/utils";

  interface Props {
    sections: Array<{ label: string }>;
    areas: Array<{ number: number }>;
  }

  let { sections, areas }: Props = $props();

  // Suggest next section label
  const suggestedLabel = $derived(
    getNextLetter(sections.map(s => s.label))
  );

  // Suggest next area number
  const suggestedNumber = $derived(
    getNextNumber(areas.map(a => a.number))
  );
</script>

<TextInput name="label" value={suggestedLabel} />
<TextInput name="number" value={String(suggestedNumber)} type="number" />
```

---

## Complete Example

Here's a complete example combining multiple helpers in a form page:

```svelte
<!-- /learning/areas/new/+page.svelte -->
<script lang="ts">
  import type { PageData } from "./$types";
  import { useSyncedSelection, createLocalSearchFns, submitFormWithIntent, slugify } from "@decodelabs/underlay/patterns";
  import { Field, TextInput } from "@poodle/svelte-primitives";
  import { getNextNumber } from "@decodelabs/underlay/utils";
  import RelationSelector from "@decodelabs/underlay/patterns/RelationSelector";

  interface Props {
    data: PageData;
  }

  let { data }: Props = $props();
  let formValues = $state<Record<string, unknown> | undefined>(undefined);

  // Selection sync for section dropdown
  const sectionSelection = useSyncedSelection<string>();

  $effect(() => {
    sectionSelection.initializeFrom(data.preselectedSectionId);
  });

  $effect(() => {
    if (typeof formValues?.sectionId === "string") {
      sectionSelection.syncFrom(formValues.sectionId);
    }
  });

  // Local search for sections
  const { search: searchSections, suggest: suggestSections } = createLocalSearchFns(
    () => data.sections,
    {
      toSelectable: (s) => ({
        id: s.sectionId,
        label: s.label,
        description: s.title
      }),
      getSearchText: (s) => [s.label, s.title]
    }
  );

  // Suggest next area number for selected section
  const selectedSection = $derived(
    data.sections.find(s => s.sectionId === sectionSelection.value)
  );

  const suggestedNumber = $derived(
    selectedSection
      ? getNextNumber(selectedSection.existingNumbers)
      : 1
  );

  // Form state
  let title = $state("");
  let slug = $state("");
</script>

<form method="post">
  <RelationSelector
    name="sectionId"
    label="Section"
    bind:value={sectionSelection.value}
    search={searchSections}
    suggestions={suggestSections}
    required
  />

  <Field label="Title">
    <TextInput name="title" bind:value={title} required />
  </Field>

  <Field label="Slug">
    <TextInput
      id="slug"
      name="slug"
      value={slug}
      on:valueChange={(event) => {
        slug = event.detail.value;
      }}
      on:blur={() => {
        slug = slugify(slug);
      }}
    />
  </Field>

  <Field label="Number">
    <TextInput
      name="number"
      type="number"
      value={String(suggestedNumber)}
      required
    />
  </Field>

  <input type="hidden" name="intent" value="save-close" />
  <button type="submit">Create</button>
</form>
```

---

## Best Practices

1. **Use `useSyncedSelection()` for bound dropdowns** - It handles initialization, form value restoration, and user changes correctly.

2. **Prefer `createLocalSearchFns()` for small datasets** - For large datasets or server-side filtering, use async search functions instead.

3. **Keep slug logic app-owned** - auto-generation, validation wiring, and submit gating should live in the form, not in a shared wrapper.

4. **Use sequence helpers for suggested defaults** - They provide sensible suggestions but allow user override.

5. **Keep intent hidden input in sync** - The `submitFormWithIntent()` helper expects a hidden input named "intent" in the form.

---

## Troubleshooting

### Selection not initializing

**Possible causes:**
- Data hasn't loaded yet when `initializeFrom()` runs
- Value is `undefined` instead of `null`

**Solution:** Ensure the `$effect` runs when data is available:

```typescript
$effect(() => {
  // This runs when pageData.data changes
  if (pageData.data?.sectionId) {
    selection.initializeFrom(pageData.data.sectionId);
  }
});
```

### Slug not auto-generating

**Possible causes:**
- User has manually edited the slug (this is intentional - their edit is preserved)
- `source` prop is empty or undefined

**Debug:**
```typescript
// Check if the current value differs from what would be auto-generated
console.log("Current slug:", slug);
console.log("Would auto-generate:", slugify(title));
```

### Search not finding items

**Possible causes:**
- `getItems()` getter returns stale data
- `getSearchText()` doesn't include the field being searched

**Solution:** Ensure the getter returns current reactive data:

```typescript
// Good - reactive getter
createLocalSearchFns(() => sections, ...)

// Bad - captured value
const sectionsCopy = sections;
createLocalSearchFns(() => sectionsCopy, ...)
```

---

## Next Steps

- [090-ui-kit](./090-ui-kit.md) - UI components used in forms
- [092-selection-suggestions](./092-selection-suggestions.md) - RelationSelector patterns
- [095-navigation-context](./095-navigation-context.md) - Navigation context for form redirects
