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
| `SlugField` | `@decodelabs/underlay/patterns` | Auto-slug field component |
| `getNextLetter()` | `@decodelabs/underlay/utils` | Next letter in sequence |
| `getNextNumber()` | `@decodelabs/underlay/utils` | Next number in sequence |

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

### `SlugField` Component

Auto-generates URL slugs from a source field (e.g., title), with built-in validation.

```svelte
<script lang="ts">
  import { SlugField } from "@decodelabs/underlay/patterns";

  let title = $state("");
  let slug = $state("");
</script>

<Field label="Title">
  <TextInput name="title" bind:value={title} />
</Field>

<SlugField
  bind:value={slug}
  source={title}
  validate={validateSlug}
/>
```

**Props:**

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `string` | `""` | Current slug (bindable) |
| `source` | `string` | `""` | Source value to generate slug from |
| `validate` | `(slug: string) => Promise<ValidationResult>` | - | Async uniqueness validator |
| `validationKey` | `unknown` | - | Revalidate when this changes |
| `label` | `string` | `"Slug"` | Field label |
| `name` | `string` | `"slug"` | Form field name |
| `prefix` | `string` | - | Static prefix to display |
| `maxlength` | `number` | - | Maximum slug length |
| `disabled` | `boolean` | `false` | Disable input |
| `required` | `boolean` | `false` | Mark as required |
| `hint` | `string` | - | Help text |
| `error` | `string` | - | Error message from form |

**Auto-slug behavior:**

The component tracks the last auto-generated slug to detect user customizations:

- **Empty value:** Auto-generates from source
- **Value matches last auto-slug:** Updates when source changes
- **Value differs (user customized):** Preserves user's slug

This handles edit mode correctly - if the server returns a custom slug that differs from what would be auto-generated, it's preserved.

**With async validation:**

```svelte
<script lang="ts">
  import { SlugField } from "@decodelabs/underlay/patterns";
  import { learningCommands } from "@cattle-grid";

  async function validateSlug(slug: string) {
    const token = auth.getToken();
    if (!token) return { valid: false, message: "Not authenticated" };

    return await learningCommands.validateField(
      {
        entity: "module",
        field: "slug",
        value: slug,
        context: { excludeId: moduleId }
      },
      fetch,
      token
    );
  }
</script>

<SlugField
  bind:value={slug}
  source={title}
  validate={validateSlug}
  validationKey={moduleId}
/>
```

**With prefix:**

```svelte
<SlugField
  bind:value={slug}
  source={title}
  prefix="acca-fa-"
/>
<!-- Displays: acca-fa-[input field] -->
```

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
  import { useSyncedSelection, createLocalSearchFns, submitFormWithIntent } from "@decodelabs/underlay/patterns";
  import { SlugField } from "@decodelabs/underlay/patterns";
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

  <SlugField
    bind:value={slug}
    source={title}
    validate={validateSlug}
  />

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

3. **Let SlugField handle auto-slug logic** - Don't add separate `$effect` blocks for slug generation; the component handles it internally.

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
