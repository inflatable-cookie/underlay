<script lang="ts">
  import { tick } from "svelte";
  import { Popover as BitsPopover } from "bits-ui";
  import Search from "lucide-svelte/icons/search";
  import Loader from "lucide-svelte/icons/loader-circle";
  import Plus from "lucide-svelte/icons/plus";
  import Check from "lucide-svelte/icons/check";
  import X from "lucide-svelte/icons/x";
  import ChevronDown from "lucide-svelte/icons/chevron-down";
  import type { SelectableRelation, FilterConfig } from "./types.js";
  import { useRelationSelector } from "./context.svelte.js";
  import Button from "../../components/Button.svelte";

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
    <div class="relation-selector-popover__filters">
      {#each ctx.props.filters as filter (filter.key)}
        <div class="relation-selector-popover__filter">
          <span class="relation-selector-popover__filter-label">{filter.label}:</span>
          <div class="relation-selector-popover__filter-dropdown">
            <button
              type="button"
              class="relation-selector-popover__filter-trigger"
              onclick={() => toggleFilterDropdown(filter.key)}
            >
              <span>{getActiveFilterLabel(filter)}</span>
              <ChevronDown size="0.8em" />
            </button>
            {#if openFilterKey === filter.key}
              <div class="relation-selector-popover__filter-menu">
                {#if filter.includeAll !== false}
                  <button
                    type="button"
                    class="relation-selector-popover__filter-option"
                    class:relation-selector-popover__filter-option--selected={ctx.state.activeFilters[filter.key] === undefined}
                    onclick={() => handleFilterSelect(filter.key, undefined)}
                  >
                    {filter.allLabel ?? "All"}
                    {#if ctx.state.activeFilters[filter.key] === undefined}
                      <Check size="0.8em" />
                    {/if}
                  </button>
                {/if}
                {#each filter.options as option (option.id)}
                  <button
                    type="button"
                    class="relation-selector-popover__filter-option"
                    class:relation-selector-popover__filter-option--selected={ctx.state.activeFilters[filter.key] === option.id}
                    onclick={() => handleFilterSelect(filter.key, option.id)}
                  >
                    {option.label}
                    {#if ctx.state.activeFilters[filter.key] === option.id}
                      <Check size="0.8em" />
                    {/if}
                  </button>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}

  <div class="relation-selector-popover__search">
    <Search size="1em" class="relation-selector-popover__search-icon" />
    <input
      bind:this={searchInputRef}
      type="text"
      class="relation-selector-popover__search-input"
      placeholder={ctx.props.searchPlaceholder ?? "Search..."}
      value={ctx.state.searchQuery}
      oninput={handleSearchInput}
      onkeydown={handleSearchKeyDown}
      aria-controls="relation-selector-popover-list"
      aria-autocomplete="list"
    />
    {#if showLoading}
      <Loader size="1em" class="relation-selector-popover__search-loader" />
    {/if}
  </div>

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
      <div class="relation-selector-popover__section">
        <div class="relation-selector-popover__section-label">
          {ctx.props.suggestionsLabel ?? "Suggestions"}
        </div>
        <ul
          bind:this={listRef}
          class="relation-selector-popover__list"
          role="listbox"
          id="relation-selector-popover-list"
          onkeydown={(e) => handleListKeyDown(e, ctx.state.suggestionItems)}
        >
          {#each ctx.state.suggestionItems as item, index (item.id)}
            {@const selected = ctx.isSelected(item.id)}
            <li
              class="relation-selector-popover__item"
              class:relation-selector-popover__item--selected={selected}
              class:relation-selector-popover__item--disabled={item.disabled}
              class:relation-selector-popover__item--focused={focusedIndex === index}
              role="option"
              aria-selected={selected}
              onclick={() => handleItemClick(item)}
              tabindex={item.disabled ? -1 : 0}
            >
              {#if ctx.props.renderItem}
                {@render ctx.props.renderItem(item as never, selected)}
              {:else}
                <div class="relation-selector-popover__item-content">
                  <span class="relation-selector-popover__item-label">{item.label}</span>
                  {#if item.description}
                    <span class="relation-selector-popover__item-description">
                      {item.description}
                    </span>
                  {/if}
                </div>
                {#if selected}
                  <Check size="0.9em" class="relation-selector-popover__item-check" />
                {/if}
              {/if}
            </li>
          {/each}
        </ul>
      </div>
    {/if}

    {#if showSearchResults}
      <div class="relation-selector-popover__section">
        <div class="relation-selector-popover__section-label">
          Results ({ctx.state.searchTotal})
        </div>
        <ul
          bind:this={listRef}
          class="relation-selector-popover__list"
          role="listbox"
          id="relation-selector-popover-list"
          onkeydown={(e) => handleListKeyDown(e, ctx.state.searchResults)}
        >
          {#each ctx.state.searchResults as item, index (item.id)}
            {@const selected = ctx.isSelected(item.id)}
            <li
              class="relation-selector-popover__item"
              class:relation-selector-popover__item--selected={selected}
              class:relation-selector-popover__item--disabled={item.disabled}
              class:relation-selector-popover__item--focused={focusedIndex === index}
              role="option"
              aria-selected={selected}
              onclick={() => handleItemClick(item)}
              tabindex={item.disabled ? -1 : 0}
            >
              {#if ctx.props.renderItem}
                {@render ctx.props.renderItem(item as never, selected)}
              {:else}
                <div class="relation-selector-popover__item-content">
                  <span class="relation-selector-popover__item-label">{item.label}</span>
                  {#if item.description}
                    <span class="relation-selector-popover__item-description">
                      {item.description}
                    </span>
                  {/if}
                </div>
                {#if selected}
                  <Check size="0.9em" class="relation-selector-popover__item-check" />
                {/if}
              {/if}
            </li>
          {/each}
        </ul>
      </div>
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

    {#if ctx.props.allowCreate && ctx.props.createForm}
      <div class="relation-selector-popover__create">
        <button
          type="button"
          class="relation-selector-popover__create-button"
          onclick={handleCreateClick}
        >
          <Plus size="0.9em" />
          <span>{ctx.props.createLabel ?? "Add new"}</span>
        </button>
      </div>
    {/if}
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
    z-index: 50;
    width: min(22rem, calc(100vw - 1rem));
    min-width: var(--bits-popover-anchor-width);
    max-height: min(24rem, 50vh);
    display: flex;
    flex-direction: column;

    border-radius: 0.5rem;
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.5));
    background: var(--underlay-color-popover-bg, var(--underlay-color-bg-surface, #020617));
    box-shadow: var(--underlay-shadow-popover, 0 8px 24px rgba(0, 0, 0, 0.4));

    /* Ensure proper stacking and overflow */
    overflow: hidden;
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

  .relation-selector-popover__filters {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    padding: 0 0.75rem 0.5rem;
    flex-shrink: 0;
  }

  .relation-selector-popover__filter {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.75rem;
  }

  .relation-selector-popover__filter-label {
    color: var(--underlay-color-text-muted, #9ca3af);
    font-weight: 500;
  }

  .relation-selector-popover__filter-dropdown {
    position: relative;
  }

  .relation-selector-popover__filter-trigger {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem 0.5rem;
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.4));
    border-radius: 0.25rem;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.12));
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 0.75rem;
    cursor: pointer;
    transition: border-color 0.15s ease, background-color 0.15s ease;
  }

  .relation-selector-popover__filter-trigger:hover {
    border-color: var(--underlay-color-border, rgba(148, 163, 184, 0.6));
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.18));
  }

  .relation-selector-popover__filter-menu {
    position: absolute;
    top: 100%;
    left: 0;
    z-index: 10;
    min-width: 100%;
    max-height: 12rem;
    overflow-y: auto;
    margin-top: 0.25rem;
    padding: 0.25rem;
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.4));
    border-radius: 0.3rem;
    background: var(--underlay-color-popover-bg, var(--underlay-color-bg-surface, #020617));
    box-shadow: var(--underlay-shadow-popover, 0 4px 12px rgba(0, 0, 0, 0.3));
  }

  .relation-selector-popover__filter-option {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    width: 100%;
    padding: 0.35rem 0.5rem;
    border: none;
    border-radius: 0.2rem;
    background: transparent;
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 0.75rem;
    text-align: left;
    cursor: pointer;
    white-space: nowrap;
  }

  .relation-selector-popover__filter-option:hover {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.2));
  }

  .relation-selector-popover__filter-option--selected {
    background: var(--underlay-color-primary, #2563eb);
    color: var(--underlay-color-on-primary, white);
  }

  .relation-selector-popover__filter-option--selected:hover {
    background: var(--underlay-color-primary-strong, #1d4ed8);
  }

  .relation-selector-popover__search {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    padding: 0 0.75rem 0.5rem;
    position: relative;
    flex-shrink: 0;
  }

  :global(.relation-selector-popover__search-icon) {
    position: absolute;
    left: 1.3rem;
    color: var(--underlay-color-text-muted, #9ca3af);
    pointer-events: none;
  }

  .relation-selector-popover__search-input {
    width: 100%;
    padding: 0.45em 0.6em 0.45em 1.9em;
    border-radius: 0.3rem;
    border: none;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 0.8rem;
  }

  .relation-selector-popover__search-input:focus {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, #2563eb);
    outline-offset: -1px;
  }

  .relation-selector-popover__search-input::placeholder {
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  :global(.relation-selector-popover__search-loader) {
    position: absolute;
    right: 1.3rem;
    color: var(--underlay-color-text-muted, #9ca3af);
    animation: spin 1s linear infinite;
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

  .relation-selector-popover__section {
    margin-bottom: 0.5rem;
  }

  .relation-selector-popover__section-label {
    font-size: 0.65rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--underlay-color-text-muted, #9ca3af);
    margin-bottom: 0.35rem;
  }

  .relation-selector-popover__list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.1rem;
  }

  .relation-selector-popover__item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.4rem;
    padding: 0.4rem 0.5rem;
    border-radius: 0.3rem;
    cursor: pointer;
    user-select: none;
    font-size: 0.8rem;
    color: var(--underlay-color-text, #e5e7eb);
  }

  .relation-selector-popover__item:hover:not(.relation-selector-popover__item--disabled) {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.2));
  }

  .relation-selector-popover__item:focus-visible,
  .relation-selector-popover__item--focused {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, #2563eb);
    outline-offset: -1px;
  }

  .relation-selector-popover__item--selected {
    background: var(--underlay-color-primary, #2563eb);
    color: var(--underlay-color-on-primary, white);
  }

  .relation-selector-popover__item--selected:hover {
    background: var(--underlay-color-primary-strong, #1d4ed8);
  }

  .relation-selector-popover__item--disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .relation-selector-popover__item-content {
    display: flex;
    flex-direction: column;
    gap: 0.05rem;
    min-width: 0;
    flex: 1;
  }

  .relation-selector-popover__item-label {
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .relation-selector-popover__item-description {
    font-size: 0.75em;
    opacity: 0.7;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.relation-selector-popover__item-check) {
    flex-shrink: 0;
    opacity: 0.9;
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

  .relation-selector-popover__create {
    border-top: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.3));
    margin-top: 0.35rem;
    padding-top: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .relation-selector-popover__create-button {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    width: 100%;
    padding: 0.45rem 0.5rem;
    border: none;
    border-radius: 0.3rem;
    background: transparent;
    color: var(--underlay-color-primary, #2563eb);
    font-size: 0.8rem;
    cursor: pointer;
    text-align: left;
  }

  .relation-selector-popover__create-button:hover {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.15));
  }

  .relation-selector-popover__create-button:focus-visible {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, #2563eb);
    outline-offset: -1px;
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
