# 110 - Admin Frontend (Admin Pattern)

This document covers creating the admin/author SvelteKit frontend following the admin frontend pattern.

## Admin Frontend Structure

The admin frontend uses **layout groups** to separate authenticated routes (with sidebar) from unauthenticated routes (login/register with minimal centered layout).

```
apps/admin/src/
├── app.html                  # HTML shell
├── app.d.ts                  # TypeScript declarations with Locals
├── hooks.server.ts           # Server hooks for auth
├── routes/
│   ├── +layout.svelte        # Minimal root layout (CSS vars, body reset)
│   ├── (app)/                # Authenticated routes WITH sidebar
│   │   ├── +layout.svelte    # App shell with sidebar navigation
│   │   ├── +layout.server.ts # Auth check, data loading
│   │   ├── +page.svelte      # Dashboard
│   │   ├── account/
│   │   ├── content/
│   │   ├── learning/
│   │   ├── logout/
│   │   └── system/
│   └── (auth)/               # Unauthenticated routes (no sidebar)
│       ├── +layout.svelte    # Centered card layout
│       └── login/
│           ├── +page.svelte
│           └── +page.server.ts
└── lib/
    ├── api/
    │   └── client.ts         # Client factory
    └── components/
```

## Layout Groups Pattern

SvelteKit layout groups (directories wrapped in parentheses) allow different routes to use completely different layouts while sharing the same root.

### Why Layout Groups?

- **Auth routes** (login, register) should show a minimal centered layout without navigation
- **App routes** (dashboard, content, etc.) should show the full admin shell with sidebar
- Both should share the same design tokens and global styles

### Root Layout (Minimal)

The root `+layout.svelte` contains only shared CSS variables and global body styles. It imports the Underlay CSS **before** custom `:root` overrides so the overrides take precedence.

```svelte
<!-- apps/admin/src/routes/+layout.svelte -->
<script lang="ts">
  import favicon from "$lib/assets/favicon.svg";
  // Import Underlay CSS FIRST so :root overrides take precedence
  import "@decodelabs/underlay/styles/tokens.css";
  import "@decodelabs/underlay/styles/forms.css";

  let { children } = $props();
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

{@render children()}

<style>
  :global(:root) {
    /* App-specific design tokens */
    --admin-color-bg: #2a3037;
    --admin-color-surface: #050608;
    --admin-color-surface-subtle: #16181d;
    --admin-color-surface-card: #101318;
    --admin-color-border-subtle: rgba(255, 255, 255, 0.1);
    --admin-color-text: #f9fafb;
    --admin-color-text-muted: #9ca3af;
    --admin-color-accent: #14b8a6;

    /* Map to Underlay tokens */
    --underlay-color-bg-surface: var(--admin-color-surface);
    --underlay-color-text: var(--admin-color-text);
    --underlay-color-surface-muted: var(--admin-color-surface-card);
    --underlay-color-on-surface: var(--admin-color-text);
    --underlay-color-primary: var(--admin-color-accent);
    --underlay-color-primary-strong: #0f766e;
    --underlay-color-border-subtle: var(--admin-color-border-subtle);
  }

  :global(html) {
    font-size: 112.5%;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    background-color: var(--admin-color-bg);
    color: var(--admin-color-text);
    font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  }
</style>
```

### App Layout (With Sidebar)

The `(app)/+layout.svelte` contains the full admin shell with sidebar navigation. It inherits from the root layout automatically.

```svelte
<!-- apps/admin/src/routes/(app)/+layout.svelte -->
<script lang="ts">
  import { setContext } from "svelte";
  import { ToastHost } from "@decodelabs/underlay/components";
  import { UNDERLAY_TOASTS_CONTEXT_KEY, createToastStore } from "@decodelabs/underlay/patterns";
  import AdminNav from "$lib/ui/AdminNav.svelte";

  let { children, data } = $props();

  const toastStore = createToastStore();
  setContext(UNDERLAY_TOASTS_CONTEXT_KEY, toastStore);
</script>

<div class="admin-shell">
  <nav class="admin-nav">
    <AdminNav currentSection={data?.currentSection} />
  </nav>
  <div class="admin-content">
    <main>
      {@render children()}
    </main>
  </div>
  <ToastHost store={toastStore} />
</div>

<style>
  /* App shell body overrides */
  :global(body) {
    padding: 0 1rem;
    overflow: hidden;
    height: 100vh;
  }

  .admin-shell {
    display: grid;
    grid-template-columns: 264px minmax(0, 1fr);
    height: calc(100vh - 4rem);
    max-width: 1650px;
    margin: 2rem auto;
    border-radius: 1.5rem;
    overflow: hidden;
    background: var(--admin-color-surface);
    box-shadow: 0 26px 70px rgba(0, 0, 0, 0.75);
  }

  .admin-nav {
    background-color: var(--admin-color-surface-subtle);
    padding: 1.1rem 0 1.1rem 0.9rem;
    overflow: hidden;
  }

  .admin-content {
    padding: 1.25rem 1.6rem;
    overflow-y: auto;
  }
</style>
```

### App Layout Server (Auth Check)

The `(app)/+layout.server.ts` handles authentication and redirects unauthenticated users to login.

```typescript
// apps/admin/src/routes/(app)/+layout.server.ts
import type { LayoutServerLoad } from "./$types";
import { redirect } from "@sveltejs/kit";
import { authCommands } from "@myorg/client";

export const load: LayoutServerLoad = async ({ locals, url, fetch }) => {
  // Redirect unauthenticated users to login
  if (!locals.isAuthenticated) {
    throw redirect(302, "/login");
  }

  // Determine current section for nav highlighting
  const currentSection =
    url.pathname.startsWith("/learning") ? "learning" :
    url.pathname.startsWith("/content") ? "content" :
    url.pathname.startsWith("/account") ? "account" :
    null;

  // Fetch current user data
  let currentUser = null;
  if (locals.authToken) {
    try {
      currentUser = await authCommands.me(fetch, locals.authToken);
    } catch {
      currentUser = null;
    }
  }

  return {
    isAuthenticated: locals.isAuthenticated,
    authToken: locals.authToken,
    currentUser,
    currentSection,
  };
};
```

### Auth Layout (Centered, No Sidebar)

The `(auth)/+layout.svelte` provides a minimal centered layout for login/register pages. It inherits from the root layout (getting the design tokens) but has no sidebar.

```svelte
<!-- apps/admin/src/routes/(auth)/+layout.svelte -->
<script lang="ts">
  let { children } = $props();
</script>

<div class="auth-layout">
  <div class="auth-layout__card">
    {@render children()}
  </div>
</div>

<style>
  .auth-layout {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1rem;
    box-sizing: border-box;
  }

  .auth-layout__card {
    width: 100%;
    max-width: 24rem;
  }
</style>
```

## App.d.ts (Locals Pattern)

```typescript
declare module "*.svelte" {
  const component: any;
  export default component;
}

declare global {
  namespace App {
    interface Locals {
      authToken: string | null;
      isAuthenticated: boolean;
    }
  }
}

export {};
```

## Server Hooks

Create `apps/admin/src/hooks.server.ts`:

```typescript
import type { Handle } from "@sveltejs/kit";

export const handle: Handle = async ({ event, resolve }) => {
  const token = event.cookies.get("myapp_admin_token") ?? null;

  event.locals.authToken = token;
  event.locals.isAuthenticated = token != null;

  return resolve(event);
};
```

## Client Factory

Create `apps/admin/src/lib/api/client.ts`:

```typescript
import { createClient as createApiClient } from "@myorg/client";
import { env } from "$env/dynamic/public";

const baseUrl = env.PUBLIC_API_URL ?? "http://127.0.0.1:3000";
const apiVersion = env.PUBLIC_API_VERSION ?? "2025-01-01";

export function createAdminClient(
  fetchFn: typeof fetch,
  authToken: string | null | undefined
) {
  return createApiClient({
    baseUrl,
    apiVersion,
    fetchFn,
    getToken: () => authToken ?? null
  });
}
```

## Admin Detail Page Pattern

Admin detail pages for entities (pathways, modules, etc.) typically use a tabbed interface to organize different views and related data. This section documents the standard patterns.

### List Display Patterns

There are two distinct patterns for displaying lists of related entities. Choosing the wrong one creates confusion and inconsistent UX.

#### Pattern 1: Tab Content Lists (Full ListCard Pattern)

**When to use**: Displaying lists of entities in their own tab (e.g., "Sections" tab, "Topics" tab, "Variants" tab).

**Components**: `PageHeader` (level=3) + `FilterBar` + `ListGrid` + entity-specific `*ListCard`

**Structure**:
```svelte
<PageHeader title="Sections" level={3}>
  {#snippet actions()}
    <Button variant="primary" onclick={addItem}>
      <Plus size={16} />
      Add Section
    </Button>
  {/snippet}
</PageHeader>

{#if items.length > 0}
  <FilterBar title="Filter">
    <Field label="Search" forId="search">
      <TextInput id="search" bind:value={searchFilter} search />
    </Field>
  </FilterBar>
{/if}

{#if filteredItems.length === 0}
  <p class="empty">No items.</p>
{:else}
  <ListGrid minItemWidth={26}>
    {#each filteredItems as item}
      <EntityListCard {item} {sourceContext} />
    {/each}
  </ListGrid>
{/if}
```

**File organization**:
- Create `EntityListCard.svelte` in `$lib/cards/` (e.g., `SectionListCard.svelte`, `TopicListCard.svelte`)
- Create `EntityTabContent.svelte` in the route folder (e.g., `SectionsTabContent.svelte`)
- Export the ListCard from `$lib/cards/index.ts`

#### Pattern 2: Detail Page Auxiliary Lists (InlineListCard)

**When to use**: Displaying small, secondary lists alongside other detail content (e.g., showing associated modules on a Bundle's Details tab, showing aliases on a Module's Details tab).

**Components**: `InlineListCard` + `InlineListItem` (often wrapped in `ContainerGrid` with other content)

**Structure**:
```svelte
<ContainerGrid>
  <DetailsGrid>
    <!-- Main entity details -->
  </DetailsGrid>

  <InlineListCard
    title="Related Items"
    emptyMessage="No items."
    hasItems={items.length > 0}
  >
    {#snippet action()}
      <IconButton label="Add" onclick={addItem}>
        <Plus size={16} />
      </IconButton>
    {/snippet}

    {#each items as item}
      <InlineListItem label={item.name} onclick={() => edit(item)}>
        {#snippet trailing()}
          <Pill>{item.status}</Pill>
        {/snippet}
      </InlineListItem>
    {/each}
  </InlineListCard>
</ContainerGrid>
```

**When InlineListCard is appropriate**:
- The list is auxiliary/secondary information (not the main focus of a tab)
- Items are typically edited via dialog rather than dedicated pages
- The list appears alongside DetailsGrid or other components
- Expected item count is small (< 10-15 items)

**When InlineListCard is NOT appropriate**:
- The list is the primary content of a tab
- Items have their own detail/edit pages
- The list needs filtering/search
- Expected item count is large

### Page Structure

A typical detail page has:
1. **PageHeader** - Title, subtitle, back navigation, and entity-level actions
2. **TabsRoot** - Container for tabbed content
3. **TabsList/TabsTrigger** - Tab navigation
4. **TabsContent** - Content for each tab

```svelte
<PageHeader
  title={entity.code}
  subtitle={entity.parentName}
  backHref={backInfo.href}
  backLabel={backInfo.label}
>
  {#snippet actions()}
    <EntityActionsMenu entity={entity} authToken={authToken} />
  {/snippet}
</PageHeader>

<TabsRoot bind:value={activeTab} variant="boxed" historyKey="tab">
  <TabsList>
    <TabsTrigger value="details">Details</TabsTrigger>
    <TabsTrigger value="children">Children</TabsTrigger>
    <TabsTrigger value="related">Related</TabsTrigger>
  </TabsList>

  <TabsContent value="details">
    <!-- Entity details -->
  </TabsContent>

  <TabsContent value="children">
    <!-- List of child entities -->
  </TabsContent>
</TabsRoot>
```

### Tab Content with Lists

When a tab displays a list of related entities (e.g., Sections within a Module), use the following structure:

1. **PageHeader (level 3)** - Section title with action buttons
2. **FilterBar** - Optional search/filter controls (only when items exist)
3. **ListGrid** - Grid of entity cards, or ReorderableList when in reorder mode

```svelte
<PageHeader title="Sections" level={3}>
  {#snippet actions()}
    {#if canReorder}
      <Button
        variant={isReorderMode ? "danger" : "subtle"}
        onclick={toggleReorderMode}
      >
        <ArrowUpDown size={16} />
        Reorder
      </Button>
    {/if}
    <Button
      variant="primary"
      onclick={() => gotoWithContext(addUrl, sourceContext)}
    >
      <Plus size={16} />
      Add Section
    </Button>
  {/snippet}
</PageHeader>

{#if items.length > 0}
  <FilterBar title="Filter">
    <Field label="Search" forId="search">
      <TextInput
        id="search"
        placeholder="Filter by title..."
        bind:value={searchFilter}
        debounce={300}
        search
      />
    </Field>
  </FilterBar>
{/if}

{#if filteredItems.length === 0}
  <p class="empty-message">
    {searchFilter ? "No items match your filter." : "No items yet."}
  </p>
{:else if isReorderMode}
  <ReorderableList
    controller={reorderController}
    oncancel={exitReorderMode}
    onsuccess={handleReorderSuccess}
  >
    {#snippet item(item)}
      <ListCard title={item.title} variant="compact" showDragHandle />
    {/snippet}
  </ReorderableList>
{:else}
  <ListGrid minItemWidth={26}>
    {#each filteredItems as item}
      <EntityListCard {item} {sourceContext} onRequestDelete={handleDelete} />
    {/each}
  </ListGrid>
{/if}
```

### List Card Actions

Entity list cards should use `CopyActionsMenu` with:
- **Copy actions**: Key, ID, slug (when present)
- **Edit action**: Navigate to edit page with context
- **Delete action**: Trigger soft delete confirmation (format: "Soft delete {entityType}")

```svelte
<ListCard title={`Section ${section.label}`} subtitle={section.title}>
  {#snippet actions({ trigger })}
    <CopyActionsMenu
      {trigger}
      copies={[
        { label: "Copy key", text: section.key },
        { label: "Copy id", text: section.sectionId },
        ...(section.slug ? [{ label: "Copy slug", text: section.slug }] : [])
      ]}
      actions={[
        {
          label: "Edit section",
          onSelect: () => gotoWithContext(editHref, sourceContext)
        },
        {
          label: "Soft delete section",
          destructive: true,
          onSelect: () => onRequestDelete(section.sectionId)
        }
      ]}
    />
  {/snippet}
</ListCard>
```

### Key Conventions

1. **Tab-level PageHeader**: Use `level={3}` for section headers within tabs
2. **Action button order**: Reorder (when applicable), then Add
3. **Reorder button state**: Use `variant="danger"` when active, `variant="subtle"` when inactive
4. **Filter visibility**: Only show FilterBar when there are items to filter
5. **Delete label format**: "Soft delete {entityType}" (e.g., "Soft delete section")
6. **Empty states**: Differentiate between "no items" and "no matches for filter"

## Complete CRUD Admin Pattern

When implementing a full admin section for a new entity type, follow this comprehensive structure. **Do NOT use dialogs for creating/editing entities** - always use dedicated routes.

### Route Structure

For an entity `Topic` nested under `Bundle`:

```
routes/(app)/learning/bundles/[bundleId]/
├── +page.svelte           # Bundle detail with Topics tab
├── topics/
│   ├── new/
│   │   └── +page.svelte   # Create topic form
│   └── [topicId]/
│       ├── +page.svelte   # Topic detail view
│       └── edit/
│           └── +page.svelte   # Edit topic form
```

### File Checklist

When creating a new admin entity, create these files in order:

1. **`$lib/cards/EntityListCard.svelte`** - List card for displaying items in a grid
2. **`$lib/forms/learning/EntityForm.svelte`** - Reusable form component
3. **`routes/.../EntityTabContent.svelte`** - Tab content with list view (navigation only, NO dialogs)
4. **`routes/.../new/+page.svelte`** - Create page using SpaFormShell
5. **`routes/.../[entityId]/+page.svelte`** - Detail view page
6. **`routes/.../[entityId]/edit/+page.svelte`** - Edit page using SpaFormShell

### Form Component Pattern

Forms should NOT contain `<form>` elements or submission logic. They render fields and use hidden inputs for values:

```svelte
<script lang="ts">
  import {
    ConfirmAction,
    Field,
    FieldSet,
    FormActions,
    FormValidationProvider,
    SaveSplitButton,
    Switch,
    TextButton,
    TextInput
  } from "@decodelabs/underlay/components";
  import { navigateOnCancel } from "@decodelabs/underlay/client";

  interface Props {
    mode?: "create" | "edit";
    values?: { name?: string; isLive?: boolean; };
    intent?: "save" | "save-close";
    errors?: Record<string, string> | null;
    cancelHref?: string;
    returnTo?: string;
    submitting?: boolean;
  }

  let {
    mode = "create",
    values = {},
    intent = $bindable("save-close"),
    errors = null,
    cancelHref = undefined,
    returnTo = undefined,
    submitting = false
  }: Props = $props();

  let nameValue = $state(values.name ?? "");
  let isLiveValue = $state(values.isLive ?? false);
  let isFormValid = $state(false);

  function handleCancel() {
    navigateOnCancel(cancelHref);
  }

  function handleDeleteConfirm() {
    const form = document.getElementById('entity-delete-form') as HTMLFormElement | null;
    form?.requestSubmit();
  }

  const dangerItems = $derived(mode === "edit"
    ? [
        { label: "Cancel", onSelect: handleCancel, destructive: false },
        { label: "Soft delete entity", onSelect: () => { /* open dialog */ }, destructive: true }
      ]
    : [
        { label: "Cancel", onSelect: handleCancel, destructive: false }
      ]);
</script>

<FormValidationProvider bind:isValid={isFormValid}>
  <FieldSet legend="Details">
    <Field label="Name" error={errors?.name} required>
      <TextInput name="name" bind:value={nameValue} required />
    </Field>
  </FieldSet>

  <FieldSet legend="Status">
    <Field label="Visibility">
      <input type="hidden" name="isLive" value={isLiveValue ? "true" : "false"} />
      <Switch leftLabel="Draft" rightLabel="Live" bind:checked={isLiveValue} />
    </Field>
  </FieldSet>
</FormValidationProvider>

<FormActions align="start" {dangerItems}>
  {#snippet danger()}
    <TextButton type="button" onclick={handleCancel} disabled={submitting}>
      Cancel
    </TextButton>
    {#if mode === "edit"}
      <ConfirmAction
        triggerLabel="Soft delete entity"
        triggerVariant="danger"
        onConfirm={handleDeleteConfirm}
      />
    {/if}
  {/snippet}

  <input type="hidden" name="intent" value={intent} />
  {#if returnTo}
    <input type="hidden" name="returnTo" value={returnTo} />
  {/if}

  <SaveSplitButton type="submit" mode={mode} disabled={submitting} bind:intent />
</FormActions>
```

### Create/Edit Page Pattern

Both create and edit pages use `SpaFormShell`:

```svelte
<script lang="ts">
  import { goto } from "$app/navigation";
  import { entityCommands } from "@client";
  import EntityForm from "$lib/forms/learning/EntityForm.svelte";
  import SpaFormShell from "@decodelabs/underlay/patterns/SpaFormShell";
  import type { SpaFormResult } from "@decodelabs/underlay/patterns/spa-form-types";
  import { consumeNavigationContext, submitFormWithIntent } from "@decodelabs/underlay/patterns";

  const { backInfo, returnTo } = consumeNavigationContext("Back", defaultBackHref);

  let intent = $state<"save" | "save-close">("save-close");
  let success = $state<boolean | null>(null);
  let error = $state<string | null>(null);
  let fieldErrors = $state<Record<string, string> | null>(null);

  async function handleSubmit(formData: FormData): Promise<SpaFormResult> {
    const formIntent = String(formData.get("intent") ?? "save-close");

    // Handle delete intent (edit mode only)
    if (formIntent === "delete") {
      await entityCommands.softDelete(entityId, fetch, token);
      return { success: true, redirectTo: listUrl };
    }

    // Validate and save
    const name = String(formData.get("name") ?? "").trim();
    if (!name) {
      return { success: false, fieldErrors: { name: "Required" } };
    }

    await entityCommands.create({ name }, fetch, token);

    if (formIntent === "save-close") {
      return { success: true, redirectTo: listUrl };
    }
    return { success: true, redirectTo: editUrl };
  }

  function handleResult(result: SpaFormResult) {
    success = result.success;
    error = result.error ?? null;
    fieldErrors = result.fieldErrors ?? null;
  }

  function handleDelete() {
    submitFormWithIntent("delete");
  }
</script>

<SpaFormShell
  title="New Entity"
  backHref={backInfo.href}
  backLabel={backInfo.label}
  success={success === true}
  error={success === false && !fieldErrors ? error : null}
  {fieldErrors}
  onSubmit={handleSubmit}
  onResult={handleResult}
  navigate={goto}
>
  <EntityForm mode="create" {errors} cancelHref={backInfo.href} {returnTo} bind:intent />
</SpaFormShell>

<!-- For edit pages: hidden delete form -->
<form id="entity-delete-form" style="display: none;" onsubmit={(e) => { e.preventDefault(); handleDelete(); }}>
  <input type="hidden" name="intent" value="delete" />
</form>
```

### Tab Content Pattern (List View)

Tab content should ONLY handle navigation and display - no forms or dialogs:

```svelte
<script lang="ts">
  import { FilterBar, PageHeader, type NavigationContext } from "@decodelabs/underlay/patterns";
  import { Button, Field, ListGrid, TextInput } from "@decodelabs/underlay/components";
  import { EntityListCard } from "$lib/cards";
  import { gotoWithContext } from "@decodelabs/underlay/client";
  import Plus from "lucide-svelte/icons/plus";

  interface Props {
    items: Entity[];
    parentId: string;
    sourceContext: NavigationContext;
    onRequestDelete?: (id: string) => void;
  }

  let { items, parentId, sourceContext, onRequestDelete }: Props = $props();

  let searchFilter = $state("");
  const filteredItems = $derived(/* filter logic */);
  const addUrl = $derived(`/learning/parents/${parentId}/entities/new`);
</script>

<PageHeader title="Entities" level={3}>
  {#snippet actions()}
    <Button variant="primary" onclick={() => gotoWithContext(addUrl, sourceContext)}>
      <Plus size={16} />
      Add Entity
    </Button>
  {/snippet}
</PageHeader>

{#if items.length > 0}
  <FilterBar title="Filter">
    <Field label="Search" forId="search">
      <TextInput id="search" bind:value={searchFilter} search />
    </Field>
  </FilterBar>
{/if}

<ListGrid minItemWidth={26}>
  {#each filteredItems as item}
    <EntityListCard {item} {parentId} {sourceContext} {onRequestDelete} />
  {/each}
</ListGrid>
```

### Detail Page Header Pattern

Detail pages show entity metadata in the PageHeader subtitle area:

```svelte
<PageHeader title={entity.name} backHref={backInfo.href} backLabel={backInfo.label}>
  <p class="entity-meta">
    <strong>ID:</strong> <code>{entity.id}</code>
    <span class="header-separator">·</span>
    <StatusBadge value={entity.isFree} trueLabel="Free" falseLabel="Restricted">
      {#snippet trueIcon()}<LockOpen size={14} />{/snippet}
      {#snippet falseIcon()}<Lock size={14} />{/snippet}
    </StatusBadge>
    <span class="header-separator">·</span>
    <StatusBadge value={entity.isLive} trueLabel="Live" falseLabel="Draft" variant="danger">
      {#snippet trueIcon()}<LockOpen size={14} />{/snippet}
      {#snippet falseIcon()}<Lock size={14} />{/snippet}
    </StatusBadge>
  </p>
</PageHeader>
```

**Important**: `StatusBadge` requires icon snippets for both states when you want icons displayed.

### List Card Trailing Pills

In ListCards, pills should only show **positive/notable states**. Don't show both states:

```svelte
{#snippet trailing()}
  {#if item.isFree}
    <Pill accent="#22c55e">Free</Pill>
  {/if}
  <!-- Do NOT show "Restricted" pill when isFree is false -->
{/snippet}
```

### Markdown Content Fields

For long-form markdown fields (descriptions, notes, etc.), use `ContentCard` instead of `DetailsItem`:

```svelte
<TabsContent value="details">
  <ContainerGrid>
    <DetailsGrid>
      <!-- Short fields in DetailsGrid -->
    </DetailsGrid>
    <InlineListCard><!-- Related items --></InlineListCard>
  </ContainerGrid>

  <!-- Markdown content OUTSIDE ContainerGrid -->
  <ContentCard
    title="Description"
    value={entity.description}
    markdown
    emptyMessage="No description set."
    maxHeight={0}
  />
</TabsContent>
```

### Navigation Context with Tabs

When a detail page has tabs, `sourceContext.href` must include the current tab so back navigation returns to the correct tab:

```svelte
let activeTab = $state("details");

// Include tab in href when not on default tab
const sourceContext = $derived(entity ? {
  label: entity.name,
  href: `/learning/entities/${entity.id}${activeTab !== "details" ? `?tab=${activeTab}` : ""}`,
  type: "detail" as const
} : null);
```

### Common Mistakes to Avoid

1. **Using dialogs for create/edit** - Always use dedicated routes
2. **Putting form submission in tab content** - Tab content is for list display only
3. **Using InlineListCard for tab content** - InlineListCard is only for auxiliary lists on detail pages
4. **Forgetting delete handling** - Parent page needs AlertDialog for list deletion
5. **Missing hidden form for delete** - Edit pages need `<form id="entity-delete-form">`
6. **Using onsubmit|preventDefault** - Use `onsubmit={(e) => { e.preventDefault(); ... }}`
7. **StatusBadge without icon snippets** - Always provide `trueIcon` and `falseIcon` snippets
8. **Showing both Pill states** - Only show pills for positive/notable states (e.g., "Free"), not negative defaults
9. **sourceContext.href missing tab** - Include `?tab={activeTab}` for proper back navigation from child routes
10. **Description in DetailsItem** - Use `ContentCard` with `markdown` prop for long-form content

## Key Points

1. **CSS Import Order**: Import Underlay CSS in the root layout BEFORE custom `:root` styles so your overrides take precedence.

2. **No `@` Reset Needed**: Since all routes inherit from the minimal root layout, you don't need `+layout@.svelte` to break out of a parent layout. The layout groups naturally separate the concerns.

3. **Auth Logic in (app)**: Only the `(app)` layout group has auth checking. The `(auth)` group is accessible without authentication.

4. **Shared Design Tokens**: Both layout groups inherit from root, so they share the same design tokens and styling.

## Next Steps

- [120-configuration.md](./120-configuration.md)
