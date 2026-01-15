# 090 - UI Kit

This document covers creating and using a shared Svelte UI kit with Underlay. Underlay provides pre-built components for common UI needs.

## Overview

The UI kit provides:
- **Form components** - Field, TextInput, Select, Switch, TextArea
- **UI primitives** - Button, Badge, Pill, Breadcrumbs, Card, Dialog, DropdownMenu, Pagination
- **Patterns** - ListCard, NavCard, NavCardGrid, Form, FormActions, PageHeader
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
- `className` - Additional CSS classes

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
- `options` - Array of `{ value, label }` objects
- `value` - Selected value (bindable)
- `placeholder` - Placeholder text
- `disabled` - Disable select

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
- `disabled` - Disable switch

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

### Button

```svelte
<script>
  import { Button } from "@decodelabs/underlay";
</script>

<!-- Primary button -->
<Button variant="primary" type="submit">
  Save Changes
</Button>

<!-- Secondary button -->
<Button variant="secondary" on:click={handleCancel}>
  Cancel
</Button>

<!-- Subtle/ghost button -->
<Button variant="subtle" on:click={handleReset}>
  Reset
</Button>

<!-- Square (non-pill) button -->
<Button pill={false}>
  Non-rounded
</Button>
```

**Props:**
- `variant` - `"primary"` | `"secondary"` | `"subtle"` (default: `"primary"`)
- `type` - `"button"` | `"submit"` | `"reset"` (default: `"button"`)
- `pill` - Rounded corners (default: `true`)
- `disabled` - Disable button
- `className` - Additional CSS classes

**Events:**
- `on:click` - Click handler

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

Container for form action buttons with consistent spacing:

```svelte
<FormActions>
  <Button type="submit">Save</Button>
  <Button variant="secondary" type="reset">Reset</Button>
</FormActions>
```

### ListCard

List display component with pagination support:

```svelte
<script>
  import { ListCard, Button } from "@decodelabs/underlay";
  
  export let data;
  
  const items = data.articles;
</script>

<ListCard title="Articles" actions={
  <Button on:click={() => goto("/articles/new")}>
    New Article
  </Button>
}>
  {#each items as article}
    <div class="list-item">
      <h3>{article.title}</h3>
      <p>{article.summary}</p>
      <a href="/articles/{article.id}">View</a>
    </div>
  {/each}
  
  {#if items.length === 0}
    <p class="empty-state">No articles found.</p>
  {/if}
</ListCard>
```

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

Page header component with title, optional subtitle, back link, and action buttons. Use this for consistent page headers across your application.

```svelte
<script>
  import { PageHeader } from "@decodelabs/underlay/patterns";
  import { Button } from "@decodelabs/underlay/components";
</script>

<!-- Simple header -->
<PageHeader title="Dashboard" />

<!-- With back link (appears below titles) -->
<PageHeader 
  title="Edit Article" 
  backHref="/articles" 
  backLabel="Back to articles" 
/>

<!-- With subtitle (for database-provided names) -->
<PageHeader 
  title="Edit Module" 
  subtitle="Introduction to Financial Accounting"
  backHref="/modules"
  backLabel="Back to modules"
/>

<!-- With actions -->
<PageHeader title="Users">
  {#snippet actions()}
    <Button href="/users/new">Add User</Button>
  {/snippet}
</PageHeader>

<!-- With meta information -->
<PageHeader 
  title="Article" 
  subtitle="Getting Started with Svelte"
  backHref="/articles"
>
  <p>Last updated: 2024-01-15</p>
  <p>Status: <code>published</code></p>
</PageHeader>
```

**Props:**
- `title` - Main page title (h1, required)
- `subtitle` - Secondary title for database-provided names (h2, optional)
- `backHref` - URL for back link (optional)
- `backLabel` - Text for back link (default: "Back")
- `actions` - Snippet for action buttons (optional)
- `children` - Snippet for meta information below header (optional)

**Layout:**
The component renders in this order:
1. Title (h1)
2. Subtitle (h2, if provided)
3. Back link (if provided)
4. Actions (aligned right of the title row)
5. Meta content (children, below the header)

**Usage Guidelines:**
- Use `title` for deterministic, static page names (e.g., "Edit Module", "Users", "Settings")
- Use `subtitle` for database-provided names (e.g., the module name, user name, etc.)
- The subtitle appears smaller and muted below the main title
- The back link appears below titles for clear visual hierarchy
- Action buttons align to the right of the title row
- Meta content (via children) appears below the title block in muted text

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

- [100-frontend-web](./100-frontend-web.md) - Frontend application patterns
- [110-admin](./110-admin.md) - Admin interface patterns
- [075-validation](./075-validation.md) - Form validation with UI kit

---

## Reference

See Underlay components source:
- Components: `underlay/ts/src/components/`
- Patterns: `underlay/ts/src/patterns/`
- Styles: `underlay/ts/src/styles/`
