<script lang="ts" generics="T extends object = Record<string, unknown>">
  import { onDestroy, setContext, untrack } from "svelte";
  import { useAuthenticatedData } from "../runtime/auth";
  import { useBatchActions } from "../patterns/batch-actions.svelte";
  import { useToasts } from "../runtime/feedback";
  import {
    FilterToolbar,
    ListContainer,
    PageLoading,
    ListGrid,
    DataTable,
    BulkActionBar,
    AlertDialog,
    Dialog,
    Callout,
    EmptyState,
    EditableList,
    ListCard,
    LogList,
    TextInput,
    Select,
    OrderBy,
    IconButton,
    Button,
    Pill,
    Pagination,
    PaginationSummary,
    CardToggleGroup
  } from "@inflatable-cookie/poodle-svelte";
  import type {
    TableColumn,
    TableRow,
    TableRowAction,
    BulkAction,
    CardToggleItem
  } from "@inflatable-cookie/poodle-svelte";
  import type { QueryParams, SortField } from "../client/query";
  import { orderByToSortFields } from "../client/query";
  import { DEFAULT_PAGE_SIZE } from "../patterns/pagination-types";
  import {
    claimListSelectionScope,
    createListSelectionScopeId,
    registerListSelectionScope,
    releaseListSelectionScope
  } from "../patterns/list-selection-scope.svelte";
  import {
    getListFilterValue,
    isListFilterActive,
    listQueryKey,
    sortFieldsEqual,
    updateListFilters
  } from "./entity-list-query";
  import { createEntityListQuery } from "./entity-list-query.svelte";
  import { useEntityListFilterOptions, useEntityListSearch } from "./entity-list-filters.svelte";
  import { isListReorderAvailable, useEntityListReorder } from "./entity-list-reorder.svelte";
  import { default as EntityReorderControls } from "./EntityReorderControls.svelte";
  import {
    UNDERLAY_ENTITY_LIST_CONTEXT_KEY,
    type EntityListContext
  } from "./entity-list-context";
  import type {
    BatchActionConfig,
    EntityListCapabilitiesLoader,
    EntityListDataLoader,
    EntityListItemContext,
    EntityListSharedProps,
    FilterConfig,
    ListVariantDefinition,
    ListCapabilities,
    PagedListResult,
    ReorderActionState,
    ReorderConfig,
    TemplateSurface
  } from "./template.types";

  type ItemContext = EntityListItemContext;

  interface Props {
    /** Optional title for inline use (omitted when inside EntityListPage) */
    title?: string;
    
    /** Data loading function. Must return paged results for the current query state. */
    dataLoader: EntityListDataLoader<T>;

    /** Optional external reload signal for non-query data refreshes. */
    reloadKey?: string | number;
    
    /** Unique identifier field (default: "id") */
    idField?: string;
    
    /** Presentation mode */
    presentation: "cards" | "table" | "log";
    
    /** For cards: render snippet for each item (receives item + selection context) */
    renderItem?: TemplateSurface;

    /** For cards: compact the underlying ListGrid spacing/layout for embedded surfaces. */
    listGridVariant?: "default" | "compact";
    listGridGap?: number | string | null;

    /** For reorder mode: custom item rendering */
    renderReorderItem?: TemplateSurface;
    
    /** For table: column definitions */
    columns?: TableColumn[];
    
    /** For table: row actions */
    rowActions?: EntityListSharedProps<T>["rowActions"];

    /** For table: whether to show the actions column */
    showRowActions?: boolean;

    /** For table: custom cell rendering */
    renderCell?: TemplateSurface;

    /** For table: expanded row rendering */
    renderExpandedRow?: TemplateSurface;

    /** For table: externally controlled expanded rows */
    expandedRowIds?: string[];

    /** For table: row action selection handler */
    onRowActionSelect?: (row: TableRow<T>, action: TableRowAction) => void;

    /** For log presentation: map loaded items into Poodle log entries. */
    toLogEntries?: EntityListSharedProps<T>["toLogEntries"];

    /** For log presentation: custom action icon snippet. */
    actionIcon?: TemplateSurface;

    /** For log presentation: custom entry details snippet. */
    entryDetails?: TemplateSurface;

    /** For log presentation: derive action type semantics. */
    getActionType?: EntityListSharedProps<T>["getActionType"];

    /** For log presentation: format action labels. */
    formatAction?: EntityListSharedProps<T>["formatAction"];

    /** For log presentation: format resource labels. */
    formatResourceType?: EntityListSharedProps<T>["formatResourceType"];

    /** For log presentation: derive actor hrefs. */
    getActorHref?: EntityListSharedProps<T>["getActorHref"];

    /** For log presentation: derive resource hrefs. */
    getResourceHref?: EntityListSharedProps<T>["getResourceHref"];
    
    /** Declarative filter configuration */
    filters?: FilterConfig[];

    /** Named baseline query variants rendered above filters. */
    queryVariants?: ListVariantDefinition[];

    /** Fallback variant used when query state does not name one explicitly. */
    defaultVariantId?: string;

    /** Optional loader for API-published list variants and filter definitions. */
    capabilitiesLoader?: EntityListCapabilitiesLoader;

    /** Effective default sort shown when the query has no explicit sort override. */
    defaultSort?: SortField[];
    
    /** Batch action configuration */
    batchActions?: BatchActionConfig[];
    
    /** Reorder configuration */
    reorder?: ReorderConfig<T>;

    /** Optional custom reorder surface for non-flat reorder workflows */
    customReorderContent?: TemplateSurface;
    
    /** Optional add button */
    onAdd?: () => void;
    addLabel?: string;
    
    /** Optional callback when data changes */
    onDataChange?: () => void;

    /** Optional callback when selected item IDs change */
    onSelectedIdsChange?: (ids: string[]) => void;
    
    /** Optional callback when visible item count changes */
    onVisibleCountChange?: (count: number) => void;

    /** Optional callback when total item count changes */
    onTotalCountChange?: (count: number) => void;

    /** External selection mode control */
    selectionMode?: boolean;

    /** External reorder mode control */
    reorderMode?: boolean;

    /** Called when selection mode changes */
    onSelectionModeChange?: (enabled: boolean) => void;

    /** Called when reorder mode changes */
    onReorderModeChange?: (enabled: boolean) => void;

    /** External query state (filters, sort, page, limit) */
    query?: EntityListSharedProps<T>["query"];

    /** Called when query changes (parent manages URL sync) */
    onQueryChange?: EntityListSharedProps<T>["onQueryChange"];

    /** Whether the filter toolbar should start collapsed. */
    filterToolbarCollapsed?: boolean;

    /**
     * Debounce (ms) applied to free-text search filters before a refetch fires.
     * Non-search filters (select, sort) always apply immediately. Set to 0 to
     * disable debouncing. Defaults to 250ms.
     */
    searchDebounceMs?: number;

    /** Called when reorder can genuinely be used for the current result set */
    onReorderAvailabilityChange?: (enabled: boolean) => void;

    /** Called when template shells need to render reorder controls outside the list body. */
    onReorderActionStateChange?: (state: ReorderActionState) => void;

    /** Custom reorder error handler for conflict recovery */
    onReorderError?: EntityListSharedProps<T>["onReorderError"];
  }

  // --- Props ---

  let {
    title,
    dataLoader,
    reloadKey,
    idField = "id",
    presentation,
    renderItem,
    listGridVariant = "default",
    listGridGap = "1rem",
    renderReorderItem,
    columns,
    rowActions,
    showRowActions = true,
    renderCell,
    renderExpandedRow,
    expandedRowIds = [],
    onRowActionSelect,
    toLogEntries,
    actionIcon,
    entryDetails,
    getActionType,
    formatAction,
    formatResourceType,
    getActorHref,
    getResourceHref,
    filters: filterConfigs = [],
    queryVariants = [],
    defaultVariantId,
    capabilitiesLoader,
    defaultSort = [],
    batchActions = [],
    reorder,
    customReorderContent,
    onAdd,
    addLabel = "Add",
    onDataChange,
    onSelectedIdsChange,
    onVisibleCountChange,
    onTotalCountChange,
    selectionMode: externalSelectionMode,
    reorderMode: externalReorderMode,
    onSelectionModeChange,
    onReorderModeChange,
    query: externalQuery,
    onQueryChange,
    filterToolbarCollapsed = false,
    searchDebounceMs = 250,
    onReorderAvailabilityChange,
    onReorderActionStateChange,
    onReorderError
  }: Props = $props();

  // --- State ---

  const toastStore = useToasts();
  const selectionScopeId = createListSelectionScopeId();

  const listQuery = createEntityListQuery({
    external: () => externalQuery,
    onChange: () => onQueryChange
  });
  const currentQuery = $derived(listQuery.current);

  const capabilities = useAuthenticatedData<ListCapabilities | null>(
    (fetch, token) => capabilitiesLoader?.(fetch, token) ?? Promise.resolve(null),
    { defaultValue: null }
  );
  const loadedCapabilities = $derived(capabilitiesLoader ? capabilities.data : null);
  let effectiveFilterConfigs = $derived(loadedCapabilities?.filters ?? filterConfigs);
  let effectiveQueryVariants = $derived(loadedCapabilities?.variants ?? queryVariants);
  let effectiveDefaultVariantId = $derived(loadedCapabilities?.defaultVariantId ?? defaultVariantId);
  let currentSort = $derived(currentQuery.sort ?? []);
  let effectiveSort = $derived(currentSort.length > 0 ? currentSort : defaultSort);
  let resolvedDefaultVariantId = $derived(
    effectiveDefaultVariantId ?? effectiveQueryVariants.find((variant) => variant.isDefault)?.id ?? null
  );
  let selectedVariantId = $state<string | null>(null);
  let variantSelectionTouched = $state(false);
  let currentVariantId = $derived(
    currentQuery.variant ?? (variantSelectionTouched ? selectedVariantId : resolvedDefaultVariantId)
  );
  const effectiveQuery = $derived({ ...currentQuery, variant: currentVariantId ?? undefined });
  let queryVariantItems = $derived<CardToggleItem[]>(
    effectiveQueryVariants.map((variant) => ({
      value: variant.id,
      label: variant.label,
      description: variant.description,
      count: variant.count
    }))
  );
  let activeQueryVariant = $derived(
    effectiveQueryVariants.find((variant) => variant.id === currentVariantId) ?? null
  );
  let activeAdHocFilterCount = $derived(
    effectiveFilterConfigs.filter((filter) => filter.type !== "sort" && isListFilterActive(currentQuery, filter)).length
  );
  let hasActiveEphemeralControls = $derived(activeAdHocFilterCount > 0 || currentSort.length > 0);
  let queryVariantColumns = $derived(Math.min(4, Math.max(1, queryVariantItems.length)) as 1 | 2 | 3 | 4);
  let currentPage = $derived(currentQuery.page ?? 1);
  let currentPageSize = $derived(currentQuery.limit ?? DEFAULT_PAGE_SIZE);

  const getSelectItems = useEntityListFilterOptions(() => effectiveFilterConfigs);
  const search = useEntityListSearch(() => effectiveFilterConfigs);

  // Selection mode (internal or external)
  let internalSelectionMode = $state(false);
  let selectionMode = $derived(externalSelectionMode ?? internalSelectionMode);

  // Reorder mode (internal or external)
  let internalReorderMode = $state(false);
  let reorderMode = $derived(externalReorderMode ?? internalReorderMode);
  const entityListContext: EntityListContext = {
    get reorderMode() {
      return reorderMode;
    }
  };

  setContext(UNDERLAY_ENTITY_LIST_CONTEXT_KEY, entityListContext);
  let lastNotifiedVisibleCount = $state<number | null>(null);
  let lastNotifiedTotalCount = $state<number | null>(null);
  let lastNotifiedReorderAvailability = $state<boolean | null>(null);

  // Custom batch action dialog
  let pendingDialogAction = $state<BatchActionConfig | null>(null);
  const selectedIdsKey = $derived.by(() => JSON.stringify(batch.selectedIds));

  const pageData = useAuthenticatedData<PagedListResult<T>>(
    (fetch, token) => untrack(() => dataLoader(fetch, token, effectiveQuery)),
    {
      defaultValue: { data: [], total: 0 },
      queryKey: () => listQueryKey(effectiveQuery, reloadKey)
    }
  );

  const items = $derived(pageData.data?.data ?? []);
  const itemCount = $derived(items.length);
  const totalCount = $derived(resolveTotalCount(pageData.data, items.length));
  const totalPages = $derived(Math.max(1, Math.ceil(totalCount / currentPageSize)));
  const hasNextPage = $derived(currentPage < totalPages || Boolean(pageData.data?.hasMore));
  const reorderAvailable = $derived(isListReorderAvailable(reorder, {
    items: itemCount,
    total: totalCount,
    page: currentPage,
    pageSize: currentPageSize
  }));
  const itemIds = $derived(items.map(getItemKey));
  const logEntries = $derived(toLogEntries ? toLogEntries(items) : []);
  // A refetch is in flight but we still have prior results on screen (e.g. a
  // debounced search). The full-page loader only shows on the first load, so
  // surface a subtle busy affordance instead of leaving the list looking frozen.
  const isRefetching = $derived(pageData.refetching && items.length > 0);

  // Batch actions
  const batch = useBatchActions<string>();
  const unregisterSelectionScope = registerListSelectionScope(selectionScopeId, () => {
    batch.clear();
    setSelectionMode(false);
  });

  onDestroy(unregisterSelectionScope);
  const reorderSession = useEntityListReorder({
    config: () => reorder,
    active: () => reorderMode,
    available: () => reorderAvailable,
    setActive: setReorderMode,
    items: () => items,
    getItemId: getItemKey,
    query: () => currentQuery,
    onEnter: () => {
      if (selectionMode) {
        setSelectionMode(false);
        batch.clear();
      }
    },
    onSaved: async () => {
      onDataChange?.();
      await pageData.refetch();
    },
    onError: () => onReorderError
  });
  const reorderController = $derived(reorderSession.controller);

  // Notify parent of visible and total count changes
  $effect(() => {
    if (lastNotifiedVisibleCount !== items.length) {
      lastNotifiedVisibleCount = items.length;
      onVisibleCountChange?.(items.length);
    }
    if (lastNotifiedTotalCount !== totalCount) {
      lastNotifiedTotalCount = totalCount;
      onTotalCountChange?.(totalCount);
    }
    if (lastNotifiedReorderAvailability !== reorderAvailable) {
      lastNotifiedReorderAvailability = reorderAvailable;
      onReorderAvailabilityChange?.(reorderAvailable);
    }
  });

  $effect(() => {
    selectedIdsKey;
    onSelectedIdsChange?.([...batch.selectedIds]);
  });

  $effect(() => {
    onReorderActionStateChange?.({
      active: reorderMode,
      available: reorderAvailable,
      dirty: Boolean(reorderController?.isDirty),
      saving: Boolean(reorderController?.isPending),
      enter: reorderSession.enter,
      save: reorderSession.save,
      cancel: reorderSession.exit
    });
  });

  $effect(() => {
    if (!selectionMode && batch.count > 0) {
      batch.clear();
    }
  });

  // Register batch actions (skip dialog actions — handled separately)
  $effect(() => {
    const directActions = batchActions.filter((action) => !action.dialog);
    untrack(() => {
      const directActionIds = new Set(directActions.map((action) => action.id));

      for (const existingAction of batch.actions) {
        if (!directActionIds.has(existingAction.id)) {
          batch.unregisterAction(existingAction.id);
        }
      }

      for (const action of directActions) {
        batch.registerAction({
          id: action.id,
          label: action.label,
          icon: action.icon,
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
                  confirmLabel: action.confirm.confirmLabel ?? action.label,
                  cancelLabel: action.confirm.cancelLabel
                }
              : undefined,
          execute: (ids: string[]) => executeBatchAction(action, ids)
        });
      }
    });
  });

  // --- Helpers ---

  async function executeBatchAction(action: BatchActionConfig, ids: string[]) {
    try {
      await action.handler(ids);
      onDataChange?.();
      await pageData.refetch();
      return { success: true, affected: ids.length };
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      toastStore.push({ message, variant: "error" });
      throw error;
    }
  }

  function resolveTotalCount(result: PagedListResult<T> | undefined, visibleCount: number): number {
    if (typeof result?.total === "number" && Number.isFinite(result.total) && result.total >= 0) {
      return result.total;
    }
    return visibleCount;
  }

  function getVariantPillTone(
    tone: ListVariantDefinition["tone"]
  ): "neutral" | "info" | "success" | "warning" | "danger" {
    switch (tone) {
      case "info":
      case "success":
      case "warning":
      case "danger":
        return tone;
      default:
        return "neutral";
    }
  }

  function getSearchPlaceholder(filter: FilterConfig): string {
    return filter.placeholder ?? `Search ${filter.label.toLowerCase()}...`;
  }

  function applyFilterChange(filter: FilterConfig, value: string) {
    if (getListFilterValue(currentQuery, filter) === value) {
      return;
    }
    listQuery.set({
      ...currentQuery,
      filters: updateListFilters(currentQuery, filter, value),
      page: 1
    });
  }

  function handleQueryVariantChange(value: string | null) {
    const nextVariant = value ?? undefined;
    const currentEffectiveVariant = effectiveQuery.variant;
    if (currentEffectiveVariant === nextVariant && variantSelectionTouched && selectedVariantId === value) {
      return;
    }
    variantSelectionTouched = true;
    selectedVariantId = value;
    listQuery.set({
      ...currentQuery,
      variant: nextVariant,
      page: 1
    });
  }

  function handleSortChange(sortFields: SortField[]) {
    const normalizedSort = sortFieldsEqual(sortFields, defaultSort) ? [] : sortFields;
    if (sortFieldsEqual(currentSort, normalizedSort)) {
      return;
    }
    listQuery.set({
      ...currentQuery,
      sort: normalizedSort,
      page: 1
    });
  }

  function handlePageChange(page: number) {
    if (page === currentPage) {
      return;
    }
    listQuery.set({
      ...currentQuery,
      page
    });
  }

  function clearSort() {
    handleSortChange([]);
  }

  function clearEphemeralControls() {
    if (!hasActiveEphemeralControls) {
      return;
    }

    search.clear();

    listQuery.set({
      ...currentQuery,
      filters: [],
      sort: [],
      page: 1
    });
  }

  function getSortOrderValue() {
    return effectiveSort.map((field) => ({
      key: field.field,
      direction: field.direction
    }));
  }

  function getItemKey(item: T): string {
    return String((item as Record<string, unknown>)[idField]);
  }

  function handleRowActionSelect(event: { row: TableRow; action: TableRowAction }) {
    // Poodle's DataTable emits a non-generic TableRow; our row payload is
    // TableRow<T> at runtime.
    onRowActionSelect?.(event.row as TableRow<T>, event.action);
  }

  function setSelectionMode(enabled: boolean) {
    if (enabled) {
      claimListSelectionScope(selectionScopeId);
    } else {
      releaseListSelectionScope(selectionScopeId);
    }
    if (externalSelectionMode === undefined) {
      internalSelectionMode = enabled;
    }
    onSelectionModeChange?.(enabled);
  }

  function setReorderMode(enabled: boolean) {
    if (externalReorderMode === undefined) {
      internalReorderMode = enabled;
    }
    onReorderModeChange?.(enabled);
  }

  function toggleSelectionMode() {
    if (reorderMode) {
      setReorderMode(false);
      reorderSession.reset();
    }
    const nextSelectionMode = !selectionMode;
    setSelectionMode(nextSelectionMode);
    if (!nextSelectionMode) {
      batch.clear();
    }
  }

  async function confirmPendingBatchAction() {
    await batch.confirmPendingAction();
  }

  function getItemContext(item: T): ItemContext {
    const itemId = getItemKey(item);
    return {
      selectionMode,
      reorderMode,
      selected: batch.isSelected(itemId),
      onToggle: (selected: boolean) => batch.toggle(itemId, selected),
      refetch: () => pageData.refetch()
    };
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.defaultPrevented) {
      return;
    }
    if (event.key === "Escape") {
      if (pendingDialogAction || batch.pendingAction) {
        return;
      }
      if (selectionMode) {
        setSelectionMode(false);
        batch.clear();
      } else if (reorderMode) {
        reorderSession.exit();
      }
    }
  }

  const tableColumns = $derived<TableColumn[]>(columns ?? []);

  // Table rows
  const tableRows = $derived<TableRow<T>[]>(
    items.map((item) => ({
      id: getItemKey(item),
      cells: tableColumns.reduce((acc, col) => {
        acc[col.id] = String((item as Record<string, unknown>)[col.id] ?? "");
        return acc;
      }, {} as Record<string, string>),
      data: item
    }))
  );

  function toBulkActionTone(tone: "default" | "danger" | "warning" | undefined): BulkAction["tone"] {
    if (tone === "danger" || tone === "warning") {
      return tone;
    }
    return "default";
  }

  function isBatchActionDisabled(action: BatchActionConfig | undefined): boolean {
    if (!action?.disabled) return false;
    return typeof action.disabled === "function"
      ? action.disabled(batch.selectedIds)
      : action.disabled;
  }

  // Convert batch actions for BulkActionBar (include dialog actions)
  const bulkActions = $derived<BulkAction[]>([
    ...batch.availableActions.map((action) => {
      const config = batchActions.find((candidate) => candidate.id === action.id);
      return {
        id: action.id,
        label: action.label,
        icon: action.icon,
        tone: toBulkActionTone(action.variant),
        disabled: isBatchActionDisabled(config)
      };
    }),
    ...batchActions
      .filter((a) => a.dialog)
      .map((action) => ({
        id: action.id,
        label: action.label,
        icon: action.icon,
        tone: toBulkActionTone(action.tone),
        disabled: isBatchActionDisabled(action)
      }))
  ]);

  // --- Batch action dialog handlers ---

  function handleBatchAction(actionId: string) {
    const actionConfig = batchActions.find((a) => a.id === actionId);
    if (isBatchActionDisabled(actionConfig)) return;

    if (actionConfig?.dialog) {
      pendingDialogAction = actionConfig;
      return;
    }

    batch.requestAction(actionId);
  }

  async function handleDialogSubmit(values: Record<string, unknown>) {
    if (!pendingDialogAction) return;
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
    }
  }

  function handleDialogCancel() {
    pendingDialogAction = null;
  }

</script>

<svelte:window onkeydown={handleKeydown} />

{#snippet filterToolbar(ariaLabel: string, queryVariantAriaLabel: string)}
  <FilterToolbar
    {ariaLabel}
    summaryText="Views & Filters"
    size="sm"
    collapsed={filterToolbarCollapsed}
  >
    {#snippet summary()}
      <span class="underlay-entity-list__toolbar-summary">
        <PaginationSummary
          currentPage={currentPage}
          totalPages={totalPages}
          totalItems={totalCount}
          pageSize={currentPageSize}
        />
        {#if isRefetching}
          <span
            class="underlay-entity-list__refetching"
            role="status"
            aria-live="polite"
          >
            Updating…
          </span>
        {/if}
        {#if activeQueryVariant}
          <Pill
            tone={getVariantPillTone(activeQueryVariant.tone)}
            appearance="badge"
            size="xs"
            typography="inherit"
          >
            {activeQueryVariant.label}
          </Pill>
        {/if}
        {#if activeAdHocFilterCount > 0}
          <Pill tone="neutral" appearance="badge" size="xs" typography="inherit">
            {activeAdHocFilterCount} filter{activeAdHocFilterCount === 1 ? "" : "s"}
          </Pill>
        {/if}
        {#if currentSort.length > 0}
          <Pill tone="neutral" appearance="badge" size="xs" typography="inherit">
            Sorted
          </Pill>
        {/if}
      </span>
    {/snippet}

    {#snippet actions()}
      {#if hasActiveEphemeralControls}
        <IconButton
          icon="x"
          variant="ghost"
          size="sm"
          ariaLabel="Clear filters and sort"
          tooltip="Clear filters and sort"
          onClick={clearEphemeralControls}
        />
      {/if}
      <IconButton
        icon="refresh-cw"
        variant="ghost"
        size="sm"
        ariaLabel="Refresh list"
        tooltip="Refresh"
        onClick={() => pageData.refetch()}
      />
    {/snippet}

    {#if queryVariantItems.length > 0}
      <div class="underlay-entity-list__query-variants">
        <CardToggleGroup
          items={queryVariantItems}
          value={currentVariantId}
          columns={queryVariantColumns}
          allowDeactivation
          size="xs"
          density="compact"
          ariaLabel={queryVariantAriaLabel}
          onValueChange={handleQueryVariantChange}
        />
      </div>
    {/if}

    {#each effectiveFilterConfigs as filter (filter.id)}
      <div
        class="underlay-entity-list__filter-control"
        data-active={isListFilterActive(currentQuery, filter)}
      >
        {#if filter.type === "search"}
          <TextInput
            id={`filter-${filter.id}`}
            type="search"
            size="sm"
            value={search.value(filter, getListFilterValue(currentQuery, filter))}
            ariaLabel={filter.label}
            placeholder={getSearchPlaceholder(filter)}
            onValueChange={(value) =>
              search.change(filter, value, searchDebounceMs, () => applyFilterChange(filter, value))}
          />
        {:else if filter.type === "select"}
          <Select
            id={`filter-${filter.id}`}
            size="sm"
            disabled={filter.disabled ?? false}
            value={getListFilterValue(currentQuery, filter)}
            options={getSelectItems(filter)}
            loadOptions={filter.searchable ? filter.loadOptions ?? null : null}
            loadKey={filter.loadKey ?? null}
            searchable={filter.searchable ?? false}
            native={false}
            clearable
            placeholder={filter.placeholder ?? filter.label}
            ariaLabel={filter.label}
            onValueChange={(value) => applyFilterChange(filter, value)}
          />
        {:else if filter.type === "sort" && filter.sortFields}
          <OrderBy
            fields={filter.sortFields}
            value={getSortOrderValue()}
            size="sm"
            ariaLabel={filter.label}
            showClearButton={false}
            onChange={(value) => handleSortChange(orderByToSortFields(value))}
            compact
          />
        {/if}
      </div>
    {/each}
  </FilterToolbar>
{/snippet}

<!--
  Shared list body for the titled (ListContainer) and untitled (EntityListPage)
  render paths. Only the log empty message differs between the two, so it is
  passed in; everything else is identical.
-->
{#snippet listBody(logEmptyMessage: string)}
  {#if reorderMode && customReorderContent}
    {@render customReorderContent({
      items,
      cancel: reorderSession.exit,
      refetch: () => pageData.refetch()
    })}
  {:else if reorderMode && reorderController}
    <EditableList
      items={reorderSession.editableItems}
      embeddedHandle
      dirty={reorderController.isDirty}
      submitting={reorderController.isPending}
      errorMessage={reorderSession.error}
      showWorkflowChrome={false}
      onSubmit={reorderSession.save}
      onCancel={reorderSession.exit}
      onReorder={reorderSession.updateOrder}
    >
      {#snippet item(reorderEntry)}
        {@const originalItem = reorderSession.originalItem(reorderEntry.id)}
        {#if renderReorderItem && originalItem}
          <div data-reorder-highlighted={reorderSession.isHighlighted(reorderEntry.id)}>
            {@render renderReorderItem(originalItem, {
              ...getItemContext(originalItem),
              highlighted: reorderSession.isHighlighted(reorderEntry.id)
            })}
          </div>
        {:else if renderItem && originalItem}
          {@render renderItem(originalItem, getItemContext(originalItem))}
        {:else}
          <ListCard title={reorderEntry.id} layout="compact" showReorderHandle />
        {/if}
      {/snippet}
    </EditableList>
  {:else if presentation === "cards"}
    <ListGrid minItemWidth="var(--underlay-list-grid-min, 20rem)" variant={listGridVariant} gap={listGridGap}>
      {#each items as item (getItemKey(item))}
        {#if renderItem}
          {@render renderItem(item, getItemContext(item))}
        {/if}
      {/each}
    </ListGrid>
  {:else if presentation === "log"}
    <LogList
      entries={logEntries}
      variant="audit"
      emptyMessage={logEmptyMessage}
      actionIcon={actionIcon}
      entryDetails={entryDetails}
      {getActionType}
      {formatAction}
      {formatResourceType}
      {getActorHref}
      {getResourceHref}
    />
  {:else if presentation === "table"}
    <DataTable
      columns={tableColumns}
      rows={tableRows}
      {expandedRowIds}
      {showRowActions}
      rowActions={rowActions as unknown as TableRowAction[] | ((row: TableRow) => TableRowAction[]) | undefined}
      selectable={selectionMode}
      selectedRowIds={batch.selectedIds}
      emptyMessage="No items found"
      onRowActionSelect={handleRowActionSelect}
      onRowToggle={({ rowId, selected }) => batch.toggle(rowId, selected)}
      onToggleAll={({ selected }) => {
        if (selected) {
          batch.selectAll(itemIds);
        } else {
          batch.clear();
        }
      }}
    >
      {#snippet cell(column, row, value)}
        {#if renderCell}
          {@render renderCell(column, row, value)}
        {:else}
          {value}
        {/if}
      {/snippet}
      {#snippet expandedRow(row)}
        {#if renderExpandedRow}
          {@render renderExpandedRow(row)}
        {/if}
      {/snippet}
    </DataTable>
  {/if}
{/snippet}

{#snippet retryCallout(message: string, retry: () => Promise<void>, title?: string)}
  <Callout tone="danger" {title} {message} announceMode="polite">
    {#snippet actions()}
      <Button type="button" variant="ghost" size="sm" onClick={retry}>Retry</Button>
    {/snippet}
  </Callout>
{/snippet}

{#if capabilitiesLoader && capabilities.error}
  {@render retryCallout(capabilities.error, () => capabilities.refetch(), "Unable to load list options")}
{/if}

{#if title}
  <!-- When used standalone (not inside EntityListPage), show ListContainer -->
  <ListContainer
    {title}
    state={pageData.loading ? "loading" : pageData.error ? "error" : items.length > 0 ? "ready" : "empty"}
    emptyTitle="No items found"
    emptyMessage="Try adjusting your filters or add a new item."
    currentPage={currentPage}
    totalPages={totalPages}
    totalItems={totalCount}
    pageSize={currentPageSize}
    showPaginationSummary={false}
    onPageChange={handlePageChange}
  >
    {#snippet actions()}
      {#if itemCount > 0 && !reorderMode && batchActions.length > 0}
        <IconButton
          type="button"
          variant="secondary"
          tone={selectionMode ? "danger" : "default"}
          icon={selectionMode ? "x" : "check-square"}
          ariaLabel={selectionMode ? "Cancel selection" : "Select items"}
          tooltip={selectionMode ? "Cancel Selection" : "Select Items"}
          onClick={toggleSelectionMode}
        />
      {/if}
      {#if reorderAvailable && !selectionMode}
        <EntityReorderControls
          active={reorderMode}
          available={reorderAvailable}
          dirty={Boolean(reorderController?.isDirty)}
          saving={Boolean(reorderController?.isPending)}
          onEnter={reorderSession.enter}
          onSave={reorderSession.save}
          onCancel={reorderSession.exit}
        />
      {/if}
      {#if !selectionMode && !reorderMode && onAdd}
        <Button variant="primary" onClick={onAdd}>{addLabel}</Button>
      {/if}
    {/snippet}

    {#snippet filters()}
      {#if (queryVariantItems.length > 0 || effectiveFilterConfigs.length > 0) && !reorderMode}
        {@render filterToolbar(`${title} filters`, `${title ?? "List"} query variants`)}
      {/if}
    {/snippet}

    {#snippet loading()}
      <PageLoading presentation="inline" message={`Loading ${title.toLowerCase()}...`} />
    {/snippet}
    {#snippet error()}
      {#if pageData.error}
        {@render retryCallout(pageData.error, () => pageData.refetch())}
      {/if}
    {/snippet}
    {#snippet empty()}
      <EmptyState
        title={`No ${title.toLowerCase()} found`}
        message="Try adjusting your filters or create a new item."
      >
        {#snippet actions()}
          {#if onAdd}
            <Button variant="primary" onClick={onAdd}>{addLabel}</Button>
          {/if}
        {/snippet}
      </EmptyState>
    {/snippet}

    {@render listBody(`No ${title.toLowerCase()} found`)}
  </ListContainer>
{:else}
  <!-- When used inside EntityListPage, don't show ListContainer shell -->
  {#if (queryVariantItems.length > 0 || effectiveFilterConfigs.length > 0) && !reorderMode}
    {@render filterToolbar("Filters", "Query variants")}
  {/if}
  
  {#if pageData.loading && items.length === 0}
    <PageLoading presentation="inline" message="Loading..." />
  {:else if pageData.error}
    {@render retryCallout(pageData.error, () => pageData.refetch())}
  {:else if items.length === 0}
    <EmptyState
      title="No items found"
      message="Try adjusting your filters or create a new item."
    />
  {:else}
    {@render listBody("No items found")}
  {/if}

  {#if totalPages > 1}
    <div class="underlay-entity-list__pagination">
      <Pagination
        currentPage={currentPage}
        totalPages={totalPages}
        showInfo={false}
        size="sm"
        compact
        ariaLabel="List pagination"
        onPageChange={handlePageChange}
      />
    </div>
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
    onClear={() => {
      batch.clear();
      setSelectionMode(false);
    }}
    onSelectAll={() => batch.selectAll(itemIds)}
    onAction={handleBatchAction}
  />
{/if}

<!-- Batch action confirmation dialog -->
{#if batch.pendingAction}
  <AlertDialog
    open={true}
    title={batch.pendingAction.confirm?.title ?? "Confirm"}
    description={batch.getConfirmDescription()}
    confirmLabel={batch.pendingAction.confirm?.confirmLabel ?? "Confirm"}
    cancelLabel={batch.pendingAction.confirm?.cancelLabel ?? "Cancel"}
    tone={batch.pendingAction.variant === "danger" ? "danger" : "warning"}
    onConfirm={confirmPendingBatchAction}
    onCancel={() => batch.cancelPendingAction()}
  />
{/if}

<!-- Batch action custom dialog -->
{#if pendingDialogAction?.dialog}
  <Dialog
    open={true}
    title={pendingDialogAction.dialog.title}
    onRequestClose={handleDialogCancel}
  >
    {@render pendingDialogAction.dialog.content({
      ids: batch.selectedIds,
      onSubmit: handleDialogSubmit,
      onCancel: handleDialogCancel
    })}
  </Dialog>
{/if}

<style>
  .underlay-entity-list__toolbar-summary {
    display: inline-flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.5rem;
    min-width: 0;
  }

  .underlay-entity-list__toolbar-summary :global(.poodle-pagination-summary) {
    font-size: 0.875em;
  }

  .underlay-entity-list__refetching {
    font-size: 0.8125em;
    opacity: 0.7;
    font-style: italic;
  }

  .underlay-entity-list__filter-control {
    min-width: 0;
    opacity: 0.68;
    transition:
      opacity 120ms ease,
      filter 120ms ease;
  }

  .underlay-entity-list__filter-control[data-active="true"],
  .underlay-entity-list__filter-control:focus-within,
  .underlay-entity-list__filter-control:hover {
    opacity: 1;
  }

  .underlay-entity-list__filter-control[data-active="false"] :global(.poodle-select__value[data-placeholder="true"]),
  .underlay-entity-list__filter-control[data-active="false"] :global(.poodle-order-by__summary[data-placeholder="true"]) {
    color: var(--poodle-color-text-muted);
  }

  .underlay-entity-list__query-variants {
    display: grid;
    grid-column: 1 / -1;
    margin-bottom: 0.125rem;
  }

  .underlay-entity-list__query-variants :global(.poodle-card) {
    --poodle-recipe-card-fill: color-mix(
      in srgb,
      var(--poodle-color-background-elevated) 95%,
      var(--poodle-color-text-primary)
    );
    --poodle-recipe-card-hover-fill: color-mix(
      in srgb,
      var(--poodle-color-background-elevated) 92%,
      var(--poodle-color-text-primary)
    );
    --poodle-recipe-card-border: color-mix(
      in srgb,
      var(--poodle-color-border-default) 28%,
      transparent
    );
    --poodle-recipe-card-shadow:
      0 0.375rem 0.875rem color-mix(in srgb, black 12%, transparent),
      inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-default) 18%, transparent);
  }

  :global([data-theme="light"]) .underlay-entity-list__query-variants :global(.poodle-card) {
    --poodle-recipe-card-fill: var(--poodle-color-background-elevated);
    --poodle-recipe-card-hover-fill: color-mix(
      in srgb,
      var(--poodle-color-background-elevated) 92%,
      var(--poodle-color-background-surface)
    );
  }

  .underlay-entity-list__pagination {
    display: flex;
    justify-content: flex-end;
  }

  [data-reorder-highlighted="true"] {
    outline: 2px solid var(--underlay-color-accent, #14b8a6);
    outline-offset: 0.25rem;
    border-radius: 0.75rem;
  }
</style>
