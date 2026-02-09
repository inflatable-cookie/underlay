<script lang="ts">
  import { tick } from "svelte";
  import type { Snippet } from "svelte";
  import { Dialog as BitsDialog } from "bits-ui";
  import Loader from "lucide-svelte/icons/loader-circle";
  import X from "lucide-svelte/icons/x";
  import RelationPickerCreateButton from "./relation-picker/RelationPickerCreateButton.svelte";
  import RelationPickerFooter from "./relation-picker/RelationPickerFooter.svelte";
  import RelationPickerList from "./relation-picker/RelationPickerList.svelte";
  import RelationPickerSearch from "./relation-picker/RelationPickerSearch.svelte";
  import type { PickableItem, PickerSection } from "./relation-picker-types.js";

  interface Props {
    /** Whether the dialog is open */
    open?: boolean;
    /** Dialog title */
    title: string;
    /** Message shown when no items available */
    emptyMessage?: string;
    /** Items to display (flat list mode) */
    items?: PickableItem[];
    /** Sections to display (sectioned mode - takes precedence over items) */
    sections?: PickerSection[];
    /** Currently selected IDs (for showing check marks) */
    selectedIds?: string[];
    /** Whether to show search input */
    searchable?: boolean;
    /** Placeholder for search input */
    searchPlaceholder?: string;
    /** Current search query (controlled mode) */
    searchQuery?: string;
    /** Whether search is in progress */
    searching?: boolean;
    /** Section label above the list - only used with flat items mode */
    sectionLabel?: string;
    /** Callback when an item is selected */
    onSelect?: (item: PickableItem) => void;
    /** Callback when dialog is closed without selection */
    onClose?: () => void;
    /** Callback when search query changes (for external search handling) */
    onSearch?: (query: string) => void;
    /** Custom rendering for items */
    renderItem?: Snippet<[item: PickableItem, selected: boolean]>;
    /** Header content (rendered after title, before search) */
    headerExtra?: Snippet;
    /** Footer content - if provided, shows a footer section */
    footer?: Snippet;

    // Error handling
    /** Error message to display */
    error?: string;
    /** Callback for retry button */
    onRetry?: () => void;

    // Clear selection
    /** Whether to show clear selection button in header */
    showClear?: boolean;
    /** Callback when clear is clicked */
    onClear?: () => void;

    // Create mode
    /** Whether to show create button */
    allowCreate?: boolean;
    /** Label for create button */
    createLabel?: string;
    /** Callback when create button is clicked */
    onCreate?: () => void;
    /** Whether the create form is currently open */
    createFormOpen?: boolean;
    /** The create form content */
    createForm?: Snippet<[onSuccess: (item: PickableItem) => void, onCancel: () => void]>;
    /** Callbacks for create form */
    onCreateSuccess?: (item: PickableItem) => void;
    onCreateCancel?: () => void;

    // Multi-select footer
    /** Whether this is a multi-select dialog with confirm/cancel */
    multiSelect?: boolean;
    /** Number of selected items (for confirm button label) */
    selectedCount?: number;
    /** Callback when confirm is clicked */
    onConfirm?: () => void;
    /** Callback when cancel is clicked */
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

  let searchInputRef: HTMLInputElement | null = $state(null);
  let listRef: HTMLUListElement | null = $state(null);
  let internalSearchQuery = $state("");
  let focusedIndex = $state(-1);

  // Use external search query if provided, otherwise internal
  const searchQuery = $derived(externalSearchQuery ?? internalSearchQuery);

  // Focus search input when dialog opens
  $effect(() => {
    if (open && searchable && !createFormOpen) {
      internalSearchQuery = "";
      focusedIndex = -1;
      void tick().then(() => {
        searchInputRef?.focus();
      });
    }
  });

  // Reset focused index when items change
  $effect(() => {
    if (items || sections) {
      focusedIndex = -1;
    }
  });

  // Get all items across all sections for keyboard navigation
  const allItems = $derived.by((): PickableItem[] => {
    if (sections && sections.length > 0) {
      return sections.flatMap((s: PickerSection) => s.items);
    }
    return items;
  });

  // Filter items based on search (only if no external search handler)
  const displayItems = $derived.by((): PickableItem[] => {
    if (onSearch || !searchQuery.trim()) return items;
    const query = searchQuery.toLowerCase();
    return items.filter(
      (item: PickableItem) =>
        item.label.toLowerCase().includes(query) ||
        item.description?.toLowerCase().includes(query)
    );
  });

  // Display sections (filtered if no external handler)
  const displaySections = $derived.by((): PickerSection[] | null => {
    if (!sections) return null;
    if (onSearch || !searchQuery.trim()) return sections;
    const query = searchQuery.toLowerCase();
    return sections.map((section: PickerSection) => ({
      ...section,
      items: section.items.filter(
        (item: PickableItem) =>
          item.label.toLowerCase().includes(query) ||
          item.description?.toLowerCase().includes(query)
      )
    })).filter((s: PickerSection) => s.items.length > 0);
  });

  function handleSearchInput(event: Event) {
    const value = (event.target as HTMLInputElement).value;
    internalSearchQuery = value;
    focusedIndex = -1;

    if (onSearch) {
      onSearch(value);
    }
  }

  function handleSearchKeyDown(event: KeyboardEvent) {
    const itemsList = displaySections
      ? displaySections.flatMap((s: PickerSection) => s.items)
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
      ? displaySections.flatMap((s: PickerSection) => s.items)
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
        searchInputRef?.focus();
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
      const items = listRef?.querySelectorAll<HTMLElement>('[role="option"]');
      if (items && items[index]) {
        items[index].focus();
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

  function handleOpenChange(newOpen: boolean) {
    if (!newOpen) {
      handleClose();
    }
  }

  // Get flat index within all sections for an item
  function getGlobalIndex(sectionIndex: number, itemIndex: number): number {
    if (!displaySections) return itemIndex;
    let globalIndex = 0;
    for (let i = 0; i < sectionIndex; i++) {
      globalIndex += displaySections[i]?.items.length ?? 0;
    }
    return globalIndex + itemIndex;
  }

  const hasItems = $derived(
    displaySections
      ? displaySections.some(s => s.items.length > 0)
      : displayItems.length > 0
  );

  const showEmpty = $derived(
    !searching && !hasItems && !createFormOpen
  );

  const showLoading = $derived(
    searching && !hasItems && !createFormOpen
  );

  const displayTitle = $derived(
    createFormOpen ? createLabel : title
  );
</script>

<BitsDialog.Root bind:open onOpenChange={handleOpenChange}>
  <BitsDialog.Portal>
    <BitsDialog.Overlay class="relation-picker-dialog__overlay" />

    <BitsDialog.Content class="relation-picker-dialog__content {createFormOpen ? 'relation-picker-dialog__content--create-mode' : ''}">
      <BitsDialog.Close class="relation-picker-dialog__close" aria-label="Close">
        <span aria-hidden="true">×</span>
      </BitsDialog.Close>

      <div class="relation-picker-dialog__header">
        <BitsDialog.Title class="relation-picker-dialog__title">
          {displayTitle}
        </BitsDialog.Title>
        {#if showClear && !createFormOpen}
          <button
            type="button"
            class="relation-picker-dialog__clear-btn"
            onclick={() => onClear?.()}
          >
            <X size="0.9em" strokeWidth={2.5} />
            <span>Clear</span>
          </button>
        {/if}
        {#if headerExtra && !createFormOpen}
          {@render headerExtra()}
        {/if}
      </div>

      {#if searchable && !createFormOpen}
        <RelationPickerSearch
          placeholder={searchPlaceholder}
          value={searchQuery}
          {searching}
          onInput={handleSearchInput}
          onKeyDown={handleSearchKeyDown}
          onInputRef={(input) => (searchInputRef = input)}
        />
      {/if}

      <div class="relation-picker-dialog__body">
        {#if createFormOpen && createForm}
          <!-- Create form mode -->
          <div class="relation-picker-dialog__create-form">
            {@render createForm(
              (item) => onCreateSuccess?.(item),
              () => onCreateCancel?.()
            )}
          </div>
        {:else}
          <!-- Selection mode -->
          {#if error}
            <div class="relation-picker-dialog__error">
              <span>{error}</span>
              {#if onRetry}
                <button
                  type="button"
                  class="relation-picker-dialog__error-retry"
                  onclick={onRetry}
                >
                  Retry
                </button>
              {/if}
            </div>
          {/if}

          {#if showLoading}
            <div class="relation-picker-dialog__loading">
              <Loader size="1.5em" class="relation-picker-dialog__loading-spinner" />
              <span>Loading...</span>
            </div>
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

          <RelationPickerCreateButton
            {allowCreate}
            {createFormOpen}
            {createForm}
            {createLabel}
            {onCreate}
          />
        {/if}
      </div>

      <RelationPickerFooter
        {footer}
        {multiSelect}
        {createFormOpen}
        {selectedCount}
        {onCancel}
        {onConfirm}
      />
    </BitsDialog.Content>
  </BitsDialog.Portal>
</BitsDialog.Root>

<style>
  :global(.relation-picker-dialog__overlay) {
    position: fixed;
    inset: 0;
    background: var(--underlay-color-overlay-backdrop, rgba(0, 0, 0, 0.65));
    z-index: 50;
  }

  :global(.relation-picker-dialog__content) {
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

  :global(.relation-picker-dialog__content--create-mode) {
    /* 40em form max-width + 2rem padding */
    width: min(calc(40em + 2rem), calc(100vw - 2rem));
    max-height: min(90vh, 50rem);
  }

  :global(.relation-picker-dialog__close) {
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

  :global(.relation-picker-dialog__close:hover) {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.2));
  }

  .relation-picker-dialog__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    padding: 1rem 1rem 0.75rem;
    padding-right: 2.5rem;
    flex-shrink: 0;
  }

  :global(.relation-picker-dialog__title) {
    margin: 0;
    font-size: 1.05rem;
    font-weight: 650;
  }

  .relation-picker-dialog__clear-btn {
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

  .relation-picker-dialog__clear-btn:hover {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.2));
    color: var(--underlay-color-danger, #ef4444);
  }

  .relation-picker-dialog__clear-btn:focus-visible {
    outline: 2px solid var(--underlay-color-primary, #2563eb);
    outline-offset: 1px;
  }

  @keyframes relation-picker-spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .relation-picker-dialog__body {
    flex: 1;
    overflow-y: auto;
    padding: 0 1rem 1rem;
  }

  .relation-picker-dialog__empty {
    padding: 2rem 1rem;
    text-align: center;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.9rem;
  }

  .relation-picker-dialog__loading {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 2rem 1rem;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.9rem;
  }

  :global(.relation-picker-dialog__loading-spinner) {
    animation: relation-picker-spin 1s linear infinite;
  }

  .relation-picker-dialog__error {
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

  .relation-picker-dialog__error-retry {
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

  .relation-picker-dialog__error-retry:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .relation-picker-dialog__error-retry:focus-visible {
    outline: 2px solid white;
    outline-offset: 2px;
  }

  .relation-picker-dialog__create-form {
    margin: 0.5rem 0 1rem;
  }

</style>
