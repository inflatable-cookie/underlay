<script lang="ts">
  import { tick } from "svelte";
  import { Dialog as BitsDialog } from "bits-ui";
  import Search from "lucide-svelte/icons/search";
  import Loader from "lucide-svelte/icons/loader-circle";
  import Plus from "lucide-svelte/icons/plus";
  import Check from "lucide-svelte/icons/check";
  import X from "lucide-svelte/icons/x";
  import type { SelectableRelation } from "./types.js";
  import { useRelationSelector } from "./context.svelte.js";
  import Button from "../../components/Button.svelte";

  const ctx = useRelationSelector<SelectableRelation>();

  let searchInputRef: HTMLInputElement | null = $state(null);
  let listRef: HTMLUListElement | null = $state(null);
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let focusedIndex = $state(-1);

  // Focus search input when modal opens
  $effect(() => {
    if (ctx.state.modalOpen) {
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
    ctx.closeModal();
  }

  function handleCancel() {
    ctx.closeModal();
  }

  function handleCreateClick() {
    ctx.toggleCreateForm();
  }

  function handleCreateSuccess(item: SelectableRelation) {
    ctx.handleCreateSuccess(item as SelectableRelation);
  }

  function handleCreateCancel() {
    ctx.closeCreateForm();
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

  function handleClear() {
    ctx.clearSelection();
  }
</script>

<BitsDialog.Root
  bind:open={ctx.state.modalOpen}
  onOpenChange={(open) => {
    if (!open) ctx.closeModal();
  }}
>
  <BitsDialog.Portal>
    <BitsDialog.Overlay class="relation-selector-modal__overlay" />

    <BitsDialog.Content class="relation-selector-modal__content {ctx.state.createFormOpen ? 'relation-selector-modal__content--create-mode' : ''}">
      <BitsDialog.Close class="relation-selector-modal__close" aria-label="Close">
        <span aria-hidden="true">×</span>
      </BitsDialog.Close>

      <div class="relation-selector-modal__header">
        <BitsDialog.Title class="relation-selector-modal__title">
          {#if ctx.state.createFormOpen}
            {ctx.props.createLabel ?? "Add new"}
          {:else}
            {ctx.props.label}
          {/if}
        </BitsDialog.Title>

        {#if showClearButton && !ctx.state.createFormOpen}
          <button
            type="button"
            class="relation-selector-modal__clear-btn"
            onclick={handleClear}
          >
            <X size="0.9em" strokeWidth={2.5} />
            <span>Clear</span>
          </button>
        {/if}
      </div>

      {#if !ctx.state.createFormOpen}
        <div class="relation-selector-modal__search">
          <Search size="1.1em" class="relation-selector-modal__search-icon" />
          <input
            bind:this={searchInputRef}
            type="text"
            class="relation-selector-modal__search-input"
            placeholder={ctx.props.searchPlaceholder ?? "Search..."}
            value={ctx.state.searchQuery}
            oninput={handleSearchInput}
            onkeydown={handleSearchKeyDown}
            aria-controls="relation-selector-list"
            aria-autocomplete="list"
          />
          {#if showLoading}
            <Loader size="1.1em" class="relation-selector-modal__search-loader" />
          {/if}
        </div>
      {/if}

      <div class="relation-selector-modal__body">
        {#if ctx.state.createFormOpen}
          <!-- Create form mode: only show the create form -->
          {#if ctx.props.createForm}
            <div class="relation-selector-modal__create-form">
              {@render ctx.props.createForm(handleCreateSuccess, handleCreateCancel)}
            </div>
          {/if}
        {:else}
          <!-- Selection mode: show search results, suggestions, etc. -->
          {#if ctx.state.searchError}
            <div class="relation-selector-modal__error">
              <span>{ctx.state.searchError}</span>
              <button
                type="button"
                class="relation-selector-modal__error-retry"
                onclick={() => ctx.retrySearch()}
              >
                Retry
              </button>
            </div>
          {/if}

          {#if showSuggestions}
            <div class="relation-selector-modal__section">
              <div class="relation-selector-modal__section-label">
                {ctx.props.suggestionsLabel ?? "Suggestions"}
              </div>
              <ul
                bind:this={listRef}
                class="relation-selector-modal__list"
                role="listbox"
                id="relation-selector-list"
                onkeydown={(e) => handleListKeyDown(e, ctx.state.suggestionItems)}
              >
                {#each ctx.state.suggestionItems as item, index (item.id)}
                  {@const selected = ctx.isSelected(item.id)}
                  <li
                    class="relation-selector-modal__item"
                    class:relation-selector-modal__item--selected={selected}
                    class:relation-selector-modal__item--disabled={item.disabled}
                    class:relation-selector-modal__item--focused={focusedIndex === index}
                    role="option"
                    aria-selected={selected}
                    onclick={() => handleItemClick(item)}
                    tabindex={item.disabled ? -1 : 0}
                  >
                    {#if ctx.props.renderItem}
                      {@render ctx.props.renderItem(item as never, selected)}
                    {:else}
                      <div class="relation-selector-modal__item-content">
                        <span class="relation-selector-modal__item-label">{item.label}</span>
                        {#if item.description}
                          <span class="relation-selector-modal__item-description">
                            {item.description}
                          </span>
                        {/if}
                      </div>
                      {#if selected}
                        <Check size="1em" class="relation-selector-modal__item-check" />
                      {/if}
                    {/if}
                  </li>
                {/each}
              </ul>
            </div>
          {/if}

          {#if showSearchResults}
            <div class="relation-selector-modal__section">
              <div class="relation-selector-modal__section-label">
                Results ({ctx.state.searchTotal})
              </div>
              <ul
                bind:this={listRef}
                class="relation-selector-modal__list"
                role="listbox"
                id="relation-selector-list"
                onkeydown={(e) => handleListKeyDown(e, ctx.state.searchResults)}
              >
                {#each ctx.state.searchResults as item, index (item.id)}
                  {@const selected = ctx.isSelected(item.id)}
                  <li
                    class="relation-selector-modal__item"
                    class:relation-selector-modal__item--selected={selected}
                    class:relation-selector-modal__item--disabled={item.disabled}
                    class:relation-selector-modal__item--focused={focusedIndex === index}
                    role="option"
                    aria-selected={selected}
                    onclick={() => handleItemClick(item)}
                    tabindex={item.disabled ? -1 : 0}
                  >
                    {#if ctx.props.renderItem}
                      {@render ctx.props.renderItem(item as never, selected)}
                    {:else}
                      <div class="relation-selector-modal__item-content">
                        <span class="relation-selector-modal__item-label">{item.label}</span>
                        {#if item.description}
                          <span class="relation-selector-modal__item-description">
                            {item.description}
                          </span>
                        {/if}
                      </div>
                      {#if selected}
                        <Check size="1em" class="relation-selector-modal__item-check" />
                      {/if}
                    {/if}
                  </li>
                {/each}
              </ul>
            </div>
          {/if}

          {#if showEmpty}
            <div class="relation-selector-modal__empty">
              {ctx.props.emptyMessage ?? "No results found"}
            </div>
          {/if}

          {#if showLoading && !showSuggestions && !showSearchResults}
            <div class="relation-selector-modal__loading">
              <Loader size="1.5em" class="relation-selector-modal__loading-spinner" />
              <span>Loading...</span>
            </div>
          {/if}

          {#if ctx.props.allowCreate && ctx.props.createForm}
            <div class="relation-selector-modal__create">
              <button
                type="button"
                class="relation-selector-modal__create-button"
                onclick={handleCreateClick}
              >
                <Plus size="1em" />
                <span>{ctx.props.createLabel ?? "Add new"}</span>
              </button>
            </div>
          {/if}
        {/if}
      </div>

      {#if ctx.isMultiSelect}
        <div class="relation-selector-modal__footer">
          <Button variant="subtle" onclick={handleCancel}>
            Cancel
          </Button>
          <Button variant="primary" onclick={handleConfirm}>
            Confirm ({ctx.selectedItems.length})
          </Button>
        </div>
      {/if}
    </BitsDialog.Content>
  </BitsDialog.Portal>
</BitsDialog.Root>

<style>
  :global(.relation-selector-modal__overlay) {
    position: fixed;
    inset: 0;
    background: var(--underlay-color-overlay-backdrop, rgba(0, 0, 0, 0.65));
    z-index: 50;
  }

  :global(.relation-selector-modal__content) {
    position: fixed;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    z-index: 51;

    width: min(32rem, calc(100vw - 2rem));
    max-height: min(80vh, 40rem);
    display: flex;
    flex-direction: column;

    border-radius: var(--underlay-radius-md, 0.75rem);
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25));
    background-color: var(--underlay-color-surface-muted, rgba(255, 255, 255, 0.02));
    box-shadow: var(--underlay-shadow-dialog, 0 20px 40px rgba(0, 0, 0, 0.55));
  }

  :global(.relation-selector-modal__content--create-mode) {
    /* 40em form max-width + 2rem padding */
    width: min(calc(40em + 2rem), calc(100vw - 2rem));
    max-height: min(90vh, 50rem);
  }

  :global(.relation-selector-modal__close) {
    position: absolute;
    top: 0.65rem;
    right: 0.65rem;
    border: none;
    border-radius: 0.5rem;
    background: transparent;
    color: inherit;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    font-size: 1.25rem;
    line-height: 1;
    z-index: 1;
  }

  :global(.relation-selector-modal__close:hover) {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.2));
  }

  .relation-selector-modal__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    padding: 1rem 1rem 0.75rem;
    padding-right: 2.5rem;
    flex-shrink: 0;
  }

  :global(.relation-selector-modal__title) {
    margin: 0;
    font-size: 1.05rem;
    font-weight: 650;
  }

  .relation-selector-modal__clear-btn {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    padding: 0.35rem 0.6rem;
    border: none;
    border-radius: 0.3rem;
    background: transparent;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.8rem;
    cursor: pointer;
    transition: background-color 0.15s ease, color 0.15s ease;
  }

  .relation-selector-modal__clear-btn:hover {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.2));
    color: var(--underlay-color-danger, #ef4444);
  }

  .relation-selector-modal__clear-btn:focus-visible {
    outline: 2px solid var(--underlay-color-primary, #2563eb);
    outline-offset: 1px;
  }

  .relation-selector-modal__search {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0 1rem 0.75rem;
    position: relative;
    flex-shrink: 0;
  }

  :global(.relation-selector-modal__search-icon) {
    position: absolute;
    left: 1.6rem;
    color: var(--underlay-color-text-muted, #9ca3af);
    pointer-events: none;
  }

  .relation-selector-modal__search-input {
    width: 100%;
    padding: 0.55em 0.7em 0.55em 2.2em;
    border-radius: 0.35rem;
    border: none;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 0.85rem;
  }

  .relation-selector-modal__search-input:focus {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, #2563eb);
    outline-offset: -1px;
  }

  .relation-selector-modal__search-input::placeholder {
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  :global(.relation-selector-modal__search-loader) {
    position: absolute;
    right: 1.6rem;
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

  .relation-selector-modal__body {
    flex: 1;
    overflow-y: auto;
    padding: 0 1rem;
  }

  .relation-selector-modal__section {
    margin-bottom: 1rem;
  }

  .relation-selector-modal__section-label {
    font-size: 0.7rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--underlay-color-text-muted, #9ca3af);
    margin-bottom: 0.5rem;
  }

  .relation-selector-modal__list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.15rem;
  }

  .relation-selector-modal__item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding: 0.5rem 0.65rem;
    border-radius: 0.35rem;
    cursor: pointer;
    user-select: none;
    font-size: 0.85rem;
    color: var(--underlay-color-text, #e5e7eb);
  }

  .relation-selector-modal__item:hover:not(.relation-selector-modal__item--disabled) {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.2));
  }

  .relation-selector-modal__item:focus-visible,
  .relation-selector-modal__item--focused {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, #2563eb);
    outline-offset: -1px;
  }

  .relation-selector-modal__item--selected {
    background: var(--underlay-color-primary, #2563eb);
    color: var(--underlay-color-on-primary, white);
  }

  .relation-selector-modal__item--selected:hover {
    background: var(--underlay-color-primary-strong, #1d4ed8);
  }

  .relation-selector-modal__item--disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .relation-selector-modal__item-content {
    display: flex;
    flex-direction: column;
    gap: 0.1rem;
    min-width: 0;
    flex: 1;
  }

  .relation-selector-modal__item-label {
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .relation-selector-modal__item-description {
    font-size: 0.8em;
    opacity: 0.7;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.relation-selector-modal__item-check) {
    flex-shrink: 0;
    opacity: 0.9;
  }

  .relation-selector-modal__empty {
    padding: 2rem 1rem;
    text-align: center;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.9rem;
  }

  .relation-selector-modal__loading {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 2rem 1rem;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.9rem;
  }

  :global(.relation-selector-modal__loading-spinner) {
    animation: spin 1s linear infinite;
  }

  .relation-selector-modal__error {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    padding: 0.75rem;
    margin-bottom: 1rem;
    border-radius: 0.35rem;
    background: var(--underlay-color-danger, #ef4444);
    color: white;
    font-size: 0.85rem;
  }

  .relation-selector-modal__error-retry {
    flex-shrink: 0;
    padding: 0.35rem 0.65rem;
    border: 1px solid rgba(255, 255, 255, 0.5);
    border-radius: 0.25rem;
    background: transparent;
    color: white;
    font-size: 0.8rem;
    cursor: pointer;
    transition: background-color 0.15s ease;
  }

  .relation-selector-modal__error-retry:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .relation-selector-modal__error-retry:focus-visible {
    outline: 2px solid white;
    outline-offset: 2px;
  }

  .relation-selector-modal__create {
    border-top: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.3));
    margin-top: 0.5rem;
    padding-top: 0.75rem;
    margin-bottom: 1rem;
  }

  .relation-selector-modal__create-button {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    width: 100%;
    padding: 0.6rem 0.7rem;
    border: none;
    border-radius: 0.35rem;
    background: transparent;
    color: var(--underlay-color-primary, #2563eb);
    font-size: 0.85rem;
    cursor: pointer;
    text-align: left;
  }

  .relation-selector-modal__create-button:hover {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.15));
  }

  .relation-selector-modal__create-button:focus-visible {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, #2563eb);
    outline-offset: -1px;
  }

  .relation-selector-modal__create-form {
    margin: 0.5rem 0 1rem;
  }

  .relation-selector-modal__footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    padding: 0.75rem 1rem 1rem;
    border-top: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.3));
    flex-shrink: 0;
  }
</style>
