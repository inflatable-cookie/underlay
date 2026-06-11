<script lang="ts" generics="T extends SelectableRelation">
  import type { Snippet } from "svelte";
  import {
    Dialog,
    Icon,
    IconButton,
    Popover,
    RelationPicker,
  } from "@poodle/svelte";
  import type {
    BrowseState,
    DrillDownConfig as PoodleDrillDownConfig,
    PickerFilterConfig,
    PickerItem,
  } from "@poodle/svelte";

  import { createRelationSelectorContext } from "./context.svelte.js";
  import type { DrillDownContext } from "./drilldown-types.js";
  import type {
    FilterConfig,
    RelationSelectorProps,
    SelectableRelation,
  } from "./types.js";

  type Props = RelationSelectorProps<T> & {
    class?: string;
    width?: "full" | "auto";
    triggerVariant?: "field" | "icon";
    onChange?: (value: string | null) => void;
    onChangeMulti?: (values: string[]) => void;
  };

  let {
    value = $bindable(null),
    onchange,
    onChange,
    initialSelection = undefined,
    values = $bindable([]),
    onchangeMulti,
    onChangeMulti,
    initialSelections = undefined,
    mode = "single",
    search,
    suggestions,
    selectionHistory,
    filters,
    drillDown,
    label,
    placeholder,
    searchPlaceholder,
    emptyMessage,
    suggestionsLabel,
    disabled = false,
    required = false,
    error,
    allowCreate = false,
    createLabel,
    onCreate,
    renderItem,
    renderTrigger,
    createForm,
    class: className = "",
    width = "full",
    triggerVariant = "field",
  }: Props = $props();

  const contextProps: RelationSelectorProps<T> = {
    get value() {
      return value;
    },
    onchange: (nextValue) => {
      value = nextValue;
      onchange?.(nextValue);
      onChange?.(nextValue);
    },
    get initialSelection() {
      return initialSelection;
    },
    get values() {
      return values;
    },
    onchangeMulti: (nextValues) => {
      values = nextValues;
      onchangeMulti?.(nextValues);
      onChangeMulti?.(nextValues);
    },
    get initialSelections() {
      return initialSelections;
    },
    get mode() {
      return mode;
    },
    get search() {
      return search;
    },
    get suggestions() {
      return drillDown ? undefined : suggestions;
    },
    get selectionHistory() {
      return selectionHistory;
    },
    get filters() {
      return filters;
    },
    get drillDown() {
      return undefined;
    },
    get label() {
      return label;
    },
    get placeholder() {
      return placeholder;
    },
    get searchPlaceholder() {
      return searchPlaceholder;
    },
    get emptyMessage() {
      return emptyMessage;
    },
    get suggestionsLabel() {
      return suggestionsLabel;
    },
    get disabled() {
      return disabled;
    },
    get required() {
      return required;
    },
    get error() {
      return error;
    },
    get allowCreate() {
      return allowCreate;
    },
    get createLabel() {
      return createLabel;
    },
    get onCreate() {
      return onCreate;
    },
    get renderItem() {
      return renderItem;
    },
    get renderTrigger() {
      return renderTrigger;
    },
    get createForm() {
      return createForm;
    },
  };

  const ctx = createRelationSelectorContext(contextProps);

  let searchTimer: ReturnType<typeof setTimeout> | null = null;
  let drillContext = $state<DrillDownContext>({});
  let drillFinalItems = $state<T[]>([]);

  const isMultiSelect = $derived(mode === "multi");
  const selectedIds = $derived(isMultiSelect ? values ?? [] : value ? [value] : []);
  const selectedItems = $derived(isMultiSelect ? ctx.selectedItems : ctx.selectedItem ? [ctx.selectedItem] : []);
  const displayLabel = $derived.by(() => {
    if (isMultiSelect) {
      if (ctx.selectedItems.length === 0) return null;
      if (ctx.selectedItems.length === 1) return ctx.selectedItems[0]?.label ?? null;
      return `${ctx.selectedItems.length} selected`;
    }

    return ctx.selectedItem?.label ?? null;
  });
  const hasSelection = $derived(selectedIds.length > 0);
  const showClearButton = $derived(!required && hasSelection && !disabled && triggerVariant === "field");
  const showInlineCreate = $derived(allowCreate && !!createForm);

  const effectiveFilters = $derived.by(() => {
    if (drillDown && Object.keys(drillContext).length >= drillDown.levels.length) {
      return drillDown.finalLevelFilters?.(drillContext) ?? filters;
    }

    return filters;
  });

  const pickerFilters = $derived(toPickerFilters(effectiveFilters));
  const pickerItems = $derived(toPickerItems(getDisplayItems()));
  const pickerSelectedItems = $derived(toPickerItems(selectedItems));
  const pickerDrillDown = $derived(toPickerDrillDown());
  const browseState = $derived(resolveBrowseState());
  const stateTitle = $derived(resolveStateTitle());
  const stateMessage = $derived(resolveStateMessage());
  const pickerRenderItem = $derived(
    renderItem
      ? (poodleRenderItem as unknown as Snippet<[PickerItem, boolean]>)
      : undefined,
  );

  $effect(() => {
    if (!ctx.state.popoverOpen || drillDown) {
      return;
    }

    const query = ctx.state.searchQuery;
    JSON.stringify(ctx.state.activeFilters);

    if (searchTimer) {
      clearTimeout(searchTimer);
    }

    searchTimer = setTimeout(() => {
      if (query.trim()) {
        void ctx.performSearch(query);
      } else {
        void ctx.loadSuggestions();
      }
    }, 250);

    return () => {
      if (searchTimer) {
        clearTimeout(searchTimer);
        searchTimer = null;
      }
    };
  });

  function toPickerItem(item: SelectableRelation): PickerItem {
    return {
      id: item.id,
      label: item.label,
      description: item.description,
      meta: typeof item.metadata?.meta === "string" ? item.metadata.meta : null,
      disabled: item.disabled,
    };
  }

  function toPickerItems(items: SelectableRelation[] | null | undefined): PickerItem[] {
    return (items ?? []).map(toPickerItem);
  }

  function toPickerFilters(configs: FilterConfig[] | undefined): PickerFilterConfig[] {
    return (configs ?? []).map((filter) => ({
      key: filter.key,
      label: filter.label,
      options: filter.options,
      includeAll: filter.includeAll,
      allLabel: filter.allLabel,
    }));
  }

  function getDisplayItems(): T[] {
    if (drillDown) {
      return drillFinalItems;
    }

    if (ctx.state.searchQuery.trim()) {
      return ctx.state.searchResults;
    }

    return ctx.state.suggestionItems;
  }

  function resolveBrowseState(): BrowseState {
    if (drillDown) {
      return "ready";
    }

    if (ctx.state.isSearching || ctx.state.isSuggestionsLoading) {
      return "loading";
    }

    if (ctx.state.searchError) {
      return "error";
    }

    if (ctx.state.searchQuery.trim() && ctx.state.searchResults.length === 0) {
      return "no-results";
    }

    if (!ctx.state.searchQuery.trim() && ctx.state.suggestionItems.length === 0) {
      return "empty";
    }

    return "ready";
  }

  function resolveStateTitle(): string | null {
    if (ctx.state.searchError) return "Relation search failed";
    if (browseState === "no-results") return emptyMessage ?? "No results found";
    if (browseState === "empty") return suggestionsLabel ? `No ${suggestionsLabel.toLowerCase()}` : "No suggestions";
    return null;
  }

  function resolveStateMessage(): string | null {
    if (ctx.state.searchError) return ctx.state.searchError;
    if (browseState === "no-results") return "Try a different search or clear the filters.";
    if (browseState === "empty") return "Start typing to search available records.";
    return null;
  }

  function handleOpenChange(open: boolean): void {
    if (open) {
      ctx.openPopover();
      return;
    }

    ctx.closePopover();
  }

  function handleSelectionChange(nextIds: string[]): void {
    if (isMultiSelect) {
      const previousIds = values ?? [];
      const addedId = nextIds.find((id) => !previousIds.includes(id));
      if (addedId) {
        const addedItem = getDisplayItems().find((item) => item.id === addedId);
        if (addedItem) {
          ctx.selectItem(addedItem);
          return;
        }
      }

      values = nextIds;
      onchangeMulti?.(nextIds);
      onChangeMulti?.(nextIds);
      return;
    }

    const nextId = nextIds[0] ?? null;
    const nextItem = nextId ? getDisplayItems().find((item) => item.id === nextId) : null;
    if (nextItem) {
      ctx.selectItem(nextItem);
      return;
    }

    value = nextId;
    onchange?.(nextId);
    onChange?.(nextId);
    ctx.closePopover();
  }

  function handleClearClick(event?: MouseEvent): void {
    event?.stopPropagation();
    if (disabled) return;
    ctx.clearSelection();
  }

  function handleCreateClick(event?: MouseEvent): void {
    event?.stopPropagation();
    if (disabled || !createForm) return;
    ctx.switchToModal();
  }

  function handleCreateSuccess(item: T): void {
    ctx.handleCreateSuccess(item);
  }

  function toPickerDrillDown(): PoodleDrillDownConfig | null {
    if (!drillDown) {
      return null;
    }

    const filterFingerprint = JSON.stringify(ctx.state.activeFilters);

    return {
      levels: drillDown.levels.map((level) => ({
        key: level.key,
        label: level.label,
        searchPlaceholder: level.searchPlaceholder,
        items: async (query: string, context: DrillDownContext) => {
          if (!query.trim() && level.suggestions) {
            return toPickerItems(await level.suggestions(context));
          }

          const result = await level.search(query, context);
          return toPickerItems(result.items);
        },
      })),
      finalItems: async (query: string, context: DrillDownContext) => {
        filterFingerprint;

        const result = await search(query, {
          limit: 20,
          offset: 0,
          filters: {
            ...context,
            ...ctx.state.activeFilters,
          },
        });

        drillFinalItems = result.items;
        return toPickerItems(result.items);
      },
    };
  }

  function handleDrillContext(nextContext: DrillDownContext): void {
    drillContext = nextContext;
  }
</script>

{#snippet poodleRenderItem(item: PickerItem, selected: boolean)}
  {#if renderItem}
    {@render renderItem(item as T, selected)}
  {/if}
{/snippet}

<div
  class={`underlay-relation-selector ${className}`}
  class:underlay-relation-selector--auto={width === "auto"}
  class:underlay-relation-selector--disabled={disabled}
  class:underlay-relation-selector--error={!!error}
>
  <div class="underlay-relation-selector__row">
    <Popover
      open={ctx.state.popoverOpen}
      block={width !== "auto"}
      {disabled}
      placement="bottom-start"
      surfaceWidth="content"
      surfaceMinWidth="min(28rem, calc(100vw - 2rem))"
      surfaceMaxWidth="min(32rem, calc(100vw - 2rem))"
      initialFocus="first-focusable"
      ariaLabel={label}
      onOpenChange={handleOpenChange}
    >
      {#snippet trigger()}
        <span
          class="underlay-relation-selector__trigger"
          class:underlay-relation-selector__trigger--icon={triggerVariant === "icon"}
          aria-disabled={disabled}
        >
          {#if renderTrigger}
            {@render renderTrigger(isMultiSelect ? ctx.selectedItems : ctx.selectedItem, ctx.openPopover)}
          {:else if triggerVariant === "icon"}
            <span class="underlay-relation-selector__icon-trigger" aria-hidden="true">
              <Icon name="plus" />
            </span>
          {:else}
            <span
              class="underlay-relation-selector__field"
              class:underlay-relation-selector__field--placeholder={!hasSelection}
              class:underlay-relation-selector__field--attached={showInlineCreate && !showClearButton}
            >
              <span class="underlay-relation-selector__label">
                {displayLabel ?? placeholder ?? "Select..."}
              </span>
              <span class="underlay-relation-selector__chevron" aria-hidden="true">
                <Icon name="chevron-down" />
              </span>
            </span>
          {/if}
        </span>
      {/snippet}

      <RelationPicker
        title={label}
        description={null}
        items={pickerItems}
        selectedItems={pickerSelectedItems}
        selectedIds={selectedIds}
        query={ctx.state.searchQuery}
        selectionMode={isMultiSelect ? "multiple" : "single"}
        variant="popover"
        state={browseState}
        searchPlaceholder={searchPlaceholder ?? "Search..."}
        filters={pickerFilters}
        filterValues={ctx.state.activeFilters}
        stateTitle={stateTitle}
        stateMessage={stateMessage}
        showFooter={isMultiSelect}
        showSelectionSummary={isMultiSelect}
        footerNote={null}
        confirmLabel={`Done (${selectedIds.length})`}
        cancelLabel="Cancel"
        drillDown={pickerDrillDown}
        renderItem={pickerRenderItem}
        onQueryChange={(nextQuery) => ctx.setSearchQuery(nextQuery)}
        onFilterChange={(key, nextValue) => ctx.setFilter(key, nextValue)}
        onSelectionChange={handleSelectionChange}
        onConfirm={() => ctx.closePopover()}
        onCancel={() => ctx.closePopover()}
        onDrillContext={handleDrillContext}
      />
    </Popover>

    {#if showClearButton}
      <span class="underlay-relation-selector__action">
        <IconButton
          icon="x"
          ariaLabel="Clear selection"
          tooltip="Clear selection"
          variant="ghost"
          tone="danger"
          size="sm"
          onClick={handleClearClick}
        />
      </span>
    {/if}

    {#if showInlineCreate}
      <span class="underlay-relation-selector__action">
        <IconButton
          icon="plus"
          ariaLabel={createLabel ?? "Create new"}
          tooltip={createLabel ?? "Create new"}
          variant="primary"
          size="sm"
          disabled={disabled}
          onClick={handleCreateClick}
        />
      </span>
    {/if}
  </div>

  <Dialog
    open={ctx.state.modalOpen}
    title={ctx.state.createFormOpen ? createLabel ?? "Create new" : label}
    width="lg"
    showCloseButton
    onOpenChange={(open) => {
      if (!open) ctx.closeModal();
    }}
    onRequestClose={() => ctx.closeModal()}
  >
    {#if ctx.state.createFormOpen && createForm}
      {@render createForm(handleCreateSuccess, ctx.closeModal)}
    {/if}
  </Dialog>

  {#if error}
    <div class="underlay-relation-selector__error">{error}</div>
  {/if}
</div>

<style>
  .underlay-relation-selector {
    position: relative;
    width: 100%;
    min-width: 0;
  }

  .underlay-relation-selector--auto {
    width: auto;
  }

  .underlay-relation-selector__row {
    display: flex;
    align-items: stretch;
    width: 100%;
    min-width: 0;
  }

  .underlay-relation-selector--auto .underlay-relation-selector__row {
    width: auto;
  }

  .underlay-relation-selector__trigger {
    flex: 1 1 auto;
    display: flex;
    width: 100%;
    min-width: 0;
  }

  .underlay-relation-selector__trigger[aria-disabled="true"] {
    opacity: 0.6;
    pointer-events: none;
  }

  .underlay-relation-selector__trigger--icon {
    width: auto;
  }

  .underlay-relation-selector__field {
    flex: 1 1 auto;
    display: inline-flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    width: 100%;
    min-width: 0;
    box-sizing: border-box;
    padding: var(--underlay-field-padding-block, 0.55em)
      var(--underlay-field-padding-inline, 0.7em);
    border-radius: 0.35rem;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 0.85rem;
    line-height: 1.2;
    cursor: pointer;
  }

  .underlay-relation-selector__field:hover {
    background: var(--underlay-color-field-bg-hover, rgba(148, 163, 184, 0.25));
  }

  .underlay-relation-selector__field--attached {
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
  }

  .underlay-relation-selector--error .underlay-relation-selector__field {
    outline: 1px solid var(--underlay-color-danger, #ef4444);
    outline-offset: -1px;
  }

  .underlay-relation-selector__field--placeholder {
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .underlay-relation-selector__label {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .underlay-relation-selector__chevron {
    display: inline-flex;
    flex-shrink: 0;
    opacity: 0.55;
  }

  .underlay-relation-selector__icon-trigger {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.6rem;
    height: 1.6rem;
    border: 0.0625rem solid rgba(148, 163, 184, 0.35);
    border-radius: 0.5rem;
    background: rgba(255, 255, 255, 0.03);
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-relation-selector__action {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .underlay-relation-selector__error {
    margin-top: 0.35rem;
    color: var(--underlay-color-danger, #ef4444);
    font-size: 0.8rem;
  }
</style>
