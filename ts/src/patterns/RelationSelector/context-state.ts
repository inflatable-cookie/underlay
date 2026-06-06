import type { DrillDownState } from "./drilldown-types.js";
import { getInitialFilterValues } from "./context-filters.js";
import type {
  FilterConfig,
  RelationSelectorState,
  SelectableRelation,
} from "./types.js";

export function createInitialRelationSelectorState<
  T extends SelectableRelation,
>(
  filters: FilterConfig[] | undefined,
  drillDown: DrillDownState | null,
): RelationSelectorState<T> {
  return {
    popoverOpen: false,
    modalOpen: false,
    searchQuery: "",
    isSearching: false,
    searchResults: [],
    searchTotal: 0,
    isSuggestionsLoading: false,
    suggestionItems: [],
    createFormOpen: false,
    searchError: null,
    activeFilters: getInitialFilterValues(filters),
    drillDown,
  };
}
