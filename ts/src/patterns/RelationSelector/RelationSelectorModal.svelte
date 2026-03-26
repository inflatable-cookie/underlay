<script lang="ts">
  import type { Snippet } from "svelte";
  import { tick } from "svelte";
  import Check from "lucide-svelte/icons/check";
  import {
    Button,
    Callout,
    Dialog as PoodleDialog,
    SearchField
  } from "@poodle/svelte-primitives";

  import type { SelectableRelation } from "./types.js";
  import { useRelationSelector } from "./context.svelte.js";

  interface ModalSection {
    label: string;
    items: SelectableRelation[];
  }

  const ctx = useRelationSelector<SelectableRelation>();
  let modalOpen = $state(ctx.state.modalOpen);
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let listRef: HTMLUListElement | null = $state(null);
  let focusedIndex = $state(-1);
  const searchFieldId = "relation-selector-modal-search";

  function handleSearch(value: string) {
    ctx.setSearchQuery(value);
    focusedIndex = -1;

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

  function handleSelect(item: SelectableRelation) {
    ctx.selectItem(item);
  }

  function handleClose() {
    ctx.closeModal();
  }

  function handleConfirm() {
    ctx.closeModal();
  }

  function handleCancel() {
    ctx.closeModal();
  }

  function handleClear() {
    ctx.clearSelection();
  }

  function handleCreate() {
    ctx.toggleCreateForm();
  }

  function handleCreateSuccess(item: SelectableRelation) {
    ctx.handleCreateSuccess(item);
  }

  function handleCreateCancel() {
    ctx.closeCreateForm();
  }

  function handleRetry() {
    ctx.retrySearch();
  }

  function isSelected(id: string): boolean {
    return ctx.isSelected(id);
  }

  function getGlobalIndex(sectionIndex: number, itemIndex: number): number {
    if (!sections) return itemIndex;
    let globalIndex = 0;
    for (let index = 0; index < sectionIndex; index++) {
      globalIndex += sections[index]?.items.length ?? 0;
    }
    return globalIndex + itemIndex;
  }

  function focusItem(index: number) {
    void tick().then(() => {
      const options = listRef?.querySelectorAll<HTMLElement>('[role="option"]');
      if (options && options[index]) {
        options[index].focus();
      }
    });
  }

  function handleSearchKeyDown(event: KeyboardEvent) {
    const itemsList = sections
      ? sections.flatMap((section) => section.items)
      : items;
    if (itemsList.length === 0) return;

    if (event.key === "ArrowDown") {
      event.preventDefault();
      focusedIndex = 0;
      focusItem(0);
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      focusedIndex = itemsList.length - 1;
      focusItem(itemsList.length - 1);
    }
  }

  function handleListKeyDown(event: KeyboardEvent) {
    const itemsList = sections
      ? sections.flatMap((section) => section.items)
      : items;
    if (itemsList.length === 0) return;

    if (event.key === "ArrowDown") {
      event.preventDefault();
      const nextIndex = focusedIndex < itemsList.length - 1 ? focusedIndex + 1 : 0;
      focusedIndex = nextIndex;
      focusItem(nextIndex);
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      if (focusedIndex <= 0) {
        focusedIndex = -1;
        (document.getElementById(searchFieldId) as HTMLInputElement | null)?.focus();
      } else {
        const prevIndex = focusedIndex - 1;
        focusedIndex = prevIndex;
        focusItem(prevIndex);
      }
    } else if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      if (focusedIndex >= 0 && focusedIndex < itemsList.length) {
        const item = itemsList[focusedIndex];
        if (item && !item.disabled) {
          handleSelect(item);
        }
      }
    }
  }

  function handleItemKeydown(event: KeyboardEvent, item: SelectableRelation) {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      handleSelect(item);
    }
  }

  $effect(() => {
    if (modalOpen !== ctx.state.modalOpen) {
      modalOpen = ctx.state.modalOpen;
    }
  });

  $effect(() => {
    if (ctx.state.modalOpen !== modalOpen) {
      ctx.state.modalOpen = modalOpen;
    }
  });

  $effect(() => {
    if (modalOpen && !ctx.state.createFormOpen) {
      focusedIndex = -1;
      void tick().then(() => {
        (document.getElementById(searchFieldId) as HTMLInputElement | null)?.focus();
      });
    }
  });

  $effect(() => {
    const _results = ctx.state.searchResults;
    const _suggestions = ctx.state.suggestionItems;
    focusedIndex = -1;
  });

  const sections = $derived.by((): ModalSection[] | undefined => {
    const result: ModalSection[] = [];

    if (ctx.state.searchQuery.trim()) {
      if (ctx.state.searchResults.length > 0) {
        result.push({
          label: `Results (${ctx.state.searchTotal})`,
          items: ctx.state.searchResults
        });
      }
    } else if (ctx.state.suggestionItems.length > 0) {
      result.push({
        label: ctx.props.suggestionsLabel ?? "Suggestions",
        items: ctx.state.suggestionItems
      });
    }

    return result.length > 0 ? result : undefined;
  });

  const items = $derived.by((): SelectableRelation[] => {
    if (sections) return [];
    return ctx.state.searchQuery.trim()
      ? ctx.state.searchResults
      : ctx.state.suggestionItems;
  });

  const hasSelection = $derived(
    ctx.isMultiSelect ? ctx.selectedItems.length > 0 : ctx.selectedItem !== null
  );

  const showClear = $derived(!ctx.props.required && hasSelection);
  const isSearching = $derived(ctx.state.isSearching || ctx.state.isSuggestionsLoading);
  const hasItems = $derived(
    sections
      ? sections.some((section) => section.items.length > 0)
      : items.length > 0
  );
  const showEmpty = $derived(!isSearching && !hasItems && !ctx.state.createFormOpen);
  const showLoading = $derived(isSearching && !hasItems && !ctx.state.createFormOpen);
  const displayTitle = $derived(
    ctx.state.createFormOpen ? (ctx.props.createLabel ?? "Add new") : ctx.props.label
  );
</script>

<PoodleDialog
  open={modalOpen}
  title={displayTitle}
  showCloseButton
  closeLabel={`Close ${displayTitle}`}
  contentClassName={`relation-selector-modal__content ${ctx.state.createFormOpen ? "relation-selector-modal__content--create-mode" : ""}`}
  overlayClassName="relation-selector-modal__overlay"
  on:openChange={(event) => {
    modalOpen = event.detail.open;
    if (!event.detail.open) {
      handleClose();
    }
  }}
>
  {#if showClear && !ctx.state.createFormOpen}
    <div class="relation-selector-modal__topbar">
      {#if showClear}
        <Button type="button" variant="ghost" size="sm" on:click={handleClear}>
          Clear
        </Button>
      {/if}
    </div>
  {/if}

  {#if !ctx.state.createFormOpen}
    <div class="relation-selector-modal__search">
      <SearchField
        id={searchFieldId}
        value={ctx.state.searchQuery}
        placeholder={ctx.props.searchPlaceholder ?? "Search..."}
        ariaLabel={`${displayTitle} search`}
        on:valueChange={(event) => handleSearch(event.detail.value)}
        on:keydown={(event) => handleSearchKeyDown(event.detail)}
      />
    </div>
  {/if}

  <div class="relation-selector-modal__body">
    {#if ctx.state.createFormOpen && ctx.props.createForm}
      <div class="relation-selector-modal__create-form">
        {@render ctx.props.createForm(
          (item) => handleCreateSuccess(item as SelectableRelation),
          handleCreateCancel
        )}
      </div>
    {:else}
      {#if ctx.state.searchError}
        <Callout tone="danger" message={ctx.state.searchError} announceMode="polite">
          <svelte:fragment slot="actions">
            <Button type="button" variant="ghost" size="sm" on:click={handleRetry}>
              Retry
            </Button>
          </svelte:fragment>
        </Callout>
      {/if}

      {#if showLoading}
        <Callout tone="pending" title="Loading" message="Loading relation candidates..." />
      {:else if showEmpty}
        <div class="relation-selector-modal__empty">
          {ctx.state.searchQuery.trim()
            ? "No matches found."
            : (ctx.props.emptyMessage ?? "No results found")}
        </div>
      {:else if sections}
        {#each sections as section, sectionIndex (section.label)}
          {#if section.items.length > 0}
            <div class="relation-selector-modal__section">
              <div class="relation-selector-modal__section-label">{section.label}</div>
              <ul
                bind:this={listRef}
                class="relation-selector-modal__list"
                role="listbox"
                onkeydown={handleListKeyDown}
              >
                {#each section.items as item, itemIndex (item.id)}
                  {@const selected = isSelected(item.id)}
                  {@const globalIndex = getGlobalIndex(sectionIndex, itemIndex)}
                  <li
                    class="relation-selector-modal__item"
                    class:relation-selector-modal__item--selected={selected}
                    class:relation-selector-modal__item--disabled={item.disabled}
                    class:relation-selector-modal__item--focused={focusedIndex === globalIndex}
                    role="option"
                    aria-selected={selected}
                    aria-disabled={item.disabled}
                    onclick={() => handleSelect(item)}
                    onkeydown={(event) => handleItemKeydown(event, item)}
                    tabindex={item.disabled ? -1 : 0}
                  >
                    {#if ctx.props.renderItem}
                      {@render (ctx.props.renderItem as Snippet<[SelectableRelation, boolean]>)(item, selected)}
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
        {/each}
      {:else}
        <ul
          bind:this={listRef}
          class="relation-selector-modal__list"
          role="listbox"
          onkeydown={handleListKeyDown}
        >
          {#each items as item, index (item.id)}
            {@const selected = isSelected(item.id)}
            <li
              class="relation-selector-modal__item"
              class:relation-selector-modal__item--selected={selected}
              class:relation-selector-modal__item--disabled={item.disabled}
              class:relation-selector-modal__item--focused={focusedIndex === index}
              role="option"
              aria-selected={selected}
              aria-disabled={item.disabled}
              onclick={() => handleSelect(item)}
              onkeydown={(event) => handleItemKeydown(event, item)}
              tabindex={item.disabled ? -1 : 0}
            >
              {#if ctx.props.renderItem}
                {@render (ctx.props.renderItem as Snippet<[SelectableRelation, boolean]>)(item, selected)}
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
      {/if}

      {#if (ctx.props.allowCreate ?? false) && ctx.props.createForm && !ctx.state.createFormOpen}
        <div class="relation-selector-modal__create">
          <Button
            type="button"
            variant="ghost"
            size="sm"
            leadingIcon="plus"
            on:click={handleCreate}
          >
            {ctx.props.createLabel ?? "Add new"}
          </Button>
        </div>
      {/if}
    {/if}
  </div>

  <svelte:fragment slot="actions">
    {#if ctx.isMultiSelect && !ctx.state.createFormOpen}
      <Button type="button" variant="ghost" on:click={handleCancel}>
        Cancel
      </Button>
      <Button type="button" variant="primary" on:click={handleConfirm}>
        Confirm ({ctx.selectedItems.length})
      </Button>
    {/if}
  </svelte:fragment>
</PoodleDialog>

<style>
  :global(.relation-selector-modal__overlay) {
    background: var(--underlay-color-overlay-backdrop, rgba(0, 0, 0, 0.65));
  }

  :global(.relation-selector-modal__content) {
    width: min(32rem, calc(100vw - 2rem));
    max-height: min(80vh, 40rem);
    display: flex;
    flex-direction: column;
  }

  :global(.relation-selector-modal__content--create-mode) {
    width: min(calc(40em + 2rem), calc(100vw - 2rem));
    max-height: min(90vh, 50rem);
  }

  .relation-selector-modal__topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    margin-bottom: 0.75rem;
  }

  .relation-selector-modal__search {
    margin-bottom: 0.75rem;
  }

  .relation-selector-modal__body {
    display: flex;
    flex: 1;
    flex-direction: column;
    min-height: 0;
    overflow-y: auto;
  }

  .relation-selector-modal__create-form {
    min-height: 0;
  }

  .relation-selector-modal__empty {
    padding: 1.5rem 1rem;
    text-align: center;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.85rem;
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

  .relation-selector-modal__create {
    display: flex;
    justify-content: flex-start;
    padding-top: 0.75rem;
    margin-top: 0.75rem;
    border-top: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.3));
  }
</style>
