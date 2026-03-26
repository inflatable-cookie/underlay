# Autonomous List Components

This guide covers the **Autonomous List Component** pattern - a set of hooks and components that enable list views to be fully self-contained, handling their own data fetching, state management, and batch operations.

## Overview

The Autonomous List Component pattern solves the "god file" problem where parent pages accumulate excessive business logic from managing multiple entity types across tabs.

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
  import { BatchActionBar } from '@decodelabs/underlay/components';

  const items = $derived(data.projects);
  const selection = useBatchSelection<string>();

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

<BatchActionBar
  selectedCount={selection.count}
  totalCount={items.length}
  onClearSelection={selection.clear}
  onSelectAll={() => selection.selectAll(items.map(i => i.id))}
  onBatchDelete={handleBatchDelete}
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
  import { AlertDialog, BatchActionBar } from '@decodelabs/underlay/components';

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
<BatchActionBar
  selectedCount={batch.count}
  totalCount={items.length}
  showDelete={false}
  registeredActions={batch.availableActions}
  onClearSelection={() => batch.clear()}
  onSelectAll={() => batch.selectAll(items.map(i => i.id))}
  onAction={(actionId) => batch.requestAction(actionId)}
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
  <PageLoading />
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

**Location:** `@decodelabs/underlay/components`

A layout component that provides consistent structure for autonomous lists with variant-aware styling.

```svelte
<script lang="ts">
  import { ListContainer } from '@decodelabs/underlay/components';
</script>

<ListContainer
  title="Areas"
  variant="tab"
  loading={list.loading}
  error={list.error}
  count={items.length}
  hasItems={items.length > 0}
  emptyMessage="No areas found."
>
  {#snippet actions()}
    <Button onclick={handleAdd}>Add</Button>
  {/snippet}

  {#snippet filters()}
    <FilterBar>
      <TextInput bind:value={search} placeholder="Search..." />
    </FilterBar>
  {/snippet}

  {#snippet content()}
    <ListGrid>
      {#each items as item}
        <ListCard title={item.title} />
      {/each}
    </ListGrid>
  {/snippet}

  {#snippet batchBar()}
    <BatchActionBar ... />
  {/snippet}
</ListContainer>
```

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `title` | `string` | required | List title |
| `variant` | `"page" \| "tab"` | `"page"` | Display variant (affects header level and spacing) |
| `loading` | `boolean` | `false` | Whether data is loading |
| `error` | `string \| null` | `null` | Error message to display |
| `count` | `number` | - | Total item count (shown in header) |
| `hasItems` | `boolean` | `true` | Whether the list has items (for empty state) |
| `emptyMessage` | `string` | `"No items found."` | Empty state message |
| `backHref` | `string \| null` | `null` | Back link URL |
| `backLabel` | `string` | `"Back"` | Back link label |

#### Snippets

- `actions` - Header action buttons (trash, add, etc.)
- `filters` - Filter bar content
- `content` - Main list content (ListGrid, etc.)
- `batchBar` - Batch action bar component
- `pagination` - Pagination component

---

### `BatchActionBar`

**Location:** `@decodelabs/underlay/components`

A fixed toolbar that appears at the bottom of the screen when items are selected.

```svelte
<BatchActionBar
  selectedCount={batch.count}
  totalCount={allItems.length}
  loading={batch.executing}
  showDelete={false}
  itemLabel="area"
  itemLabelPlural="areas"
  registeredActions={batch.availableActions}
  onClearSelection={() => batch.clear()}
  onSelectAll={() => batch.selectAll(allItems.map(i => i.id))}
  onAction={(actionId) => batch.requestAction(actionId)}
/>
```

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `selectedCount` | `number` | required | Number of selected items |
| `totalCount` | `number` | `0` | Total items available for selection |
| `loading` | `boolean` | `false` | Whether an operation is in progress |
| `showDelete` | `boolean` | `true` | Show built-in delete button |
| `showStatusUpdate` | `boolean` | `false` | Show status update button |
| `statusOptions` | `array` | `[]` | Status options for status update |
| `itemLabel` | `string` | `"item"` | Singular item label |
| `itemLabelPlural` | `string` | `"items"` | Plural item label |
| `registeredActions` | `DynamicAction[]` | `[]` | Dynamic actions from `useBatchActions` |
| `onClearSelection` | `() => void` | required | Clear selection callback |
| `onSelectAll` | `() => void` | - | Select all callback |
| `onBatchDelete` | `() => void` | - | Built-in delete callback |
| `onAction` | `(actionId: string) => void` | - | Dynamic action callback |

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
    FilterBar,
    PageHeader,
    useToasts,
    type NavigationContext
  } from "@decodelabs/underlay/patterns";
  import {
    AlertDialog,
    BatchActionBar,
    Button,
    Callout,
    ListGrid,
    PageLoading,
    Tooltip
  } from "@decodelabs/underlay/components";
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

<PageHeader title="Areas" level={headerLevel} count={areas.length}>
  {#snippet actions()}
    {#if areas.length > 0}
      <Tooltip content={selectionMode ? "Exit selection (Esc)" : "Select items"} inline>
        {#snippet trigger()}
          <Button
            variant={selectionMode ? "primary" : "subtle"}
            size="icon"
            onclick={toggleSelectionMode}
          >
            <CheckSquare size={16} />
          </Button>
        {/snippet}
      </Tooltip>
    {/if}
    <Tooltip content="Add Area" inline>
      {#snippet trigger()}
        <Button variant="primary" size="icon" disabled={selectionMode}>
          <Plus size={16} />
        </Button>
      {/snippet}
    </Tooltip>
  {/snippet}
</PageHeader>

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
  <PageLoading message="Loading areas..." />
{:else if pageData.error}
  <Callout tone="danger" message={pageData.error} announceMode="polite" />
{:else if areas.length === 0}
  <p>No areas defined yet.</p>
{:else}
  <ListGrid minItemWidth={26}>
    {#each areas as area (area.areaId)}
      <AreaListCard
        {area}
        selected={selectionMode ? batch.isSelected(area.areaId) : undefined}
        onSelectionChange={selectionMode
          ? (selected) => batch.toggle(area.areaId, selected)
          : undefined}
      />
    {/each}
  </ListGrid>
{/if}

<BatchActionBar
  selectedCount={batch.count}
  totalCount={allAreaIds.length}
  loading={batch.executing}
  showDelete={false}
  itemLabel="area"
  itemLabelPlural="areas"
  onClearSelection={() => batch.clear()}
  onSelectAll={() => batch.selectAll(allAreaIds)}
  registeredActions={batch.availableActions}
  onAction={(actionId) => batch.requestAction(actionId)}
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
<TabsContent value="areas">
  <AreasList
    variant="tab"
    sectionId={section.sectionId}
    onDataChange={() => pageData.refetch()}
  />
</TabsContent>
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
