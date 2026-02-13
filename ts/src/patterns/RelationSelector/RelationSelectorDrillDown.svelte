<script lang="ts">
  import { tick } from "svelte";
  import ArrowLeft from "lucide-svelte/icons/arrow-left";
  import ChevronRight from "lucide-svelte/icons/chevron-right";
  import Loader from "lucide-svelte/icons/loader-circle";
  import type { SelectableRelation } from "./types.js";
  import type { DrillDownItem } from "./drilldown-types.js";
  import { useRelationSelector } from "./context.svelte.js";
  import RelationSelectorPopoverSearch from "./RelationSelectorPopoverSearch.svelte";
  import RelationSelectorPopoverFilters from "./RelationSelectorPopoverFilters.svelte";
  import type { FilterConfig } from "./types.js";

  const ctx = useRelationSelector<SelectableRelation>();
  const dd = ctx.drillDown;

  // Local UI state
  let searchInputRef: HTMLInputElement | null = $state(null);
  let listRef: HTMLUListElement | null = $state(null);
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let focusedIndex = $state(-1);
  let openFilterKey = $state<string | null>(null);

  // Focus search input when level changes
  $effect(() => {
    if (ctx.state.drillDown) {
      // Track depth changes to refocus
      const _depth = ctx.state.drillDown.depth;
      focusedIndex = -1;
      void tick().then(() => {
        searchInputRef?.focus();
      });
    }
  });

  // Reset focused index when items change
  $effect(() => {
    if (ctx.state.drillDown) {
      const _results = ctx.state.drillDown.searchResults;
      const _suggestions = ctx.state.drillDown.suggestionItems;
      focusedIndex = -1;
    }
  });

  // Current level config
  const currentLevel = $derived(dd.currentDrillDownLevel);
  const breadcrumbs = $derived(dd.drillDownBreadcrumbs);

  // Display items: search results if searching, otherwise suggestions
  const displayItems = $derived.by(() => {
    const ddState = ctx.state.drillDown;
    if (!ddState) return [];
    if (ddState.searchQuery.trim()) {
      return ddState.searchResults;
    }
    return ddState.suggestionItems;
  });

  const showSuggestions = $derived.by(() => {
    const ddState = ctx.state.drillDown;
    if (!ddState) return false;
    return !ddState.searchQuery.trim() && ddState.suggestionItems.length > 0;
  });

  const showSearchResults = $derived.by(() => {
    const ddState = ctx.state.drillDown;
    if (!ddState) return false;
    return ddState.searchQuery.trim() && ddState.searchResults.length > 0;
  });

  const showEmpty = $derived.by(() => {
    const ddState = ctx.state.drillDown;
    if (!ddState) return false;
    return (
      ddState.searchQuery.trim() &&
      !ddState.searching &&
      ddState.searchResults.length === 0
    );
  });

  const showLoading = $derived.by(() => {
    const ddState = ctx.state.drillDown;
    if (!ddState) return false;
    return ddState.searching || ddState.suggestionsLoading;
  });

  // Filter handling
  function toggleFilterDropdown(filterKey: string) {
    openFilterKey = openFilterKey === filterKey ? null : filterKey;
  }

  function handleFilterSelect(filterKey: string, optionId: string | undefined) {
    dd.setDrillDownFilter(filterKey, optionId);
    openFilterKey = null;
  }

  function getActiveFilterLabel(filter: FilterConfig): string {
    const ddState = ctx.state.drillDown;
    if (!ddState) return filter.allLabel ?? "All";
    const activeValue = ddState.activeFilters[filter.key];
    if (activeValue === undefined) {
      return filter.allLabel ?? "All";
    }
    const option = filter.options.find((o) => o.id === activeValue);
    return option?.label ?? filter.allLabel ?? "All";
  }

  // Search handling
  function handleSearchInput(event: Event) {
    const value = (event.target as HTMLInputElement).value;
    dd.setDrillDownSearch(value);
    focusedIndex = -1;

    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }

    if (value.trim()) {
      debounceTimer = setTimeout(() => {
        void dd.performDrillDownSearch(value);
      }, 300);
    } else {
      // Clear search results, show suggestions
      const ddState = ctx.state.drillDown;
      if (ddState) {
        ddState.searchResults = [];
        ddState.searchTotal = 0;
        ddState.searchError = null;
      }
    }
  }

  // Keyboard navigation
  function handleSearchKeyDown(event: KeyboardEvent) {
    const items = displayItems;
    if (event.key === "ArrowDown" && items.length > 0) {
      event.preventDefault();
      focusedIndex = 0;
      focusItem(0);
    } else if (event.key === "ArrowUp" && items.length > 0) {
      event.preventDefault();
      focusedIndex = items.length - 1;
      focusItem(items.length - 1);
    } else if (event.key === "Escape") {
      if (breadcrumbs.length > 0) {
        dd.drillDownBack();
      } else {
        ctx.closePopover();
      }
    } else if (event.key === "Backspace") {
      const ddState = ctx.state.drillDown;
      if (ddState && !ddState.searchQuery && breadcrumbs.length > 0) {
        dd.drillDownBack();
      }
    }
  }

  function handleListKeyDown(event: KeyboardEvent, items: DrillDownItem[]) {
    if (items.length === 0) return;

    if (event.key === "ArrowDown") {
      event.preventDefault();
      const nextIndex = focusedIndex < items.length - 1 ? focusedIndex + 1 : 0;
      focusedIndex = nextIndex;
      focusItem(nextIndex);
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      if (focusedIndex <= 0) {
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
      if (breadcrumbs.length > 0) {
        dd.drillDownBack();
      } else {
        ctx.closePopover();
      }
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

  function handleItemClick(item: DrillDownItem) {
    if (item.disabled) return;
    dd.drillDownSelect(item);
  }

  function handleBackClick() {
    dd.drillDownBack();
  }

  function handleBreadcrumbClick(depth: number) {
    dd.drillDownNavigateTo(depth);
  }

  function handlePopoverClick(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (!target.closest(".relation-selector-popover__filter-dropdown")) {
      openFilterKey = null;
    }
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
<div class="drilldown" role="presentation" onclick={handlePopoverClick}>
  {#if breadcrumbs.length > 0}
    <div class="drilldown__breadcrumbs">
      <button
        type="button"
        class="drilldown__back-btn"
        onclick={handleBackClick}
        aria-label="Go back"
      >
        <ArrowLeft size="0.9em" strokeWidth={2.5} />
      </button>
      <div class="drilldown__breadcrumb-trail">
        {#each breadcrumbs as crumb, i (crumb.key)}
          {#if i > 0}
            <span class="drilldown__breadcrumb-sep">/</span>
          {/if}
          <button
            type="button"
            class="drilldown__breadcrumb-item"
            onclick={() => handleBreadcrumbClick(crumb.depth)}
          >
            {crumb.itemLabel}
          </button>
        {/each}
      </div>
    </div>
  {/if}

  {#if currentLevel}
    <div class="drilldown__header">
      <span class="drilldown__level-label">{currentLevel.label}</span>
    </div>

    {#if currentLevel.filters && currentLevel.filters.length > 0}
      <RelationSelectorPopoverFilters
        filters={currentLevel.filters}
        activeFilters={ctx.state.drillDown?.activeFilters ?? {}}
        {openFilterKey}
        {getActiveFilterLabel}
        onToggleFilter={toggleFilterDropdown}
        onSelectFilter={handleFilterSelect}
      />
    {/if}

    <RelationSelectorPopoverSearch
      placeholder={currentLevel.searchPlaceholder ?? `Search ${currentLevel.label.toLowerCase()}...`}
      value={ctx.state.drillDown?.searchQuery ?? ""}
      {showLoading}
      onInput={handleSearchInput}
      onKeyDown={handleSearchKeyDown}
      onInputRef={(input) => (searchInputRef = input)}
    />

    <div class="drilldown__body">
      {#if ctx.state.drillDown?.searchError}
        <div class="drilldown__error">
          <span>{ctx.state.drillDown.searchError}</span>
          <button
            type="button"
            class="drilldown__error-retry"
            onclick={() => dd.performDrillDownSearch(ctx.state.drillDown?.searchQuery ?? "")}
          >
            Retry
          </button>
        </div>
      {/if}

      {#if showSuggestions || showSearchResults}
        {@const items = displayItems}
        {@const sectionLabel = showSearchResults
          ? `Results (${ctx.state.drillDown?.searchTotal ?? 0})`
          : ""}
        {#if sectionLabel}
          <div class="drilldown__section-label">{sectionLabel}</div>
        {/if}
        <ul
          bind:this={listRef}
          class="drilldown__list"
          role="listbox"
          onkeydown={(event) => handleListKeyDown(event, items)}
        >
          {#each items as item, index (item.id)}
            <li
              class="drilldown__item"
              class:drilldown__item--disabled={item.disabled}
              class:drilldown__item--focused={focusedIndex === index}
              role="option"
              aria-selected={false}
              onclick={() => handleItemClick(item)}
              onkeydown={(event) => {
                if (event.key === "Enter" || event.key === " ") {
                  event.preventDefault();
                  handleItemClick(item);
                }
              }}
              tabindex={item.disabled ? -1 : 0}
            >
              <div class="drilldown__item-content">
                <span class="drilldown__item-label">{item.label}</span>
                {#if item.description}
                  <span class="drilldown__item-description">{item.description}</span>
                {/if}
              </div>
              <div class="drilldown__item-meta">
                {#if item.count !== undefined}
                  <span class="drilldown__item-count">{item.count}</span>
                {/if}
                <ChevronRight size="0.85em" class="drilldown__item-chevron" />
              </div>
            </li>
          {/each}
        </ul>
      {/if}

      {#if showEmpty}
        <div class="drilldown__empty">
          No results found
        </div>
      {/if}

      {#if showLoading && !showSuggestions && !showSearchResults}
        <div class="drilldown__loading">
          <Loader size="1.2em" class="drilldown__loading-spinner" />
          <span>Loading...</span>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .drilldown {
    display: flex;
    flex-direction: column;
    min-height: 0;
    flex: 1;
  }

  .drilldown__breadcrumbs {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.5rem 0.75rem 0.3rem;
    flex-shrink: 0;
  }

  .drilldown__back-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    flex-shrink: 0;
    padding: 0;
    border: none;
    border-radius: 0.25rem;
    background: transparent;
    color: var(--underlay-color-text-muted, #9ca3af);
    cursor: pointer;
    transition: background-color 0.15s ease, color 0.15s ease;
  }

  .drilldown__back-btn:hover {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.2));
    color: var(--underlay-color-text, #e5e7eb);
  }

  .drilldown__breadcrumb-trail {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    min-width: 0;
    overflow: hidden;
  }

  .drilldown__breadcrumb-sep {
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.7rem;
    flex-shrink: 0;
    opacity: 0.6;
  }

  .drilldown__breadcrumb-item {
    padding: 0.15rem 0.3rem;
    border: none;
    border-radius: 0.2rem;
    background: transparent;
    color: var(--underlay-color-primary, #2563eb);
    font-size: 0.75rem;
    font-weight: 500;
    cursor: pointer;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 8rem;
    transition: background-color 0.15s ease;
  }

  .drilldown__breadcrumb-item:hover {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.15));
  }

  .drilldown__header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.4rem 0.75rem 0.3rem;
    flex-shrink: 0;
  }

  .drilldown__level-label {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.03em;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .drilldown__body {
    flex: 1;
    overflow-y: auto;
    padding: 0 0.75rem;
  }

  .drilldown__section-label {
    font-size: 0.65rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--underlay-color-text-muted, #9ca3af);
    margin-bottom: 0.35rem;
  }

  .drilldown__list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.1rem;
    margin-bottom: 0.5rem;
  }

  .drilldown__item {
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

  .drilldown__item:hover:not(.drilldown__item--disabled) {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.2));
  }

  .drilldown__item:focus-visible,
  .drilldown__item--focused {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, #2563eb);
    outline-offset: -1px;
  }

  .drilldown__item--disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .drilldown__item-content {
    display: flex;
    flex-direction: column;
    gap: 0.05rem;
    min-width: 0;
    flex: 1;
  }

  .drilldown__item-label {
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .drilldown__item-description {
    font-size: 0.75em;
    opacity: 0.7;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .drilldown__item-meta {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    flex-shrink: 0;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .drilldown__item-count {
    font-size: 0.7rem;
    opacity: 0.7;
  }

  :global(.drilldown__item-chevron) {
    opacity: 0.5;
  }

  .drilldown__empty {
    padding: 1.25rem 0.75rem;
    text-align: center;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.8rem;
  }

  .drilldown__loading {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.4rem;
    padding: 1.25rem 0.75rem;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.8rem;
  }

  :global(.drilldown__loading-spinner) {
    animation: drilldown-spin 1s linear infinite;
  }

  @keyframes drilldown-spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .drilldown__error {
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

  .drilldown__error-retry {
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

  .drilldown__error-retry:hover {
    background: rgba(255, 255, 255, 0.15);
  }
</style>
