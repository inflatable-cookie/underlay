<script lang="ts">
  import { tick } from "svelte";
  import type { Snippet } from "svelte";
  import {
    Button,
    Callout,
    Dialog as PoodleDialog,
    SearchField
  } from "@poodle/svelte-primitives";

  import RelationPickerList from "./relation-picker/RelationPickerList.svelte";
  import type { PickableItem, PickerSection } from "./relation-picker-types.js";

  interface Props {
    open?: boolean;
    title: string;
    emptyMessage?: string;
    items?: PickableItem[];
    sections?: PickerSection[];
    selectedIds?: string[];
    searchable?: boolean;
    searchPlaceholder?: string;
    searchQuery?: string;
    searching?: boolean;
    sectionLabel?: string;
    onSelect?: (item: PickableItem) => void;
    onClose?: () => void;
    onSearch?: (query: string) => void;
    renderItem?: Snippet<[item: PickableItem, selected: boolean]>;
    headerExtra?: Snippet;
    footer?: Snippet;
    error?: string;
    onRetry?: () => void;
    showClear?: boolean;
    onClear?: () => void;
    allowCreate?: boolean;
    createLabel?: string;
    onCreate?: () => void;
    createFormOpen?: boolean;
    createForm?: Snippet<[onSuccess: (item: PickableItem) => void, onCancel: () => void]>;
    onCreateSuccess?: (item: PickableItem) => void;
    onCreateCancel?: () => void;
    multiSelect?: boolean;
    selectedCount?: number;
    onConfirm?: () => void;
    onCancel?: () => void;
  }

  let {
    open = $bindable(false),
    title,
    emptyMessage = "No items available.",
    items = [],
    sections,
    selectedIds = [],
    searchable = true,
    searchPlaceholder = "Search...",
    searchQuery: externalSearchQuery,
    searching = false,
    sectionLabel,
    onSelect,
    onClose,
    onSearch,
    renderItem,
    headerExtra,
    footer,
    error,
    onRetry,
    showClear = false,
    onClear,
    allowCreate = false,
    createLabel = "Add new",
    onCreate,
    createFormOpen = false,
    createForm,
    onCreateSuccess,
    onCreateCancel,
    multiSelect = false,
    selectedCount = 0,
    onConfirm,
    onCancel
  }: Props = $props();

  let listRef: HTMLUListElement | null = $state(null);
  let internalSearchQuery = $state("");
  let focusedIndex = $state(-1);
  const searchFieldId = "relation-picker-dialog-search";

  const searchQuery = $derived(externalSearchQuery ?? internalSearchQuery);

  $effect(() => {
    if (open && searchable && !createFormOpen) {
      internalSearchQuery = "";
      focusedIndex = -1;
      void tick().then(() => {
        (document.getElementById(searchFieldId) as HTMLInputElement | null)?.focus();
      });
    }
  });

  $effect(() => {
    if (items || sections) {
      focusedIndex = -1;
    }
  });

  const displayItems = $derived.by((): PickableItem[] => {
    if (onSearch || !searchQuery.trim()) return items;
    const query = searchQuery.toLowerCase();
    return items.filter(
      (item: PickableItem) =>
        item.label.toLowerCase().includes(query) ||
        item.description?.toLowerCase().includes(query)
    );
  });

  const displaySections = $derived.by((): PickerSection[] | null => {
    if (!sections) return null;
    if (onSearch || !searchQuery.trim()) return sections;
    const query = searchQuery.toLowerCase();
    return sections
      .map((section: PickerSection) => ({
        ...section,
        items: section.items.filter(
          (item: PickableItem) =>
            item.label.toLowerCase().includes(query) ||
            item.description?.toLowerCase().includes(query)
        )
      }))
      .filter((section: PickerSection) => section.items.length > 0);
  });

  function handleSearchInput(value: string) {
    internalSearchQuery = value;
    focusedIndex = -1;

    if (onSearch) {
      onSearch(value);
    }
  }

  function handleSearchKeyDown(event: KeyboardEvent) {
    const itemsList = displaySections
      ? displaySections.flatMap((section: PickerSection) => section.items)
      : displayItems;
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
    const itemsList = displaySections
      ? displaySections.flatMap((section: PickerSection) => section.items)
      : displayItems;
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
          handleItemClick(item);
        }
      }
    }
  }

  function focusItem(index: number) {
    void tick().then(() => {
      const options = listRef?.querySelectorAll<HTMLElement>('[role="option"]');
      if (options && options[index]) {
        options[index].focus();
      }
    });
  }

  function handleItemClick(item: PickableItem) {
    if (item.disabled) return;
    onSelect?.(item);
  }

  function handleClose() {
    onClose?.();
    open = false;
  }

  function handleOpenChange(nextOpen: boolean) {
    if (!nextOpen) {
      handleClose();
    }
  }

  function getGlobalIndex(sectionIndex: number, itemIndex: number): number {
    if (!displaySections) return itemIndex;
    let globalIndex = 0;
    for (let index = 0; index < sectionIndex; index++) {
      globalIndex += displaySections[index]?.items.length ?? 0;
    }
    return globalIndex + itemIndex;
  }

  const hasItems = $derived(
    displaySections
      ? displaySections.some((section) => section.items.length > 0)
      : displayItems.length > 0
  );

  const showEmpty = $derived(!searching && !hasItems && !createFormOpen);
  const showLoading = $derived(searching && !hasItems && !createFormOpen);
  const displayTitle = $derived(createFormOpen ? createLabel : title);
</script>

<PoodleDialog
  {open}
  title={displayTitle}
  showCloseButton
  closeLabel={`Close ${displayTitle}`}
  contentClassName={`relation-picker-dialog__content ${createFormOpen ? "relation-picker-dialog__content--create-mode" : ""}`}
  overlayClassName="relation-picker-dialog__overlay"
  on:openChange={(event) => handleOpenChange(event.detail.open)}
>
  {#if (showClear || headerExtra) && !createFormOpen}
    <div class="relation-picker-dialog__topbar">
      {#if showClear}
        <Button type="button" variant="ghost" size="sm" on:click={() => onClear?.()}>
          Clear
        </Button>
      {/if}
      {#if headerExtra}
        <div class="relation-picker-dialog__header-extra">
          {@render headerExtra()}
        </div>
      {/if}
    </div>
  {/if}

  {#if searchable && !createFormOpen}
    <div class="relation-picker-dialog__search">
      <SearchField
        id={searchFieldId}
        value={searchQuery}
        placeholder={searchPlaceholder}
        ariaLabel={`${displayTitle} search`}
        on:valueChange={(event) => handleSearchInput(event.detail.value)}
        on:keydown={(event) => handleSearchKeyDown(event.detail)}
      />
    </div>
  {/if}

  <div class="relation-picker-dialog__body">
    {#if createFormOpen && createForm}
      <div class="relation-picker-dialog__create-form">
        {@render createForm(
          (item) => onCreateSuccess?.(item),
          () => onCreateCancel?.()
        )}
      </div>
    {:else}
      {#if error}
        <Callout tone="danger" message={error} announceMode="polite">
          <svelte:fragment slot="actions">
            {#if onRetry}
              <Button type="button" variant="ghost" size="sm" on:click={onRetry}>
                Retry
              </Button>
            {/if}
          </svelte:fragment>
        </Callout>
      {/if}

      {#if showLoading}
        <Callout tone="pending" title="Loading" message="Loading relation candidates..." />
      {:else if showEmpty}
        <div class="relation-picker-dialog__empty">
          {searchQuery.trim() ? "No matches found." : emptyMessage}
        </div>
      {:else}
        <RelationPickerList
          {displaySections}
          {displayItems}
          {sectionLabel}
          {selectedIds}
          {focusedIndex}
          {renderItem}
          onItemClick={handleItemClick}
          onListKeyDown={handleListKeyDown}
          {getGlobalIndex}
          onListRef={(node) => (listRef = node)}
        />
      {/if}

      {#if allowCreate && createForm && !createFormOpen}
        <div class="relation-picker-dialog__create">
          <Button
            type="button"
            variant="ghost"
            size="sm"
            leadingIcon="plus"
            on:click={() => onCreate?.()}
          >
            {createLabel}
          </Button>
        </div>
      {/if}
    {/if}
  </div>

  <svelte:fragment slot="actions">
    {#if footer && !createFormOpen}
      {@render footer()}
    {:else if multiSelect && !createFormOpen}
      <Button type="button" variant="ghost" on:click={() => onCancel?.()}>
        Cancel
      </Button>
      <Button type="button" variant="primary" on:click={() => onConfirm?.()}>
        Confirm ({selectedCount})
      </Button>
    {/if}
  </svelte:fragment>
</PoodleDialog>

<style>
  :global(.relation-picker-dialog__overlay) {
    background: var(--underlay-color-overlay-backdrop, rgba(0, 0, 0, 0.65));
  }

  :global(.relation-picker-dialog__content) {
    width: min(32rem, calc(100vw - 2rem));
    max-height: min(80vh, 40rem);
    display: flex;
    flex-direction: column;
  }

  :global(.relation-picker-dialog__content--create-mode) {
    width: min(calc(40em + 2rem), calc(100vw - 2rem));
    max-height: min(90vh, 50rem);
  }

  .relation-picker-dialog__topbar {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 0.75rem;
    padding-bottom: 0.75rem;
    flex-shrink: 0;
  }

  .relation-picker-dialog__header-extra {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-left: auto;
  }

  .relation-picker-dialog__search {
    padding-bottom: 0.75rem;
    flex-shrink: 0;
  }

  .relation-picker-dialog__body {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    min-height: 0;
  }

  .relation-picker-dialog__create-form {
    min-height: 0;
  }

  .relation-picker-dialog__empty {
    padding: 2rem 0;
    text-align: center;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.9rem;
  }

  .relation-picker-dialog__create {
    padding-top: 0.25rem;
    border-top: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.3));
  }
</style>
