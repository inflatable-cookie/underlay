import type {
  SelectableRelation,
  SearchResult,
  SearchOptions,
  SuggestionOptions,
  RelationSearchFn,
  RelationSuggestionsFn,
} from "./RelationSelector/types.js";

export interface LocalSearchOptions<
  TItem,
  TSelectable extends SelectableRelation,
> {
  toSelectable: (item: TItem) => TSelectable;

  getSearchText: (item: TItem) => string[];

  /**
   * Optional function to apply active filter values before text search.
   */
  applyFilters?: (
    items: TItem[],
    filters: Record<string, string | undefined> | undefined,
  ) => TItem[];

  maxSuggestions?: number;
}

export interface LocalSearchFns<TSelectable extends SelectableRelation> {
  search: RelationSearchFn<TSelectable>;
  suggest: RelationSuggestionsFn<TSelectable>;
}

/**
 * Create local search and suggest functions for app-local selector shells.
 *
 * Uses a getter so callers can expose reactive item lists.
 */
export function createLocalSearchFns<
  TItem,
  TSelectable extends SelectableRelation = SelectableRelation,
>(
  getItems: () => TItem[],
  options: LocalSearchOptions<TItem, TSelectable>,
): LocalSearchFns<TSelectable> {
  const { toSelectable, getSearchText, applyFilters, maxSuggestions } = options;

  const search: RelationSearchFn<TSelectable> = async (
    query: string,
    searchOptions?: SearchOptions,
  ): Promise<SearchResult<TSelectable>> => {
    const items = getItems();
    const q = query.toLowerCase();

    const filteredByFilters = applyFilters
      ? applyFilters(items, searchOptions?.filters)
      : items;

    const filtered = filteredByFilters.filter((item) =>
      getSearchText(item).some((text) => text.toLowerCase().includes(q)),
    );

    const selectables = filtered.map(toSelectable);

    return {
      items: selectables,
      total: selectables.length,
    };
  };

  const suggest: RelationSuggestionsFn<TSelectable> = async (
    suggestionOptions?: SuggestionOptions,
  ): Promise<TSelectable[]> => {
    const items = getItems();

    const filteredByFilters = applyFilters
      ? applyFilters(items, suggestionOptions?.filters)
      : items;

    const limited = maxSuggestions
      ? filteredByFilters.slice(0, maxSuggestions)
      : filteredByFilters;

    return limited.map(toSelectable);
  };

  return { search, suggest };
}
