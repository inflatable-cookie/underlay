import { setContext } from "svelte";
import type {
  SelectableRelation,
  RelationSelectorProps,
  SearchResult,
} from "./types.js";
import { createDrillDownContext } from "./drilldown-context.svelte.js";
import { RELATION_SELECTOR_CONTEXT_KEY } from "./context-key.js";
import type { RelationSelectorContext } from "./context-types.js";
import { createInitialRelationSelectorState } from "./context-state.js";
import {
  mergeResolvedItem,
  mergeResolvedItems,
  resolveSelectedItem,
  resolveSelectedItems,
  toggleSelectionValue,
} from "./context-selection.js";
import { createRelationSelectorUiActions } from "./context-ui-actions.js";
import { createRelationSelectorContextValue } from "./context-value.js";

export type { RelationSelectorContext } from "./context-types.js";
export { useRelationSelector } from "./context-use.js";

/**
 * Creates a new RelationSelector context.
 * Should be called in the root RelationSelector component.
 */
export function createRelationSelectorContext<T extends SelectableRelation>(
  props: RelationSelectorProps<T>,
): RelationSelectorContext<T> {
  const ddCtx = createDrillDownContext(() => props.drillDown);

  let state = $state(
    createInitialRelationSelectorState<T>(
      props.filters,
      props.drillDown ? ddCtx.state : null,
    ),
  );

  let resolvedItems = $state<Map<string, T>>(new Map());

  $effect(() => {
    const newResolved = mergeResolvedItem(
      resolvedItems,
      props.initialSelection,
    );
    if (newResolved) {
      resolvedItems = newResolved;
    }
  });

  $effect(() => {
    const newResolved = mergeResolvedItems(
      resolvedItems,
      props.initialSelections,
    );
    if (newResolved) {
      resolvedItems = newResolved;
    }
  });

  let lastSearchQuery = $state<string>("");

  const isMultiSelect = $derived(props.mode === "multi");

  const selectedItem = $derived.by(() => {
    if (isMultiSelect || !props.value) return null;
    return resolveSelectedItem(resolvedItems, props.value);
  });

  const selectedItems = $derived.by(() => {
    if (!isMultiSelect || !props.values) return [];
    return resolveSelectedItems(resolvedItems, props.values);
  });

  // When drill-down reaches the final level, load final-level suggestions
  let lastDrillDownDepth = 0;
  $effect(() => {
    const dd = state.drillDown;
    if (!dd || !props.drillDown) {
      lastDrillDownDepth = 0;
      return;
    }
    const atFinalLevel = dd.depth === props.drillDown.levels.length;
    const depthChanged = dd.depth !== lastDrillDownDepth;
    lastDrillDownDepth = dd.depth;
    if (
      atFinalLevel &&
      depthChanged &&
      state.popoverOpen &&
      props.suggestions
    ) {
      state.suggestionItems = [];
      void loadSuggestions();
    }
  });

  let hasLoadedInitialSuggestions = false;

  // Load suggestions on mount if there's a pre-selected value
  // This ensures the trigger can display the correct label for pre-selected values
  $effect(() => {
    // Only run once - don't keep retrying if suggestions returns empty
    if (hasLoadedInitialSuggestions) return;

    const hasValue = isMultiSelect
      ? (props.values ?? []).length > 0
      : !!props.value;

    // Load suggestions if we have a value but haven't loaded suggestions yet
    if (hasValue && props.suggestions && !state.isSuggestionsLoading) {
      hasLoadedInitialSuggestions = true;
      void loadSuggestions();
    }
  });

  const uiActions = createRelationSelectorUiActions({
    state,
    drillDown: ddCtx.actions,
    hasDrillDown: () => !!props.drillDown,
    hasSuggestions: () => !!props.suggestions,
    loadSuggestions: () => {
      void loadSuggestions();
    },
  });

  function selectItem(item: T) {
    const newResolved = new Map(resolvedItems);
    newResolved.set(item.id, item);
    resolvedItems = newResolved;

    props.selectionHistory?.track(item.id);

    if (isMultiSelect) {
      const currentValues = props.values ?? [];
      props.onchangeMulti?.(toggleSelectionValue(currentValues, item.id));
    } else {
      props.onchange?.(item.id);
      if (state.popoverOpen) {
        uiActions.closePopover();
      } else {
        uiActions.closeModal();
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

  function handleCreateSuccess(item: T) {
    selectItem(item);
    uiActions.closeCreateForm();
    props.onCreate?.(item);
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
      const mergedFilters = props.drillDown
        ? { ...ddCtx.actions.getDrillDownFilters(), ...state.activeFilters }
        : state.activeFilters;
      const result: SearchResult<T> = await props.search(query, {
        limit: 20,
        offset: 0,
        filters: mergedFilters,
      });

      state.searchResults = result.items;
      state.searchTotal = result.total;

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
      const recentHints = props.selectionHistory?.getRecentIds();
      const mergedFilters = props.drillDown
        ? { ...ddCtx.actions.getDrillDownFilters(), ...state.activeFilters }
        : state.activeFilters;
      const items = await props.suggestions({
        recentHints,
        filters: mergedFilters,
      });
      state.suggestionItems = items;

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

  function setFilter(filterKey: string, optionId: string | undefined): void {
    state.activeFilters = {
      ...state.activeFilters,
      [filterKey]: optionId,
    };
    state.searchResults = [];
    state.suggestionItems = [];
    if (state.searchQuery.trim()) {
      void performSearch(state.searchQuery);
    } else {
      void loadSuggestions();
    }
  }

  const context = createRelationSelectorContextValue<T>({
    props,
    state,
    getSelectedItem: () => selectedItem,
    getSelectedItems: () => selectedItems,
    getIsMultiSelect: () => isMultiSelect,
    actions: {
      ...uiActions,
      selectItem,
      deselectItem,
      clearSelection,
      handleCreateSuccess,
      performSearch,
      retrySearch,
      loadSuggestions,
      retrySuggestions,
      isSelected,
      setFilter,
    },
    drillDown: ddCtx.actions,
  });

  setContext(RELATION_SELECTOR_CONTEXT_KEY, context);
  return context;
}
