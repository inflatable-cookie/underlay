# 090 - UI Kit

This document covers creating and using a shared Svelte UI kit with Underlay. Underlay provides pre-built components for common UI needs.

## Overview

The UI kit provides:
- **Form components** - Field, TextInput, Select, Switch, TextArea
- **UI primitives** - Button, Badge, Pill, Breadcrumbs, Card, Dialog, DropdownMenu, OrderBy, Pagination
- **Patterns** - ListCard, NavCard, NavCardGrid, Form, FormActions, PageHeader, ReorderableList
- **Design tokens** - CSS custom properties for theming

## UI Kit Structure

> **Naming Convention**: Choose names that reflect your project's domain. For example, a project called "Acme" might use `acme-ui` for its UI kit. The examples below use placeholder names like `myapp-ui` - replace these with your own project-specific names.

```
libs/myapp-ui/src/
├── components/          # Reusable Svelte components
│   ├── Button.svelte
│   ├── TextInput.svelte
│   ├── Select.svelte
│   ├── Switch.svelte
│   ├── Field.svelte
│   ├── Card.svelte
│   ├── Dialog.svelte
│   ├── DropdownMenu.svelte
│   └── index.ts
├── patterns/            # Higher-level UI patterns
│   ├── Form.svelte
│   ├── FormActions.svelte
│   ├── ListCard.svelte
│   └── index.ts
├── styles/              # Design tokens and CSS
│   ├── tokens.css
│   └── global.css
└── index.ts             # Public exports
```

---

## Form Components

### Field Wrapper

`Field` wraps form inputs with label, hint, and error display:

```svelte
<script lang="ts">
  import { Field, TextInput } from "@decodelabs/underlay";

  export let form; // ActionData from SvelteKit
</script>

<Field
  label="Email"
  forId="email"
  hint="We'll never share your email"
  error={form?.errors?.email}
  required
>
  <TextInput
    id="email"
    name="email"
    type="email"
    value={form?.values?.email ?? ""}
    required
  />
</Field>
```

**Props:**
- `label` - Field label (optional)
- `forId` - Links label to input
- `hint` - Help text shown next to label
- `error` - Error message shown below input
- `required` - Shows a red asterisk (*) indicator next to the label (default: `false`)
- `span` - Grid column span: `"full"` spans all columns, number (1-6) specifies exact span
- `wide` - Remove max-width constraint for wide content like editors (default: `false`)

### TextInput

Standard text input with consistent styling:

```svelte
<script>
  import { Field, TextInput } from "@decodelabs/underlay";
</script>

<Field label="Username" forId="username">
  <TextInput
    id="username"
    name="username"
    value=""
    placeholder="Enter username"
    required
    autocomplete="username"
  />
</Field>
```

**Props:**
- All standard `<input>` attributes
- `oninput` - Callback fired on every keystroke with the current value
- `onchange` - Callback fired on blur (or after debounce delay if `debounce` is set)
- `debounce` - Optional delay in milliseconds; when set, `onchange` fires after delay instead of on blur
- `search` - When true, uses `type="search"` with native clear button (default: `false`)
- `className` - Additional CSS classes

#### Search Input for Filters

Use the `search` prop for filter fields to get a native clear button. Combine with `debounce` for a complete filter input experience.

```svelte
<Field label="Search" forId="search">
  <TextInput
    id="search"
    value={searchTerm}
    onchange={handleSearch}
    search
    debounce={500}
    placeholder="Search..."
  />
</Field>
```

The native clear button (×) appears when the input has content. Clicking it clears the input and immediately fires `onchange` with an empty value, even if debounce is enabled.

#### Debounced Input

Use the `debounce` prop for filter/search fields that trigger server-side queries. This prevents excessive API calls as users type.

```svelte
<script>
  import { Field, TextInput } from "@decodelabs/underlay/components";

  function handleSearch(value: string) {
    // Called after user stops typing for 500ms
    console.log("Searching for:", value);
  }
</script>

<Field label="Search" forId="search">
  <TextInput
    id="search"
    value=""
    onchange={handleSearch}
    debounce={500}
    placeholder="Search..."
  />
</Field>

<!-- With custom debounce delay (1 second) -->
<Field label="Filter" forId="filter">
  <TextInput
    id="filter"
    value={filterValue}
    onchange={handleFilter}
    debounce={1000}
    placeholder="Filter by name..."
  />
</Field>
```

**Debounce behavior:**
- When `debounce` is set, `onchange` fires after the delay instead of on blur
- As the user types, the timer resets with each keystroke
- Once the user stops typing for the debounce duration, `onchange` fires
- The `value` prop updates immediately (for controlled input display)

**Use cases for debounce:**
- Filter inputs that trigger server-side queries
- Search boxes with live results
- Any text input that triggers expensive operations

### TextArea

Multi-line text input:

```svelte
<Field label="Description" forId="description">
  <TextArea
    id="description"
    name="description"
    rows={4}
    placeholder="Enter description..."
  />
</Field>
```

### Select

Dropdown select component:

```svelte
<script>
  import { Field, Select } from "@decodelabs/underlay";
  
  const options = [
    { value: "active", label: "Active" },
    { value: "inactive", label: "Inactive" },
    { value: "pending", label: "Pending" }
  ];
  
  let selected = "active";
</script>

<Field label="Status" forId="status">
  <Select
    id="status"
    name="status"
    {options}
    bind:value={selected}
  />
</Field>
```

**Props:**
- `items` - Array of `{ value, label }` objects
- `value` - Selected value (bindable)
- `placeholder` - Placeholder text
- `disabled` - Disable select
- `clearable` - Show a clear button (×) to reset to default value (default: `false`)
- `defaultValue` - Value to reset to when cleared (default: `""`)

#### Clearable Select

Use `clearable` for filter dropdowns where users should be able to reset to a default state:

```svelte
<script>
  import { Field, Select } from "@decodelabs/underlay/components";

  const yearOptions = [
    { value: "All", label: "All years" },
    { value: "2024", label: "2024" },
    { value: "2023", label: "2023" }
  ];

  let selectedYear = "All";
</script>

<Field label="Year" forId="year">
  <Select
    id="year"
    items={yearOptions}
    bind:value={selectedYear}
    placeholder="All years"
    clearable
    defaultValue="All"
  />
</Field>
```

The clear button appears when a non-default value is selected. Clicking it resets to `defaultValue` and fires `onchange`.

### Switch

Toggle switch (checkbox alternative):

```svelte
<script>
  import { Field, Switch } from "@decodelabs/underlay";
  
  let enabled = false;
</script>

<Field label="Enable notifications">
  <Switch
    name="notifications"
    bind:checked={enabled}
  />
</Field>
```

**Props:**
- `checked` - Boolean value (bindable)
- `name` - Form field name
- `leftLabel` - Label for off state (default: `"Off"`)
- `rightLabel` - Label for on state (default: `"On"`)
- `leftVariant` - Color variant for off state: `"default"` | `"primary"` | `"success"` | `"warning"` | `"danger"` (default: `"default"`)
- `rightVariant` - Color variant for on state: `"default"` | `"primary"` | `"success"` | `"warning"` | `"danger"` (default: `"primary"`)
- `variant` - Legacy prop: `"default"` | `"danger-off"` (deprecated, use leftVariant/rightVariant instead)
- `disabled` - Disable switch

**Color Variants:**

Use `leftVariant` and `rightVariant` to set semantic colors for each state:

- `default` - Grey/muted (standard inactive appearance)
- `primary` - Blue (default for "on" state)
- `success` - Green (positive states like "Live", "Free", "Enabled")
- `warning` - Orange (caution states like "Restricted")
- `danger` - Red (warning states like "Draft", "Disabled")

#### Visibility Fields Pattern

For `isLive` or visibility toggle fields, use consistent styling across the application:

```svelte
<Field label="Visibility">
  <Switch
    name="isLive"
    leftLabel="Draft"
    rightLabel="Live"
    bind:checked={isLive}
    leftVariant="danger"
    rightVariant="success"
  />
</Field>
```

**Convention:**
- **Label**: Use "Visibility" (not "Is Live" or "Status")
- **Left label**: "Draft" (off state, red)
- **Right label**: "Live" (on state, green)

#### Access Fields Pattern

For `isFree` or access control fields:

```svelte
<Field label="Access">
  <Switch
    name="isFree"
    leftLabel="Restricted"
    rightLabel="Free"
    bind:checked={isFree}
    leftVariant="warning"
    rightVariant="success"
  />
</Field>
```

**Convention:**
- **Label**: Use "Access"
- **Left label**: "Restricted" (off state, orange)
- **Right label**: "Free" (on state, green)

These patterns apply to all content that can be published or access-controlled: pathways, modules, sections, areas, outcomes, activities, documents, Q&A items, summaries, videos, and audio items.

---

## UI Primitives

### Badge

Status indicators, counts, and labels:

```svelte
<script>
  import { Badge } from "@decodelabs/underlay";
</script>

<!-- Variants -->
<Badge>Default</Badge>
<Badge variant="success">Active</Badge>
<Badge variant="warning">Pending</Badge>
<Badge variant="danger">Error</Badge>
<Badge variant="info">New</Badge>
<Badge variant="muted">Archived</Badge>

<!-- Sizes -->
<Badge size="sm">Small</Badge>
<Badge size="md">Medium</Badge>
<Badge size="lg">Large</Badge>

<!-- Pill shape -->
<Badge pill>Pill Badge</Badge>

<!-- With icon -->
<Badge variant="success" icon="✓">Complete</Badge>
```

**Props:**
- `variant` - `"default"` | `"success"` | `"warning"` | `"danger"` | `"info"` | `"muted"` (default: `"default"`)
- `size` - `"sm"` | `"md"` | `"lg"` (default: `"md"`)
- `pill` - Fully rounded shape (default: `false`)
- `icon` - Optional icon to display before text
- `className` - Additional CSS classes

### Pill

Compact inline labels for metadata like type, category, year, or code. Unlike Badge, Pill supports custom accent colors via `color-mix()`.

```svelte
<script>
  import { Pill } from "@decodelabs/underlay";
</script>

<!-- Neutral (default) - gray/slate colored -->
<Pill>2024</Pill>
<Pill>FA1</Pill>
<Pill>Module Code</Pill>

<!-- With accent color - uses color-mix for bg/border/text -->
<Pill accent="#14b8a6">Outcome</Pill>
<Pill accent="#a855f7">Bundle</Pill>
<Pill accent="#fb923c">PreSeen</Pill>
<Pill accent="#60a5fa">Analysis</Pill>
```

**Props:**
- `accent` - Optional hex color for accent styling (uses `color-mix()` for background, border, and text)
- `className` - Additional CSS classes

**When to use Pill vs Badge:**
- Use **Pill** for inline metadata labels within content (year, code, type, category)
- Use **Badge** for status indicators with semantic variants (success, warning, danger)

**Styling Notes:**
- Uses `em` units for font-size (0.7em) so it scales with container
- Neutral pills use a muted slate color scheme
- Accent pills derive background (18% mix), border (30% mix), and text (88% mix) from the accent color

### ProgressBar

Visual progress indicator for task completion, upload progress, or metrics:

```svelte
<script>
  import { ProgressBar } from "@decodelabs/underlay/components";
</script>

<!-- Basic usage -->
<ProgressBar value={50} />
<ProgressBar value={75} max={100} />

<!-- Variants -->
<ProgressBar value={100} variant="success" />
<ProgressBar value={50} variant="warning" />
<ProgressBar value={25} variant="danger" />
<ProgressBar value={80} variant="info" />

<!-- Sizes -->
<ProgressBar value={60} size="sm" />
<ProgressBar value={60} size="md" />
<ProgressBar value={60} size="lg" />

<!-- With label -->
<ProgressBar value={75} showLabel />

<!-- Animated stripes -->
<ProgressBar value={50} animated />

<!-- Custom label format -->
<ProgressBar
  value={3}
  max={10}
  showLabel
  formatLabel={(value, max) => `${value} of ${max} tasks`}
/>

<!-- Custom label snippet -->
<ProgressBar value={75}>
  {#snippet label({ value, percentage })}
    <strong>{percentage.toFixed(0)}%</strong> complete
  {/snippet}
</ProgressBar>
```

**Props:**
- `value` - Current progress value (required)
- `max` - Maximum value (default: `100`)
- `variant` - `"default"` | `"success"` | `"warning"` | `"danger"` | `"info"` (default: `"default"`)
- `size` - `"sm"` | `"md"` | `"lg"` (default: `"md"`)
- `showLabel` - Show percentage label (default: `false`)
- `animated` - Enable striped animation (default: `false`)
- `formatLabel` - Custom label format function `(value, max, percentage) => string`
- `label` - Snippet for fully custom label content (receives `{ value, max, percentage }`)
- `className` - Additional CSS classes

**CSS Variables:**
- `--underlay-color-progress-track` - Track background color
- `--underlay-color-progress-default` - Default fill color
- `--underlay-color-progress-success` - Success fill color
- `--underlay-color-progress-warning` - Warning fill color
- `--underlay-color-progress-danger` - Danger fill color
- `--underlay-color-progress-info` - Info fill color

### Breadcrumbs

Navigation breadcrumb trail:

```svelte
<script>
  import { Breadcrumbs } from "@decodelabs/underlay";
  
  const items = [
    { label: "Home", href: "/" },
    { label: "Products", href: "/products" },
    { label: "Widgets", href: "/products/widgets" },
    { label: "Widget Pro" }  // Current page (no href)
  ];
</script>

<Breadcrumbs {items} />

<!-- With custom separator -->
<Breadcrumbs {items} separator="/" />

<!-- With icons -->
<Breadcrumbs items={[
  { label: "Home", href: "/", icon: "🏠" },
  { label: "Settings", href: "/settings", icon: "⚙️" },
  { label: "Profile" }
]} />
```

**Props:**
- `items` - Array of `{ label, href?, icon? }` objects
- `separator` - Separator character (default: `"›"`)
- `collapseOnMobile` - Collapse middle items on small screens (default: `true`)
- `maxItems` - Max items before collapsing (default: `4`)
- `className` - Additional CSS classes

**Accessibility:**
- Uses `<nav>` with `aria-label="Breadcrumb"`
- Current page marked with `aria-current="page"`

### Pagination

Standalone pagination component for navigating pages:

```svelte
<script>
  import { Pagination } from "@decodelabs/underlay";
  import { goto } from "$app/navigation";
  
  export let data;
  
  let page = 1;
  let limit = 20;
  
  function handlePageChange(event) {
    page = event.detail;
    goto(`?page=${page}&limit=${limit}`);
  }
  
  function handleLimitChange(event) {
    limit = event.detail;
    page = 1;  // Reset to first page
    goto(`?page=1&limit=${limit}`);
  }
</script>

<Pagination
  {page}
  {limit}
  total={data.total}
  on:page={handlePageChange}
/>

<!-- With limit selector -->
<Pagination
  {page}
  {limit}
  total={data.total}
  showLimitSelector
  limitOptions={[10, 25, 50, 100]}
  on:page={handlePageChange}
  on:limit={handleLimitChange}
/>

<!-- Compact mode -->
<Pagination
  {page}
  {limit}
  total={data.total}
  compact
  on:page={handlePageChange}
/>
```

**Props:**
- `page` - Current page (1-based)
- `limit` - Items per page
- `total` - Total number of items
- `showLimitSelector` - Show items-per-page dropdown (default: `false`)
- `limitOptions` - Available limit choices (default: `[10, 20, 50, 100]`)
- `showInfo` - Show "Showing X to Y of Z" text (default: `true`)
- `compact` - Smaller padding, hide info on mobile (default: `false`)
- `className` - Additional CSS classes

**Events:**
- `on:page` - Fired with new page number
- `on:limit` - Fired with new limit value

**Note:** For data tables, use the built-in pagination in `DataTable` component. This standalone `Pagination` component is for non-table contexts like card grids, galleries, or custom list layouts.

### OrderBy

Multi-field sorting component with drag-and-drop reordering. Use this for list views that need configurable sort order.

> **Important**: Filters and sorting should be implemented **server-side** for production use. Client-side filtering/sorting won't scale with pagination, large datasets, or complex queries. Use the OrderBy component to build the UI, but send sort parameters to your API and let the server handle the actual sorting. See "Server-Side Sorting" below for the recommended pattern.

#### Overview

The OrderBy component provides a popover-based UI for configuring multi-field sorting. Users can:

- Add sort fields from a dropdown
- Reorder fields by dragging
- Toggle sort direction (ascending/descending) per field
- Remove individual fields or clear all
- Reverse all directions at once

```
┌─────────────────────────────────────┐
│ [≡] Title                    ↑  [×] │  ← Drag handle, label, direction, remove
│ [≡] Created                  ↓  [×] │
├─────────────────────────────────────┤
│ [+ Add field ▾]                     │  ← Dropdown to add more fields
├─────────────────────────────────────┤
│ [↕ Reverse All]        [Clear]      │  ← Global actions
└─────────────────────────────────────┘
```

#### Basic Usage

```svelte
<script>
  import { OrderBy, type OrderByValue } from "@decodelabs/underlay/components";

  const fields = [
    { key: "title", label: "Title" },
    { key: "createdAt", label: "Created", defaultDirection: "desc" },
    { key: "updatedAt", label: "Updated", defaultDirection: "desc" },
    { key: "status", label: "Status" }
  ];

  let orderBy: OrderByValue = $state([
    { key: "createdAt", direction: "desc" }
  ]);
</script>

<OrderBy {fields} bind:value={orderBy} />
```

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `fields` | `OrderByFieldDefinition[]` | required | Available fields that can be sorted |
| `value` | `OrderByValue` | `[]` | Current sort order (bindable) |
| `onChange` | `(value: OrderByValue) => void` | - | Callback when sort order changes |
| `maxFields` | `number` | unlimited | Maximum number of sort fields allowed |
| `compact` | `boolean` | `false` | Truncate trigger text in tight spaces |
| `class` | `string` | - | Additional CSS class for trigger |

#### Types

```typescript
interface OrderByFieldDefinition {
  key: string;           // Unique field identifier (matches data property name)
  label: string;         // Display label shown in UI
  defaultDirection?: "asc" | "desc";  // Direction when field is first added (default: "asc")
}

interface OrderByField {
  key: string;           // Field key from definition
  direction: "asc" | "desc";  // Current sort direction
}

type OrderByValue = OrderByField[];  // Ordered array of active sort fields
```

#### Trigger Button Display

The trigger button shows a summary of the current sort order:

| State | Display |
|-------|---------|
| No fields selected | "Sort by..." (placeholder style) |
| Single field | "Title ↑" |
| Multiple fields | "Title ↑, Created ↓" |
| Compact mode (3+ fields) | "Title ↑, Created ↓ +2" |

#### Usage in FilterBar

The most common pattern is placing OrderBy inside a FilterBar alongside other filters:

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import { FilterBar } from "@decodelabs/underlay/patterns";
  import {
    Field,
    ListGrid,
    OrderBy,
    Select,
    type OrderByValue
  } from "@decodelabs/underlay/components";
  import { initPageState } from "@decodelabs/underlay/client";

  interface Props {
    data: {
      items: Array<{
        id: string;
        title: string;
        category: string;
        createdAt: string;
      }>;
      categories: Array<{ id: string; name: string }>;
    };
  }

  let { data }: Props = $props();

  // Filter and sort state
  let selectedCategory = $state("All");
  let orderBy: OrderByValue = $state([]);

  // Define sortable fields (keys must match data property names)
  const sortFields = [
    { key: "title", label: "Title" },
    { key: "category", label: "Category" },
    { key: "createdAt", label: "Created", defaultDirection: "desc" }
  ];

  // Build category filter options
  const categoryItems = $derived([
    { value: "All", label: "All categories" },
    ...data.categories.map((c) => ({ value: c.id, label: c.name }))
  ]);

  // Filter and sort data
  const displayItems = $derived(() => {
    // Apply filters
    let result = data.items.filter((item) =>
      selectedCategory === "All" || item.category === selectedCategory
    );

    // Apply sorting
    if (orderBy.length > 0) {
      result = [...result].sort((a, b) => {
        for (const { key, direction } of orderBy) {
          const aVal = String(a[key as keyof typeof a] ?? "").toLowerCase();
          const bVal = String(b[key as keyof typeof b] ?? "").toLowerCase();
          const cmp = aVal.localeCompare(bVal);
          if (cmp !== 0) {
            return direction === "asc" ? cmp : -cmp;
          }
        }
        return 0;
      });
    }

    return result;
  });

  // Restore state on back navigation
  onMount(() => {
    const restored = initPageState({
      selectedCategory: "All",
      orderBy: []
    });
    selectedCategory = restored.selectedCategory;
    orderBy = restored.orderBy;
  });
</script>

<FilterBar title="Filters">
  <Field label="Category" forId="category">
    <Select
      id="category"
      bind:value={selectedCategory}
      items={categoryItems}
      placeholder="All categories"
    />
  </Field>
  <Field label="Sort" forId="sort">
    <OrderBy fields={sortFields} bind:value={orderBy} />
  </Field>
</FilterBar>

<ListGrid>
  {#each displayItems() as item}
    <ItemCard {item} />
  {/each}
</ListGrid>
```

#### Client-Side Sorting Implementation

The sorting logic handles multiple fields with different directions:

```typescript
function sortItems<T extends Record<string, unknown>>(
  items: T[],
  orderBy: OrderByValue
): T[] {
  if (orderBy.length === 0) return items;

  return [...items].sort((a, b) => {
    for (const { key, direction } of orderBy) {
      const aVal = String(a[key] ?? "").toLowerCase();
      const bVal = String(b[key] ?? "").toLowerCase();
      const cmp = aVal.localeCompare(bVal);
      if (cmp !== 0) {
        return direction === "asc" ? cmp : -cmp;
      }
    }
    return 0;
  });
}
```

**Sorting notes:**

- Fields are compared in order (first field is primary sort, second is secondary, etc.)
- String comparison uses `localeCompare()` for proper alphabetical ordering
- Values are converted to lowercase for case-insensitive sorting
- Null/undefined values are treated as empty strings

For numeric or date fields, use appropriate comparison:

```typescript
// Numeric comparison
const aVal = Number(a[key] ?? 0);
const bVal = Number(b[key] ?? 0);
const cmp = aVal - bVal;

// Date comparison
const aVal = new Date(a[key] as string).getTime();
const bVal = new Date(b[key] as string).getTime();
const cmp = aVal - bVal;
```

#### State Persistence

Use `initPageState` to restore sort order when users navigate back:

```svelte
<script>
  import { onMount } from "svelte";
  import { gotoWithContext, initPageState } from "@decodelabs/underlay/client";

  let orderBy: OrderByValue = $state([]);

  onMount(() => {
    const restored = initPageState({ orderBy: [] });
    orderBy = restored.orderBy;
  });

  // When navigating away, include state
  function navigateToDetail(id: string) {
    gotoWithContext(`/items/${id}`, {
      label: "Items",
      href: "/items",
      type: "list",
      state: { orderBy }
    });
  }
</script>
```

#### URL Parameter Sync

For shareable/bookmarkable sort states, sync with URL parameters:

```svelte
<script>
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { OrderBy, type OrderByValue } from "@decodelabs/underlay/components";

  const fields = [
    { key: "title", label: "Title" },
    { key: "createdAt", label: "Created", defaultDirection: "desc" }
  ];

  // Parse sort from URL: ?sort=title:asc,createdAt:desc
  let orderBy: OrderByValue = $derived(
    parseOrderByFromUrl($page.url.searchParams.get("sort"))
  );

  function handleOrderChange(value: OrderByValue) {
    const url = new URL($page.url);
    if (value.length > 0) {
      url.searchParams.set("sort", serializeOrderBy(value));
    } else {
      url.searchParams.delete("sort");
    }
    goto(url.toString(), { replaceState: true });
  }

  function parseOrderByFromUrl(param: string | null): OrderByValue {
    if (!param) return [];
    return param.split(",").map((part) => {
      const [key, dir] = part.split(":");
      return { key, direction: dir === "desc" ? "desc" : "asc" };
    });
  }

  function serializeOrderBy(value: OrderByValue): string {
    return value.map((f) => `${f.key}:${f.direction}`).join(",");
  }
</script>

<OrderBy {fields} value={orderBy} onChange={handleOrderChange} />
```

#### Limiting Sort Fields

Use `maxFields` to restrict the number of sort fields:

```svelte
<!-- Allow only single-field sorting -->
<OrderBy fields={sortFields} bind:value={orderBy} maxFields={1} />

<!-- Allow up to 3 sort fields -->
<OrderBy fields={sortFields} bind:value={orderBy} maxFields={3} />
```

When the limit is reached, the "Add field" dropdown is hidden.

#### Compact Mode

Use `compact` for tight spaces (e.g., mobile layouts):

```svelte
<OrderBy fields={sortFields} bind:value={orderBy} compact />
```

In compact mode, the trigger text truncates after 2 fields: "Title ↑, Created ↓ +2"

#### Accessibility

The OrderBy component includes:

- **Keyboard navigation** - Tab through controls, Enter/Space to activate
- **ARIA labels** - Descriptive labels for screen readers ("Toggle direction to descending", "Remove Title from sort")
- **Focus management** - Focus returns to trigger when popover closes

#### Styling

The component uses BEM class names with `underlay-order-by` prefix:

| Class | Element |
|-------|---------|
| `.underlay-order-by-trigger` | Trigger button |
| `.underlay-order-by-content` | Popover content container |
| `.underlay-order-by-list` | Sortable field list |
| `.underlay-order-by-item` | Individual field row |
| `.underlay-order-by-item__handle` | Drag handle (⠿) |
| `.underlay-order-by-item__label` | Field label text |
| `.underlay-order-by-item__direction` | Direction toggle button |
| `.underlay-order-by-item__remove` | Remove button |
| `.underlay-order-by-add` | Add field section |
| `.underlay-order-by-actions` | Footer actions (Reverse All, Clear) |
| `.underlay-order-by-empty` | Empty state message |

Override styles using CSS custom properties or global class selectors.

### Button

```svelte
<script>
  import { Button } from "@decodelabs/underlay/components";
</script>

<!-- Primary button (blue) -->
<Button variant="primary" type="submit">
  Save Changes
</Button>

<!-- Secondary button (orange) -->
<Button variant="secondary" onclick={handleCancel}>
  Cancel
</Button>

<!-- Subtle/ghost button (muted background) -->
<Button variant="subtle" onclick={handleReset}>
  Reset
</Button>

<!-- Danger button (red) - for destructive or cancel actions -->
<Button variant="danger" onclick={handleDelete}>
  Delete
</Button>

<!-- Square (non-pill) button -->
<Button pill={false}>
  Non-rounded
</Button>
```

**Props:**
- `variant` - `"primary"` | `"secondary"` | `"subtle"` | `"danger"` (default: `"primary"`)
- `type` - `"button"` | `"submit"` | `"reset"` (default: `"button"`)
- `pill` - Rounded corners (default: `true`)
- `disabled` - Disable button
- `class` - Additional CSS classes

**Variant colors:**
- `primary` - Blue (#2563eb) - main actions
- `secondary` - Orange (#ea580c) - alternative actions
- `subtle` - Muted background - low-emphasis actions
- `danger` - Red (#dc2626) - destructive or cancel actions

### Code

Inline code styling component for displaying technical values like IDs, slugs, MIME types, and other monospace content.

```svelte
<script>
  import { Code } from "@decodelabs/underlay/components";
</script>

<!-- Display an ID -->
<p><strong>ID:</strong> <Code>{item.id}</Code></p>

<!-- Display a MIME type -->
<p>Type: <Code>{file.mimeType}</Code></p>

<!-- Display a slug -->
<p>Slug: <Code>{page.slug}</Code></p>
```

**Props:**
- `class` - Additional CSS classes
- All standard HTML `<code>` element attributes

**Styling:**
- Monospace font family
- Smaller font size (0.8em)
- Subtle background and border
- Small border radius

The component provides consistent inline code styling across the application, replacing raw `<code>` elements with properly styled output that matches the design system.

### Card

Container component with consistent styling:

```svelte
<script>
  import { Card } from "@decodelabs/underlay";
</script>

<Card>
  <h2>Card Title</h2>
  <p>Card content goes here.</p>
</Card>

<!-- With custom class -->
<Card className="custom-card">
  <slot />
</Card>
```

### ContentCard

Display card for rendering rich text content with a title. Automatically detects the content type and renders appropriately:
- **NightfireValue objects** are rendered using the NightfireRenderer
- **HTML strings** are rendered directly (for pre-rendered HTML)
- **Markdown strings** are parsed and rendered when `markdown` prop is set

Shows a subtle empty message when no content is set.

```svelte
<script>
  import { ContentCard } from "@decodelabs/underlay/components";
</script>

<!-- With Nightfire content -->
<ContentCard
  title="Description"
  value={data.description}
  emptyMessage="No description set."
/>

<!-- With HTML string -->
<ContentCard
  title="Notes"
  value={data.notesHtml}
  emptyMessage="No notes added."
/>

<!-- With Markdown string -->
<ContentCard
  title="Learning Aims"
  value={data.learningAims}
  markdown
  emptyMessage="No learning aims set."
/>

<!-- With custom max height (no collapse) -->
<ContentCard
  title="Full Content"
  value={data.content}
  maxHeight={0}
/>

<!-- Scroll instead of reveal toggle -->
<ContentCard
  title="Stitched Preview"
  value={data.stitchedMarkdown}
  markdown
  maxHeight="10em"
  overflowBehavior="scroll"
/>
```

**Props:**
- `title` - Card heading text (required)
- `value` - NightfireValue object, HTML string, or Markdown string to render (optional)
- `markdown` - When true, string values are parsed as Markdown (default: false)
- `emptyMessage` - Message shown when value is empty (default: "No content set.")
- `maxHeight` - Max height when constrained. Number values are interpreted as pixels; strings can use CSS units like `"10em"` (default: `200`)
- `overflowBehavior` - Overflow mode when `maxHeight` is set: `"reveal"` (default, show more/less toggle) or `"scroll"` (fixed height with internal scroll)

**Features:**
- Auto-detects content type (object = Nightfire, string = HTML/Markdown)
- Markdown parsing via `marked` when `markdown` prop is set
- Collapsible reveal mode with "Show more/less" toggle when `overflowBehavior="reveal"`
- Scroll mode with fixed-height content when `overflowBehavior="scroll"`
- Gradient fade effect on collapsed content for visual indication
- Max-width of 65ch for comfortable reading
- Card styling with subtle background and border
- Uppercase legend-style title
- Automatic empty state handling with italic muted message
- Built-in styling for common Markdown elements (headings, lists, code, blockquotes)
- Proper paragraph margin handling within rendered content

**Note:** When rendering HTML strings without the `markdown` prop, only use with trusted/sanitized content.

### DetailList

Compact horizontal key-value list for displaying related information. Each item shows a label on the left and value on the right. Use within a `Card` for visual grouping.

```svelte
<script>
  import { Card, DetailList, DetailItem } from "@decodelabs/underlay/components";
</script>

<Card>
  <DetailList title="Locale">
    <DetailItem label="Time Zone" value={profile.timeZone} />
    <DetailItem label="Language" value={profile.language} />
    <DetailItem label="Country" value={profile.countryCode} />
  </DetailList>
</Card>

<!-- Without title -->
<Card>
  <DetailList>
    <DetailItem label="Status" value="Active" />
    <DetailItem label="Created" value="Jan 30, 2026" />
  </DetailList>
</Card>

<!-- With boolean values (auto-formatted as Yes/No) -->
<DetailList title="Settings">
  <DetailItem label="Marketing Emails" value={user.emailMarketingOptIn} />
  <DetailItem label="Notifications" value={true} />
</DetailList>

<!-- With code formatting -->
<DetailList title="System">
  <DetailItem label="User ID" value={user.id} code />
  <DetailItem label="API Key" value={apiKey} code />
</DetailList>

<!-- With capitalize -->
<DetailList title="Preferences">
  <DetailItem label="Email Frequency" value={user.emailFrequency} capitalize />
</DetailList>

<!-- Custom content via children snippet -->
<DetailItem label="Status">
  <Badge variant="success">Active</Badge>
</DetailItem>
```

**DetailList Props:**
- `title` - Optional section title (uppercase, muted style)
- `class` - Additional CSS classes

**DetailItem Props:**
- `label` - The key/label text (required)
- `value` - Plain text, number, or boolean value (optional)
- `code` - Display value in monospace font (default: false)
- `capitalize` - Capitalize the value text (default: false)
- `class` - Additional CSS classes
- `children` - Snippet for custom content instead of plain value

**Features:**
- Horizontal layout with label left, value right
- Boolean values automatically formatted as "Yes"/"No"
- Empty values show "Not set" in muted style
- Compact 0.875rem font size
- Works well in Card grids for dashboard-style layouts

**When to use:**
- Account/profile overview pages
- Settings summaries
- Entity detail sidebars
- Dashboard stat cards

**Comparison with DetailsCard/DetailsItem:**
- `DetailList` is horizontal (label left, value right) - compact for simple key-value pairs
- `DetailsCard` is vertical (label above value) - better for longer values or grid layouts

### Dialog

Modal dialog component:

```svelte
<script>
  import { Dialog, Button } from "@decodelabs/underlay";
  
  let open = false;
</script>

<Button on:click={() => open = true}>
  Open Dialog
</Button>

<Dialog bind:open title="Confirm Action">
  <p>Are you sure you want to proceed?</p>
  
  <svelte:fragment slot="actions">
    <Button variant="secondary" on:click={() => open = false}>
      Cancel
    </Button>
    <Button variant="primary" on:click={handleConfirm}>
      Confirm
    </Button>
  </svelte:fragment>
</Dialog>
```

**Props:**
- `open` - Boolean controlling visibility (bindable)
- `title` - Dialog title

**Slots:**
- Default slot - Dialog content
- `actions` - Action buttons

### AlertDialog

Confirmation dialog with destructive action styling:

```svelte
<script>
  import { AlertDialog, Button } from "@decodelabs/underlay";
  
  let open = false;
</script>

<AlertDialog
  bind:open
  title="Delete Article"
  description="This action cannot be undone."
  confirmText="Delete"
  cancelText="Cancel"
  on:confirm={handleDelete}
>
  <p>Are you sure you want to delete this article?</p>
</AlertDialog>
```

### DropdownMenu

Dropdown menu component:

```svelte
<script>
  import { DropdownMenu, Button } from "@decodelabs/underlay";
</script>

<DropdownMenu>
  <Button slot="trigger" variant="secondary">
    Actions ▼
  </Button>
  
  <button on:click={handleEdit}>Edit</button>
  <button on:click={handleDuplicate}>Duplicate</button>
  <hr />
  <button class="danger" on:click={handleDelete}>Delete</button>
</DropdownMenu>
```

### Tooltip

Tooltip component for showing additional information on hover. Supports both icon triggers (default) and inline text triggers.

```svelte
<script>
  import { Tooltip } from "@decodelabs/underlay/components";
</script>

<!-- Default icon trigger (ⓘ) -->
<Tooltip content="This is helpful information">
  <!-- Uses default ⓘ icon -->
</Tooltip>

<!-- Custom trigger label -->
<Tooltip content="Click for more info" triggerLabel="?">
  <!-- Uses ? instead of ⓘ -->
</Tooltip>

<!-- Inline text trigger (for TimeAgo, definitions, etc.) -->
<Tooltip content="January 21, 2026 at 3:45 PM" inline>
  {#snippet trigger()}
    <span>3 days ago</span>
  {/snippet}
</Tooltip>

<!-- Custom snippet trigger -->
<Tooltip content="User profile settings">
  {#snippet trigger()}
    <button class="custom-trigger">⚙️ Settings</button>
  {/snippet}
</Tooltip>

<!-- Positioning -->
<Tooltip content="Appears below" side="bottom" />
<Tooltip content="Appears left" side="left" align="start" />
```

**Props:**
- `content` - Tooltip text content (required)
- `open` - Boolean controlling visibility (bindable)
- `showTrigger` - Show trigger element (default: `true`)
- `triggerLabel` - Label for default trigger (default: `"ⓘ"`)
- `side` - `"top"` | `"right"` | `"bottom"` | `"left"` (default: `"top"`)
- `sideOffset` - Distance from trigger in pixels (default: `6`)
- `align` - `"start"` | `"center"` | `"end"` (default: `"center"`)
- `alignOffset` - Alignment offset in pixels (default: `0`)
- `delayDuration` - Delay before showing in ms (default: `500`)
- `disabled` - Disable tooltip (default: `false`)
- `inline` - Use inline trigger styling for text content (default: `false`)
- `trigger` - Custom trigger snippet
- `class` - Additional CSS classes for trigger

**Snippets:**
- `trigger` - Custom content for the trigger element

**Inline Mode:**

Use `inline={true}` when the tooltip trigger is text within a sentence or paragraph. This mode:
- Inherits font size, color, and line-height from the parent
- Removes default trigger dimensions and background
- Sets `cursor: help` for the trigger
- Preserves text flow without disrupting layout

```svelte
<p>
  Updated <Tooltip content="January 21, 2026" inline>
    {#snippet trigger()}
      <span>3 days ago</span>
    {/snippet}
  </Tooltip> by admin.
</p>
```

**Styling:**

The tooltip uses these CSS classes:
- `.underlay-tooltip-trigger` - Default icon trigger styling
- `.underlay-tooltip-trigger--inline` - Inline trigger styling
- `.underlay-tooltip-content` - Tooltip popup container
- `.underlay-tooltip-arrow` - Arrow pointing to trigger

### TimeAgo

Displays a date as relative time (e.g., "3 days ago", "just now") with a tooltip showing the full date. Useful for timestamps, activity feeds, and metadata displays.

```svelte
<script>
  import { TimeAgo } from "@decodelabs/underlay/components";
</script>

<!-- Basic usage with Date object -->
<TimeAgo date={new Date()} />

<!-- With ISO string -->
<TimeAgo date="2026-01-18T10:30:00Z" />

<!-- Different tooltip formats -->
<TimeAgo date={item.createdAt} tooltipFormat="date" />
<TimeAgo date={item.createdAt} tooltipFormat="datetime" />
<TimeAgo date={item.createdAt} tooltipFormat="full" />

<!-- In context -->
<p>Created <TimeAgo date={article.createdAt} /></p>
<p>Last updated <TimeAgo date={article.updatedAt} /></p>
```

**Props:**
- `date` - Date to display. Accepts ISO string or Date object (required)
- `tooltipFormat` - Format for tooltip display (default: `"datetime"`)
  - `"date"` - "January 21, 2026"
  - `"datetime"` - "January 21, 2026 at 3:45 PM"
  - `"full"` - "January 21, 2026 at 3:45:30 PM EST"
- `class` - Additional CSS classes

**Relative Time Output:**

| Time Difference | Output |
|-----------------|--------|
| < 10 seconds | "just now" |
| < 60 seconds | "45 seconds ago" |
| 1 minute | "1 minute ago" |
| < 60 minutes | "23 minutes ago" |
| 1 hour | "1 hour ago" |
| < 24 hours | "5 hours ago" |
| 1 day | "yesterday" |
| < 7 days | "3 days ago" |
| 1 week | "1 week ago" |
| < 4 weeks | "2 weeks ago" |
| 1 month | "1 month ago" |
| < 12 months | "6 months ago" |
| 1 year | "1 year ago" |
| > 1 year | "2 years ago" |

**Future Dates:**

TimeAgo also handles future dates:
- "in a few seconds"
- "in 5 minutes"
- "in 2 hours"
- "in 3 days"

**Styling:**

The component renders as a `<time>` element with semantic `datetime` attribute. The text has a dotted underline to indicate interactivity:

```css
.time-ago {
  text-decoration: underline;
  text-decoration-style: dotted;
  text-underline-offset: 2px;
  text-decoration-color: var(--underlay-color-text-muted);
}

.time-ago:hover {
  text-decoration-color: var(--underlay-color-text);
}
```

**Usage in Tables/Lists:**

```svelte
<script>
  import { DataTable, TimeAgo } from "@decodelabs/underlay/components";
</script>

<DataTable
  data={articles}
  columns={[
    { key: "title", label: "Title" },
    {
      key: "createdAt",
      label: "Created",
      render: (row) => {
        return { component: TimeAgo, props: { date: row.createdAt } };
      }
    }
  ]}
/>
```

**With Page Headers:**

```svelte
<PageHeader title="Edit Article" subtitle={article.title}>
  <div class="meta">
    <span>Created: <TimeAgo date={article.createdAt} /></span>
    <span>Updated: <TimeAgo date={article.updatedAt} /></span>
  </div>
</PageHeader>
```

### DateRange

Displays a date range using adaptive formatting:
- Same month/year: `1st to 25th Feb 2025`
- Same year: `1st Feb to 25th Mar 2025`
- Different years: `1st Feb 2025 to 25th Mar 2026`

```svelte
<script>
  import { DateRange } from "@decodelabs/underlay/components";
</script>

<DateRange startDate="2025-02-01" endDate="2025-02-25" />
<DateRange startDate="2025-02-01" endDate="2025-03-25" />
<DateRange startDate="2025-12-01" endDate="2026-01-15" />
```

**Props:**
- `startDate` - Range start date (ISO string or `Date`)
- `endDate` - Range end date (ISO string or `Date`)
- `locale` - Locale for month names (default: `"en-GB"`)
- `style` - `"adaptive"` (default) or `"full"`
- `emptyText` - Text when start/end is missing or invalid (default: `"—"`)
- `class` - Additional CSS classes

You can also use the shared formatter for places that require plain text (e.g. `ListCard.subtitle`):

```ts
import { formatAdaptiveDateRange } from "@decodelabs/underlay/components";

const label = formatAdaptiveDateRange(startDate, endDate, { locale: "en-GB" }) ?? "No date window";
```

### DetailsGrid & DetailsItem

Grid layout for displaying key-value detail information in a visually appealing format. Use for detail pages, settings displays, and metadata sections.

```svelte
<script>
  import { DetailsGrid, DetailsItem, TimeAgo } from "@decodelabs/underlay/components";
</script>

<!-- Basic usage -->
<DetailsGrid>
  <DetailsItem label="Name" value="John Smith" />
  <DetailsItem label="Email" value="john@example.com" />
  <DetailsItem label="Role" value="Administrator" />
  <DetailsItem label="Status" value="Active" />
</DetailsGrid>

<!-- With code values and custom content -->
<DetailsGrid>
  <DetailsItem label="Slug" value="my-article" code />
  <DetailsItem label="ID" value="018f2a3b-3c4d-7e8f" code />
  <DetailsItem label="Created">
    <TimeAgo date={createdAt} />
  </DetailsItem>
  <DetailsItem label="Last Updated">
    <TimeAgo date={updatedAt} />
  </DetailsItem>
</DetailsGrid>

<!-- Spanning multiple columns -->
<DetailsGrid>
  <DetailsItem label="Title" value={article.title} span={2} />
  <DetailsItem label="Author" value={article.author} />
  <DetailsItem label="Category" value={article.category} />
  <DetailsItem label="Description" value={article.description} span="full" />
</DetailsGrid>
```

**DetailsGrid Props:**
- `columns` - Number of columns at full width: `2 | 3 | 4` (default: `4`)
- `minItemWidth` - Minimum item width before wrapping (default: `"14rem"`)
- `accent` - Optional accent color for styling
- `class` - Additional CSS classes

**DetailsItem Props:**
- `label` - The label/key for this detail item (required)
- `value` - Plain text or number value (use children snippet for complex content)
- `code` - Display value as monospace code (default: `false`)
- `span` - Column span: number (`1-4`) or `"full"` for entire row
- `muted` - Show value in muted/secondary style (default: `false`)
- `class` - Additional CSS classes

**Empty Values:**

When `value` is `null` or `undefined` (and no children provided), a muted dash is displayed:

```svelte
<DetailsItem label="Middle Name" value={user.middleName} />
<!-- Shows "—" if middleName is null -->
```

**Custom Content:**

Use the children snippet for complex content like badges, links, or custom components:

```svelte
<DetailsItem label="Status">
  <Badge variant="success">Active</Badge>
</DetailsItem>

<DetailsItem label="Website">
  <a href={website}>{website}</a>
</DetailsItem>

<DetailsItem label="Tags">
  {#each tags as tag}
    <Pill>{tag}</Pill>
  {/each}
</DetailsItem>
```

**Grid Layout:**

The grid uses CSS `auto-fill` with `minmax()` for responsive column counts:
- At wide widths: up to 4 columns (configurable)
- Items automatically wrap based on `minItemWidth`
- Single-pixel gap lines create a subtle grid effect
- Rounded corners with subtle shadow for visual depth

**Usage in Detail Pages:**

```svelte
<script lang="ts">
  import {
    DetailsGrid,
    DetailsItem,
    TimeAgo,
    Badge
  } from "@decodelabs/underlay/components";
  import { PageHeader } from "@decodelabs/underlay/patterns";

  let { data } = $props();
</script>

<PageHeader
  title={data.user.name}
  subtitle="User Details"
  backHref="/users"
  backLabel="Back to users"
/>

<DetailsGrid>
  <DetailsItem label="Email" value={data.user.email} />
  <DetailsItem label="Username" value={data.user.username} code />
  <DetailsItem label="Role" value={data.user.role} />
  <DetailsItem label="Status">
    <Badge variant={data.user.active ? "success" : "muted"}>
      {data.user.active ? "Active" : "Inactive"}
    </Badge>
  </DetailsItem>
  <DetailsItem label="Created">
    <TimeAgo date={data.user.createdAt} />
  </DetailsItem>
  <DetailsItem label="Last Login">
    <TimeAgo date={data.user.lastLoginAt} />
  </DetailsItem>
  <DetailsItem label="Bio" value={data.user.bio} span="full" />
</DetailsGrid>
```

**Styling:**

The component uses these CSS classes:
- `.details-grid` - Container with grid layout and border
- `.details-item` - Individual item cell
- `.details-item__label` - Uppercase muted label
- `.details-item__value` - Value text
- `.details-item__code` - Monospace code styling
- `.details-item__empty` - Muted dash for empty values

Items have a subtle hover effect for better scannability.

### ContainerGrid

A responsive two-column grid layout that uses CSS container queries to collapse to a single column when space is limited. Unlike media queries which respond to viewport width, container queries respond to the actual container width, making this component work correctly inside tabs, sidebars, or any constrained layout.

```svelte
<script>
  import { ContainerGrid, DetailsGrid, DetailsItem } from "@decodelabs/underlay/components";
  import { InlineListCard, InlineListItem } from "@decodelabs/underlay/components";
</script>

<!-- Basic usage: DetailsGrid alongside an InlineListCard -->
<ContainerGrid>
  <DetailsGrid>
    <DetailsItem label="Name" value={item.name} />
    <DetailsItem label="Status" value={item.status} />
  </DetailsGrid>

  <InlineListCard title="Related Items" hasItems={items.length > 0}>
    {#each items as item}
      <InlineListItem label={item.name} href={item.href} />
    {/each}
  </InlineListCard>
</ContainerGrid>

<!-- With custom gap -->
<ContainerGrid gap="2rem">
  <div>Left column</div>
  <div>Right column</div>
</ContainerGrid>

<!-- With additional class for margins -->
<ContainerGrid class="my-section">
  <div>Content</div>
  <div>Content</div>
</ContainerGrid>
```

**Props:**
- `breakpoint` - Container width at which to collapse to single column (default: `700`)
- `gap` - Gap between grid items (default: `"1.5rem"`)
- `stretch` - When `true`, items stretch to fill the row height (default: `false`)
- `class` - Additional CSS class for the wrapper (useful for margins)

**Behavior:**
- At container widths above `breakpoint`, displays as a two-column grid
- At container widths at or below `breakpoint`, collapses to single column
- Automatically removes `margin-top` and `max-width` from nested `DetailsGrid` and `InlineListCard` components
- Adds bottom margin between consecutive `ContainerGrid` components (removed on last child)

**Why Container Queries?**

Container queries allow the grid to respond to its own available space rather than the viewport width. This is essential for components that may be rendered inside:
- Tab panels that don't span the full viewport
- Sidebars or split layouts
- Modal dialogs
- Any constrained container

### InlineListCard

A compact card for displaying related items in a vertical list. Commonly used alongside `DetailsGrid` within a `ContainerGrid` to show associations like "Related Items" or "Assigned Users".

```svelte
<script>
  import { InlineListCard, InlineListItem, Button } from "@decodelabs/underlay/components";

  const items = [
    { id: "1", name: "Item One", href: "/items/1" },
    { id: "2", name: "Item Two", href: "/items/2" }
  ];
</script>

<!-- Basic usage -->
<InlineListCard title="Related Items" hasItems={items.length > 0}>
  {#each items as item}
    <InlineListItem label={item.name} href={item.href} />
  {/each}
</InlineListCard>

<!-- With action button -->
<InlineListCard
  title="Assigned Users"
  hasItems={users.length > 0}
  emptyMessage="No users assigned."
>
  {#snippet action()}
    <Button size="sm" variant="ghost" onclick={handleAdd}>Add</Button>
  {/snippet}

  {#each users as user}
    <InlineListItem label={user.name} href={`/users/${user.id}`} />
  {/each}
</InlineListCard>
```

**Props:**
- `title` - Card title displayed in uppercase
- `hasItems` - Whether the list has items (controls empty state display)
- `emptyMessage` - Message shown when there are no items (default: `"No items."`)
- `action` - Optional snippet for action button (typically "Add")
- `children` - List items to render (use `InlineListItem`)

### InlineListItem

A list item for use inside `InlineListCard`. Supports links, click handlers, accent colors, badges, and trailing content.

```svelte
<script>
  import { InlineListCard, InlineListItem, Pill } from "@decodelabs/underlay/components";
</script>

<InlineListCard title="Modules" hasItems={true}>
  <!-- Basic link item -->
  <InlineListItem label="Getting Started" href="/modules/getting-started" />

  <!-- With accent color (colored dot) -->
  <InlineListItem
    label="Advanced Topics"
    href="/modules/advanced"
    accent="#14b8a6"
  />

  <!-- With badge (inline after label) -->
  <InlineListItem label="FA1" href="/modules/fa1">
    {#snippet badge()}
      <Pill accent="#6b7280">2024</Pill>
    {/snippet}
  </InlineListItem>

  <!-- With trailing content (right-aligned) -->
  <InlineListItem label="Module ABC" href="/modules/abc">
    {#snippet trailing()}
      <Pill accent="#6366f1">after:10</Pill>
    {/snippet}
  </InlineListItem>

  <!-- With both badge and trailing -->
  <InlineListItem label="Complete Example" href="/items/1">
    {#snippet badge()}
      <Pill>Code</Pill>
    {/snippet}
    {#snippet trailing()}
      <Pill accent="#22c55e">Active</Pill>
    {/snippet}
  </InlineListItem>

  <!-- With delete button (appears on hover) -->
  <InlineListItem
    label="Deletable Item"
    href="/items/2"
    showDelete
    ondelete={() => handleDelete(item.id)}
  />

  <!-- Click handler instead of link -->
  <InlineListItem
    label="Clickable Item"
    onclick={() => console.log("clicked")}
  />
</InlineListCard>
```

**Props:**
- `label` - Primary text to display
- `href` - Optional link URL
- `onclick` - Optional click handler (used when no href)
- `accent` - Optional hex color for the indicator dot
- `badge` - Snippet for inline content immediately after the label
- `trailing` - Snippet for right-aligned content (badges, pills, etc.)
- `showDelete` - Whether to show delete button on hover (default: `false`)
- `ondelete` - Delete handler called when delete button is clicked

**Styling Notes:**
- The `badge` snippet renders inline next to the label, useful for codes or years
- The `trailing` snippet is pushed to the right with `margin-left: auto`
- When `showDelete` is enabled, trailing content shifts left on hover to make room for the delete button

### Tabs

A tabbed interface component with multiple visual variants. Supports URL query synchronization, responsive collapse to dropdown, and a dedicated `form` variant for large multi-section forms.

```svelte
<script>
  import { TabsRoot, TabsList, TabsTrigger, TabsContent } from "@decodelabs/underlay/components";

  let activeTab = $state("details");
</script>

<!-- Basic tabs with pills variant (default) -->
<TabsRoot bind:value={activeTab}>
  <TabsList>
    <TabsTrigger value="details">Details</TabsTrigger>
    <TabsTrigger value="settings">Settings</TabsTrigger>
  </TabsList>

  <TabsContent value="details">
    <p>Details content here</p>
  </TabsContent>
  <TabsContent value="settings">
    <p>Settings content here</p>
  </TabsContent>
</TabsRoot>

<!-- Underline variant -->
<TabsRoot bind:value={activeTab} variant="underline">
  <TabsList>
    <TabsTrigger value="overview">Overview</TabsTrigger>
    <TabsTrigger value="history">History</TabsTrigger>
  </TabsList>
  <!-- TabsContent sections... -->
</TabsRoot>

<!-- Boxed variant -->
<TabsRoot bind:value={activeTab} variant="boxed">
  <TabsList>
    <TabsTrigger value="code">Code</TabsTrigger>
    <TabsTrigger value="preview">Preview</TabsTrigger>
  </TabsList>
  <!-- TabsContent sections... -->
</TabsRoot>

<!-- With URL history synchronization -->
<TabsRoot bind:value={activeTab} historyKey="tab">
  <TabsList>
    <TabsTrigger value="details">Details</TabsTrigger>
    <TabsTrigger value="modules">Modules</TabsTrigger>
  </TabsList>
  <!-- Tab state syncs with ?tab=details or ?tab=modules -->
</TabsRoot>

<!-- With count badges -->
<TabsRoot bind:value={activeTab}>
  <TabsList>
    <TabsTrigger value="all">All</TabsTrigger>
    <TabsTrigger value="active" count={12}>Active</TabsTrigger>
    <TabsTrigger value="archived" count={3}>Archived</TabsTrigger>
  </TabsList>
</TabsRoot>

<!-- Collapsible tabs (collapse to dropdown on narrow screens) -->
<TabsRoot bind:value={activeTab}>
  <TabsList
    collapsible
    tabs={[
      { value: "details", label: "Details" },
      { value: "modules", label: "Modules", count: 5 },
      { value: "settings", label: "Settings" }
    ]}
  >
    <TabsTrigger value="details">Details</TabsTrigger>
    <TabsTrigger value="modules" count={5}>Modules</TabsTrigger>
    <TabsTrigger value="settings">Settings</TabsTrigger>
  </TabsList>
</TabsRoot>
```

**TabsRoot Props:**
- `value` - Current active tab value (bindable)
- `variant` - Visual style: `"pills"` (default), `"underline"`, `"boxed"`, `"plain"`, or `"form"`
- `size` - Size variant: `"default"` or `"sm"`
- `historyKey` - When provided, syncs tab state with URL query param (e.g., `historyKey="tab"` stores as `?tab=value`)

**TabsList Props:**
- `class` - Additional CSS class
- `collapsible` - Enable responsive collapse to dropdown (requires `tabs` prop)
- `tabs` - Array of `{ value, label, count? }` for collapsible mode

**TabsTrigger Props:**
- `value` - Tab value (must match `TabsContent` value)
- `disabled` - Disable the tab
- `count` - Optional count badge displayed after the label
- `class` - Additional CSS class

**Variant Styling:**
- **pills** - Rounded pill-shaped tabs with subtle background, contained in a rounded border
- **underline** - Minimal tabs with bottom border highlight on active tab
- **boxed** - Traditional raised tabs with background color
- **plain** - Minimal text tabs without underline border treatment
- **form** - Larger section-navigation tabs intended for long, multi-section forms

#### Form Tabs (Multi-Section Forms)

Use this pattern for long forms (2+ sections) where each section has its own validation state.

```svelte
<script lang="ts">
  import {
    Field,
    FieldSet,
    FormTabsProvider,
    FormTabsSection,
    FormValidationProvider,
    TabsRoot,
    TabsList,
    TabsTrigger,
    TabsContent,
    TextInput
  } from "@decodelabs/underlay/components";

  let activeTab = $state("details");
  let isFormValid = $state(false);
  let title = $state("");
  let notes = $state("");
</script>

<FormValidationProvider bind:isValid={isFormValid}>
  <FormTabsProvider>
    <TabsRoot bind:value={activeTab} variant="form">
      <TabsList
        collapsible
        tabs={[
          { value: "details", label: "Details" },
          { value: "notes", label: "Notes" }
        ]}
      >
        <TabsTrigger value="details">Details</TabsTrigger>
        <TabsTrigger value="notes">Notes</TabsTrigger>
      </TabsList>

      <TabsContent value="details">
        <FormTabsSection sectionId="details">
          <div class="underlay-form-grid">
            <FieldSet legend="Core">
              <Field label="Title" required>
                <TextInput name="title" bind:value={title} required />
              </Field>
            </FieldSet>
          </div>
        </FormTabsSection>
      </TabsContent>

      <TabsContent value="notes">
        <FormTabsSection sectionId="notes">
          <div class="underlay-form-grid">
            <FieldSet legend="Notes">
              <Field label="Notes">
                <TextInput name="notes" bind:value={notes} />
              </Field>
            </FieldSet>
          </div>
        </FormTabsSection>
      </TabsContent>
    </TabsRoot>
  </FormTabsProvider>
</FormValidationProvider>
```

**Form tabs components:**
- `FormTabsProvider` - Creates a section registry and wires tab state to form validation state
- `FormTabsSection sectionId="..."` - Assigns enclosed fields to a tab section (required for validation dots)

**How section validation indicators work:**
- `invalid` - At least one field in the section has validation errors
- `incomplete` - A required field has no value
- `valid` - No errors and required fields are filled
- `idle` - No registered fields yet

Those states automatically appear as dots on `TabsTrigger` and in collapsed dropdown tabs.

**Important requirements:**
- Wrap form tabs in `FormValidationProvider` first
- Wrap tabbed sections in `FormTabsProvider`
- Put each tab panel's form controls inside a matching `FormTabsSection`
- Keep `TabsTrigger.value`, `TabsContent.value`, and `FormTabsSection.sectionId` aligned by section

**Editor compatibility note:**
- The `form` tab variant keeps inactive panels mounted (hidden with height/visibility, not `display: none`) so editors like CodeMirror/EasyMDE don't break when switching tabs

---

## Pattern Components

### Form

Form wrapper with enhanced submission handling:

```svelte
<script>
  import { Form, Field, TextInput, FormActions, Button } from "@decodelabs/underlay";
  import { enhance } from "$app/forms";
  import type { ActionData } from "./$types";
  
  export let form: ActionData;
  
  let loading = false;
</script>

<Form method="POST" use:enhance={() => {
  loading = true;
  return async ({ update }) => {
    await update();
    loading = false;
  };
}}>
  <Field label="Title" forId="title" error={form?.errors?.title}>
    <TextInput
      id="title"
      name="title"
      value={form?.values?.title ?? ""}
      required
      disabled={loading}
    />
  </Field>
  
  <Field label="Content" forId="content" error={form?.errors?.content}>
    <TextArea
      id="content"
      name="content"
      value={form?.values?.content ?? ""}
      rows={6}
      disabled={loading}
    />
  </Field>
  
  <FormActions>
    <Button type="submit" disabled={loading}>
      {loading ? "Saving..." : "Save"}
    </Button>
    <Button variant="secondary" type="button" on:click={() => history.back()}>
      Cancel
    </Button>
  </FormActions>
</Form>
```

### FormActions

Container for form action buttons with consistent spacing. Supports a right-aligned `danger` slot for secondary actions:

```svelte
<script>
  import { FormActions, Button, TextButton } from "@decodelabs/underlay/components";
</script>

<!-- Basic usage -->
<FormActions>
  <Button type="submit">Save</Button>
</FormActions>

<!-- With right-aligned secondary action -->
<FormActions>
  <Button type="submit" variant="primary">Save Changes</Button>
  {#snippet danger()}
    <TextButton onclick={handleCancel}>Cancel</TextButton>
  {/snippet}
</FormActions>
```

**Props:**

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `align` | `"start" \| "end"` | `"start"` | Horizontal alignment of primary buttons |
| `dangerItems` | `DangerMenuItem[]` | - | Menu items for collapsed menu on small screens |

**Slots:**

| Slot | Purpose |
|------|---------|
| `children` | Primary action buttons (left-aligned by default) |
| `danger` | Right-aligned actions (Cancel, Delete, etc.). Collapses to dropdown menu on small screens when `dangerItems` is provided. |

### ListCard

Card component for displaying items in a list, with support for media, actions, and a compact variant for reorder mode.

```svelte
<script>
  import { ListCard } from "@decodelabs/underlay/components";
  import BookOpen from "lucide-svelte/icons/book-open";
</script>

<!-- Standard list card with link -->
<ListCard
  href="/articles/123"
  title="Getting Started with Svelte"
  subtitle="A beginner's guide to Svelte 5"
  accent="#14b8a6"
>
  {#snippet media()}
    <BookOpen size={30} />
  {/snippet}
  
  {#snippet trailing()}
    <Pill>Tutorial</Pill>
  {/snippet}
  
  <span class="meta">Published: 2024-01-15</span>
</ListCard>

<!-- With actions menu -->
<ListCard
  href="/modules/abc"
  title="FA1"
  subtitle="Introduction to Financial Accounting"
  accent="#14b8a6"
>
  {#snippet media()}
    <BookOpen size={30} />
  {/snippet}
  
  {#snippet actions({ trigger })}
    <CopyActionsMenu
      {trigger}
      copies={[{ label: "Copy ID", text: module.id }]}
      actions={[{ label: "Edit", onSelect: handleEdit }]}
    />
  {/snippet}
</ListCard>

<!-- Compact variant for reorder mode -->
<ListCard
  title="Section A: Revenue Recognition"
  variant="compact"
  showDragHandle
  accent="#14b8a6"
>
  {#snippet media()}
    <Layers size={16} />
  {/snippet}
</ListCard>
```

**Props:**

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `href` | `string \| null` | `null` | Link destination (makes card clickable) |
| `title` | `string` | required | Primary card title |
| `subtitle` | `string \| null` | `null` | Secondary text below title |
| `ariaLabel` | `string \| null` | `null` | Accessible label (defaults to title) |
| `accent` | `string \| null` | `null` | Accent color for media background/border |
| `variant` | `"default" \| "compact"` | `"default"` | Visual variant |
| `isLive` | `boolean` | `true` | When false, shows draft styling (grayscale, dashed border) |
| `showDragHandle` | `boolean` | `false` | Show drag handle (compact variant only) |
| `onclick` | `(event: MouseEvent) => void` | `null` | Click handler |

**Snippets:**

| Snippet | Parameters | Description |
|---------|------------|-------------|
| `media` | none | Icon or image in the media area |
| `trailing` | none | Content at the end of title row (badges, pills) |
| `actions` | `{ trigger: Snippet }` | Actions menu - receives trigger snippet for custom menus |
| `children` | none | Additional content below title/subtitle |

**Variants:**

- **default** - Full card with 76px media area, title, subtitle, and optional content
- **compact** - Minimal 48px height card with small icon and title only, designed for reorder mode

**Draft State (`isLive={false}`):**
- Reduced opacity (0.7)
- Grayscale filter
- Dashed border
- Restores on hover

**Usage with Actions:**

When the `actions` snippet is provided, the media area becomes a clickable trigger containing the icon plus a "⋯" indicator. This allows the icon to open a dropdown menu while the rest of the card remains a link.

```svelte
{#snippet actions({ trigger })}
  <CopyActionsMenu
    {trigger}
    toastStore={toastStore}
    copies={[
      { label: "Copy slug", text: item.slug, successMessage: "Copied slug" }
    ]}
    actions={[
      { label: "Edit", onSelect: () => goto(`/items/${item.id}/edit`) },
      { label: "Delete", destructive: true, onSelect: handleDelete }
    ]}
  />
{/snippet}
```

**Child Counts Pattern:**

When displaying related entity counts (e.g., "3 modules", "5 sections"), use an icon + number with a tooltip for the full label. This provides a compact visual while remaining accessible.

```svelte
<script>
  import { ListCard, Tooltip } from "@decodelabs/underlay/components";
  import BookOpen from "lucide-svelte/icons/book-open";
  import Layers from "lucide-svelte/icons/layers";
  import Package from "lucide-svelte/icons/package";
</script>

<ListCard href="/pathways/123" title="ACCA" subtitle="Association of Chartered Certified Accountants">
  {#snippet media()}
    <Route size={30} />
  {/snippet}

  <!-- Child counts with icon + tooltip -->
  <span class="counts">
    <Tooltip content="Modules: {pathway.moduleCount}" inline delayDuration={200}>
      {#snippet trigger()}
        <span class="counts__item">
          <BookOpen size={14} />
          {pathway.moduleCount}
        </span>
      {/snippet}
    </Tooltip>
    <Tooltip content="Levels: {pathway.levelCount}" inline delayDuration={200}>
      {#snippet trigger()}
        <span class="counts__item">
          <Layers size={14} />
          {pathway.levelCount}
        </span>
      {/snippet}
    </Tooltip>
  </span>
</ListCard>

<style>
  .counts {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .counts__item {
    display: inline-flex;
    align-items: center;
    gap: 0.3rem;
  }
</style>
```

**Guidelines:**

- Use a relevant icon for each count type (e.g., `BookOpen` for modules, `Layers` for sections/levels, `Package` for bundles, `Target` for outcomes)
- Icon size should be 14px for consistency
- Tooltip format: `"{Label}: {count}"` (e.g., "Modules: 5")
- Set `delayDuration={200}` for quick hover response
- Use `inline` mode on the Tooltip component
- Multiple counts are separated with `gap: 0.75rem`

### NavCard & NavCardGrid

Navigation card components for index/dashboard pages. NavCards are substantial link blocks with icons and descriptions, ideal for section landing pages.

```svelte
<script>
  import { NavCard, NavCardGrid } from "@decodelabs/underlay/patterns";
  import Users from "lucide-svelte/icons/users";
  import Settings from "lucide-svelte/icons/settings";
  import Trash2 from "lucide-svelte/icons/trash-2";
</script>

<h1>Dashboard</h1>

<NavCardGrid>
  <NavCard
    href="/users"
    title="Users"
    description="Manage user accounts and permissions."
    icon={Users}
  />
  <NavCard
    href="/settings"
    title="Settings"
    description="Configure application preferences."
    icon={Settings}
  />
  <NavCard
    href="/trash"
    title="Trash"
    description="View and restore deleted items."
    icon={Trash2}
    variant="danger"
  />
</NavCardGrid>
```

**NavCard Props:**
- `href` - Link destination (required)
- `title` - Card title (required)
- `description` - Description text shown below title
- `icon` - Svelte component for the icon (e.g., lucide-svelte icons)
- `variant` - `"default"` | `"danger"` (default: `"default"`)
- `children` - Optional extra content via slot

**NavCardGrid Props:**
- `children` - NavCard components (required)

**Grid Behavior:**
- Uses CSS `auto-fit` with `minmax(20rem, 1fr)` for adaptive columns
- Automatically adjusts column count based on available width
- Falls back to single column on narrow viewports (<480px)

**Variants:**
- `default` - Standard styling with primary-colored icon badge
- `danger` - Red-tinted styling for destructive actions (trash, delete sections)

**Usage Notes:**
- NavCards work with any icon component that accepts a `class` prop
- Icons are displayed in a colored badge (primary blue for default, red gradient for danger)
- The grid has no maximum column count - it adapts entirely to available space
- Use for section index pages where links are the primary content

### PageHeader

Page header component with section heading, optional entity title, breadcrumbs, back link, and action buttons. Use this for consistent page headers across your application.

```svelte
<script>
  import {
    PageHeader,
    PageHeaderMeta,
    PageHeaderMetaRow,
    PageHeaderMetaItem,
    PageHeaderMetaSeparator,
    type BreadcrumbItem
  } from "@decodelabs/underlay/patterns";
  import { Button, Code, Pill } from "@decodelabs/underlay/components";
</script>

<!-- List page: section only (renders as h1) -->
<PageHeader section="Projects" backHref="/" backLabel="Back to dashboard">
  {#snippet actions()}
    <Button href="/projects/new">Add Project</Button>
  {/snippet}
</PageHeader>

<!-- List page with count badge -->
<PageHeader section="Users" count={total} backHref="/" backLabel="Back to dashboard" />

<!-- Detail page: section (h1) + title (h2) -->
<PageHeader
  section="Project"
  title={project.name}
  backHref="/projects"
  backLabel="Back to projects"
>
  <PageHeaderMeta>
    <PageHeaderMetaRow>
      <PageHeaderMetaItem label="ID">
        <Code copy>{project.id}</Code>
      </PageHeaderMetaItem>
      <PageHeaderMetaSeparator />
      <Pill accent="#22c55e">Active</Pill>
    </PageHeaderMetaRow>
  </PageHeaderMeta>
</PageHeader>

<!-- Detail page with breadcrumbs -->
<PageHeader
  section="Section"
  subtitle={section.title}
  breadcrumbs={[
    { label: pathway.name, href: `/pathways/${pathway.id}` },
    { label: module.code, href: `/modules/${module.id}` },
    { label: `Section ${section.label}` }
  ]}
  backHref={`/modules/${module.id}`}
  backLabel="Back to module"
/>

<!-- Edit form page: action phrase as section -->
<SpaFormShell
  section="Edit Project"
  subtitle={project.name}
  backHref={`/projects/${project.id}`}
  backLabel="Back to project"
  ...
/>

<!-- New form page: action phrase as section -->
<SpaFormShell
  section="New Project"
  subtitle="Create a new project to organize your tasks"
  ...
/>
```

**Props:**
- `section` - Section heading (renders as h1). Use for the page type or action phrase.
- `title` - Entity-specific title (renders as h2 below section, when both are set). Optional.
- `subtitle` - Secondary text below the heading (shown when no breadcrumbs). Optional.
- `breadcrumbs` - Array of `{ label, href? }` for navigation trail. Items without `href` render as plain text (for the current page). Optional.
- `level` - Heading level: 1 (page), 2, 3 (section), 4 (subsection). Default: 1.
- `count` - Badge count after the heading (e.g., total items on list pages). Optional.
- `backHref` - URL for back link. Optional.
- `backLabel` - Text for back link (default: "Back").
- `backIsContextual` - Shows a green dot when the back link came from navigation context.
- `bannerMessage` - Warning/info banner below the header. Optional.
- `bannerVariant` - Banner style: "warning" | "error" | "info". Default: "warning".
- `actions` - Snippet for action buttons (aligned right of the heading row). Optional.
- `titleSuffix` - Snippet for inline content after the title (e.g., a Pill). Optional.
- `subtitleSuffix` - Snippet for inline content after the subtitle. Optional.
- `children` - Snippet for meta information below header. Optional.

**Heading Hierarchy:**

When `section` is set, it renders as the primary h1 heading. When `title` is also provided, it renders as a smaller, muted h2 below the section heading. When only `title` is set (no `section`), it renders as h1 — this is the legacy behaviour and still works for backward compatibility.

**Layout:**
The component renders in this order:
1. Section heading (h1) — or title if no section
2. Entity title (h2, if both section and title are set)
3. Breadcrumbs (if provided) — or subtitle (if no breadcrumbs)
4. Back link (inline on wide screens, below on narrow)
5. Actions (aligned right of the heading row)
6. Meta content (children, below the header)
7. Banner (if bannerMessage is set)

**Usage Guidelines by Page Type:**

| Page type | `section` | `title` | Example |
|-----------|-----------|---------|---------|
| List page | Plural entity name | — | `section="Projects"` |
| Hub page | Hub name | — | `section="System"` |
| Detail page | Singular entity name | Entity title/name | `section="Project"` `title={project.name}` |
| Edit form | "Edit Entity" | — | `section="Edit Project"` |
| New form | "New Entity" | — | `section="New Project"` |
| Nested header | — | Sub-heading text | `title={module.title}` `level={3}` |

- Use Title Case for section names (e.g., "Audit Log", not "Audit log")
- On detail pages, use `subtitle` for supplementary text (e.g., a slug) and `title` for the primary entity identifier
- Use `breadcrumbs` on deeply nested pages to show the navigation hierarchy. The last breadcrumb item can omit `href` to render as plain text for the current page.
- The `count` badge stays on the section/title h1 heading
- Use `PageHeaderMeta*` components to standardize metadata layout below the heading

**Breadcrumbs:**

```typescript
interface BreadcrumbItem {
  label: string;
  href?: string; // omit for current page (renders as plain text)
}
```

Breadcrumbs replace the subtitle when provided. They render as a horizontal trail with chevron separators. Items with `href` render as links; items without render as plain text with `aria-current="page"`.

### ReorderableList & Reorder Controller

Drag-and-drop reordering pattern for admin list pages. Uses `svelte-dnd-action` under the hood with batch-commit semantics (changes are saved only when user clicks "Save Order").

#### Overview

The reordering pattern consists of two parts:
1. **`createReorderController`** - Svelte 5 reactive state controller for managing reorder state
2. **`ReorderableList`** - UI component that wraps items with drag-and-drop and Save/Cancel buttons

#### Basic Usage

```svelte
<script lang="ts">
  import { ReorderableList, createReorderController } from "@decodelabs/underlay/patterns";
  import { ListCard, Button } from "@decodelabs/underlay/components";
  import { myApi } from "$lib/api";

  let { data } = $props();

  let isReorderMode = $state(false);

  // Items must have an 'id' field - map if needed
  const reorderItems = $derived(
    data.items.map((item) => ({ ...item, id: item.itemId }))
  );

  // Create controller with items and submit function
  const controller = $derived(
    createReorderController(reorderItems, async (orderedIds) => {
      await myApi.reorderItems(orderedIds);
    })
  );
</script>

<header>
  <h2>Items</h2>
  {#if !isReorderMode && data.items.length > 1}
    <Button variant="subtle" onclick={() => isReorderMode = true}>
      Reorder
    </Button>
  {/if}
</header>

{#if isReorderMode}
  <ReorderableList
    {controller}
    oncancel={() => isReorderMode = false}
    onsuccess={() => isReorderMode = false}
  >
    {#snippet item(item)}
      <ListCard
        title={item.name}
        variant="compact"
        showDragHandle
        accent="#14b8a6"
      >
        {#snippet media()}
          <MyIcon size={16} />
        {/snippet}
      </ListCard>
    {/snippet}
  </ReorderableList>
{:else}
  <!-- Normal list view -->
  {#each data.items as item}
    <MyItemCard {item} />
  {/each}
{/if}
```

#### createReorderController

Factory function that creates a reactive controller for managing reorder state.

```typescript
import { createReorderController } from "@decodelabs/underlay/patterns";

const controller = createReorderController(items, submitFn);
```

**Parameters:**
- `items: T[]` - Initial array of items. Each item must have an `id: string` property.
- `submitFn: (orderedIds: string[]) => Promise<void>` - Async function called with the new order of IDs when user saves.

**Returns: `ReorderController<T>`**

| Property/Method | Type | Description |
|----------------|------|-------------|
| `pending` | `T[]` | Current working order (mutable, used by drag-drop) |
| `original` | `readonly T[]` | Original order for comparison |
| `isDirty` | `boolean` | Whether order has changed from original |
| `isPending` | `boolean` | Whether submit is in progress |
| `error` | `Error \| null` | Error from last submit attempt |
| `move(from, to)` | `(number, number) => void` | Programmatically move item |
| `reset()` | `() => void` | Reset to original order |
| `submit()` | `() => Promise<void>` | Submit the new order |
| `updatePending(items)` | `(T[]) => void` | Update pending items (used by DnD handlers) |
| `mergeNewItems(items)` | `(T[]) => void` | Merge new items (conflict resolution) |
| `removeItems(ids)` | `(string[]) => void` | Remove items by ID (conflict resolution) |

**Example with ID mapping:**

```svelte
<script lang="ts">
  // When your items use a different ID field name
  const reorderItems = $derived(
    data.pathways.map((p) => ({ ...p, id: p.pathwayId }))
  );

  const controller = $derived(
    createReorderController(reorderItems, async (orderedIds) => {
      await learningCommands.reorderPathways(orderedIds, fetch, authToken);
    })
  );
</script>
```

#### ReorderableList Component

Wraps a list of items with drag-and-drop functionality, Save/Cancel header, and error handling.

```svelte
<ReorderableList
  controller={controller}
  oncancel={handleCancel}
  onsuccess={handleSuccess}
  flipDurationMs={200}
  saveLabel="Save Order"
  cancelLabel="Cancel"
>
  {#snippet item(itemData)}
    <!-- Render each item -->
  {/snippet}
  
  {#snippet empty()}
    <p>No items to reorder.</p>
  {/snippet}
</ReorderableList>
```

**Props:**

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `controller` | `ReorderController<T>` | required | Controller from `createReorderController` |
| `oncancel` | `() => void` | required | Called when user clicks Cancel |
| `onsuccess` | `() => void` | optional | Called after successful submit |
| `onsubmiterror` | `(error: unknown) => void \| string \| Promise<void \| string>` | optional | Optional hook to transform submit errors (for conflict recovery) |
| `flipDurationMs` | `number` | `200` | Animation duration for reorder transitions |
| `disabled` | `boolean` | `false` | Disable drag-and-drop |
| `saveLabel` | `string` | `"Save Order"` | Custom save button text |
| `cancelLabel` | `string` | `"Cancel"` | Custom cancel button text |

**Snippets:**

| Snippet | Parameters | Description |
|---------|------------|-------------|
| `item` | `(T)` | Required. Renders each draggable item |
| `empty` | none | Optional. Shown when list is empty |

#### ListCard Compact Variant

When in reorder mode, use `ListCard` with `variant="compact"` and `showDragHandle` for a streamlined drag-and-drop experience.

```svelte
<ListCard
  title={item.name}
  variant="compact"
  showDragHandle
  accent="#14b8a6"
>
  {#snippet media()}
    <BookOpen size={16} />
  {/snippet}
</ListCard>
```

**Compact variant differences:**
- Smaller height (~48px vs full card height)
- Reduced icon size (28px vs 76px)
- Title only (no subtitle, meta, or actions)
- Grab cursor for drag indication
- Optional drag handle icon on the left

**Props for compact mode:**

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` | `"default" \| "compact"` | `"default"` | Visual variant |
| `showDragHandle` | `boolean` | `false` | Show 6-dot drag handle icon |

#### Complete Example: Reorderable Sections

A full example showing reorder mode for sections within a module:

```svelte
<script lang="ts">
  import { ReorderableList, createReorderController } from "@decodelabs/underlay/patterns";
  import { Button, ListCard } from "@decodelabs/underlay/components";
  import { learningCommands } from "@cattle-grid";
  import Layers from "lucide-svelte/icons/layers";

  let { data } = $props();

  let isSectionReorderMode = $state(false);

  // Map sections to have 'id' field
  const sectionReorderItems = $derived(
    data.syllabus.sections.map((s) => ({ ...s, id: s.sectionId }))
  );

  // Create controller for sections
  const sectionController = $derived(
    createReorderController(sectionReorderItems, async (orderedIds) => {
      await learningCommands.reorderSectionsInModule(
        data.module.moduleId,
        orderedIds,
        fetch,
        data.authToken!
      );
    })
  );

  function enterReorderMode() {
    isSectionReorderMode = true;
  }

  function exitReorderMode() {
    isSectionReorderMode = false;
  }
</script>

<section>
  <header class="section-header">
    <h2>Sections</h2>
    {#if !isSectionReorderMode && data.syllabus.sections.length > 1}
      <Button variant="subtle" onclick={enterReorderMode}>
        Reorder Sections
      </Button>
    {/if}
  </header>

  {#if data.syllabus.sections.length === 0}
    <p>No sections defined.</p>
  {:else if isSectionReorderMode}
    <ReorderableList
      controller={sectionController}
      oncancel={exitReorderMode}
      onsuccess={exitReorderMode}
    >
      {#snippet item(section)}
        <ListCard
          title={`Section ${section.label}: ${section.title}`}
          variant="compact"
          showDragHandle
          accent="#14b8a6"
        >
          {#snippet media()}
            <Layers size={16} />
          {/snippet}
        </ListCard>
      {/snippet}
    </ReorderableList>
  {:else}
    {#each data.syllabus.sections as section}
      <SectionCard {section} />
    {/each}
  {/if}
</section>

<style>
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .section-header h2 {
    margin: 0;
  }
</style>
```

#### Nested Reordering

For hierarchical data (e.g., sections containing areas), you can have multiple reorder modes. Use separate state variables and controllers for each level:

```svelte
<script lang="ts">
  let isSectionReorderMode = $state(false);
  let reorderingAreaSectionId = $state<string | null>(null);

  // Section controller
  const sectionController = $derived(
    createReorderController(sectionItems, async (ids) => {
      await api.reorderSections(moduleId, ids);
    })
  );

  // Area controller (only active when reordering a specific section)
  const reorderingSection = $derived(
    reorderingAreaSectionId
      ? sections.find((s) => s.id === reorderingAreaSectionId)
      : null
  );

  const areaController = $derived(
    reorderingAreaSectionId && reorderingSection
      ? createReorderController(reorderingSection.areas, async (ids) => {
          await api.reorderAreas(reorderingAreaSectionId!, ids);
        })
      : null
  );
</script>

{#if isSectionReorderMode}
  <!-- Section reorder UI -->
{:else if reorderingAreaSectionId && areaController}
  <!-- Context label -->
  <p>Reordering areas in: <strong>{reorderingSection?.title}</strong></p>
  <!-- Area reorder UI -->
{:else}
  <!-- Normal view with "Reorder Areas" buttons per section -->
{/if}
```

#### Best Practices

1. **Only show Reorder button when useful** - Hide when there's 0 or 1 item
2. **Invalidate data on success** - Call `invalidateAll()` in `onsuccess` to refresh the list with new order
3. **Use consistent icons** - Pick meaningful icons for each item type
4. **Map IDs correctly** - Ensure items have `id` field (map from `itemId`, `sectionId`, etc.)
5. **Use compact ListCard** - The `variant="compact"` + `showDragHandle` combo is designed for reorder mode
6. **Handle nested structures** - Use separate state/controllers for each reorderable level
7. **Place Reorder button logically** - Next to section headers, not in page header

**Refreshing data after reorder:**

```svelte
<script lang="ts">
  import { invalidateAll } from "$app/navigation";

  let isReorderMode = $state(false);

  async function handleReorderSuccess() {
    isReorderMode = false;
    await invalidateAll(); // Refetch page data with new order
  }
</script>

<ReorderableList
  controller={controller}
  oncancel={() => isReorderMode = false}
  onsuccess={handleReorderSuccess}
>
  ...
</ReorderableList>
```

#### API Backend Requirements

Your backend needs a reorder endpoint that:
1. Accepts an array of IDs in the desired order
2. Updates the `sort_order` (or equivalent) column for each item
3. Returns success confirmation

Example endpoint pattern:
```
POST /v1/admin/learning/modules/{moduleId}/sections/reorder
Body: { "ids": ["section-1", "section-3", "section-2"] }
Response: { "reorderedCount": 3 }
```

#### Conflict Recovery Contract

For concurrency-safe reorder UX, treat reorder as optimistic with server-side conflict detection.

Backend conflict response requirements:

1. Return `409 Conflict` when submitted IDs are out of sync with current server state.
2. Include conflict context:

```json
{
  "error": {
    "code": "learning.reorder_conflict",
    "message": "Items have changed since you started reordering."
  },
  "context": {
    "added_ids": ["new-id-1"],
    "removed_ids": ["deleted-id-1"]
  }
}
```

Frontend recovery pattern:

1. Parse conflict payload from the error.
2. Remove deleted items from pending reorder state.
3. Append newly added items (from latest snapshot) to pending state.
4. Keep user in reorder mode and require an explicit second save.

Shared helpers:

- `extractReorderConflict(error)`
- `applyReorderConflict(controller, conflict, latestItems)`

Example:

```svelte
<ReorderableList
  controller={controller}
  oncancel={exitReorderMode}
  onsuccess={handleSuccess}
  onsubmiterror={async (error) => {
    const conflict = extractReorderConflict(error);
    if (!conflict) return;

    const latestItems = await loadLatestItems();
    applyReorderConflict(controller, conflict, latestItems);
    return "List changed while reordering. Review updates and save again.";
  }}
>
  ...
</ReorderableList>
```

### RelationSelector

Modal-based relation picker for selecting related records. Provides a richer experience than simple `<select>` dropdowns for relation fields with large datasets.

#### Overview

The RelationSelector provides:
- **Two-tier interaction**: Quick dropdown for fast selection, full modal for search/browse
- **Server-side search**: Debounced search with configurable suggestions
- **Intelligent suggestions**: Track user selections and prioritize recently-used items (see [Selection Suggestions Guide](./092-selection-suggestions.md))
- **Single and multi-select modes**: Toggle individual items or select multiple with confirmation
- **Embedded create form**: Add new related records without leaving the modal
- **Full accessibility**: Keyboard navigation, ARIA attributes, focus management

```
┌─────────────────────────────────────┐
│  [Selected: Applied Skills    ▼]   │  ← Trigger button
└─────────────────────────────────────┘
                 │
                 ▼ (click)
┌─────────────────────────────────────┐
│  Recent                             │  ← Quick dropdown (tier 1)
│  ┌───────────────────────────────┐  │
│  │ Applied Skills            ✓   │  │
│  │ Strategic Professional        │  │
│  │ Operational                   │  │
│  └───────────────────────────────┘  │
│  ┌───────────────────────────────┐  │
│  │ 🔍 Search all levels...       │  │  ← Opens full modal
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
```

#### Basic Single-Select

```svelte
<script lang="ts">
  import { RelationSelector, type SelectableRelation } from "@decodelabs/underlay/patterns";

  let pathwayId = $state<string | null>(null);

  // Server-side search function
  async function searchPathways(query: string): Promise<{ items: SelectableRelation[]; total: number }> {
    const response = await fetch(`/api/pathways/search?q=${encodeURIComponent(query)}`);
    return response.json();
  }

  // Optional: fetch recent/suggested items
  async function getSuggestions(): Promise<SelectableRelation[]> {
    const response = await fetch('/api/pathways/recent');
    return response.json();
  }
</script>

<Field label="Pathway" forId="pathway">
  <RelationSelector
    label="Select Pathway"
    bind:value={pathwayId}
    search={searchPathways}
    suggestions={getSuggestions}
    suggestionsLabel="Recent"
    placeholder="Choose a pathway..."
  />
</Field>
```

#### Multi-Select with Pills

```svelte
<script lang="ts">
  import { RelationSelector } from "@decodelabs/underlay/patterns";

  let moduleIds = $state<string[]>([]);
</script>

<Field label="Modules" forId="modules">
  <RelationSelector
    label="Select Modules"
    mode="multi"
    bind:values={moduleIds}
    search={searchModules}
    placeholder="Choose modules..."
  />
</Field>
```

Multi-select mode shows:
- Pills for selected items (up to 3 visible, then "+N more")
- Checkboxes in dropdown and modal
- Confirm/Cancel buttons in modal

#### Dependent Fields

Handle field dependencies (e.g., Level depends on Pathway) at the form level:

```svelte
<script lang="ts">
  let pathwayId = $state<string | null>(null);
  let levelId = $state<string | null>(null);

  // Clear dependent field when parent changes
  $effect(() => {
    if (!pathwayId) {
      levelId = null;
    }
  });

  // Search function scoped to selected pathway
  const searchLevels = (query: string) =>
    api.searchLevels(pathwayId!, query);
</script>

<Field label="Pathway" required>
  <RelationSelector
    label="Select Pathway"
    bind:value={pathwayId}
    search={searchPathways}
  />
</Field>

<Field label="Level">
  <RelationSelector
    label="Select Level"
    bind:value={levelId}
    search={searchLevels}
    disabled={!pathwayId}
    placeholder={!pathwayId ? "Select a pathway first" : "Select a level..."}
  />
</Field>
```

#### With Create Form

Embed a form to create new items without leaving the modal:

```svelte
<RelationSelector
  label="Select Level"
  bind:value={levelId}
  search={searchLevels}
  allowCreate
  createLabel="Add new level"
>
  {#snippet createForm(onSuccess, onCancel)}
    <LevelQuickCreateForm
      pathwayId={pathwayId}
      onSuccess={(level) => onSuccess(level)}
      onCancel={onCancel}
    />
  {/snippet}
</RelationSelector>
```

The create form:
- Appears as a collapsible section in the modal
- Calls `onSuccess(item)` when created - item is auto-selected
- Calls `onCancel()` to close without creating

#### Custom Item Rendering

Use the `renderItem` snippet for custom item display:

```svelte
<RelationSelector
  label="Select Module"
  bind:value={moduleId}
  search={searchModules}
>
  {#snippet renderItem(item, selected)}
    <div class="module-item" class:selected>
      <span class="code">{item.metadata?.code}</span>
      <span class="title">{item.label}</span>
      <span class="pathway">{item.metadata?.pathwayName}</span>
    </div>
  {/snippet}
</RelationSelector>
```

#### Props Reference

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `string \| null` | `null` | Selected value ID (single-select) |
| `values` | `string[]` | `[]` | Selected value IDs (multi-select) |
| `mode` | `"single" \| "multi"` | `"single"` | Selection mode |
| `search` | `(query, options?) => Promise<SearchResult>` | required | Server-side search function |
| `suggestions` | `(options?: SuggestionOptions) => Promise<T[]>` | - | Fetch suggestions/recent items |
| `selectionHistory` | `SelectionHistory` | - | Track selections for intelligent suggestions ([guide](./092-selection-suggestions.md)) |
| `label` | `string` | required | Modal title |
| `placeholder` | `string` | `"Select..."` | Trigger button placeholder |
| `searchPlaceholder` | `string` | `"Search..."` | Search input placeholder |
| `emptyMessage` | `string` | `"No results found"` | Empty search results message |
| `suggestionsLabel` | `string` | `"Suggestions"` | Label for suggestions section |
| `searchAllLabel` | `string` | `"Search all..."` | Label for search button in dropdown |
| `disabled` | `boolean` | `false` | Disable the selector |
| `required` | `boolean` | `false` | Mark as required field |
| `error` | `string` | - | Error message to display |
| `quickSelect` | `boolean` | `true` if suggestions provided | Enable quick-select dropdown |
| `quickSelectLimit` | `number` | `5` | Max items in quick dropdown |
| `allowCreate` | `boolean` | `false` | Show "Add new" button |
| `createLabel` | `string` | `"Add new"` | Create button label |
| `onCreate` | `(item: T) => void` | - | Called when item created |

#### Callbacks

| Callback | Type | Description |
|----------|------|-------------|
| `onchange` | `(value: string \| null) => void` | Single-select value change |
| `onchangeMulti` | `(values: string[]) => void` | Multi-select values change |
| `onCreate` | `(item: T) => void` | New item created |

#### Snippets

| Snippet | Parameters | Description |
|---------|------------|-------------|
| `renderItem` | `(item: T, selected: boolean)` | Custom item rendering |
| `renderTrigger` | `(selected: T \| T[] \| null, open: () => void)` | Custom trigger button |
| `renderSelectedPill` | `(item: T, remove: () => void)` | Custom pill rendering (multi-select) |
| `createForm` | `(onSuccess: (item: T) => void, onCancel: () => void)` | Embedded create form |

#### Types

```typescript
interface SelectableRelation {
  id: string;
  label: string;
  description?: string | null;
  disabled?: boolean;
  metadata?: Record<string, unknown>;
}

interface SearchResult<T> {
  items: T[];
  total: number;
}

type RelationSearchFn<T> = (
  query: string,
  options?: { limit?: number; offset?: number }
) => Promise<SearchResult<T>>;

type RelationSuggestionsFn<T> = () => Promise<T[]>;
```

#### Backend Requirements

Search endpoints should follow this pattern:

```
GET /v1/admin/learning/levels/search?q=applied&pathwayId=xxx&limit=20&offset=0
```

Response format:

```json
{
  "items": [
    { "id": "level-1", "label": "Applied Skills", "description": "3 modules" }
  ],
  "total": 15
}
```

#### Keyboard Navigation

| Key | Action |
|-----|--------|
| `Tab` | Move through interactive elements |
| `Enter`/`Space` | Select/toggle item, activate buttons |
| `Arrow Up/Down` | Navigate through list items |
| `Escape` | Close dropdown/modal |

#### Accessibility

- Trigger has `role="combobox"` with `aria-expanded` and `aria-haspopup`
- Lists have `role="listbox"` with `role="option"` items
- `aria-selected` indicates selection state
- Focus is managed: search input focused on modal open
- Error retry button for network failures

---

## Design Tokens

### CSS Custom Properties

Underlay uses CSS custom properties for theming. You can override these in your app:

```css
/* In your global.css */
:root {
  /* Colors */
  --underlay-color-primary: #3b82f6;
  --underlay-color-primary-hover: #2563eb;
  --underlay-color-error: #ef4444;
  --underlay-color-success: #10b981;
  
  /* Text */
  --underlay-color-text: #f3f4f6;
  --underlay-color-text-muted: #9ca3af;
  
  /* Spacing */
  --underlay-space-1: 0.25rem;
  --underlay-space-2: 0.5rem;
  --underlay-space-3: 0.75rem;
  --underlay-space-4: 1rem;
  
  /* Typography */
  --underlay-font-size-xs: 0.75rem;
  --underlay-font-size-sm: 0.875rem;
  --underlay-font-size-base: 1rem;
  --underlay-font-size-lg: 1.125rem;
  
  /* Borders */
  --underlay-border-radius: 0.375rem;
  --underlay-border-color: rgba(255, 255, 255, 0.1);
}
```

### Dark Mode Support

Components automatically adapt to dark mode via CSS custom properties:

```css
:root {
  --underlay-color-bg: #ffffff;
  --underlay-color-text: #1f2937;
}

@media (prefers-color-scheme: dark) {
  :root {
    --underlay-color-bg: #1f2937;
    --underlay-color-text: #f3f4f6;
  }
}
```

---

## CSS Utility Classes

Underlay provides utility classes for common layout patterns. Import the base styles to use them:

```ts
// In your app's layout or entry point
import "@decodelabs/underlay/styles/base.css";
```

### Details Content Layout

The `.underlay-details-content` class provides consistent vertical spacing for admin detail pages. Use it to wrap the content of a "Details" tab to ensure uniform gaps between sections.

```svelte
<TabsContent value="details">
  <PageHeader title={item.title} level={3}>
    <p><strong>Slug:</strong> <code>{item.slug}</code></p>
  </PageHeader>

  <div class="underlay-details-content">
    <DetailsGrid>
      <DetailsSection legend="Configuration">
        <DetailsItem label="Name" value={item.name} />
        <DetailsItem label="Status" value={item.status} />
      </DetailsSection>
    </DetailsGrid>

    <ContentCard
      title="Description"
      value={item.description}
      emptyMessage="No description set."
    />

    <TabsRoot value="tab1" variant="underline" size="sm">
      <!-- Nested tabs for grouped content -->
    </TabsRoot>
  </div>
</TabsContent>
```

**What it does:**
- Applies `display: flex` with `flex-direction: column` and `gap: 1.5rem`
- Removes top margin from `ContentCard` components when nested inside
- Handles nested underline tabs correctly

**When to use:**
- Inside `TabsContent value="details"` on admin detail pages
- When you have multiple sections (DetailsGrid, ContentCard, nested tabs) that need consistent spacing
- To avoid manual margin adjustments between sections

---

## Complete Form Example

```svelte
<script lang="ts">
  import { 
    Form, 
    Field, 
    TextInput, 
    TextArea, 
    Select, 
    Switch, 
    FormActions, 
    Button 
  } from "@decodelabs/underlay";
  import { enhance } from "$app/forms";
  import type { ActionData } from "./$types";
  
  export let form: ActionData;
  
  let loading = false;
  let published = false;
  
  const categories = [
    { value: "tech", label: "Technology" },
    { value: "design", label: "Design" },
    { value: "business", label: "Business" }
  ];
</script>

<h1>Create Article</h1>

{#if form?.error}
  <div class="alert error" role="alert">
    {form.error}
  </div>
{/if}

<Form method="POST" use:enhance={() => {
  loading = true;
  return async ({ update }) => {
    await update();
    loading = false;
  };
}}>
  <Field label="Title" forId="title" error={form?.errors?.title}>
    <TextInput
      id="title"
      name="title"
      value={form?.values?.title ?? ""}
      placeholder="Enter article title"
      required
      disabled={loading}
    />
  </Field>
  
  <Field label="Category" forId="category" error={form?.errors?.category}>
    <Select
      id="category"
      name="category"
      options={categories}
      value={form?.values?.category ?? "tech"}
      disabled={loading}
    />
  </Field>
  
  <Field 
    label="Content" 
    forId="content" 
    error={form?.errors?.content}
    hint="Markdown supported"
  >
    <TextArea
      id="content"
      name="content"
      value={form?.values?.content ?? ""}
      rows={12}
      placeholder="Write your article content..."
      required
      disabled={loading}
    />
  </Field>
  
  <Field label="Publish immediately">
    <Switch
      name="published"
      bind:checked={published}
      disabled={loading}
    />
  </Field>
  
  <FormActions>
    <Button type="submit" variant="primary" disabled={loading}>
      {loading ? "Saving..." : "Save Article"}
    </Button>
    <Button type="button" variant="secondary" on:click={() => history.back()}>
      Cancel
    </Button>
  </FormActions>
</Form>

<style>
  .alert {
    padding: 1rem;
    border-radius: 0.375rem;
    margin-bottom: 1.5rem;
  }
  
  .alert.error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #ef4444;
  }
</style>
```

---

## Accessibility

All Underlay components follow accessibility best practices:

### Form Labels

Always use `Field` with `label` and `forId`:

```svelte
<!-- ✅ CORRECT -->
<Field label="Email" forId="email">
  <TextInput id="email" name="email" type="email" />
</Field>

<!-- ❌ WRONG -->
<TextInput name="email" type="email" />
```

### Error Announcements

Errors are automatically announced to screen readers:

```svelte
<Field label="Email" error="Invalid email address">
  <TextInput
    id="email"
    aria-invalid="true"
    aria-describedby="email-error"
  />
</Field>
```

### Keyboard Navigation

- `Tab` - Navigate between fields
- `Space` - Toggle switches
- `Enter` - Activate buttons/submit forms
- `Escape` - Close dialogs/dropdowns

### Focus Management

Components handle focus automatically:
- Dialog opens → focus moves to dialog
- Dialog closes → focus returns to trigger
- Form error → focus moves to first error field

---

## Customization

### Extending Components

Create wrapper components for app-specific defaults:

```svelte
<!-- MyButton.svelte -->
<script>
  import { Button } from "@decodelabs/underlay";
  
  export let variant = "primary";
  export let pill = false; // Override default
</script>

<Button {variant} {pill} {...$$restProps}>
  <slot />
</Button>
```

### Custom Styles

Override component styles with custom classes:

```svelte
<Button className="my-custom-button">
  Click Me
</Button>

<style>
  :global(.my-custom-button) {
    background: linear-gradient(45deg, #f093fb, #f5576c);
  }
</style>
```

---

## Error Handling Patterns

### Display Validation Errors

Use the `Field` error prop to show validation errors:

```svelte
<script lang="ts">
  import { Field, TextInput } from "@decodelabs/underlay";
  import type { ActionData } from "./$types";
  
  export let form: ActionData;
  
  // Helper to get field errors
  function getFieldError(field: string): string | undefined {
    return form?.errors?.[field]?.[0]; // Get first error
  }
</script>

<Field label="Email" forId="email" error={getFieldError("email")}>
  <TextInput
    id="email"
    name="email"
    type="email"
    value={form?.values?.email ?? ""}
    aria-invalid={!!getFieldError("email")}
  />
</Field>
```

### Global Error Messages

Display form-level errors above the form:

```svelte
{#if form?.error}
  <div class="alert alert--error" role="alert">
    <strong>Error:</strong> {form.error}
  </div>
{/if}

<style>
  .alert {
    padding: 1rem;
    border-radius: var(--underlay-border-radius);
    margin-bottom: 1.5rem;
  }
  
  .alert--error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: var(--underlay-color-error);
  }
  
  .alert--success {
    background: rgba(16, 185, 129, 0.1);
    border: 1px solid rgba(16, 185, 129, 0.3);
    color: var(--underlay-color-success);
  }
</style>
```

### Loading States

Show loading indicators during async operations:

```svelte
<script>
  import { Button } from "@decodelabs/underlay";
  
  let loading = false;
  
  async function handleSubmit() {
    loading = true;
    try {
      await saveData();
    } finally {
      loading = false;
    }
  }
</script>

<Button type="submit" disabled={loading} on:click={handleSubmit}>
  {#if loading}
    <svg class="spinner" viewBox="0 0 50 50">
      <circle cx="25" cy="25" r="20" fill="none" stroke="currentColor" stroke-width="5" />
    </svg>
    Saving...
  {:else}
    Save Changes
  {/if}
</Button>

<style>
  .spinner {
    width: 1em;
    height: 1em;
    animation: spin 1s linear infinite;
  }
  
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  
  .spinner circle {
    stroke-dasharray: 90, 150;
    stroke-dashoffset: 0;
    stroke-linecap: round;
  }
</style>
```

### Error Boundaries (Page Level)

Handle errors at the page level with SvelteKit's error handling:

```svelte
<!-- +error.svelte -->
<script>
  import { page } from "$app/stores";
  import { Button } from "@decodelabs/underlay";
</script>

<div class="error-page">
  <h1>Oops! Something went wrong</h1>
  
  {#if $page.status === 404}
    <p>The page you're looking for doesn't exist.</p>
  {:else if $page.status === 403}
    <p>You don't have permission to access this page.</p>
  {:else}
    <p>An unexpected error occurred. Please try again.</p>
  {/if}
  
  <p class="error-message">{$page.error?.message}</p>
  
  <div class="actions">
    <Button on:click={() => window.location.reload()}>
      Try Again
    </Button>
    <Button variant="secondary" on:click={() => history.back()}>
      Go Back
    </Button>
  </div>
</div>

<style>
  .error-page {
    max-width: 600px;
    margin: 4rem auto;
    text-align: center;
    padding: 2rem;
  }
  
  .error-message {
    color: var(--underlay-color-text-muted);
    font-size: var(--underlay-font-size-sm);
    margin: 1rem 0 2rem;
  }
  
  .actions {
    display: flex;
    gap: 1rem;
    justify-content: center;
  }
</style>
```

### Toast Notifications (Optional)

For temporary success/error messages:

```svelte
<script>
  import { writable } from "svelte/store";
  
  const toasts = writable<Array<{ id: number; message: string; type: "success" | "error" }>>([]);
  
  export function showToast(message: string, type: "success" | "error" = "success") {
    const id = Date.now();
    toasts.update(t => [...t, { id, message, type }]);
    
    setTimeout(() => {
      toasts.update(t => t.filter(toast => toast.id !== id));
    }, 5000);
  }
</script>

<!-- Toast Container -->
<div class="toast-container">
  {#each $toasts as toast (toast.id)}
    <div class="toast toast--{toast.type}">
      {toast.message}
      <button on:click={() => toasts.update(t => t.filter(t => t.id !== toast.id))}>
        ×
      </button>
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    top: 1rem;
    right: 1rem;
    z-index: 9999;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .toast {
    padding: 1rem 1.5rem;
    border-radius: var(--underlay-border-radius);
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    display: flex;
    align-items: center;
    gap: 1rem;
    animation: slide-in 0.3s ease-out;
  }
  
  .toast--success {
    background: var(--underlay-color-success);
    color: white;
  }
  
  .toast--error {
    background: var(--underlay-color-error);
    color: white;
  }
  
  @keyframes slide-in {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }
</style>
```

---

## Domain UI Kit Pattern

For app-specific customizations, create a domain UI kit that wraps Underlay components.

> **Naming Your Domain UI Kit**: Choose a name that reflects your project. For example:
> - A learning platform called "EduPro" might use `edupro-ui`
> - An e-commerce site called "ShopMax" might use `shopmax-components`
> - A SaaS product called "Acme" might use `@acme/ui`
>
> The examples below use `myapp-ui` as a placeholder - **replace this with your own project name**.

### Structure

```
myapp-ui/src/
├── components/
│   ├── MyAppButton.svelte      # Wrapped Underlay Button
│   ├── MyAppField.svelte       # Wrapped Underlay Field
│   ├── MyAppTextInput.svelte   # Wrapped Underlay TextInput
│   ├── ProductCard.svelte      # Domain-specific component
│   └── OrderList.svelte        # Domain-specific component
├── styles/
│   └── theme.css               # App-specific tokens
└── index.ts
```

### Wrapping Components

Create wrappers with app-specific defaults:

```svelte
<!-- myapp-ui/src/components/MyAppButton.svelte -->
<script lang="ts">
  import { Button } from "@decodelabs/underlay";
  
  // Override Underlay defaults for your app
  export let variant: "primary" | "secondary" | "subtle" = "primary";
  export let pill: boolean = false; // Your app uses square buttons by default
  export let type: "button" | "submit" | "reset" = "button";
</script>

<Button {variant} {pill} {type} {...$$restProps}>
  <slot />
</Button>
```

### Domain-Specific Components

Build on Underlay primitives for domain features:

```svelte
<!-- myapp-ui/src/components/ProductCard.svelte -->
<script lang="ts">
  import { Card, Button } from "@decodelabs/underlay";
  
  export let product: {
    id: string;
    title: string;
    description: string;
    price: number;
  };
</script>

<Card className="product-card">
  <div class="product-header">
    <h3>{product.title}</h3>
    <span class="price">${product.price}</span>
  </div>
  
  <p class="product-description">{product.description}</p>
  
  <div class="product-actions">
    <Button href="/products/{product.id}">
      View Details
    </Button>
  </div>
</Card>

<style>
  .product-card {
    padding: 1.5rem;
  }
  
  .product-header {
    display: flex;
    justify-content: space-between;
    margin-bottom: 1rem;
  }
  
  .price {
    font-weight: 600;
    color: var(--underlay-color-primary);
  }
  
  .product-description {
    color: var(--underlay-color-text-muted);
    margin-bottom: 1rem;
  }
</style>
```

### Theme Customization

Override Underlay tokens in your domain UI kit:

```css
/* myapp-ui/src/styles/theme.css */

:root {
  /* Brand colors */
  --underlay-color-primary: #8b5cf6;
  --underlay-color-primary-hover: #7c3aed;
  
  /* Custom spacing for tighter layouts */
  --underlay-space-4: 0.875rem;
  
  /* Custom typography */
  --underlay-font-family: "Inter", sans-serif;
  
  /* App-specific tokens (use your own prefix) */
  --myapp-color-product-bg: rgba(139, 92, 246, 0.1);
  --myapp-color-success: #10b981;
  --myapp-color-error: #ef4444;
}
```

### Exporting Domain Kit

```typescript
// myapp-ui/src/index.ts

// Re-export Underlay components
export { 
  Dialog, 
  AlertDialog, 
  Card, 
  Select,
  Switch,
  TextArea,
  FormActions,
  ListCard
} from "@decodelabs/underlay";

// Export wrapped components
export { default as Button } from "./components/MyAppButton.svelte";
export { default as Field } from "./components/MyAppField.svelte";
export { default as TextInput } from "./components/MyAppTextInput.svelte";

// Export domain components
export { default as ProductCard } from "./components/ProductCard.svelte";
export { default as OrderList } from "./components/OrderList.svelte";
```

### Usage in App

```svelte
<!-- apps/storefront/src/routes/products/+page.svelte -->
<script>
  // Import from YOUR domain UI kit, not Underlay directly
  import { ProductCard, Button } from "@myorg/myapp-ui";
  
  export let data;
</script>

<h1>Products</h1>

<div class="product-grid">
  {#each data.products as product}
    <ProductCard {product} />
  {/each}
</div>
```

### Benefits of Domain UI Kit

1. **Consistency** - All apps use the same visual language
2. **Customization** - Override defaults without forking Underlay
3. **Domain Components** - Build app-specific components once, use everywhere
4. **Theming** - Centralize brand colors and spacing
5. **Upgrades** - Update Underlay without touching app code

### When to Use Domain UI Kit

**Use a domain UI kit when:**
- Building multiple frontends (e.g., customer + admin apps)
- Need app-specific component defaults
- Have domain-specific components (ProductCard, OrderList, etc.)
- Want centralized theming

**Use Underlay directly when:**
- Single frontend application
- Underlay defaults work perfectly
- No domain-specific components needed

---

## Best Practices

1. **Always use Field wrapper** for form inputs
2. **Provide error prop** for validation feedback
3. **Use semantic button types** (`submit`, `button`, `reset`)
4. **Disable inputs during loading** to prevent double submission
5. **Use proper ARIA attributes** for custom interactions
6. **Test keyboard navigation** for all interactive elements
7. **Provide loading states** with descriptive text ("Saving..." not just "...")
8. **Create domain UI kit** for multi-app projects
9. **Override tokens, not components** - prefer CSS custom properties over forking components

---

## Next Steps

- [095-navigation-context](./095-navigation-context.md) - Contextual back buttons and form redirects
- [100-frontend-web](./100-frontend-web.md) - Frontend application patterns
- [110-admin](./110-admin.md) - Admin interface patterns
- [075-validation](./075-validation.md) - Form validation with UI kit

---

## Reference

See Underlay components source:
- Components: `underlay/ts/src/components/`
- Patterns: `underlay/ts/src/patterns/`
- Styles: `underlay/ts/src/styles/`
