<script lang="ts">
  import type { Component, SvelteComponent, Snippet } from "svelte";
  import PageHeader from "../PageHeader.svelte";
  import FilterBar from "../FilterBar.svelte";
  import ReorderableList from "../ReorderableList.svelte";
  import ListGrid from "../../components/ListGrid.svelte";
  import ListCard from "../../components/ListCard.svelte";
  import PageLoading from "../../components/PageLoading.svelte";
  import FormError from "../../components/FormError.svelte";
  import EmptyState from "../../components/EmptyState.svelte";
  import Pagination from "../../components/Pagination.svelte";
  import BatchActionBar from "../../components/BatchActionBar.svelte";
  import BatchConfirmDialog from "../../components/BatchConfirmDialog.svelte";
  import Button from "../../components/Button.svelte";
  import TextInput from "../../components/TextInput.svelte";
  import type { BatchAction } from "../batch-actions.svelte";
  import type { PageHeaderLevel, BreadcrumbItem } from "../types";
  import type {
    ListFilterField,
    ListReorderConfig,
    ListItemContext,
    FilterBarContext,
    ServerFetcher
  } from "./autonomous-list-types";
  import { createAutonomousListState } from "./autonomous-list-context.svelte";

  type IconComponent =
    | Component<{ size?: number }>
    | (new (...args: any[]) => SvelteComponent);

  interface Props<T> {
    title: string;
    section?: string;
    level?: PageHeaderLevel;
    breadcrumbs?: BreadcrumbItem[];
    variant?: "page" | "tab";
    fetcher?: (fetchFn: typeof fetch, token: string, filters: Record<string, unknown>) => Promise<T[]>;
    serverFetcher?: ServerFetcher<T>;
    idField?: string;
    pageSize?: number;
    persistKey?: string;
    filters?: ListFilterField[];
    filterBar?: Snippet<[FilterBarContext]>;
    batchActions?: BatchAction<string>[];
    reorderable?: ListReorderConfig;
    addHref?: string;
    addLabel?: string;
    headerActions?: Snippet;
    emptyMessage?: string;
    emptyIcon?: IconComponent;
    sourceContext?: Record<string, unknown>;
    item?: Snippet<[T, ListItemContext]>;
    reorderItem?: Snippet<[T]>;
    onDataChange?: () => void;
    class?: string;
  }

  let {
    title,
    section = undefined,
    level = undefined,
    breadcrumbs = undefined,
    variant = "page",
    fetcher = undefined,
    serverFetcher = undefined,
    idField = "id",
    pageSize = undefined,
    persistKey = undefined,
    filters: filterFields = [],
    filterBar: filterBarSnippet = undefined,
    batchActions = [],
    reorderable = undefined,
    addHref = undefined,
    addLabel = "Add",
    headerActions: headerActionsSnippet = undefined,
    emptyMessage = "No items found",
    emptyIcon = undefined,
    item: itemSnippet,
    reorderItem: reorderItemSnippet,
    onDataChange = undefined,
    class: className = ""
  }: Props<any> = $props();

  // Derive header level from variant if not explicitly set
  const effectiveLevel = $derived(level ?? (variant === "page" ? 1 : 3));

  const listState = createAutonomousListState({
    fetcher,
    serverFetcher,
    idField,
    batchActions,
    reorderable,
    pageSize,
    persistKey,
    onDataChange
  });

  const { batch } = listState;

  // Active filter state for FilterBar display
  let activeFilterValues = $state<Record<string, string>>({});

  function handleFilterChange(key: string, value: string | undefined) {
    if (value) {
      activeFilterValues = { ...activeFilterValues, [key]: value };
    } else {
      const { [key]: _, ...rest } = activeFilterValues;
      activeFilterValues = rest;
    }

    if (serverFetcher) {
      // For server mode, update the filter state on the pagination controller
      // and reset to page 1
      const pag = listState.pagination as any;
      if (pag?._setFilters) {
        pag._setFilters(activeFilterValues);
      }
      listState.resetPagination();
    } else {
      // For client mode, pass through to list controller
      // We need to access list controller's setFilter indirectly
      // The list controller is inside the state — we pass filter values via
      // the fetcher's closure. Trigger a refresh.
      listState.resetPagination();
    }
  }

  function clearFilter(key: string) {
    const { [key]: _, ...rest } = activeFilterValues;
    activeFilterValues = rest;
    handleFilterChange(key, undefined);
  }

  function clearAllFilters() {
    activeFilterValues = {};
    if (serverFetcher) {
      const pag = listState.pagination as any;
      if (pag?._setFilters) {
        pag._setFilters({});
      }
    }
    listState.resetPagination();
  }

  // FilterBar context for custom snippets
  const filterBarContext = $derived<FilterBarContext>({
    filters: activeFilterValues,
    setFilter: handleFilterChange,
    clearFilter,
    clearAll: clearAllFilters,
    refresh: () => listState.refresh()
  });

  // Build active filters for FilterBar display
  const activeFilters = $derived(
    Object.entries(activeFilterValues).map(([key, value]) => {
      const field = filterFields.find((f) => f.key === key);
      if (!field) return null;
      let displayValue = value;
      if (field.type === "select" && field.options) {
        const option = field.options.find((o) => o.value === value);
        if (option) displayValue = option.label;
      }
      return { id: key, label: field.label, value: displayValue };
    }).filter(Boolean) as Array<{ id: string; label: string; value: string }>
  );

  function getItemId(item: any): string {
    return String(item[idField] ?? "");
  }
</script>

<svelte:window onkeydown={listState.handleKeydown} />

<div class="underlay-autonomous-list {className}">
  <PageHeader
    {title}
    section={variant === "page" ? section : undefined}
    level={effectiveLevel}
    {breadcrumbs}
    count={listState.total ?? undefined}
  >
    {#snippet actions()}
      <div class="underlay-autonomous-list__actions">
        {#if batchActions.length > 0}
          <Button
            variant={listState.selectionMode ? "primary" : "subtle"}
            size="icon-sm"
            onclick={listState.toggleSelectionMode}
            aria-label={listState.selectionMode ? "Exit selection" : "Select items"}
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <rect x="3" y="3" width="18" height="18" rx="2" /><path d="m9 12 2 2 4-4" />
            </svg>
          </Button>
        {/if}

        {#if listState.canReorder}
          <Button
            variant={listState.reorderMode ? "primary" : "subtle"}
            size="icon-sm"
            onclick={() => listState.reorderMode ? listState.exitReorderMode() : listState.enterReorderMode()}
            aria-label={listState.reorderMode ? "Exit reorder" : "Reorder items"}
          >
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="m3 16 4 4 4-4" /><path d="M7 20V4" /><path d="m21 8-4-4-4 4" /><path d="M17 4v16" />
            </svg>
          </Button>
        {/if}

        {#if headerActionsSnippet}
          {@render headerActionsSnippet()}
        {/if}

        {#if addHref}
          <Button variant="primary" size="sm" onclick={() => { window.location.href = addHref! }}>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M5 12h14" /><path d="M12 5v14" />
            </svg>
            {addLabel}
          </Button>
        {/if}
      </div>
    {/snippet}
  </PageHeader>

  {#if filterBarSnippet}
    {@render filterBarSnippet(filterBarContext)}
  {:else if filterFields.length > 0}
    <FilterBar
      {activeFilters}
      onClearFilter={(id) => clearFilter(id)}
      onClearAllFilters={clearAllFilters}
      onRefresh={() => listState.refresh()}
    >
      {#each filterFields as field (field.key)}
        {#if field.type === "text"}
          <TextInput
            placeholder={field.placeholder ?? "Search..."}
            value={activeFilterValues[field.key] ?? ""}
            oninput={(value) => handleFilterChange(field.key, value || undefined)}
            debounce={field.debounce ?? 400}
          />
        {:else if field.type === "select" && field.options}
          <select
            class="underlay-autonomous-list__select"
            value={activeFilterValues[field.key] ?? ""}
            onchange={(e) => handleFilterChange(field.key, e.currentTarget.value || undefined)}
          >
            {#if field.includeAll !== false}
              <option value="">{field.allLabel ?? `All ${field.label.toLowerCase()}`}</option>
            {/if}
            {#each field.options as option (option.value)}
              <option value={option.value}>{option.label}</option>
            {/each}
          </select>
        {/if}
      {/each}
    </FilterBar>
  {/if}

  {#if listState.loading}
    <PageLoading message={`Loading ${title.toLowerCase()}...`} />
  {:else if listState.error}
    <FormError message={listState.error} />
  {:else if listState.reorderMode && listState.reorder}
    <ReorderableList
      controller={listState.reorder}
      oncancel={listState.exitReorderMode}
      onsuccess={listState.handleReorderSuccess}
    >
      {#snippet item(reorderableItem)}
        {#if reorderItemSnippet}
          {@const sourceItem = listState.items.find((i) => getItemId(i) === reorderableItem.id)}
          {#if sourceItem}
            {@render reorderItemSnippet(sourceItem)}
          {:else}
            <ListCard title={reorderableItem.label} variant="compact" />
          {/if}
        {:else}
          <ListCard title={reorderableItem.label} variant="compact" />
        {/if}
      {/snippet}
    </ReorderableList>
  {:else if listState.items.length === 0}
    <EmptyState
      icon={emptyIcon}
      title={emptyMessage}
    />
  {:else if itemSnippet}
    <ListGrid>
      {#each listState.items as listItem (getItemId(listItem))}
        {@const itemId = getItemId(listItem)}
        {@render itemSnippet(listItem, {
          selected: batch.isSelected(itemId),
          onSelectionChange: (selected) => batch.toggle(itemId, selected),
          selectionMode: listState.selectionMode
        })}
      {/each}
    </ListGrid>

    {#if listState.pagination}
      <Pagination
        controller={listState.pagination}
        variant="simple"
        showLimitSelector
        scrollTarget=".underlay-list-grid"
      />
    {/if}
  {/if}

  {#if batch.hasSelection}
    <BatchActionBar
      selectedCount={batch.count}
      totalCount={listState.allItemIds.length}
      loading={batch.executing}
      onClearSelection={() => batch.clear()}
      onSelectAll={() => batch.selectAll(listState.allItemIds)}
      registeredActions={batch.availableActions}
      onAction={(actionId) => batch.requestAction(actionId)}
    />
  {/if}

  {#if batch.pendingAction}
    <BatchConfirmDialog
      open={true}
      title={batch.pendingAction.confirm?.title ?? "Confirm action"}
      description={batch.getConfirmDescription()}
      confirmLabel={batch.pendingAction.confirm?.confirmLabel ?? "Confirm"}
      onConfirm={async () => { await batch.confirmPendingAction(); }}
      onCancel={() => batch.cancelPendingAction()}
    />
  {/if}
</div>

<style>
  :global(.underlay-autonomous-list) {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  :global(.underlay-autonomous-list__actions) {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  :global(.underlay-autonomous-list__select) {
    appearance: none;
    background-color: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, inherit);
    border: 1px solid var(--underlay-color-border, rgba(148, 163, 184, 0.12));
    border-radius: var(--underlay-radius-md, 0.5rem);
    padding: 0.375rem 1.75rem 0.375rem 0.625rem;
    font-size: var(--underlay-font-size-sm, 0.875rem);
    cursor: pointer;
    background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'%3e%3cpath fill='none' stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='m2 5 6 6 6-6'/%3e%3c/svg%3e");
    background-repeat: no-repeat;
    background-position: right 0.5rem center;
    background-size: 0.75rem;
  }

  :global(.underlay-autonomous-list__select:focus-visible) {
    outline: 2px solid var(--underlay-color-primary, #2563eb);
    outline-offset: 2px;
  }
</style>
