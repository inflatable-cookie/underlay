import { getContext, setContext } from "svelte";
import type {
  SelectableRelation,
  RelationSelectorProps,
  RelationSelectorState,
  SearchResult
} from "./types.js";

const RELATION_SELECTOR_CONTEXT_KEY = Symbol("relationSelector");

/**
 * Context value for RelationSelector components.
 * Provides shared state and actions.
 */
export interface RelationSelectorContext<T extends SelectableRelation> {
  // === Props (readonly) ===
  props: RelationSelectorProps<T>;

  // === State ===
  state: RelationSelectorState<T>;

  // === Derived values ===
  /** Currently selected item (single-select mode) */
  selectedItem: T | null;
  /** Currently selected items (multi-select mode) */
  selectedItems: T[];
  /** Whether in multi-select mode */
  isMultiSelect: boolean;

  // === Actions ===
  /** Open the popover for quick selection */
  openPopover: () => void;
  /** Close the popover */
  closePopover: () => void;
  /** Open the full modal (for create form) */
  openModal: () => void;
  /** Close the full modal */
  closeModal: () => void;
  /** Switch from popover to modal (e.g., when opening create form) */
  switchToModal: () => void;
  /** Set the search query */
  setSearchQuery: (query: string) => void;
  /** Clear the search query */
  clearSearch: () => void;
  /** Select an item (single-select: replaces, multi-select: toggles) */
  selectItem: (item: T) => void;
  /** Deselect an item (multi-select only) */
  deselectItem: (itemId: string) => void;
  /** Clear all selections */
  clearSelection: () => void;
  /** Toggle the create form */
  toggleCreateForm: () => void;
  /** Close the create form */
  closeCreateForm: () => void;
  /** Handle successful creation of a new item */
  handleCreateSuccess: (item: T) => void;
  /** Perform a search */
  performSearch: (query: string) => Promise<void>;
  /** Retry the last search */
  retrySearch: () => Promise<void>;
  /** Load suggestions */
  loadSuggestions: () => Promise<void>;
  /** Retry loading suggestions */
  retrySuggestions: () => Promise<void>;
  /** Check if an item is selected */
  isSelected: (itemId: string) => boolean;
}

/**
 * Creates a new RelationSelector context.
 * Should be called in the root RelationSelector component.
 */
export function createRelationSelectorContext<T extends SelectableRelation>(
  props: RelationSelectorProps<T>
): RelationSelectorContext<T> {
  // Initialize state with Svelte 5 runes
  let state = $state<RelationSelectorState<T>>({
    popoverOpen: false,
    modalOpen: false,
    searchQuery: "",
    isSearching: false,
    searchResults: [],
    searchTotal: 0,
    isSuggestionsLoading: false,
    suggestionItems: [],
    createFormOpen: false,
    searchError: null
  });

  // Track resolved items for selected values
  let resolvedItems = $state<Map<string, T>>(new Map());

  // Track last search query for retry
  let lastSearchQuery = $state<string>("");

  // Derived: is multi-select mode
  const isMultiSelect = $derived(props.mode === "multi");

  // Derived: selected item for single-select
  const selectedItem = $derived.by(() => {
    if (isMultiSelect || !props.value) return null;
    return resolvedItems.get(props.value) ?? null;
  });

  // Derived: selected items for multi-select
  const selectedItems = $derived.by(() => {
    if (!isMultiSelect || !props.values) return [];
    return props.values
      .map((id) => resolvedItems.get(id))
      .filter((item): item is T => item !== undefined);
  });

  // Load suggestions on mount if there's a pre-selected value
  // This ensures the trigger can display the correct label for pre-selected values
  $effect(() => {
    const hasValue = isMultiSelect
      ? (props.values ?? []).length > 0
      : !!props.value;

    // Load suggestions if we have a value but haven't loaded suggestions yet
    if (hasValue && props.suggestions && !state.isSuggestionsLoading && state.suggestionItems.length === 0) {
      void loadSuggestions();
    }
  });

  // === Actions ===

  function openPopover() {
    state.popoverOpen = true;
    // Load suggestions when popover opens
    if (props.suggestions && state.suggestionItems.length === 0) {
      void loadSuggestions();
    }
  }

  function closePopover() {
    state.popoverOpen = false;
    state.searchQuery = "";
    state.searchResults = [];
    state.searchError = null;
  }

  function openModal() {
    state.modalOpen = true;
    // Load suggestions when modal opens
    if (props.suggestions && state.suggestionItems.length === 0) {
      void loadSuggestions();
    }
  }

  function closeModal() {
    state.modalOpen = false;
    state.searchQuery = "";
    state.searchResults = [];
    state.searchError = null;
    state.createFormOpen = false;
  }

  function switchToModal() {
    // Close popover and open modal (used when switching to create form)
    state.popoverOpen = false;
    state.modalOpen = true;
    state.createFormOpen = true;
  }

  function setSearchQuery(query: string) {
    state.searchQuery = query;
  }

  function clearSearch() {
    state.searchQuery = "";
    state.searchResults = [];
    state.searchError = null;
  }

  function selectItem(item: T) {
    // Store the item for future reference - reassign map to trigger reactivity
    const newResolved = new Map(resolvedItems);
    newResolved.set(item.id, item);
    resolvedItems = newResolved;

    // Track selection in history (if provided)
    props.selectionHistory?.track(item.id);

    if (isMultiSelect) {
      // Multi-select: toggle
      const currentValues = props.values ?? [];
      if (currentValues.includes(item.id)) {
        // Already selected, remove it
        props.onchangeMulti?.(currentValues.filter((id) => id !== item.id));
      } else {
        // Not selected, add it
        props.onchangeMulti?.([...currentValues, item.id]);
      }
    } else {
      // Single-select: replace and close whichever is open
      props.onchange?.(item.id);
      if (state.popoverOpen) {
        closePopover();
      } else {
        closeModal();
      }
    }
  }

  function deselectItem(itemId: string) {
    if (!isMultiSelect) return;
    const currentValues = props.values ?? [];
    props.onchangeMulti?.(currentValues.filter((id) => id !== itemId));
  }

  function clearSelection() {
    if (isMultiSelect) {
      props.onchangeMulti?.([]);
    } else {
      props.onchange?.(null);
    }
  }

  function toggleCreateForm() {
    if (state.popoverOpen) {
      // If in popover, switch to modal with create form open
      switchToModal();
    } else {
      // Already in modal, just toggle the create form
      state.createFormOpen = !state.createFormOpen;
    }
  }

  function closeCreateForm() {
    state.createFormOpen = false;
  }

  function handleCreateSuccess(item: T) {
    // selectItem will store the item and trigger reactivity
    selectItem(item);
    closeCreateForm();
    // Notify parent
    props.onCreate?.(item);
    // Refresh suggestions so the new item appears in the list
    if (props.suggestions) {
      void loadSuggestions();
    }
  }

  async function performSearch(query: string): Promise<void> {
    if (!query.trim()) {
      state.searchResults = [];
      state.searchTotal = 0;
      state.searchError = null;
      return;
    }

    lastSearchQuery = query;
    state.isSearching = true;
    state.searchError = null;

    try {
      const result: SearchResult<T> = await props.search(query, {
        limit: 20,
        offset: 0
      });

      state.searchResults = result.items;
      state.searchTotal = result.total;

      // Store items for reference - reassign map to trigger reactivity
      const newResolved = new Map(resolvedItems);
      for (const item of result.items) {
        newResolved.set(item.id, item);
      }
      resolvedItems = newResolved;
    } catch (error) {
      state.searchError =
        error instanceof Error ? error.message : "Search failed";
      state.searchResults = [];
      state.searchTotal = 0;
    } finally {
      state.isSearching = false;
    }
  }

  async function retrySearch(): Promise<void> {
    if (lastSearchQuery) {
      await performSearch(lastSearchQuery);
    }
  }

  async function loadSuggestions(): Promise<void> {
    if (!props.suggestions) return;

    state.isSuggestionsLoading = true;

    try {
      // Get recent hints from selection history (if provided)
      const recentHints = props.selectionHistory?.getRecentIds();
      const items = await props.suggestions({ recentHints });
      state.suggestionItems = items;

      // Store items for reference - reassign map to trigger reactivity
      const newResolved = new Map(resolvedItems);
      for (const item of items) {
        newResolved.set(item.id, item);
      }
      resolvedItems = newResolved;
    } catch (error) {
      console.error("Failed to load suggestions:", error);
      state.suggestionItems = [];
    } finally {
      state.isSuggestionsLoading = false;
    }
  }

  async function retrySuggestions(): Promise<void> {
    await loadSuggestions();
  }

  function isSelected(itemId: string): boolean {
    if (isMultiSelect) {
      return props.values?.includes(itemId) ?? false;
    }
    return props.value === itemId;
  }

  const context: RelationSelectorContext<T> = {
    get props() {
      return props;
    },
    get state() {
      return state;
    },
    get selectedItem() {
      return selectedItem;
    },
    get selectedItems() {
      return selectedItems;
    },
    get isMultiSelect() {
      return isMultiSelect;
    },
    openPopover,
    closePopover,
    openModal,
    closeModal,
    switchToModal,
    setSearchQuery,
    clearSearch,
    selectItem,
    deselectItem,
    clearSelection,
    toggleCreateForm,
    closeCreateForm,
    handleCreateSuccess,
    performSearch,
    retrySearch,
    loadSuggestions,
    retrySuggestions,
    isSelected
  };

  setContext(RELATION_SELECTOR_CONTEXT_KEY, context);
  return context;
}

/**
 * Gets the RelationSelector context.
 * Must be called from within a RelationSelector component tree.
 */
export function useRelationSelector<
  T extends SelectableRelation
>(): RelationSelectorContext<T> {
  const context = getContext<RelationSelectorContext<T>>(
    RELATION_SELECTOR_CONTEXT_KEY
  );
  if (!context) {
    throw new Error(
      "useRelationSelector must be called within a RelationSelector component"
    );
  }
  return context;
}
