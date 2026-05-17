<script lang="ts" generics="T">
  import {
    Callout,
    Dialog,
    IconButton,
    ListGrid,
    PageLoading,
    Pagination
  } from "@poodle/svelte";

  import { DEFAULT_PAGE_SIZE } from "../patterns/pagination-types";
  import { useAuthenticatedData } from "../runtime/auth";
  import EntityActionsMenu from "./EntityActionsMenu.svelte";
  import EntityDetailModule from "./EntityDetailModule.svelte";
  import EntityList from "./EntityList.svelte";
  import EntityReorderControls from "./EntityReorderControls.svelte";

  import type { QueryParams } from "../client/query";
  import type {
    BatchActionConfig,
    EntityListDataLoader,
    EntityListSharedProps,
    InlineListDialogConfig,
    InlineListItemActionConfig,
    InlineListItemDeleteConfig,
    PagedListResult,
    ReorderActionState,
    ReorderConfig,
    TemplateSurface
  } from "./template.types";

  interface InlineListItemContext<TItem> {
    refetch: () => Promise<void>;
    openActions: boolean;
    selectionMode: boolean;
    reorderMode: boolean;
    selected: boolean;
    onToggle: (selected: boolean) => void;
  }

  interface Props {
    title: string;
    items?: T[];
    item: TemplateSurface;
    actions?: TemplateSurface;
    emptyMessage?: string | null;
    span?: "half" | "full";
    listGridVariant?: "default" | "compact";
    listGridGap?: number | string | null;
    dataLoader?: EntityListDataLoader<T>;
    query?: QueryParams;
    onQueryChange?: (query: QueryParams) => void;
    pageSize?: number;
    onDataChange?: () => void;
    onAdd?: () => void;
    addLabel?: string;
    addDialog?: InlineListDialogConfig;
    idField?: string;
    itemActions?: ((item: T) => InlineListItemActionConfig<T>[]) | undefined;
    itemDelete?: InlineListItemDeleteConfig<T> | undefined;
    renderReorderItem?: TemplateSurface;
    batchActions?: BatchActionConfig[];
    reorder?: ReorderConfig<T>;
    onReorderError?: EntityListSharedProps<T>["onReorderError"];
  }

  let {
    title,
    items = [],
    item,
    actions,
    emptyMessage = "No items yet.",
    span = "half",
    listGridVariant = "compact",
    listGridGap = "0.5rem",
    dataLoader,
    query,
    onQueryChange,
    pageSize = 5,
    onDataChange,
    onAdd,
    addLabel = "Add",
    addDialog,
    idField = "id",
    itemActions,
    itemDelete,
    renderReorderItem,
    batchActions = [],
    reorder,
    onReorderError
  }: Props = $props();

  let internalQuery = $state<QueryParams>({
    page: 1,
    limit: DEFAULT_PAGE_SIZE
  });
  let addDialogOpen = $state(false);
  let refreshVersion = $state(0);
  let totalCount = $state(0);
  let visibleItemCount = $state(0);
  let selectionMode = $state(false);
  let reorderMode = $state(false);
  let selectedIds = $state<string[]>([]);
  let reorderAvailable = $state(false);
  let reorderActionState = $state<ReorderActionState | null>(null);
  let previousQueryKey = $state<string | null>(null);

  $effect(() => {
    if (query || internalQuery.limit !== DEFAULT_PAGE_SIZE || pageSize === DEFAULT_PAGE_SIZE) {
      return;
    }

    internalQuery = {
      ...internalQuery,
      limit: pageSize
    };
  });

  const usesLoader = $derived(Boolean(dataLoader));
  const effectiveQuery = $derived.by(() => {
    const source = query ?? internalQuery;
    return {
      ...source,
      page: Math.max(1, source.page ?? 1),
      limit: Math.max(1, source.limit ?? pageSize)
    };
  });
  const queryKey = $derived.by(() =>
    JSON.stringify({
      query: effectiveQuery,
      title,
      refreshVersion
    })
  );
  const getDataLoader = () => dataLoader;
  const getEffectiveQuery = () => effectiveQuery;

  const loadedData = useAuthenticatedData<PagedListResult<T>>(
    async (fetch, token) => {
      const loader = getDataLoader();
      if (!loader) {
        return {
          data: [],
          total: 0
        };
      }

      return await loader(fetch, token, getEffectiveQuery());
    },
    {
      defaultValue: {
        data: [],
        total: 0
      }
    }
  );

  $effect(() => {
    const currentKey = queryKey;
    if (previousQueryKey === null) {
      previousQueryKey = currentKey;
      return;
    }

    if (previousQueryKey !== currentKey) {
      previousQueryKey = currentKey;
      void loadedData.refetch();
    }
  });

  const visibleItems = $derived(usesLoader ? (loadedData.data?.data ?? []) : items);
  const derivedTotalCount = $derived.by(() => {
    if (!usesLoader) {
      return items.length;
    }

    const total = loadedData?.data?.total;
    if (typeof total === "number" && Number.isFinite(total) && total >= 0) {
      return total;
    }

    return visibleItems.length;
  });
  const totalPages = $derived(
    Math.max(1, Math.ceil(derivedTotalCount / Math.max(1, effectiveQuery.limit ?? DEFAULT_PAGE_SIZE)))
  );
  const currentPage = $derived(effectiveQuery.page ?? 1);
  const showPagination = $derived(usesLoader && totalPages > 1);
  const menuNeeded = $derived(Boolean(itemActions || itemDelete));
  const showManagedList = $derived(usesLoader);

  $effect(() => {
    if (!usesLoader) {
      totalCount = items.length;
      visibleItemCount = items.length;
    }
  });

  function setQuery(nextQuery: QueryParams): void {
    if (onQueryChange) {
      onQueryChange(nextQuery);
      return;
    }

    internalQuery = nextQuery;
  }

  function handlePageChange(page: number): void {
    if (page === currentPage) {
      return;
    }

    setQuery({
      ...effectiveQuery,
      page
    });
  }

  function openAddDialog(): void {
    addDialogOpen = true;
  }

  function closeAddDialog(): void {
    addDialogOpen = false;
  }

  async function refetch(): Promise<void> {
    if (showManagedList) {
      refreshVersion += 1;
      return;
    }

    if (loadedData) {
      await loadedData.refetch();
    }
  }

  async function handleAddSuccess(): Promise<void> {
    onDataChange?.();
    await refetch();
    closeAddDialog();
  }

  function getItemContext(): InlineListItemContext<T> {
    return {
      refetch,
      openActions: menuNeeded,
      selectionMode: false,
      reorderMode: false,
      selected: false,
      onToggle: () => {}
    };
  }

  function toggleSelectionMode(): void {
    if (reorderMode) {
      reorderMode = false;
    }
    selectionMode = !selectionMode;
  }

  function toggleReorderMode(): void {
    if (selectionMode) {
      selectionMode = false;
    }
    if (reorderActionState) {
      if (reorderMode) {
        reorderActionState.cancel();
      } else {
        void reorderActionState.enter();
      }
      return;
    }
    reorderMode = !reorderMode;
  }
</script>

{#snippet managedItem(entry: T, listContext: {
  selectionMode: boolean;
  reorderMode: boolean;
  selected: boolean;
  onToggle: (selected: boolean) => void;
})}
  {#if itemActions || itemDelete}
    <div class="underlay-inline-list-module__managed-item">
      <div class="underlay-inline-list-module__item-body">
        {@render item(entry, {
          ...listContext,
          refetch,
          openActions: menuNeeded
        })}
      </div>

      {#if !listContext.selectionMode && !listContext.reorderMode}
        <div class="underlay-inline-list-module__item-actions">
          <EntityActionsMenu
            triggerAriaLabel={`${title} item actions`}
            triggerTooltip="Actions"
            customActions={(itemActions?.(entry) ?? []).map((action) => ({
              label: action.label,
              disabled: action.disabled,
              destructive: action.destructive,
              separator: action.separator,
              onSelect: async () => {
                await action.handler(entry);
                onDataChange?.();
                await refetch();
              }
            }))}
            deleteConfig={itemDelete
              ? {
                  title: itemDelete.title,
                  description: itemDelete.description,
                  confirmLabel: itemDelete.confirmLabel,
                  entityLabel: itemDelete.entityLabel?.(entry) ?? null,
                  execute: async () => {
                    await itemDelete.handler(entry);
                  }
                }
              : undefined}
            onDeleteSuccess={async () => {
              onDataChange?.();
              await refetch();
            }}
          >
            {#snippet trigger()}
              <IconButton
                type="button"
                icon="ellipsis"
                variant="ghost"
                size="sm"
                ariaLabel={`${title} item actions`}
                tooltip="Actions"
              />
            {/snippet}
          </EntityActionsMenu>
        </div>
      {/if}
    </div>
  {:else}
    {@render item(entry, {
      ...listContext,
      refetch,
      openActions: menuNeeded
    })}
  {/if}
{/snippet}

<EntityDetailModule {span}>
  <section class="underlay-inline-list-module" aria-label={title}>
    <div class="underlay-inline-list-module__header">
      <div class="underlay-inline-list-module__heading">
        <h4 class="underlay-inline-list-module__title">{title}</h4>
      </div>

      <div class="underlay-inline-list-module__header-actions">
        {#if actions}
          {@render actions()}
        {/if}

        {#if (visibleItemCount > 0 || selectionMode) && batchActions.length > 0}
          <IconButton
            type="button"
            variant="secondary"
            sizeRole="chrome"
            size="sm"
            icon={selectionMode ? "x" : "check-square"}
            ariaLabel={selectionMode ? "Cancel selection" : "Select items"}
            tooltip={selectionMode ? "Cancel Selection" : "Select Items"}
            disabled={reorderMode}
            onClick={toggleSelectionMode}
          />
        {/if}

        {#if reorder}
          <EntityReorderControls
            active={reorderMode}
            available={reorderAvailable}
            dirty={reorderActionState?.dirty ?? false}
            saving={reorderActionState?.saving ?? false}
            disabled={selectionMode}
            sizeRole="chrome"
            onEnter={async () => {
              if (selectionMode) selectionMode = false;
              if (reorderActionState) {
                await reorderActionState.enter();
              } else {
                toggleReorderMode();
              }
            }}
            onSave={reorderActionState?.save}
            onCancel={() => {
              if (reorderActionState) {
                reorderActionState.cancel();
              } else {
                toggleReorderMode();
              }
            }}
          />
        {/if}

        {#if addDialog}
          <IconButton
            type="button"
            variant="primary"
            sizeRole="chrome"
            size="sm"
            icon="plus"
            ariaLabel={addLabel}
            tooltip={addLabel}
            disabled={selectionMode || reorderMode}
            onClick={openAddDialog}
          />
        {:else if onAdd}
          <IconButton
            type="button"
            variant="primary"
            sizeRole="chrome"
            size="sm"
            icon="plus"
            ariaLabel={addLabel}
            tooltip={addLabel}
            disabled={selectionMode || reorderMode}
            onClick={onAdd}
          />
        {/if}
      </div>
    </div>

    {#if showManagedList}
      <EntityList
        dataLoader={dataLoader!}
        reloadKey={queryKey}
        {idField}
        presentation="cards"
        renderItem={managedItem}
        {listGridVariant}
        {listGridGap}
        {renderReorderItem}
        {batchActions}
        {reorder}
        query={effectiveQuery}
        onQueryChange={setQuery}
        {onReorderError}
        selectionMode={selectionMode}
        reorderMode={reorderMode}
        onSelectionModeChange={(enabled) => {
          selectionMode = enabled;
        }}
        onReorderModeChange={(enabled) => {
          reorderMode = enabled;
        }}
        onVisibleCountChange={(count) => {
          visibleItemCount = count;
        }}
        onTotalCountChange={(count) => {
          totalCount = count;
        }}
        onSelectedIdsChange={(ids) => {
          selectedIds = ids;
        }}
        onReorderAvailabilityChange={(enabled) => {
          reorderAvailable = enabled;
        }}
        onReorderActionStateChange={(state) => {
          reorderActionState = state;
        }}
      />
    {:else if loadedData?.loading && visibleItems.length === 0}
      <PageLoading presentation="inline" message={`Loading ${title.toLowerCase()}...`} />
    {:else if loadedData?.error}
      <Callout tone="danger" message={loadedData.error} announceMode="polite" />
    {:else if visibleItems.length === 0}
      {#if emptyMessage}
        <p class="underlay-inline-list-module__empty">{emptyMessage}</p>
      {/if}
    {:else}
      <ListGrid minItemWidth="26rem" variant={listGridVariant} gap={listGridGap}>
        {#each visibleItems as entry}
          {@render item(entry, getItemContext())}
        {/each}
      </ListGrid>
    {/if}

    {#if !showManagedList && showPagination}
      <div class="underlay-inline-list-module__pagination">
        <Pagination
          currentPage={currentPage}
          totalPages={totalPages}
          showInfo={false}
          size="sm"
          compact
          variant="simple"
          ariaLabel={`${title} pagination`}
          onPageChange={handlePageChange}
        />
      </div>
    {/if}
  </section>
</EntityDetailModule>

{#if addDialog && addDialogOpen}
  <Dialog
    open={true}
    title={addDialog.title}
    description={addDialog.description ?? null}
    width={addDialog.width ?? "md"}
    onRequestClose={closeAddDialog}
  >
    {@render addDialog.content({
      close: closeAddDialog,
      refetch: handleAddSuccess
    })}
  </Dialog>
{/if}

<style>
  .underlay-inline-list-module {
    display: grid;
    gap: var(--poodle-space-stack-sm);
  }

  .underlay-inline-list-module__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
  }

  .underlay-inline-list-module__heading {
    display: flex;
    align-items: center;
    min-width: 0;
  }

  .underlay-inline-list-module__title {
    margin: 0;
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--poodle-color-text-secondary);
  }

  .underlay-inline-list-module__header-actions {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 0.375rem;
    flex-wrap: wrap;
  }

  .underlay-inline-list-module__item-body {
    flex: 1;
    min-width: 0;
  }

  .underlay-inline-list-module__item-actions {
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }

  .underlay-inline-list-module__managed-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
  }

  .underlay-inline-list-module__pagination {
    display: flex;
    justify-content: flex-end;
    padding-top: 0.25rem;
  }

  .underlay-inline-list-module__empty {
    margin: 0;
    font-size: var(--poodle-typography-body-size);
    font-style: italic;
    color: var(--poodle-color-text-secondary);
  }
</style>
