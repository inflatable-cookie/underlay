# Autonomous List Components

This guide covers the **Autonomous List Component** pattern - a set of hooks and components that enable list views to be fully self-contained, handling their own data fetching, state management, and batch operations.

## Overview

The Autonomous List Component pattern solves the "god file" problem where parent pages accumulate excessive business logic from managing multiple entity types across tabs.

Storybook coverage:
- Poodle `LogList`
- Poodle `BulkActionBar`
- `Patterns/CopyActionsMenu`

Run `effigy storybook` from the repo root to inspect the retained batch/list helper surface interactively.

**Traditional approach problems:**
- Parent pages fetch ALL data for ALL tabs upfront
- Parent pages define individual CRUD handlers for each entity type
- Parent pages manage dialog state for each entity type
- Tab content components are "dumb" renderers that callback to parent
- Same entity has different code for root list vs tab view

**Autonomous approach benefits:**
- Each list component fetches its own data based on filter props
- Single component works in both root page (`variant="page"`) and tab contexts (`variant="tab"`)
- Batch operations are built into each list component
- Parent pages become thin coordinators
- Adding new actions happens in one place per entity

## Component Interface Contract

Use a consistent props contract for all autonomous list components:

```ts
interface EntityListProps {
  // Context filters. More specific scopes override broader scopes.
  pathwayId?: string;
  moduleId?: string;
  sectionId?: string;
  areaId?: string;
  outcomeId?: string;

  // Shared navigation context for redirects/back links.
  sourceContext: NavigationContext;

  // UI presentation.
  variant?: "page" | "tab";

  // Optional parent coordination callback.
  onDataChange?: () => void;
}
```

### Variant Rules (`page` vs `tab`)

- `variant="page"`: use root-page framing (`h1`, full spacing, root actions visible).
- `variant="tab"`: use embedded framing (`h3`, tighter spacing, context-reduced actions).
- Keep data and action behavior identical across variants; only presentation/context should differ.

### Filter Precedence Rules

When multiple context filters are present, apply the narrowest scope:

1. `outcomeId`
2. `areaId`
3. `sectionId`
4. `moduleId`
5. `pathwayId`

If a narrower filter is set, ignore broader filters in the query call to avoid ambiguous fetch behavior.

## Core Hooks

### `useBatchSelection`

**Location:** `@decodelabs/underlay/patterns`

Basic selection state management for multi-select list operations. Use this when you only need selection without registered batch actions.

```svelte
<script lang="ts">
  import { useBatchSelection } from '@decodelabs/underlay/patterns';
  import { AlertDialog, BulkActionBar } from '@poodle/svelte-primitives';

  const items = $derived(data.projects);
  const selection = useBatchSelection<string>();
  let showBatchDeleteConfirm = $state(false);

  async function handleBatchDelete() {
    const ids = selection.selectedIds;
    await deleteItems(ids);
    selection.clear();
  }
</script>

{#each items as item}
  <input
    type="checkbox"
    checked={selection.isSelected(item.id)}
    onchange={(e) => selection.toggle(item.id, e.currentTarget.checked)}
  />
{/each}

<BulkActionBar
  selectionCount={selection.count}
  totalCount={items.length}
  actions={[{ id: "delete", label: "Delete", icon: "trash-2", tone: "danger" }]}
  showSelectAll
  on:clear={selection.clear}
  on:selectAll={() => selection.selectAll(items.map(i => i.id))}
  on:action={() => { showBatchDeleteConfirm = true; }}
/>

<AlertDialog
  open={showBatchDeleteConfirm}
  title="Delete selected items"
  description={`Delete ${selection.count} selected item${selection.count === 1 ? "" : "s"}?`}
  confirmLabel={`Delete ${selection.count} item${selection.count === 1 ? "" : "s"}`}
  tone="danger"
  onConfirm={handleBatchDelete}
  onCancel={() => { showBatchDeleteConfirm = false; }}
/>
```

#### API Reference

```typescript
interface BatchSelectionResult<T> {
  /** Current selected IDs as an array (reactive) */
  readonly selectedIds: T[];

  /** Number of selected items (reactive) */
  readonly count: number;

  /** Whether any items are selected (reactive) */
  readonly hasSelection: boolean;

  /** Check if an item is selected */
  isSelected: (id: T) => boolean;

  /** Toggle an item's selection state */
  toggle: (id: T, selected: boolean) => void;

  /** Select a single item */
  select: (id: T) => void;

  /** Deselect a single item */
  deselect: (id: T) => void;

  /** Select all items from the provided list */
  selectAll: (ids: T[]) => void;

  /** Clear all selections */
  clear: () => void;

  /** Replace the selection with a new set of IDs */
  set: (ids: T[]) => void;
}
```

---

### `useBatchActions`

**Location:** `@decodelabs/underlay/patterns`

Extends `useBatchSelection` with action registration and execution. This is the recommended hook for autonomous list components.

```svelte
<script lang="ts">
  import { useBatchActions } from '@decodelabs/underlay/patterns';
  import { AlertDialog, BulkActionBar } from '@poodle/svelte-primitives';

  const batch = useBatchActions<string>();

  // Register actions (do this outside $effect, typically at component init)
  batch.registerAction({
    id: 'delete',
    label: 'Delete',
    variant: 'danger',
    confirm: {
      title: 'Delete Items',
      description: (count) => `Are you sure you want to delete ${count} items?`,
      confirmLabel: 'Delete',
    },
    execute: async (ids) => {
      await deleteItems(ids);
      return { success: true, affected: ids.length };
    },
  });
</script>

{#each items as item}
  <ListCard
    selected={batch.isSelected(item.id)}
    onSelectionChange={(selected) => batch.toggle(item.id, selected)}
  />
{/each}

<!-- Batch action bar (fixed at bottom of screen when items selected) -->
<BulkActionBar
  selectionCount={batch.count}
  totalCount={items.length}
  actions={batch.availableActions.map((action) => ({
    id: action.id,
    label: action.label,
    icon: action.icon,
    tone: action.variant === "danger" ? "danger" : action.variant === "warning" ? "warning" : "default"
  }))}
  showSelectAll
  on:clear={() => batch.clear()}
  on:selectAll={() => batch.selectAll(items.map(i => i.id))}
  on:action={(event) => batch.requestAction(event.detail.id)}
/>

<!-- Confirmation dialog for pending actions -->
{#if batch.pendingAction}
  <AlertDialog
    open={true}
    title={batch.pendingAction.confirm?.title ?? 'Confirm'}
    description={batch.getConfirmDescription()}
    confirmLabel={batch.pendingAction.confirm?.confirmLabel ?? 'Confirm'}
    cancelLabel="Cancel"
    onConfirm={async () => { await batch.confirmPendingAction(); }}
    onCancel={() => batch.cancelPendingAction()}
    tone="danger"
  />
{/if}
```

This example now uses Poodle `ListCard` directly. The earlier `g01.046`
reassessment moved the generic card behavior into Poodle, and `g01.058`
retired public Underlay `AutonomousList`, so list assembly should now compose
directly over Poodle list surfaces plus lower-level Underlay state helpers.

#### Action Registration

Actions are registered with the following structure:

```typescript
interface BatchAction<T = string> {
  /** Unique identifier for the action */
  id: string;

  /** Display label for the action */
  label: string;

  /** Optional icon component (from lucide-svelte) */
  icon?: Component;

  /** Visual variant: "default" | "danger" | "warning" */
  variant?: string;

  /** Optional function to check if action is available for current selection */
  isAvailable?: (selectedIds: T[]) => boolean;

  /** Execute the action on selected items */
  execute: (selectedIds: T[]) => Promise<BatchActionResult>;

  /** Optional confirmation dialog configuration */
  confirm?: {
    title: string;
    description: string | ((count: number) => string);
    confirmLabel?: string;
    cancelLabel?: string;
  };
}

interface BatchActionResult {
  success: boolean;
  affected: number;
  message?: string;
}
```

#### API Reference

Includes all methods from `useBatchSelection`, plus:

```typescript
interface BatchActionsResult<T> {
  // ... selection methods ...

  /** Registered actions (reactive) */
  readonly actions: BatchAction<T>[];

  /** Currently available actions based on selection (reactive) */
  readonly availableActions: BatchAction<T>[];

  /** Action pending confirmation (reactive) */
  readonly pendingAction: BatchAction<T> | null;

  /** Whether an action is currently executing (reactive) */
  readonly executing: boolean;

  /** Error message from last failed action (reactive) */
  readonly error: string | null;

  /** Register a batch action */
  registerAction: (action: BatchAction<T>) => void;

  /** Unregister a batch action */
  unregisterAction: (actionId: string) => void;

  /** Request an action (shows confirmation if needed, otherwise executes) */
  requestAction: (actionId: string) => Promise<BatchActionResult | null>;

  /** Get the confirmation description for the pending action */
  getConfirmDescription: () => string;

  /** Confirm and execute the pending action */
  confirmPendingAction: () => Promise<BatchActionResult | null>;

  /** Cancel the pending action */
  cancelPendingAction: () => void;

  /** Clear the error state */
  clearError: () => void;
}
```

---

### `createListController`

**Location:** `@decodelabs/underlay/patterns`

Provides unified state management for list data fetching with filters. Use this when you need coordinated data fetching with filter state.

```svelte
<script lang="ts">
  import { createListController } from '@decodelabs/underlay/patterns';
  import { auth, authLoading, currentUser } from '$lib/stores/auth';

  interface AreaFilters {
    moduleId?: string;
    sectionId?: string;
    search?: string;
  }

  let { moduleId, sectionId } = $props<{ moduleId?: string; sectionId?: string }>();

  const list = createListController<Area, AreaFilters>(
    async (fetch, token, filters) => {
      return await getAreas(fetch, token, filters);
    },
    {
      initialFilters: { moduleId, sectionId },
      getToken: () => auth.getToken()
    }
  );

  // Trigger fetch when auth is ready
  $effect(() => {
    list.tryFetch($authLoading, $currentUser);
  });

  // Re-fetch when filter props change
  $effect(() => {
    list.setFilters({ moduleId, sectionId });
  });
</script>

{#if list.loading}
  <PageLoading presentation="inline" />
{:else if list.error}
  <Callout tone="danger" message={list.error} announceMode="polite" />
{:else}
  {#each list.items as item}
    <ListCard title={item.title} />
  {/each}
{/if}
```

#### API Reference

```typescript
interface ListControllerOptions<T, F> {
  /** Function to get the current access token */
  getToken?: () => string | null;

  /** Initial filter values */
  initialFilters?: Partial<F>;

  /** Whether to fetch automatically when filters change (default: true) */
  autoFetchOnFilterChange?: boolean;

  /** Callback after successful fetch */
  onSuccess?: (items: T[]) => void;

  /** Callback when an error occurs */
  onError?: (error: Error) => void;

  /** Callback when items change */
  onItemsChange?: (items: T[]) => void;
}

interface ListControllerResult<T, F> {
  /** Current items in the list (reactive) */
  readonly items: T[];

  /** Whether data is being fetched for the first time */
  readonly loading: boolean;

  /** Whether data is being refetched (data already exists) */
  readonly refetching: boolean;

  /** Error message if fetch failed */
  readonly error: string | null;

  /** Current filter values (reactive) */
  readonly filters: F;

  /** Whether data has been fetched at least once */
  readonly fetched: boolean;

  /** Attempt to fetch data if auth is ready */
  tryFetch: (authLoading: boolean, currentUser: unknown) => Promise<void>;

  /** Force a refetch of the data with current filters */
  refresh: () => Promise<void>;

  /** Update one or more filter values */
  setFilters: (newFilters: Partial<F>) => void;

  /** Set a single filter value */
  setFilter: <K extends keyof F>(key: K, value: F[K]) => void;

  /** Reset filters to initial values and refetch */
  resetFilters: () => Promise<void>;

  /** Update items locally without refetching (for optimistic updates) */
  updateItems: (updater: (items: T[]) => T[]) => void;

  /** Remove an item locally by ID */
  removeItem: (id: string, idField?: string) => void;
}
```

---

## Components

### `ListContainer`

Underlay `ListContainer` is retired. Use Poodle `ListContainer` for the page-level list shell, then compose batch controls, filter content, card or grid content, and pagination around it.

**Location:** `@poodle/svelte-composites`

A list shell that provides consistent structure for autonomous lists with caller-owned filters, batch actions, content, and pagination.

```svelte
<script lang="ts">
  import { ListContainer } from '@poodle/svelte-composites';
</script>

<ListContainer
  title="Areas"
  subtitle="Managed learning areas"
  eyebrow="Learning"
  state={list.loading ? "loading" : list.error ? "error" : pagination.items.length > 0 ? "ready" : "empty"}
  errorMessage={list.error}
  emptyTitle="No areas found"
  emptyMessage="Create an area to get started."
  currentPage={pagination.currentPage}
  totalPages={pagination.totalPages ?? 1}
  totalItems={items.length}
  pageSize={pagination.pageSize}
  on:pageChange={(event) => pagination.goToPage?.(event.detail.page)}
>
  {#snippet actions()}
    <Button onclick={handleAdd}>Add</Button>
  {/snippet}

  {#snippet filters()}
    <FilterToolbar ariaLabel="Area filters" summaryText="Filters">
      <TextInput bind:value={search} placeholder="Search..." />
    </FilterToolbar>
  {/snippet}

  {#snippet batch()}
    <BulkActionBar ... />
  {/snippet}

  <Grid columns="repeat(auto-fit, minmax(min(22.5rem, 100%), 1fr))" gap="lg">
    {#each pagination.items as item}
      <ListCard title={item.title} />
    {/each}
  </Grid>
</ListContainer>
```

---

### `BulkActionBar`

`BulkActionBar` is now the shared batch-selection surface. Keep destructive confirmation and status-update flows explicit in the surrounding route or list controller instead of hiding them inside a wrapper.

**Location:** `@poodle/svelte-primitives`

A fixed toolbar that appears at the bottom of the screen when items are selected.

```svelte
<BulkActionBar
  selectionCount={batch.count}
  totalCount={allItems.length}
  loading={batch.executing}
  actions={batch.availableActions.map((action) => ({
    id: action.id,
    label: action.label,
    icon: action.icon,
    tone: action.variant === "danger" ? "danger" : action.variant === "warning" ? "warning" : "default"
  }))}
  showSelectAll
  on:clear={() => batch.clear()}
  on:selectAll={() => batch.selectAll(allItems.map(i => i.id))}
  on:action={(event) => batch.requestAction(event.detail.id)}
/>
```

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `selectionCount` | `number` | required | Number of selected items |
| `totalCount` | `number` | `0` | Total items available for selection |
| `loading` | `boolean` | `false` | Whether an operation is in progress |
| `actions` | `BulkAction[]` | `[]` | Bulk actions rendered in the bar |
| `showSelectAll` | `boolean` | `false` | Show the select-all / deselect-all control |
| `allSelected` | `boolean` | `false` | Whether the current selection already covers the whole list |
| `on:clear` | event | - | Clear selection callback |
| `on:selectAll` | event | - | Select all or deselect all callback |
| `on:action` | `{ id: string }` | - | Bulk action callback |

---

## Building an Autonomous List Component

Here's a complete example of building an autonomous list component:

```svelte
<!-- AreasList.svelte -->
<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { browser } from "$app/environment";
  import {
    useBatchActions,
    useAuthenticatedData,
    PageHeader,
    type NavigationContext
  } from "@decodelabs/underlay/patterns";
  import { useToasts } from "@decodelabs/underlay/runtime/feedback";
  import { FilterToolbar } from "@poodle/svelte-composites";
  import {
    Button,
    BulkActionBar,
    AlertDialog,
    Callout
  } from "@poodle/svelte-primitives";
  import { PageLoading } from "@poodle/svelte-composites";
  import { Tooltip } from "@poodle/svelte-primitives";
  import { gotoWithContext } from "@decodelabs/underlay/client";
  import { learningCommands } from "@cattle-grid";
  import { auth } from "$lib/stores/auth";
  import { AreaListCard } from "$lib/cards";
  import Plus from "lucide-svelte/icons/plus";
  import Trash2 from "lucide-svelte/icons/trash-2";
  import CheckSquare from "lucide-svelte/icons/check-square";

  type ListVariant = "page" | "tab";

  interface Props {
    variant?: ListVariant;
    sectionId?: string;
    onDataChange?: () => void;
  }

  let { variant = "page", sectionId, onDataChange }: Props = $props();

  const toastStore = useToasts();
  const isConstrained = $derived(!!sectionId);

  // Data fetching — auto-fetches when auth is ready via global configureAuth()
  const pageData = useAuthenticatedData(
    async (fetchFn, token) => {
      return await learningCommands.getAreas(fetchFn, token, { sectionId });
    },
    { defaultValue: [] }
  );

  // Batch selection and actions
  const batch = useBatchActions<string>();

  batch.registerAction({
    id: "delete",
    label: "Delete",
    variant: "danger",
    confirm: {
      title: "Delete Areas",
      description: (count) =>
        `Are you sure you want to delete ${count} area${count === 1 ? "" : "s"}?`,
      confirmLabel: "Delete"
    },
    execute: async (ids) => {
      const token = auth.getToken();
      if (!token) return { success: false, affected: 0, message: "Not authenticated" };

      for (const id of ids) {
        await learningCommands.softDeleteArea(id, window.fetch.bind(window), token);
      }

      await pageData.refetch();
      onDataChange?.();
      toastStore.push({ variant: "success", message: `Deleted ${ids.length} areas` });
      return { success: true, affected: ids.length };
    }
  });

  // UI state
  let selectionMode = $state(false);
  const headerLevel = $derived(variant === "page" ? 1 : 3);
  const areas = $derived(pageData.data ?? []);
  const allAreaIds = $derived(areas.map((a) => a.areaId));

  function toggleSelectionMode() {
    selectionMode = !selectionMode;
    if (!selectionMode) batch.clear();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape" && selectionMode) {
      selectionMode = false;
      batch.clear();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<PoodlePageHeader title="Areas" count={areas.length}>
  {#snippet actions()}
    {#if areas.length > 0}
      <Tooltip content={selectionMode ? "Exit selection (Esc)" : "Select items"} delayMs={200}>
        <Button
          variant={selectionMode ? "primary" : "subtle"}
          size="icon"
          onclick={toggleSelectionMode}
        >
          <CheckSquare size={16} />
        </Button>
      </Tooltip>
    {/if}
    <Tooltip content="Add Area" delayMs={200}>
      <Button variant="primary" size="icon" disabled={selectionMode}>
        <Plus size={16} />
      </Button>
    </Tooltip>
  {/snippet}
</PoodlePageHeader>

{#if batch.pendingAction}
  <AlertDialog
    open={true}
    showTrigger={false}
    title={batch.pendingAction.confirm?.title ?? "Confirm"}
    description={batch.getConfirmDescription()}
    confirmLabel={batch.pendingAction.confirm?.confirmLabel ?? "Confirm"}
    onConfirm={async () => { await batch.confirmPendingAction(); }}
    onCancel={() => batch.cancelPendingAction()}
  />
{/if}

{#if pageData.loading}
  <PageLoading presentation="inline" message="Loading areas..." />
{:else if pageData.error}
  <Callout tone="danger" message={pageData.error} announceMode="polite" />
{:else if areas.length === 0}
  <p>No areas defined yet.</p>
{:else}
  <Grid columns="repeat(auto-fit, minmax(min(26em, 100%), 1fr))" gap="lg">
    {#each areas as area (area.areaId)}
      <AreaListCard
        {area}
        selected={selectionMode ? batch.isSelected(area.areaId) : undefined}
        onSelectionChange={selectionMode
          ? (selected) => batch.toggle(area.areaId, selected)
          : undefined}
      />
    {/each}
  </Grid>
{/if}

<BulkActionBar
  selectionCount={batch.count}
  totalCount={allAreaIds.length}
  loading={batch.executing}
  actions={batch.availableActions.map((action) => ({
    id: action.id,
    label: action.label,
    icon: action.icon,
    tone: action.variant === "danger" ? "danger" : action.variant === "warning" ? "warning" : "default"
  }))}
  showSelectAll
  on:clear={() => batch.clear()}
  on:selectAll={() => batch.selectAll(allAreaIds)}
  on:action={(event) => batch.requestAction(event.detail.id)}
/>
```

### Usage in Different Contexts

**As a root page:**
```svelte
<!-- /routes/(app)/areas/+page.svelte -->
<AreasList variant="page" />
```

**As an embedded tab:**
```svelte
<!-- /routes/(app)/sections/[sectionId]/+page.svelte -->
{#if activeValue === "areas"}
  <AreasList
    variant="tab"
    sectionId={section.sectionId}
    onDataChange={() => pageData.refetch()}
  />
{/if}
```

---

## Best Practices

1. **Register actions outside `$effect`** - Action registration should happen at component initialization, not inside reactive blocks.

2. **Use `onDataChange` for parent coordination** - When data changes in a tab, notify the parent so it can update counts and other derived state.

3. **Handle Escape key** - Allow users to exit selection mode by pressing Escape.

4. **Show selection toggle button** - Make it clear when selection mode is active with a visual indicator.

5. **Disable other actions during selection** - Prevent navigation actions (Add, View Trash) while in selection mode.

6. **Pass selection props only when in selection mode** - Only pass `selected` and `onSelectionChange` to cards when `selectionMode` is true, otherwise pass `undefined`.

7. **Use `variant` prop consistently** - `"page"` for root pages (h1 header), `"tab"` for embedded tabs (h3 header).
`Pagination` is now a direct Poodle surface, so list pages should compose over
Poodle `Pagination` rather than expecting a retained Underlay list wrapper to
own that controller-driven integration.
