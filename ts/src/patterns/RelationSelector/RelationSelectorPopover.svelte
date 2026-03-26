<script lang="ts">
  import { Button, Callout, SearchField } from "@poodle/svelte-primitives";
  import { tick } from "svelte";
  import { Popover as BitsPopover } from "bits-ui";
  import X from "lucide-svelte/icons/x";
  import type { SelectableRelation, FilterConfig } from "./types.js";
  import { useRelationSelector } from "./context.svelte.js";
  import RelationSelectorPopoverFilters from "./RelationSelectorPopoverFilters.svelte";
  import RelationSelectorPopoverListSection from "./RelationSelectorPopoverListSection.svelte";
  import RelationSelectorDrillDown from "./RelationSelectorDrillDown.svelte";
  import ArrowLeft from "lucide-svelte/icons/arrow-left";

  const ctx = useRelationSelector<SelectableRelation>();

  // Drill-down state
  const hasDrillDown = $derived(!!ctx.props.drillDown);
  const isDrillDownActive = $derived(ctx.drillDown.isDrillDownActive);
  const drillDownBreadcrumbs = $derived(ctx.drillDown.drillDownBreadcrumbs);

  // At the final selection level, use finalLevelFilters if configured, otherwise fall back to props.filters
  const effectiveFilters = $derived.by(() => {
    if (hasDrillDown && !isDrillDownActive) {
      const computed = ctx.drillDown.finalLevelFilters;
      if (computed) return computed;
    }
    return ctx.props.filters;
  });

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

  let listRef: HTMLUListElement | null = $state(null);
  let focusedIndex = $state(-1);
  const searchFieldId = "relation-selector-popover-search";

  // Focus search input when popover opens
  $effect(() => {
    if (ctx.state.popoverOpen) {
      focusedIndex = -1;
      void tick().then(() => {
        document.getElementById(searchFieldId)?.focus();
      });
    }
  });

  // Reset focused index when items change
  $effect(() => {
    if (ctx.state.searchResults || ctx.state.suggestionItems) {
      focusedIndex = -1;
    }
  });

  function handleSearchInput(value: string) {
    ctx.setSearchQuery(value);
    focusedIndex = -1;

    if (!value.trim()) {
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
      document.getElementById(searchFieldId)?.focus();
    }}
    onInteractOutside={() => {
      ctx.closePopover();
    }}
    onEscapeKeydown={() => {
      ctx.closePopover();
    }}
    onclick={handlePopoverClick}
  >
  {#if hasDrillDown && isDrillDownActive}
    <!-- Drill-down mode: show hierarchy navigation -->
    <RelationSelectorDrillDown />
  {:else}
    <!-- Normal selection mode (or final level of drill-down) -->
    <div class="relation-selector-popover__header">
      <span class="relation-selector-popover__title">{ctx.props.label}</span>
      {#if showClearButton}
        <Button
          type="button"
          variant="ghost"
          size="sm"
          leadingIcon="x"
          className="relation-selector-popover__clear-btn"
          on:click={handleClear}
        >
          Clear
        </Button>
      {/if}
    </div>

    {#if hasDrillDown && drillDownBreadcrumbs.length > 0}
      <div class="relation-selector-popover__drilldown-breadcrumbs">
        <Button
          type="button"
          variant="ghost"
          size="sm"
          leadingIcon="arrow-left"
          ariaLabel="Go back"
          className="relation-selector-popover__drilldown-back"
          on:click={() => ctx.drillDown.drillDownBack()}
        />
        <div class="relation-selector-popover__drilldown-trail">
          {#each drillDownBreadcrumbs as crumb, i (crumb.key)}
            {#if i > 0}
              <span class="relation-selector-popover__drilldown-sep">/</span>
            {/if}
            <Button
              type="button"
              variant="ghost"
              size="sm"
              className="relation-selector-popover__drilldown-crumb"
              on:click={() => ctx.drillDown.drillDownNavigateTo(crumb.depth)}
            >
              {crumb.itemLabel}
            </Button>
          {/each}
        </div>
      </div>
    {/if}

    {#if effectiveFilters && effectiveFilters.length > 0}
      <RelationSelectorPopoverFilters
        filters={effectiveFilters}
        activeFilters={ctx.state.activeFilters}
        {openFilterKey}
        {getActiveFilterLabel}
        onToggleFilter={toggleFilterDropdown}
        onSelectFilter={handleFilterSelect}
      />
    {/if}

    <div class="relation-selector-popover__search">
      <SearchField
        id={searchFieldId}
        value={ctx.state.searchQuery}
        debounce={300}
        placeholder={ctx.props.searchPlaceholder ?? "Search..."}
        ariaLabel={`${ctx.props.label} search`}
        on:valueChange={(event) => handleSearchInput(event.detail.value)}
        on:submit={(event) => {
          const value = event.detail.value;
          if (value.trim()) {
            void ctx.performSearch(value);
          }
        }}
        on:clear={() => ctx.clearSearch()}
        on:keydown={(event) => handleSearchKeyDown(event.detail)}
      />
    </div>

    <div class="relation-selector-popover__body">
      {#if ctx.state.searchError}
        <Callout tone="danger" message={ctx.state.searchError} announceMode="polite">
          <svelte:fragment slot="actions">
            <Button type="button" variant="ghost" size="sm" on:click={() => ctx.retrySearch()}>
              Retry
            </Button>
          </svelte:fragment>
        </Callout>
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
        <Callout tone="pending" title="Loading" message="Loading relation candidates..." />
      {/if}

      {#if ctx.props.allowCreate && ctx.props.createForm}
        <div class="relation-selector-popover__create">
          <Button
            type="button"
            variant="ghost"
            size="sm"
            leadingIcon="plus"
            on:click={handleCreateClick}
          >
            {ctx.props.createLabel ?? "Add new"}
          </Button>
        </div>
      {/if}
    </div>

    {#if ctx.isMultiSelect}
      <div class="relation-selector-popover__footer">
        <Button variant="primary" on:click={handleConfirm}>
          Done ({ctx.selectedItems.length})
        </Button>
      </div>
    {/if}
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

  /* Tighten header bottom padding when breadcrumbs follow */
  .relation-selector-popover__header:has(+ .relation-selector-popover__drilldown-breadcrumbs) {
    padding-bottom: 0;
  }

  .relation-selector-popover__title {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.03em;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  :global(.relation-selector-popover__clear-btn) {
    min-width: 0;
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

  .relation-selector-popover__footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.4rem;
    padding: 0.5rem 0.75rem 0.6rem;
    border-top: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.3));
    flex-shrink: 0;
  }

  /* Drill-down breadcrumbs at final selection level */
  .relation-selector-popover__drilldown-breadcrumbs {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.2rem 0.75rem 0.35rem;
    flex-shrink: 0;
  }

  :global(.relation-selector-popover__drilldown-back) {
    min-width: 0;
    width: 1.75rem;
    padding-inline: 0;
  }

  .relation-selector-popover__drilldown-trail {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    min-width: 0;
    overflow: hidden;
  }

  .relation-selector-popover__drilldown-sep {
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.7rem;
    flex-shrink: 0;
    opacity: 0.6;
  }

  :global(.relation-selector-popover__drilldown-crumb) {
    min-width: 0;
    padding-inline: 0.375rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 8rem;
  }
</style>
