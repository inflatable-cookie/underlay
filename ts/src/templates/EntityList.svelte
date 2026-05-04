<script lang="ts">
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  type T = any;
  import type { Snippet } from "svelte";
  import { useAuthenticatedData } from "../runtime/auth";
  import { useBatchActions } from "../patterns/batch-actions.svelte";
  import { createReorderController } from "../patterns/reorder-controller.svelte";
  import { useToasts } from "../runtime/feedback";
  import {
    FilterToolbar,
    ListContainer,
    PageLoading,
    Grid,
    DataTable,
    BulkActionBar,
    AlertDialog,
    Dialog,
    Callout,
    EmptyState,
    EditableList,
    ListCard,
    Field,
    TextInput,
    Select,
    OrderBy,
    IconButton,
    Button
  } from "@poodle/svelte";
  import type { TableColumn, TableRow, BulkAction, EditableListItem } from "@poodle/svelte";
  import type { SortField, SortDirection } from "../client/query";

  // --- Types ---

  interface FilterConfig {
    id: string;
    type: "search" | "select" | "date" | "number" | "sort";
    label: string;
    options?: { value: string; label: string }[];
    loadOptions?: () => Promise<{ value: string; label: string }[]>;
    placeholder?: string;
    /** For sort filters: available sort fields */
    sortFields?: { key: string; label: string; defaultDirection?: SortDirection }[];
  }

  interface ItemContext {
    selectionMode: boolean;
    selected: boolean;
    onToggle: (selected: boolean) => void;
    refetch: () => Promise<void>;
  }

  interface BatchDialogContext {
    ids: string[];
    onSubmit: (values: Record<string, unknown>) => void;
    onCancel: () => void;
  }

  interface BatchDialogConfig {
    title: string;
    content: Snippet<[BatchDialogContext]>;
  }

  interface BatchActionConfig {
    id: string;
    label: string;
    tone?: "default" | "danger" | "warning";
    icon?: string;
    confirm?: boolean | { title: string; description: string | ((count: number) => string) };
    dialog?: BatchDialogConfig;
    handler: (ids: string[], values?: Record<string, unknown>) => Promise<void>;
  }

  interface ReorderConfig {
    enabled: boolean;
    handler: (orderedIds: string[]) => Promise<void>;
  }

  interface Props {
    /** Optional title for inline use (omitted when inside EntityListPage) */
    title?: string;
    
    /** Data loading function */
    dataLoader: (fetch: typeof window.fetch, token: string | null, query: Record<string, unknown>) => Promise<T[]>;
    
    /** Unique identifier field (default: "id") */
    idField?: string;
    
    /** Presentation mode */
    presentation: "cards" | "table";
    
    /** For cards: render snippet for each item (receives item + selection context) */
    renderItem?: Snippet<[T, ItemContext]>;
    
    /** For table: column definitions */
    columns?: TableColumn[];
    
    /** For table: row actions */
    rowActions?: (row: TableRow<T>) => { value: string; label: string }[];
    
    /** Declarative filter configuration */
    filters?: FilterConfig[];
    
    /** Batch action configuration */
    batchActions?: BatchActionConfig[];
    
    /** Reorder configuration */
    reorder?: ReorderConfig;
    
    /** Optional add button */
    onAdd?: () => void;
    addLabel?: string;
    
    /** Optional callback when data changes */
    onDataChange?: () => void;
    
    /** Optional callback when item count changes */
    onCountChange?: (count: number) => void;
    
    /** Optional class for styling */
    class?: string;

    /** External selection mode control */
    selectionMode?: boolean;

    /** External reorder mode control */
    reorderMode?: boolean;

    /** External filter values (bypasses internal state if provided) */
    filterValues?: Record<string, string>;

    /** External sort state */
    sort?: SortField[];

    /** Called when a filter changes (parent manages URL sync) */
    onFilterChange?: (id: string, value: string) => void;

    /** Called when sort changes (parent manages URL sync) */
    onSortChange?: (sort: SortField[]) => void;

    /** Custom reorder error handler for conflict recovery */
    onReorderError?: (error: unknown) => Promise<string | void> | string | void;
  }

  // --- Props ---

  let {
    title,
    dataLoader,
    idField = "id",
    presentation,
    renderItem,
    columns,
    rowActions,
    filters = [],
    batchActions = [],
    reorder,
    onAdd,
    addLabel = "Add",
    onDataChange,
    onCountChange,
    class: className,
    selectionMode: externalSelectionMode,
    reorderMode: externalReorderMode,
    filterValues: externalFilterValues,
    sort: externalSort,
    onFilterChange,
    onSortChange,
    onReorderError
  }: Props = $props();

  // --- State ---

  const toastStore = useToasts();

  // Filter state (external or internal)
  let internalFilterValues = $state<Record<string, string>>({});
  let filterValues = $derived(externalFilterValues ?? internalFilterValues);

  // Sort state (external or internal)
  let internalSort = $state<SortField[]>([]);
  let currentSort = $derived(externalSort ?? internalSort);

  // Async filter options
  let loadedFilterOptions = $state<Record<string, { value: string; label: string }[]>>({});
  let filterOptionsLoading = $state<Record<string, boolean>>({});

  // Selection mode (internal or external)
  let internalSelectionMode = $state(false);
  let selectionMode = $derived(externalSelectionMode ?? internalSelectionMode);

  // Reorder mode (internal or external)
  let internalReorderMode = $state(false);
  let reorderMode = $derived(externalReorderMode ?? internalReorderMode);
  let reorderError = $state<string | null>(null);

  // Custom batch action dialog
  let pendingDialogAction = $state<BatchActionConfig | null>(null);
  let dialogSubmitting = $state(false);

  // Data loading (includes filters and sort)
  const pageData = useAuthenticatedData<T[]>(
    async (fetch, token) => {
      const query = buildQueryFromFilters(filterValues, currentSort);
      return await dataLoader(fetch, token, query);
    },
    { defaultValue: [] }
  );

  const items = $derived(pageData.data ?? []);
  const itemIds = $derived(items.map((item) => String((item as Record<string, unknown>)[idField])));

  // Notify parent of count changes
  $effect(() => {
    onCountChange?.(items.length);
  });

  // Load async filter options on mount
  $effect(() => {
    for (const filter of filters) {
      if (filter.loadOptions && !loadedFilterOptions[filter.id] && !filterOptionsLoading[filter.id]) {
        filterOptionsLoading = { ...filterOptionsLoading, [filter.id]: true };
        filter.loadOptions().then((options) => {
          loadedFilterOptions = { ...loadedFilterOptions, [filter.id]: options };
          filterOptionsLoading = { ...filterOptionsLoading, [filter.id]: false };
        }).catch(() => {
          filterOptionsLoading = { ...filterOptionsLoading, [filter.id]: false };
        });
      }
    }
  });

  // Batch actions
  const batch = useBatchActions<string>();

  // Register batch actions (skip dialog actions — handled separately)
  $effect(() => {
    for (const action of batchActions) {
      if (action.dialog) continue;
      batch.registerAction({
        id: action.id,
        label: action.label,
        variant: action.tone,
        confirm: action.confirm === true 
          ? {
              title: `${action.label} items`,
              description: (count: number) => `Are you sure you want to ${action.label.toLowerCase()} ${count} item${count === 1 ? "" : "s"}?`,
              confirmLabel: action.label
            }
          : typeof action.confirm === "object" 
            ? {
                title: action.confirm.title,
                description: action.confirm.description,
                confirmLabel: action.label
              }
            : undefined,
        execute: async (ids: string[]) => {
          await action.handler(ids);
          onDataChange?.();
          await pageData.refetch();
          return { success: true, affected: ids.length };
        }
      });
    }
  });

  // Reorder controller
  let reorderController = $derived(
    reorder?.enabled
      ? createReorderController(
          items.map((item) => ({
            id: String((item as Record<string, unknown>)[idField]),
            ...item
          })),
          async (orderedIds) => {
            await reorder!.handler(orderedIds);
          }
        )
      : null
  );

  // --- Helpers ---

  function buildQueryFromFilters(values: Record<string, string>, sortFields: SortField[]): Record<string, unknown> {
    const query: Record<string, unknown> = {};
    for (const [key, value] of Object.entries(values)) {
      if (value && value !== "All") {
        query[key] = value;
      }
    }
    if (sortFields.length > 0) {
      query.sort = sortFields.map((s) => `${s.field}:${s.direction}`).join(",");
    }
    return query;
  }

  function handleFilterChange(id: string, value: string) {
    if (onFilterChange) {
      onFilterChange(id, value);
    } else {
      internalFilterValues = { ...internalFilterValues, [id]: value };
    }
    pageData.refetch();
  }

  function handleSortChange(sortFields: SortField[]) {
    if (onSortChange) {
      onSortChange(sortFields);
    } else {
      internalSort = sortFields;
    }
    pageData.refetch();
  }

  function toggleSelectionMode() {
    if (internalReorderMode) {
      internalReorderMode = false;
      reorderController?.reset();
    }
    internalSelectionMode = !internalSelectionMode;
    if (!internalSelectionMode) {
      batch.clear();
    }
  }

  function enterReorderMode() {
    if (internalSelectionMode) {
      internalSelectionMode = false;
      batch.clear();
    }
    internalReorderMode = true;
    reorderError = null;
  }

  function exitReorderMode() {
    internalReorderMode = false;
    reorderError = null;
    reorderController?.reset();
  }

  async function handleReorderSubmit() {
    if (!reorderController) return;
    reorderError = null;
    try {
      await reorderController.submit();
      exitReorderMode();
      onDataChange?.();
      await pageData.refetch();
    } catch (error) {
      if (onReorderError) {
        const result = await onReorderError(error);
        if (result) {
          reorderError = result;
        } else {
          reorderError = error instanceof Error ? error.message : String(error);
        }
      } else {
        reorderError = error instanceof Error ? error.message : String(error);
      }
    }
  }

  function getItemContext(item: T): ItemContext {
    const itemId = String((item as Record<string, unknown>)[idField]);
    return {
      selectionMode,
      selected: batch.isSelected(itemId),
      onToggle: (selected: boolean) => batch.toggle(itemId, selected),
      refetch: () => pageData.refetch()
    };
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      if (selectionMode) {
        selectionMode = false;
        batch.clear();
      } else if (reorderMode) {
        exitReorderMode();
      }
    }
  }

  // Table rows
  const tableRows = $derived<TableRow<T>[]>(
    items.map((item) => ({
      id: String((item as Record<string, unknown>)[idField]),
      cells: columns?.reduce((acc, col) => {
        acc[col.id] = String((item as Record<string, unknown>)[col.id] ?? "");
        return acc;
      }, {} as Record<string, string>) ?? {},
      data: item
    })) ?? []
  );

  // Convert batch actions for BulkActionBar (include dialog actions)
  const bulkActions = $derived<BulkAction[]>([
    ...batch.availableActions.map((action) => ({
      id: action.id,
      label: action.label,
      icon: action.icon,
      tone: action.variant === "danger" ? "danger" : action.variant === "warning" ? "warning" : "default"
    })),
    ...batchActions
      .filter((a) => a.dialog)
      .map((action) => ({
        id: action.id,
        label: action.label,
        icon: action.icon,
        tone: action.tone === "danger" ? "danger" : action.tone === "warning" ? "warning" : "default"
      }))
  ]);

  // --- Batch action dialog handlers ---

  function handleBatchAction(event: CustomEvent<{ id: string }>) {
    const actionId = event.detail.id;
    const actionConfig = batchActions.find((a) => a.id === actionId);

    if (actionConfig?.dialog) {
      pendingDialogAction = actionConfig;
      return;
    }

    batch.requestAction(actionId);
  }

  async function handleDialogSubmit(values: Record<string, unknown>) {
    if (!pendingDialogAction) return;
    dialogSubmitting = true;
    try {
      const ids = batch.selectedIds;
      await pendingDialogAction.handler(ids, values);
      onDataChange?.();
      await pageData.refetch();
      batch.clear();
      pendingDialogAction = null;
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      toastStore.push({ message, variant: "error" });
    } finally {
      dialogSubmitting = false;
    }
  }

  function handleDialogCancel() {
    pendingDialogAction = null;
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if title}
  <!-- When used standalone (not inside EntityListPage), show ListContainer -->
  <ListContainer
    {title}
    state={pageData.loading ? "loading" : pageData.error ? "error" : items.length > 0 ? "ready" : "empty"}
    errorMessage={pageData.error ?? undefined}
    emptyTitle="No items found"
    emptyMessage="Try adjusting your filters or add a new item."
  >
    {#snippet actions()}
      {#if onAdd}
        <Button variant="primary" onclick={onAdd}>{addLabel}</Button>
      {/if}
    {/snippet}

    {#snippet filters()}
      {#if filters.length > 0 && !reorderMode}
        <FilterToolbar ariaLabel={`${title} filters`} summaryText="Filters">
          {#snippet actions()}
            <IconButton
              icon="refresh-cw"
              variant="ghost"
              tooltip="Refresh"
              onclick={() => pageData.refetch()}
            />
          {/snippet}
          
          {#each filters as filter}
            <Field id={`filter-${filter.id}`} label={filter.label}>
              {#if filter.type === "search"}
                <TextInput
                  id={`filter-${filter.id}`}
                  type="search"
                  value={filterValues[filter.id] ?? ""}
                  placeholder={filter.placeholder ?? `Search ${filter.label.toLowerCase()}...`}
                  oninput={(e: Event) => handleFilterChange(filter.id, (e.currentTarget as HTMLInputElement).value)}
                />
              {:else if filter.type === "select"}
                <Select
                  id={`filter-${filter.id}`}
                  value={filterValues[filter.id] ?? "All"}
                  items={[{ value: "All", label: `All ${filter.label.toLowerCase()}` }, ...(loadedFilterOptions[filter.id] ?? filter.options ?? [])]}
                  onchange={(e: Event) => handleFilterChange(filter.id, (e.currentTarget as HTMLSelectElement).value)}
                />
              {:else if filter.type === "sort" && filter.sortFields}
                <OrderBy
                  fields={filter.sortFields}
                  value={currentSort.map((s) => ({ key: s.field, direction: s.direction }))}
                  onChange={(value: { key: string; direction: string }[]) => handleSortChange(value.map((v) => ({ field: v.key, direction: v.direction as "asc" | "desc" })))}
                  compact
                />
              {/if}
            </Field>
          {/each}
        </FilterToolbar>
      {/if}
    {/snippet}

    <!-- Content rendered below -->
    {#if pageData.loading && items.length === 0}
      <PageLoading presentation="inline" message={`Loading ${title.toLowerCase()}...`} />
    {:else if pageData.error}
      <Callout tone="danger" message={pageData.error} announceMode="polite" />
    {:else if items.length === 0}
      <EmptyState
        title={`No ${title.toLowerCase()} found`}
        message="Try adjusting your filters or create a new item."
      >
        {#if onAdd}
          <Button variant="primary" onclick={onAdd}>{addLabel}</Button>
        {/if}
      </EmptyState>
    {:else if reorderMode && reorderController}
      <EditableList
        items={reorderController.pending}
        dirty={reorderController.isDirty}
        submitting={reorderController.isPending}
        errorMessage={reorderError}
        onsubmit={handleReorderSubmit}
        oncancel={exitReorderMode}
        on:reorder={(event: CustomEvent<{ items: EditableListItem[] }>) => reorderController?.updatePending(event.detail.items)}
      >
        {#snippet item(reorderEntry: EditableListItem)}
          {@const originalItem = items.find((item) => 
            String((item as Record<string, unknown>)[idField]) === reorderEntry.id
          )}
          {#if renderItem && originalItem}
            {@render renderItem(originalItem, getItemContext(originalItem))}
          {:else}
            <ListCard title={reorderEntry.id} layout="compact" showReorderHandle />
          {/if}
        {/snippet}
      </EditableList>
    {:else if presentation === "cards"}
      <Grid columns="repeat(auto-fit, minmax(min(26em, 100%), 1fr))" gap="lg">
        {#each items as item (String((item as Record<string, unknown>)[idField]))}
          {#if renderItem}
            {@render renderItem(item, getItemContext(item))}
          {/if}
        {/each}
      </Grid>
    {:else if presentation === "table"}
      <DataTable
        {columns}
        rows={tableRows}
        rowActions={rowActions}
        emptyMessage="No items found"
      >
      {#snippet cell(column: TableColumn, row: TableRow<T>, value: string)}
          {#if selectionMode && column.id === "__selection"}
            <input
              type="checkbox"
              checked={batch.isSelected(row.id)}
              onchange={(e: Event) => batch.toggle(row.id, (e.currentTarget as HTMLInputElement).checked)}
            />
          {:else}
            {value}
          {/if}
        {/snippet}
      </DataTable>
    {/if}
  </ListContainer>
{:else}
  <!-- When used inside EntityListPage, don't show ListContainer shell -->
  {#if filters.length > 0 && !reorderMode}
    <FilterToolbar ariaLabel="Filters" summaryText="Filters">
      {#snippet actions()}
        <IconButton
          icon="refresh-cw"
          variant="ghost"
          tooltip="Refresh"
          onclick={() => pageData.refetch()}
        />
      {/snippet}
      
      {#each filters as filter}
        <Field id={`filter-${filter.id}`} label={filter.label}>
          {#if filter.type === "search"}
            <TextInput
              id={`filter-${filter.id}`}
              type="search"
              value={filterValues[filter.id] ?? ""}
              placeholder={filter.placeholder ?? `Search ${filter.label.toLowerCase()}...`}
              oninput={(e: Event) => handleFilterChange(filter.id, (e.currentTarget as HTMLInputElement).value)}
            />
          {:else if filter.type === "select"}
            <Select
              id={`filter-${filter.id}`}
              value={filterValues[filter.id] ?? "All"}
              items={[{ value: "All", label: `All ${filter.label.toLowerCase()}` }, ...(loadedFilterOptions[filter.id] ?? filter.options ?? [])]}
              onchange={(e: Event) => handleFilterChange(filter.id, (e.currentTarget as HTMLSelectElement).value)}
            />
          {:else if filter.type === "sort" && filter.sortFields}
            <OrderBy
              fields={filter.sortFields}
              value={currentSort.map((s) => ({ key: s.field, direction: s.direction }))}
              onChange={(value: { key: string; direction: string }[]) => handleSortChange(value.map((v) => ({ field: v.key, direction: v.direction as "asc" | "desc" })))}
              compact
            />
          {/if}
        </Field>
      {/each}
    </FilterToolbar>
  {/if}
  
  {#if pageData.loading && items.length === 0}
    <PageLoading presentation="inline" message="Loading..." />
  {:else if pageData.error}
    <Callout tone="danger" message={pageData.error} announceMode="polite" />
  {:else if items.length === 0}
    <EmptyState
      title="No items found"
      message="Try adjusting your filters or create a new item."
    />
  {:else if reorderMode && reorderController}
    <EditableList
      items={reorderController.pending}
      dirty={reorderController.isDirty}
      submitting={reorderController.isPending}
      errorMessage={reorderError}
      onsubmit={handleReorderSubmit}
      oncancel={exitReorderMode}
      on:reorder={(event: CustomEvent<{ items: EditableListItem[] }>) => reorderController?.updatePending(event.detail.items)}
    >
      {#snippet item(reorderEntry: EditableListItem)}
        {@const originalItem = items.find((item) => 
          String((item as Record<string, unknown>)[idField]) === reorderEntry.id
        )}
        {#if renderItem && originalItem}
          {@render renderItem(originalItem, getItemContext(originalItem))}
        {:else}
          <ListCard title={reorderEntry.id} layout="compact" showReorderHandle />
        {/if}
      {/snippet}
    </EditableList>
  {:else if presentation === "cards"}
    <Grid columns="repeat(auto-fit, minmax(min(26em, 100%), 1fr))" gap="lg">
      {#each items as item (String((item as Record<string, unknown>)[idField]))}
        {#if renderItem}
          {@render renderItem(item, getItemContext(item))}
        {/if}
      {/each}
    </Grid>
  {:else if presentation === "table"}
    <DataTable
      {columns}
      rows={tableRows}
      rowActions={rowActions}
      emptyMessage="No items found"
    >
      {#snippet cell(column: TableColumn, row: TableRow<T>, value: string)}
        {#if selectionMode && column.id === "__selection"}
          <input
            type="checkbox"
            checked={batch.isSelected(row.id)}
            onchange={(e: Event) => batch.toggle(row.id, (e.currentTarget as HTMLInputElement).checked)}
          />
        {:else}
          {value}
        {/if}
      {/snippet}
    </DataTable>
  {/if}
{/if}

<!-- Batch action bar -->
{#if batchActions.length > 0 && selectionMode}
  <BulkActionBar
    selectionCount={batch.count}
    totalCount={items.length}
    actions={bulkActions}
    loading={batch.executing}
    showSelectAll
    allSelected={batch.count > 0 && batch.count === items.length}
    on:clear={() => { batch.clear(); selectionMode = false; }}
    on:selectAll={() => batch.selectAll(itemIds)}
    on:action={handleBatchAction}
  />
{/if}

<!-- Batch action confirmation dialog -->
{#if batch.pendingAction}
  <AlertDialog
    open={true}
    title={batch.pendingAction.confirm?.title ?? "Confirm"}
    description={batch.getConfirmDescription()}
    confirmLabel={batch.pendingAction.confirm?.confirmLabel ?? "Confirm"}
    cancelLabel="Cancel"
    tone={batch.pendingAction.variant === "danger" ? "danger" : "warning"}
    onConfirm={async () => { await batch.confirmPendingAction(); }}
    onCancel={() => batch.cancelPendingAction()}
  />
{/if}

<!-- Batch action custom dialog -->
{#if pendingDialogAction?.dialog}
  <Dialog
    open={true}
    title={pendingDialogAction.dialog.title}
    onClose={handleDialogCancel}
  >
    {@render pendingDialogAction.dialog.content({
      ids: batch.selectedIds,
      onSubmit: handleDialogSubmit,
      onCancel: handleDialogCancel
    })}
  </Dialog>
{/if}

<style>

</style>
