<script lang="ts">
  import { tick } from "svelte";
  import { Popover as BitsPopover } from "bits-ui";
  import Loader from "lucide-svelte/icons/loader-circle";
  import X from "lucide-svelte/icons/x";
  import type { SelectableRelation, FilterConfig } from "./types.js";
  import { useRelationSelector } from "./context.svelte.js";
  import Button from "../../components/Button.svelte";
  import RelationSelectorPopoverCreateAction from "./RelationSelectorPopoverCreateAction.svelte";
  import RelationSelectorPopoverFilters from "./RelationSelectorPopoverFilters.svelte";
  import RelationSelectorPopoverListSection from "./RelationSelectorPopoverListSection.svelte";
  import RelationSelectorPopoverSearch from "./RelationSelectorPopoverSearch.svelte";

  const ctx = useRelationSelector<SelectableRelation>();

  // Filter state
  let openFilterKey = $state<string | null>(null);

  function toggleFilterDropdown(filterKey: string) {
    openFilterKey = openFilterKey === filterKey ? null : filterKey;
  }

  function handleFilterSelect(filterKey: string, optionId: string | undefined) {
    ctx.setFilter(filterKey, optionId);
    openFilterKey = null;
  }

  function getActiveFilterLabel(filter: FilterConfig): string {
    const activeValue = ctx.state.activeFilters[filter.key];
    if (activeValue === undefined) {
      return filter.allLabel ?? "All";
    }
    const option = filter.options.find((o) => o.id === activeValue);
    return option?.label ?? filter.allLabel ?? "All";
  }

  // Close filter dropdown when clicking outside
  function handlePopoverClick(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (!target.closest(".relation-selector-popover__filter-dropdown")) {
      openFilterKey = null;
    }
  }

  let searchInputRef: HTMLInputElement | null = $state(null);
  let listRef: HTMLUListElement | null = $state(null);
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let focusedIndex = $state(-1);

  // Focus search input when popover opens
  $effect(() => {
    if (ctx.state.popoverOpen) {
      focusedIndex = -1;
      void tick().then(() => {
        searchInputRef?.focus();
      });
    }
  });

  // Reset focused index when items change
  $effect(() => {
    if (ctx.state.searchResults || ctx.state.suggestionItems) {
      focusedIndex = -1;
    }
  });

  function handleSearchInput(event: Event) {
    const value = (event.target as HTMLInputElement).value;
    ctx.setSearchQuery(value);
    focusedIndex = -1;

    // Debounce the search
    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }

    if (value.trim()) {
      debounceTimer = setTimeout(() => {
        void ctx.performSearch(value);
      }, 300);
    } else {
      ctx.clearSearch();
    }
  }

  function handleSearchKeyDown(event: KeyboardEvent) {
    const items = getCurrentItems();
    if (items.length === 0) return;

    if (event.key === "ArrowDown") {
      event.preventDefault();
      focusedIndex = 0;
      focusItem(0);
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      focusedIndex = items.length - 1;
      focusItem(items.length - 1);
    } else if (event.key === "Escape") {
      ctx.closePopover();
    }
  }

  function handleListKeyDown(event: KeyboardEvent, items: SelectableRelation[]) {
    if (items.length === 0) return;

    if (event.key === "ArrowDown") {
      event.preventDefault();
      const nextIndex = focusedIndex < items.length - 1 ? focusedIndex + 1 : 0;
      focusedIndex = nextIndex;
      focusItem(nextIndex);
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      if (focusedIndex <= 0) {
        // Move focus back to search input
        focusedIndex = -1;
        searchInputRef?.focus();
      } else {
        const prevIndex = focusedIndex - 1;
        focusedIndex = prevIndex;
        focusItem(prevIndex);
      }
    } else if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      if (focusedIndex >= 0 && focusedIndex < items.length) {
        const item = items[focusedIndex];
        if (item && !item.disabled) {
          handleItemClick(item);
        }
      }
    } else if (event.key === "Escape") {
      ctx.closePopover();
    }
  }

  function focusItem(index: number) {
    void tick().then(() => {
      const items = listRef?.querySelectorAll<HTMLElement>('[role="option"]');
      if (items && items[index]) {
        items[index].focus();
      }
    });
  }

  function getCurrentItems(): SelectableRelation[] {
    if (ctx.state.searchQuery.trim()) {
      return ctx.state.searchResults;
    }
    return ctx.state.suggestionItems;
  }

  function handleItemClick(item: SelectableRelation) {
    if (item.disabled) return;
    ctx.selectItem(item as SelectableRelation);
  }

  function handleConfirm() {
    ctx.closePopover();
  }

  function handleCreateClick() {
    // This will switch to modal with create form open
    ctx.toggleCreateForm();
  }

  function handleClear() {
    ctx.clearSelection();
  }

  // Items to show: search results if searching, otherwise suggestions
  const displayItems = $derived.by(() => {
    if (ctx.state.searchQuery.trim()) {
      return ctx.state.searchResults;
    }
    return ctx.state.suggestionItems;
  });

  const showSuggestions = $derived(
    !ctx.state.searchQuery.trim() && ctx.state.suggestionItems.length > 0
  );

  const showSearchResults = $derived(
    ctx.state.searchQuery.trim() && ctx.state.searchResults.length > 0
  );

  const showEmpty = $derived(
    ctx.state.searchQuery.trim() &&
      !ctx.state.isSearching &&
      ctx.state.searchResults.length === 0
  );

  const showLoading = $derived(
    ctx.state.isSearching || ctx.state.isSuggestionsLoading
  );

  const hasSelection = $derived(
    ctx.isMultiSelect ? ctx.selectedItems.length > 0 : ctx.selectedItem !== null
  );

  const showClearButton = $derived(
    !ctx.props.required && hasSelection
  );
</script>

<BitsPopover.Portal>
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <BitsPopover.Content
    class="relation-selector-popover__content"
    side="bottom"
    sideOffset={4}
    align="end"
    alignOffset={0}
    avoidCollisions={true}
    collisionPadding={8}
    onOpenAutoFocus={(e) => {
      e.preventDefault();
      searchInputRef?.focus();
    }}
    onInteractOutside={() => {
      ctx.closePopover();
    }}
    onEscapeKeydown={() => {
      ctx.closePopover();
    }}
    onclick={handlePopoverClick}
  >
  <div class="relation-selector-popover__header">
    <span class="relation-selector-popover__title">{ctx.props.label}</span>
    {#if showClearButton}
      <button
        type="button"
        class="relation-selector-popover__clear-btn"
        onclick={handleClear}
      >
        <X size="0.85em" strokeWidth={2.5} />
        <span>Clear</span>
      </button>
    {/if}
  </div>

  {#if ctx.props.filters && ctx.props.filters.length > 0}
    <RelationSelectorPopoverFilters
      filters={ctx.props.filters}
      activeFilters={ctx.state.activeFilters}
      {openFilterKey}
      {getActiveFilterLabel}
      onToggleFilter={toggleFilterDropdown}
      onSelectFilter={handleFilterSelect}
    />
  {/if}

  <RelationSelectorPopoverSearch
    placeholder={ctx.props.searchPlaceholder ?? "Search..."}
    value={ctx.state.searchQuery}
    {showLoading}
    onInput={handleSearchInput}
    onKeyDown={handleSearchKeyDown}
    onInputRef={(input) => (searchInputRef = input)}
  />

  <div class="relation-selector-popover__body">
    {#if ctx.state.searchError}
      <div class="relation-selector-popover__error">
        <span>{ctx.state.searchError}</span>
        <button
          type="button"
          class="relation-selector-popover__error-retry"
          onclick={() => ctx.retrySearch()}
        >
          Retry
        </button>
      </div>
    {/if}

    {#if showSuggestions}
      <RelationSelectorPopoverListSection
        label={ctx.props.suggestionsLabel ?? "Suggestions"}
        items={ctx.state.suggestionItems}
        {focusedIndex}
        isSelected={ctx.isSelected}
        onItemClick={handleItemClick}
        onListKeyDown={handleListKeyDown}
        renderItem={ctx.props.renderItem as any}
        onListRef={(node) => (listRef = node)}
      />
    {/if}

    {#if showSearchResults}
      <RelationSelectorPopoverListSection
        label={`Results (${ctx.state.searchTotal})`}
        items={ctx.state.searchResults}
        {focusedIndex}
        isSelected={ctx.isSelected}
        onItemClick={handleItemClick}
        onListKeyDown={handleListKeyDown}
        renderItem={ctx.props.renderItem as any}
        onListRef={(node) => (listRef = node)}
      />
    {/if}

    {#if showEmpty}
      <div class="relation-selector-popover__empty">
        {ctx.props.emptyMessage ?? "No results found"}
      </div>
    {/if}

    {#if showLoading && !showSuggestions && !showSearchResults}
      <div class="relation-selector-popover__loading">
        <Loader size="1.2em" class="relation-selector-popover__loading-spinner" />
        <span>Loading...</span>
      </div>
    {/if}

    <RelationSelectorPopoverCreateAction
      allowCreate={ctx.props.allowCreate}
      hasCreateForm={!!ctx.props.createForm}
      createLabel={ctx.props.createLabel ?? "Add new"}
      onCreate={handleCreateClick}
    />
  </div>

  {#if ctx.isMultiSelect}
    <div class="relation-selector-popover__footer">
      <Button variant="primary" onclick={handleConfirm}>
        Done ({ctx.selectedItems.length})
      </Button>
    </div>
  {/if}
  </BitsPopover.Content>
</BitsPopover.Portal>

<style>
  :global(.relation-selector-popover__content) {
    /* bits-ui exposes --bits-popover-anchor-width for matching trigger width */
    /* z-index must be higher than dialogs (51) to work when nested inside dialogs */
    z-index: 60;
    width: min(22rem, calc(100vw - 1rem));
    min-width: var(--bits-popover-anchor-width);
    max-height: min(24rem, 50vh);
    display: flex;
    flex-direction: column;

    border-radius: 0.5rem;
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.5));
    background: var(--underlay-color-popover-bg, var(--underlay-color-bg-surface, #020617));
    box-shadow: var(--underlay-shadow-popover, 0 8px 24px rgba(0, 0, 0, 0.4));

    /* Ensure proper stacking - allow overflow for filter dropdowns */
  }

  :global(.relation-selector-popover__content[data-state="open"]) {
    animation: popover-in 0.15s ease-out;
  }

  @keyframes popover-in {
    from {
      opacity: 0;
      transform: scale(0.95);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }

  .relation-selector-popover__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding: 0.6rem 0.75rem 0.4rem;
    flex-shrink: 0;
  }

  .relation-selector-popover__title {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.03em;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .relation-selector-popover__clear-btn {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.2rem 0.4rem;
    border: none;
    border-radius: 0.25rem;
    background: transparent;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.7rem;
    cursor: pointer;
    transition: background-color 0.15s ease, color 0.15s ease;
  }

  .relation-selector-popover__clear-btn:hover {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.2));
    color: var(--underlay-color-danger, #ef4444);
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .relation-selector-popover__body {
    flex: 1;
    overflow-y: auto;
    padding: 0 0.75rem;
  }

  .relation-selector-popover__empty {
    padding: 1.25rem 0.75rem;
    text-align: center;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.8rem;
  }

  .relation-selector-popover__loading {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.4rem;
    padding: 1.25rem 0.75rem;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.8rem;
  }

  :global(.relation-selector-popover__loading-spinner) {
    animation: spin 1s linear infinite;
  }

  .relation-selector-popover__error {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding: 0.5rem;
    margin-bottom: 0.5rem;
    border-radius: 0.3rem;
    background: var(--underlay-color-danger, #ef4444);
    color: white;
    font-size: 0.75rem;
  }

  .relation-selector-popover__error-retry {
    flex-shrink: 0;
    padding: 0.25rem 0.5rem;
    border: 1px solid rgba(255, 255, 255, 0.5);
    border-radius: 0.2rem;
    background: transparent;
    color: white;
    font-size: 0.7rem;
    cursor: pointer;
    transition: background-color 0.15s ease;
  }

  .relation-selector-popover__error-retry:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .relation-selector-popover__footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.4rem;
    padding: 0.5rem 0.75rem 0.6rem;
    border-top: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.3));
    flex-shrink: 0;
  }
</style>
