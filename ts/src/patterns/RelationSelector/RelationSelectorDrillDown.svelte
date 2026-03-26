<script lang="ts">
  import { Button, Callout, Pill, SearchField } from "@poodle/svelte-primitives";
  import { tick } from "svelte";
  import ChevronRight from "lucide-svelte/icons/chevron-right";
  import type { SelectableRelation } from "./types.js";
  import type { DrillDownItem } from "./drilldown-types.js";
  import { useRelationSelector } from "./context.svelte.js";
  import RelationSelectorPopoverFilters from "./RelationSelectorPopoverFilters.svelte";
  import type { FilterConfig } from "./types.js";

  const ctx = useRelationSelector<SelectableRelation>();
  const dd = ctx.drillDown;

  // Local UI state
  let listRef: HTMLUListElement | null = $state(null);
  let focusedIndex = $state(-1);
  let openFilterKey = $state<string | null>(null);
  const searchFieldId = "relation-selector-drilldown-search";

  // Focus search input when level changes
  $effect(() => {
    if (ctx.state.drillDown) {
      // Track depth changes to refocus
      const _depth = ctx.state.drillDown.depth;
      focusedIndex = -1;
      void tick().then(() => {
        document.getElementById(searchFieldId)?.focus();
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
  function handleSearchInput(value: string) {
    dd.setDrillDownSearch(value);
    focusedIndex = -1;

    if (!value.trim()) {
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
        document.getElementById(searchFieldId)?.focus();
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
      <Button
        type="button"
        variant="ghost"
        size="sm"
        leadingIcon="arrow-left"
        ariaLabel="Go back"
        className="drilldown__back-btn"
        on:click={handleBackClick}
      />
      <div class="drilldown__breadcrumb-trail">
        {#each breadcrumbs as crumb, i (crumb.key)}
          {#if i > 0}
            <span class="drilldown__breadcrumb-sep">/</span>
          {/if}
          <Button
            type="button"
            variant="ghost"
            size="sm"
            className="drilldown__breadcrumb-item"
            on:click={() => handleBreadcrumbClick(crumb.depth)}
          >
            {crumb.itemLabel}
          </Button>
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

    <div class="drilldown__search">
      <SearchField
        id={searchFieldId}
        value={ctx.state.drillDown?.searchQuery ?? ""}
        debounce={300}
        placeholder={currentLevel.searchPlaceholder ?? `Search ${currentLevel.label.toLowerCase()}...`}
        ariaLabel={`${currentLevel.label} search`}
        on:valueChange={(event) => handleSearchInput(event.detail.value)}
        on:submit={(event) => {
          const value = event.detail.value;
          if (value.trim()) {
            void dd.performDrillDownSearch(value);
          }
        }}
        on:keydown={(event) => handleSearchKeyDown(event.detail)}
      />
    </div>

    <div class="drilldown__body">
      {#if ctx.state.drillDown?.searchError}
        <Callout tone="danger" message={ctx.state.drillDown.searchError} announceMode="polite">
          <svelte:fragment slot="actions">
            <Button
              type="button"
              variant="ghost"
              size="sm"
              on:click={() => dd.performDrillDownSearch(ctx.state.drillDown?.searchQuery ?? "")}
            >
              Retry
            </Button>
          </svelte:fragment>
        </Callout>
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
                  <Pill tone="neutral" appearance="badge" size="sm" muted ariaLabel={`${item.count}`}>
                    {item.count}
                  </Pill>
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
        <Callout tone="pending" title="Loading" message="Loading relation candidates..." />
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

  :global(.drilldown__back-btn) {
    min-width: 0;
    width: 1.75rem;
    padding-inline: 0;
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

  :global(.drilldown__breadcrumb-item) {
    min-width: 0;
    padding-inline: 0.375rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 8rem;
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

  :global(.drilldown__item-chevron) {
    opacity: 0.5;
  }

  .drilldown__empty {
    padding: 1.25rem 0.75rem;
    text-align: center;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.8rem;
  }

</style>
