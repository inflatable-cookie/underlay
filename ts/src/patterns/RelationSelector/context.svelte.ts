import { onDestroy, setContext, untrack } from "svelte";
import type {
  SelectableRelation,
  RelationSelectorProps,
  SearchResult,
} from "./types.js";
import type { DrillDownContext } from "./drilldown-types.js";
import { createDrillDownContext } from "./drilldown-context.svelte.js";
import { RELATION_SELECTOR_CONTEXT_KEY } from "./context-key.js";
import { createInitialRelationSelectorState } from "./context-state.js";
import { getInitialFilterValues } from "./context-filters.js";
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
  drillDownNavigation: "context" | "picker" = "context",
) {
  const pickerOwnsNavigation = drillDownNavigation === "picker";
  const ddCtx = createDrillDownContext(() =>
    pickerOwnsNavigation ? undefined : props.drillDown,
  );

  const state = $state(
    createInitialRelationSelectorState<T>(
      props.filters,
      !pickerOwnsNavigation && props.drillDown ? ddCtx.state : null,
    ),
  );

  let resolvedItems = $state<Map<string, T>>(new Map());

  $effect(() => {
    const newResolved = mergeResolvedItem(
      untrack(() => resolvedItems),
      props.initialSelection,
    );
    if (newResolved) {
      resolvedItems = newResolved;
    }
  });

  $effect(() => {
    const newResolved = mergeResolvedItems(
      untrack(() => resolvedItems),
      props.initialSelections,
    );
    if (newResolved) {
      resolvedItems = newResolved;
    }
  });

  let lastSearchQuery = $state<string>("");
  let searchSeq = 0;
  let suggestionsSeq = 0;
  let pickerDrillContext = $state<DrillDownContext>({});

  // Supersede any in-flight search or suggestions request on unmount so a late
  // response never writes into torn-down state.
  onDestroy(() => {
    searchSeq += 1;
    suggestionsSeq += 1;
  });

  const pickerAtFinalLevel = $derived(
    props.drillDown?.levels.every(({ key }) => Object.hasOwn(pickerDrillContext, key)) ?? true,
  );
  const pickerFilters = $derived(
    (pickerAtFinalLevel && props.drillDown?.finalLevelFilters?.(pickerDrillContext)) ||
      props.filters,
  );

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
      void untrack(() => loadSuggestions(getBrowseFilters(), true));
    }
  });

  const uiActions = createRelationSelectorUiActions({
    state,
    drillDown: ddCtx.actions,
    hasDrillDown: () => !pickerOwnsNavigation && !!props.drillDown,
    hasSuggestions: () => !pickerOwnsNavigation && !!props.suggestions,
    loadSuggestions: () => {
      void loadSuggestions();
    },
  });

  if (pickerOwnsNavigation) {
    $effect(() => {
      if (!state.popoverOpen || !pickerAtFinalLevel) return;

      const query = state.searchQuery;
      const filters = getBrowseFilters();
      const searchByDefault = props.drillDown && !props.suggestions;
      if (!query.trim()) {
        untrack(() => {
          if (searchByDefault) {
            void performSearch(query, filters);
          } else if (!state.suggestionItems.length && !state.isSuggestionsLoading) {
            void loadSuggestions(filters);
          }
        });
        return;
      }

      const timer = setTimeout(() => void performSearch(query, filters), 250);
      return () => clearTimeout(timer);
    });
  }

  function getBrowseFilters() {
    if (!props.drillDown) return state.activeFilters;
    const drillFilters = pickerOwnsNavigation
      ? pickerDrillContext
      : ddCtx.actions.getDrillDownFilters();
    return { ...drillFilters, ...state.activeFilters };
  }

  function resetSearch(): void {
    searchSeq += 1;
    state.searchResults = [];
    state.searchTotal = 0;
    state.searchError = null;
    state.isSearching = false;
  }

  function resetSuggestions(): void {
    suggestionsSeq += 1;
    state.suggestionItems = [];
    state.suggestionsError = null;
    state.isSuggestionsLoading = false;
  }

  function setPickerDrillContext(nextContext: DrillDownContext): void {
    const currentContext = untrack(() => pickerDrillContext);
    const keys = Object.keys(nextContext);
    if (
      keys.length === Object.keys(currentContext).length &&
      keys.every((key) => nextContext[key] === currentContext[key])
    ) return;

    pickerDrillContext = nextContext;
    state.activeFilters = untrack(() => getInitialFilterValues(pickerFilters));
    state.searchQuery = "";
    resetSearch();
    resetSuggestions();
  }

  function setSearchQuery(query: string): void {
    if (pickerOwnsNavigation && query !== state.searchQuery) resetSearch();
    uiActions.setSearchQuery(query);
  }

  function closePopover(): void {
    uiActions.closePopover();
    if (pickerOwnsNavigation) {
      setPickerDrillContext({});
      resetSearch();
    }
  }

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
        closePopover();
      } else {
        uiActions.closeModal();
      }
    }
  }

  function setMultiSelection(nextIds: string[]): void {
    const previousIds = new Set(props.values);
    for (const id of nextIds) {
      if (!previousIds.has(id) && resolvedItems.has(id)) {
        props.selectionHistory?.track(id);
        previousIds.add(id);
      }
    }
    props.onchangeMulti?.(nextIds);
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

  async function performSearch(query: string, filters = getBrowseFilters()): Promise<void> {
    if (!query.trim() && !(pickerOwnsNavigation && props.drillDown && pickerAtFinalLevel)) {
      resetSearch();
      return;
    }

    const seq = ++searchSeq;
    lastSearchQuery = query;
    state.isSearching = true;
    state.searchError = null;

    try {
      const result: SearchResult<T> = await props.search(query, {
        limit: 20,
        offset: 0,
        filters,
      });

      if (seq !== searchSeq) return;

      state.searchResults = result.items;
      state.searchTotal = result.total;

      const newResolved = new Map(resolvedItems);
      for (const item of result.items) {
        newResolved.set(item.id, item);
      }
      resolvedItems = newResolved;
    } catch (error) {
      if (seq !== searchSeq) return;
      state.searchError =
        error instanceof Error ? error.message : "Search failed";
      state.searchResults = [];
      state.searchTotal = 0;
    } finally {
      if (seq === searchSeq) {
        state.isSearching = false;
      }
    }
  }

  async function retrySearch(): Promise<void> {
    if (lastSearchQuery) {
      await performSearch(lastSearchQuery);
    }
  }

  async function loadSuggestions(filters = getBrowseFilters(), hydrateSelection = false): Promise<void> {
    if (!props.suggestions) return;

    const seq = ++suggestionsSeq;
    state.isSuggestionsLoading = true;
    state.suggestionsError = null;

    try {
      const recentHints = props.selectionHistory?.getRecentIds();
      const items = await props.suggestions({
        recentHints,
        filters,
      });

      if (seq !== suggestionsSeq) {
        // Entering a drill level can supersede the browse list while the
        // initial suggestions are still needed to label the selected IDs.
        if (hydrateSelection) {
          const newResolved = mergeResolvedItems(resolvedItems, items.filter(({ id }) => isSelected(id)));
          if (newResolved) resolvedItems = newResolved;
        }
        return;
      }

      state.suggestionItems = items;

      const newResolved = new Map(resolvedItems);
      for (const item of items) {
        newResolved.set(item.id, item);
      }
      resolvedItems = newResolved;
    } catch (error) {
      if (seq !== suggestionsSeq) return;
      state.suggestionsError =
        error instanceof Error ? error.message : "Failed to load suggestions";
      state.suggestionItems = [];
    } finally {
      if (seq === suggestionsSeq) {
        state.isSuggestionsLoading = false;
      }
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
    if (pickerOwnsNavigation) {
      resetSearch();
      resetSuggestions();
      return;
    }
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
      closePopover,
      setSearchQuery,
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

  return setContext(RELATION_SELECTOR_CONTEXT_KEY, Object.assign(context, {
    getResolvedItem: (id: string) => resolvedItems.get(id),
    getPickerFilters: () => pickerFilters,
    isPickerAtFinalLevel: () => pickerAtFinalLevel,
    setPickerDrillContext,
    setMultiSelection,
  }));
}
