<script lang="ts">
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  type T = any;
  import { onDestroy, untrack } from "svelte";
  import { getAuthConfig } from "../patterns/auth";
  import { useAuthenticatedData } from "../runtime/auth";
  import { useBatchActions } from "../patterns/batch-actions.svelte";
  import {
    createReorderController,
    type ReorderController,
    type ReorderableItem
  } from "../patterns/reorder-controller.svelte";
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
    Pagination,
    PaginationSummary,
    CardToggleGroup
  } from "@poodle/svelte";
  import type {
    TableColumn,
    TableRow,
    TableRowAction,
    BulkAction,
    EditableListItem,
    LogActor,
    CardToggleItem
  } from "@poodle/svelte";
  import type { FilterField, QueryParams, SortField } from "../client/query";
  import { DEFAULT_PAGE_SIZE } from "../patterns/pagination-types";
  import {
    claimListSelectionScope,
    createListSelectionScopeId,
    registerListSelectionScope,
    releaseListSelectionScope
  } from "../patterns/list-selection-scope.svelte";
  import { default as EntityReorderControls } from "./EntityReorderControls.svelte";
  import type {
    BatchActionConfig,
    CustomReorderConfig,
    EntityListCapabilitiesLoader,
    EntityListDataLoader,
    EntityListSharedProps,
    FilterConfig,
    FetchFn,
    ListVariantDefinition,
    ListCapabilities,
    LoadedReorderConfig,
    PagedListResult,
    ReorderActionState,
    ReorderErrorResult,
    ReorderConfig,
    TemplateSurface
  } from "./template.types";

  interface ItemContext {
    selectionMode: boolean;
    reorderMode: boolean;
    selected: boolean;
    onToggle: (selected: boolean) => void;
    refetch: () => Promise<void>;
  }

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
    onReorderAvailabilityChange,
    onReorderActionStateChange,
    onReorderError
  }: Props = $props();

  // --- State ---

  const toastStore = useToasts();
  const selectionScopeId = createListSelectionScopeId();

  let internalQuery = $state<QueryParams>({
    page: 1,
    limit: DEFAULT_PAGE_SIZE
  });
  let currentQuery = $derived(normalizeQuery(externalQuery ?? internalQuery));
  let loadedCapabilities = $state<ListCapabilities | null>(null);
  let effectiveFilterConfigs = $derived(loadedCapabilities?.filters ?? filterConfigs);
  let effectiveQueryVariants = $derived(loadedCapabilities?.variants ?? queryVariants);
  let effectiveDefaultVariantId = $derived(loadedCapabilities?.defaultVariantId ?? defaultVariantId);
  let currentSort = $derived(currentQuery.sort ?? []);
  let resolvedDefaultVariantId = $derived(
    effectiveDefaultVariantId ?? effectiveQueryVariants.find((variant) => variant.isDefault)?.id ?? null
  );
  let currentVariantId = $derived(currentQuery.variant ?? resolvedDefaultVariantId);
  let queryVariantItems = $derived<CardToggleItem[]>(
    effectiveQueryVariants.map((variant) => ({
      value: variant.id,
      label: variant.label,
      description: variant.description,
      count: variant.count
    }))
  );
  let queryVariantColumns = $derived(Math.min(4, Math.max(1, queryVariantItems.length)) as 1 | 2 | 3 | 4);
  let currentPage = $derived(currentQuery.page ?? 1);
  let currentPageSize = $derived(currentQuery.limit ?? DEFAULT_PAGE_SIZE);

  // Async filter options
  let loadedFilterOptions = $state<Record<string, { value: string; label: string }[]>>({});
  let filterOptionsLoading = $state<Record<string, boolean>>({});
  let filterLoadKeys = $state<Record<string, string>>({});

  // Selection mode (internal or external)
  let internalSelectionMode = $state(false);
  let selectionMode = $derived(externalSelectionMode ?? internalSelectionMode);

  // Reorder mode (internal or external)
  let internalReorderMode = $state(false);
  let reorderMode = $derived(externalReorderMode ?? internalReorderMode);
  let reorderError = $state<string | null>(null);
  let lastNotifiedVisibleCount = $state<number | null>(null);
  let lastNotifiedTotalCount = $state<number | null>(null);
  let lastNotifiedReorderAvailability = $state<boolean | null>(null);

  // Custom batch action dialog
  let pendingDialogAction = $state<BatchActionConfig | null>(null);
  let reorderController = $state<ReorderController<ReorderableItem & T> | null>(null);
  let loadedReorderItems = $state<Array<T & ReorderableItem>>([]);
  let reorderHighlightedIds = $state<string[]>([]);
  let reorderSessionPending = $state(false);
  let previousListQueryKey = $state<string | null>(null);
  let pendingFetchQuery = $state<QueryParams | null>(null);
  let handledListQueryKey = $state<string | null>(null);

  const listQueryKey = $derived.by(() => JSON.stringify({
    query: currentQuery,
    reloadKey: reloadKey ?? null
  }));
  const selectedIdsKey = $derived.by(() => JSON.stringify(batch.selectedIds));
  const getCurrentQuery = () => pendingFetchQuery ?? currentQuery;
  const getDataLoader = () => dataLoader;

  async function loadPageData(fetch: FetchFn, token: string | null) {
    return await getDataLoader()(fetch, token, getCurrentQuery());
  }

  async function loadCapabilities(fetch: FetchFn, token: string | null) {
    if (!capabilitiesLoader) {
      return null;
    }
    const result = await capabilitiesLoader(fetch, token);
    loadedCapabilities = result;
    return result;
  }

  // Data loading (includes filters and sort)
  const pageData = useAuthenticatedData<PagedListResult<T>>(
    loadPageData,
    {
      defaultValue: {
        data: [],
        total: 0
      }
    }
  );

  useAuthenticatedData<ListCapabilities | null>(
    loadCapabilities,
    {
      defaultValue: null
    }
  );

  $effect(() => {
    const currentKey = listQueryKey;
    if (previousListQueryKey === null) {
      previousListQueryKey = currentKey;
      return;
    }

    if (previousListQueryKey !== currentKey) {
      previousListQueryKey = currentKey;
      if (handledListQueryKey === currentKey) {
        handledListQueryKey = null;
        return;
      }
      void pageData.refetch();
    }
  });

  $effect(() => {
    if (!pendingFetchQuery) {
      return;
    }

    if (JSON.stringify(normalizeQuery(pendingFetchQuery)) === JSON.stringify(currentQuery)) {
      pendingFetchQuery = null;
    }
  });

  const items = $derived(pageData.data?.data ?? []);
  const itemCount = $derived(items.length);
  const totalCount = $derived(resolveTotalCount(pageData.data, items.length));
  const totalPages = $derived(Math.max(1, Math.ceil(totalCount / currentPageSize)));
  const hasNextPage = $derived(currentPage < totalPages || Boolean(pageData.data?.hasMore));
  const reorderAvailable = $derived(
    isCustomReorderConfig(reorder)
      ? Boolean(reorder.enabled) && totalCount > 0
      : isLoadedReorderConfig(reorder)
      ? Boolean(reorder.enabled) && totalCount > 1
      : Boolean(reorder?.enabled) &&
          currentPage === 1 &&
          itemCount > 1 &&
          totalCount > 0 &&
          totalCount <= currentPageSize
  );
  const itemIds = $derived(items.map((item) => String((item as Record<string, unknown>)[idField])));
  const logEntries = $derived(toLogEntries ? toLogEntries(items) : []);

  // Batch actions
  const batch = useBatchActions<string>();
  const unregisterSelectionScope = registerListSelectionScope(selectionScopeId, () => {
    batch.clear();
    setSelectionMode(false);
  });

  onDestroy(unregisterSelectionScope);

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
      enter: enterReorderMode,
      save: handleReorderSubmit,
      cancel: exitReorderMode
    });
  });

  $effect(() => {
    if (!selectionMode && batch.count > 0) {
      batch.clear();
    }
  });

  // Load async filter options on mount
  $effect(() => {
    for (const filter of effectiveFilterConfigs) {
      const currentLoadKey = filter.loadKey ?? "";
      if (filterLoadKeys[filter.id] !== currentLoadKey) {
        filterLoadKeys = { ...filterLoadKeys, [filter.id]: currentLoadKey };
        if (loadedFilterOptions[filter.id]) {
          const nextOptions = { ...loadedFilterOptions };
          delete nextOptions[filter.id];
          loadedFilterOptions = nextOptions;
        }
      }
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

  // Register batch actions (skip dialog actions — handled separately)
  $effect(() => {
    untrack(() => {
      const directActions = batchActions.filter((action) => !action.dialog);
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
    await action.handler(ids);
    onDataChange?.();
    await pageData.refetch();
    return { success: true, affected: ids.length };
  }

  async function submitReorder(orderedIds: string[]) {
    if (reorder && "handler" in reorder) {
      await reorder.handler(orderedIds);
    }
  }

  function normalizeQuery(query: QueryParams): QueryParams {
    return {
      ...query,
      variant: query.variant ?? resolvedDefaultVariantId ?? undefined,
      page: Math.max(1, query.page ?? 1),
      limit: Math.max(1, query.limit ?? DEFAULT_PAGE_SIZE),
      filters:
        query.filters?.filter((filter: FilterField) => filter.value.trim() !== "") ?? [],
      sort: query.sort?.filter((field: SortField) => field.field.trim() !== "") ?? []
    };
  }

  function resolveTotalCount(result: PagedListResult<T> | undefined, visibleCount: number): number {
    if (typeof result?.total === "number" && Number.isFinite(result.total) && result.total >= 0) {
      return result.total;
    }
    return visibleCount;
  }

  function setQuery(nextQuery: QueryParams) {
    const normalizedQuery = normalizeQuery(nextQuery);
    pendingFetchQuery = normalizedQuery;
    handledListQueryKey = JSON.stringify({
      query: normalizedQuery,
      reloadKey: reloadKey ?? null
    });
    if (onQueryChange) {
      onQueryChange(normalizedQuery);
    } else {
      internalQuery = normalizedQuery;
    }
    void pageData.refetch();
  }

  function getFilterValue(filter: FilterConfig): string {
    const activeFilter = currentQuery.filters?.find(
      (entry: FilterField) => entry.field === filter.id
    );
    if (!activeFilter) {
      return filter.type === "select" ? "All" : "";
    }
    if (filter.type === "search" && activeFilter.operator === "like") {
      return activeFilter.value.replace(/^%|%$/g, "");
    }
    return activeFilter.value;
  }

  function buildNextFilters(filter: FilterConfig, value: string): FilterField[] {
    const nextFilters = (currentQuery.filters ?? []).filter(
      (entry: FilterField) => entry.field !== filter.id
    );

    if (!value || value === "All") {
      return nextFilters;
    }

    if (filter.type === "search") {
      const trimmedValue = value.trim();
      if (!trimmedValue) {
        return nextFilters;
      }
      return [
        ...nextFilters,
        {
          field: filter.id,
          operator: "like",
          value: `%${trimmedValue}%`
        }
      ];
    }

    return [
      ...nextFilters,
      {
        field: filter.id,
        value
      }
    ];
  }

  function sortFieldsEqual(left: SortField[], right: SortField[]): boolean {
    if (left.length !== right.length) return false;
    return left.every((field, index) => {
      const other = right[index];
      return field.field === other.field && field.direction === other.direction;
    });
  }

  function getSelectItems(filter: FilterConfig): { value: string; label: string }[] {
    const providedItems = loadedFilterOptions[filter.id] ?? filter.options ?? [];
    if (providedItems.some((item) => item.value === "All")) {
      return providedItems;
    }
    return [{ value: "All", label: `All ${filter.label.toLowerCase()}` }, ...providedItems];
  }

  function getFilterAriaLabel(filter: FilterConfig): string {
    return filter.label;
  }

  function getSearchPlaceholder(filter: FilterConfig): string {
    return filter.placeholder ?? `Search ${filter.label.toLowerCase()}...`;
  }

  function getSelectPlaceholder(filter: FilterConfig): string {
    return `All ${filter.label.toLowerCase()}`;
  }

  function isFilterActive(filter: FilterConfig): boolean {
    if (filter.type === "sort") {
      return currentSort.length > 0;
    }

    const value = getFilterValue(filter);
    if (filter.type === "search") {
      return value.trim().length > 0;
    }

    if (filter.type === "select") {
      return value !== "" && value !== "All";
    }

    return value.trim().length > 0;
  }

  function handleFilterChange(filter: FilterConfig, value: string) {
    if (getFilterValue(filter) === value) {
      return;
    }
    setQuery({
      ...currentQuery,
      filters: buildNextFilters(filter, value),
      page: 1
    });
  }

  function handleQueryVariantChange(value: string | null) {
    const nextVariant = value ?? resolvedDefaultVariantId ?? undefined;
    if ((currentQuery.variant ?? resolvedDefaultVariantId ?? undefined) === nextVariant) {
      return;
    }
    setQuery({
      ...currentQuery,
      variant: nextVariant,
      page: 1
    });
  }

  function handleSortChange(sortFields: SortField[]) {
    if (sortFieldsEqual(currentSort, sortFields)) {
      return;
    }
    setQuery({
      ...currentQuery,
      sort: sortFields,
      page: 1
    });
  }

  function handlePageChange(page: number) {
    if (page === currentPage) {
      return;
    }
    setQuery({
      ...currentQuery,
      page
    });
  }

  function clearSort() {
    handleSortChange([]);
  }

  function getSortOrderValue() {
    return currentSort.map((field) => ({
      key: field.field,
      direction: field.direction
    }));
  }

  function handleSortOrderChange(value: { key: string; direction: string }[]) {
    handleSortChange(value.map((field) => ({
      field: field.key,
      direction: field.direction as "asc" | "desc"
    })));
  }

  function handleEditableReorder(nextItems: EditableListItem[]) {
    reorderController?.updatePending(nextItems);
  }

  function getItemKey(item: T): string {
    return String((item as Record<string, unknown>)[idField]);
  }

  function handleRowActionSelect(event: { row: TableRow<T>; action: TableRowAction }) {
    onRowActionSelect?.(event.row, event.action);
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
      reorderController?.reset();
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

  function isLoadedReorderConfig(
    config: ReorderConfig<T> | undefined
  ): config is LoadedReorderConfig<T> {
    return Boolean(config && config.strategy === "loaded");
  }

  function isCustomReorderConfig(
    config: ReorderConfig<T> | undefined
  ): config is CustomReorderConfig<T> {
    return Boolean(config && config.strategy === "custom");
  }

  async function createReorderSession(): Promise<boolean> {
    if (!reorder?.enabled) return false;
    reorderSessionPending = true;
    reorderHighlightedIds = [];

    if (isCustomReorderConfig(reorder)) {
      reorderController = null;
      loadedReorderItems = [];
      reorderError = null;
      reorderSessionPending = false;
      return true;
    }

    if (isLoadedReorderConfig(reorder)) {
      const token = getAuthConfig()?.getToken() ?? null;
      const result = await reorder.loadItems(fetch, token, currentQuery);
      if (result.error) {
        toastStore.push({ message: result.error, variant: "error" });
        reorderSessionPending = false;
        return false;
      }

      loadedReorderItems = reorder.mapItems
        ? reorder.mapItems(result.items)
        : result.items.map((item) => ({
            ...(item as T & Record<string, unknown>),
            id: String((item as Record<string, unknown>)[idField])
          })) as Array<T & ReorderableItem>;

      reorderController = createReorderController(loadedReorderItems, submitReorder);
      reorderError = null;
      reorderSessionPending = false;
      return true;
    }

    loadedReorderItems = [];
    reorderController = createReorderController(
      items.map((item) => ({
        id: String((item as Record<string, unknown>)[idField]),
        ...item
      })),
      submitReorder
    );
    reorderError = null;
    reorderSessionPending = false;
    return true;
  }

  async function enterReorderMode() {
    if (!reorderAvailable || !reorder?.enabled) return;
    if (selectionMode) {
      setSelectionMode(false);
      batch.clear();
    }
    const ready = await createReorderSession();
    if (!ready) return;
    setReorderMode(true);
  }

  function exitReorderMode() {
    setReorderMode(false);
    reorderError = null;
    reorderHighlightedIds = [];
    reorderController?.reset();
    reorderController = null;
    loadedReorderItems = [];
  }

  async function handleReorderSubmit() {
    if (!reorderController) return;
    reorderError = null;
    try {
      await reorderController.submit();
      if (reorder && "successMessage" in reorder && reorder.successMessage) {
        toastStore.push({ message: reorder.successMessage, variant: "success" });
      }
      exitReorderMode();
      onDataChange?.();
      await pageData.refetch();
    } catch (error) {
      if (onReorderError && reorderController) {
        const result = await onReorderError({
          error,
          controller: reorderController,
          items: isLoadedReorderConfig(reorder)
            ? loadedReorderItems
            : items.map((item) => ({
                id: String((item as Record<string, unknown>)[idField]),
                ...item
              })) as Array<T & ReorderableItem>
        });
        if (typeof result === "string") {
          reorderError = result;
          reorderHighlightedIds = [];
        } else if (result) {
          const recovery = result as ReorderErrorResult;
          reorderError = recovery.message;
          reorderHighlightedIds = recovery.highlightedIds ?? [];
        } else {
          reorderError = error instanceof Error ? error.message : String(error);
          reorderHighlightedIds = [];
        }
      } else {
        reorderError = error instanceof Error ? error.message : String(error);
        reorderHighlightedIds = [];
      }
    }
  }

  function getItemContext(item: T): ItemContext {
    const itemId = String((item as Record<string, unknown>)[idField]);
    return {
      selectionMode,
      reorderMode,
      selected: batch.isSelected(itemId),
      onToggle: (selected: boolean) => batch.toggle(itemId, selected),
      refetch: () => pageData.refetch()
    };
  }

  $effect(() => {
    if (reorderMode) {
      if (!reorderAvailable || !reorder?.enabled) {
        if (reorderController) {
          reorderController.reset();
          reorderController = null;
        }
        reorderError = null;
        reorderHighlightedIds = [];
        loadedReorderItems = [];
        if (externalReorderMode !== undefined) {
          onReorderModeChange?.(false);
        } else {
          internalReorderMode = false;
        }
        return;
      }
      if (!reorderController && !reorderSessionPending) {
        void createReorderSession().then((ready) => {
          if (!ready) {
            setReorderMode(false);
          }
        });
      }
      return;
    }

    if (reorderController) {
      reorderController.reset();
      reorderController = null;
      reorderError = null;
      reorderHighlightedIds = [];
    }
    loadedReorderItems = [];
  });

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      if (selectionMode) {
        setSelectionMode(false);
        batch.clear();
      } else if (reorderMode) {
        exitReorderMode();
      }
    }
  }

  const tableColumns = $derived<TableColumn[]>(columns ?? []);

  function isReorderItemHighlighted(reorderEntryId: string): boolean {
    return reorderHighlightedIds.includes(reorderEntryId);
  }

  // Table rows
  const tableRows = $derived<TableRow<T>[]>(
    items.map((item) => ({
      id: String((item as Record<string, unknown>)[idField]),
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

  function getReorderOriginalItem(reorderEntryId: string): T | undefined {
    if (isLoadedReorderConfig(reorder)) {
      return loadedReorderItems.find((item) => item.id === reorderEntryId);
    }

    return items.find(
      (item) => String((item as Record<string, unknown>)[idField]) === reorderEntryId
    );
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
          onEnter={enterReorderMode}
          onSave={handleReorderSubmit}
          onCancel={exitReorderMode}
        />
      {/if}
      {#if !selectionMode && !reorderMode && onAdd}
        <Button variant="primary" onClick={onAdd}>{addLabel}</Button>
      {/if}
    {/snippet}

    {#snippet filters()}
      {#if queryVariantItems.length > 0 && !reorderMode}
        <div class="underlay-entity-list__query-variants">
          <CardToggleGroup
            items={queryVariantItems}
            value={currentVariantId}
            columns={queryVariantColumns}
            allowDeactivation={Boolean(resolvedDefaultVariantId)}
            ariaLabel={`${title ?? "List"} query variants`}
            onValueChange={handleQueryVariantChange}
          />
        </div>
      {/if}

      {#if effectiveFilterConfigs.length > 0 && !reorderMode}
        <FilterToolbar ariaLabel={`${title} filters`} summaryText="Filters">
          {#snippet summary()}
            <PaginationSummary
              currentPage={currentPage}
              totalPages={totalPages}
              totalItems={totalCount}
              pageSize={currentPageSize}
            />
          {/snippet}

          {#snippet actions()}
            {#if currentSort.length > 0}
              <IconButton
                icon="x"
                variant="ghost"
                size="sm"
                ariaLabel="Clear sort"
                tooltip="Clear sort"
                onClick={clearSort}
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
          
          {#each effectiveFilterConfigs as filter}
            <div
              class="underlay-entity-list__filter-control"
              data-active={isFilterActive(filter)}
            >
              {#if filter.type === "search"}
                <TextInput
                  id={`filter-${filter.id}`}
                  type="search"
                  value={getFilterValue(filter)}
                  ariaLabel={getFilterAriaLabel(filter)}
                  placeholder={getSearchPlaceholder(filter)}
                  onValueChange={(nextValue) => handleFilterChange(filter, nextValue)}
                />
              {:else if filter.type === "select"}
                <Select
                  id={`filter-${filter.id}`}
                  value={getFilterValue(filter)}
                  options={getSelectItems(filter)}
                  ariaLabel={getFilterAriaLabel(filter)}
                  onValueChange={(value) => handleFilterChange(filter, value)}
                />
              {:else if filter.type === "sort" && filter.sortFields}
                <OrderBy
                  fields={filter.sortFields}
                  value={getSortOrderValue()}
                  ariaLabel={getFilterAriaLabel(filter)}
                  showClearButton={false}
                  onChange={handleSortOrderChange}
                  compact
                />
              {/if}
            </div>
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
        {#snippet actions()}
          {#if onAdd}
            <Button variant="primary" onClick={onAdd}>{addLabel}</Button>
          {/if}
        {/snippet}
      </EmptyState>
    {:else if reorderMode && customReorderContent}
      {@render customReorderContent({
        items,
        cancel: exitReorderMode,
        refetch: () => pageData.refetch()
      })}
    {:else if reorderMode && reorderController}
      <EditableList
        items={reorderController.pending}
        embeddedHandle
        dirty={reorderController.isDirty}
        submitting={reorderController.isPending}
        errorMessage={reorderError}
        showWorkflowChrome={false}
        onSubmit={handleReorderSubmit}
        onCancel={exitReorderMode}
        onReorder={handleEditableReorder}
      >
        {#snippet item(reorderEntry)}
          {@const originalItem = getReorderOriginalItem(reorderEntry.id)}
          {#if renderReorderItem && originalItem}
            <div data-reorder-highlighted={isReorderItemHighlighted(reorderEntry.id)}>
              {@render renderReorderItem(originalItem, {
                ...getItemContext(originalItem),
                highlighted: isReorderItemHighlighted(reorderEntry.id)
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
      <ListGrid minItemWidth="26rem" variant={listGridVariant} gap={listGridGap}>
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
        emptyMessage={`No ${title.toLowerCase()} found`}
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
        rowActions={rowActions}
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
  </ListContainer>
{:else}
  <!-- When used inside EntityListPage, don't show ListContainer shell -->
  {#if queryVariantItems.length > 0 && !reorderMode}
    <div class="underlay-entity-list__query-variants">
      <CardToggleGroup
        items={queryVariantItems}
        value={currentVariantId}
        columns={queryVariantColumns}
        allowDeactivation={Boolean(resolvedDefaultVariantId)}
        ariaLabel="Query variants"
        onValueChange={handleQueryVariantChange}
      />
    </div>
  {/if}

  {#if effectiveFilterConfigs.length > 0 && !reorderMode}
    <FilterToolbar ariaLabel="Filters" summaryText="Filters">
      {#snippet summary()}
        <PaginationSummary
          currentPage={currentPage}
          totalPages={totalPages}
          totalItems={totalCount}
          pageSize={currentPageSize}
        />
      {/snippet}

      {#snippet actions()}
        {#if currentSort.length > 0}
          <IconButton
            icon="x"
            variant="ghost"
            size="sm"
            ariaLabel="Clear sort"
            tooltip="Clear sort"
            onClick={clearSort}
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
      
      {#each effectiveFilterConfigs as filter}
        <div
          class="underlay-entity-list__filter-control"
          data-active={isFilterActive(filter)}
        >
          {#if filter.type === "search"}
            <TextInput
              id={`filter-${filter.id}`}
              type="search"
              value={getFilterValue(filter)}
              ariaLabel={getFilterAriaLabel(filter)}
              placeholder={getSearchPlaceholder(filter)}
              onValueChange={(nextValue) => handleFilterChange(filter, nextValue)}
            />
          {:else if filter.type === "select"}
            <Select
              id={`filter-${filter.id}`}
              value={getFilterValue(filter)}
              options={getSelectItems(filter)}
              ariaLabel={getFilterAriaLabel(filter)}
              onValueChange={(value) => handleFilterChange(filter, value)}
            />
          {:else if filter.type === "sort" && filter.sortFields}
            <OrderBy
              fields={filter.sortFields}
              value={getSortOrderValue()}
              ariaLabel={getFilterAriaLabel(filter)}
              showClearButton={false}
              onChange={handleSortOrderChange}
              compact
            />
          {/if}
        </div>
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
  {:else if reorderMode && customReorderContent}
    {@render customReorderContent({
      items,
      cancel: exitReorderMode,
      refetch: () => pageData.refetch()
    })}
  {:else if reorderMode && reorderController}
    <EditableList
      items={reorderController.pending}
      embeddedHandle
      dirty={reorderController.isDirty}
      submitting={reorderController.isPending}
      errorMessage={reorderError}
      showWorkflowChrome={false}
      onSubmit={handleReorderSubmit}
      onCancel={exitReorderMode}
      onReorder={handleEditableReorder}
    >
      {#snippet item(reorderEntry)}
        {@const originalItem = getReorderOriginalItem(reorderEntry.id)}
        {#if renderReorderItem && originalItem}
          <div data-reorder-highlighted={isReorderItemHighlighted(reorderEntry.id)}>
            {@render renderReorderItem(originalItem, {
              ...getItemContext(originalItem),
              highlighted: isReorderItemHighlighted(reorderEntry.id)
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
    <ListGrid minItemWidth="26rem" variant={listGridVariant} gap={listGridGap}>
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
      emptyMessage="No items found"
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
      rowActions={rowActions}
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
    margin-bottom: 0.75rem;
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
