/**
 * Local search helpers for app-local selector shells over the retained
 * relation-selector helper layer.
 *
 * Creates search and suggest functions that filter client-side data,
 * perfect for local selector shells when all options are already loaded.
 *
 * @example
 * ```typescript
 * import { createLocalSearchFns } from '@decodelabs/underlay/patterns';
 *
 * const { search, suggest } = createLocalSearchFns(
 *   () => sections,
 *   {
 *     toSelectable: (s) => ({ id: s.sectionId, label: s.label, description: s.title }),
 *     getSearchText: (s) => [s.label, s.title, s.moduleName ?? ""]
 *   }
 * );
 *
 * <CategorySelector search={search} suggestions={suggest} />
 * ```
 *
 * @module
 */

import type {
  SelectableRelation,
  SearchResult,
  SearchOptions,
  SuggestionOptions,
  RelationSearchFn,
  RelationSuggestionsFn
} from "./RelationSelector/types.js";

/**
 * Options for createLocalSearchFns.
 */
export interface LocalSearchOptions<TItem, TSelectable extends SelectableRelation> {
  /**
   * Convert an item to a SelectableRelation.
   * This is called for each item that matches the search or is included in suggestions.
   */
  toSelectable: (item: TItem) => TSelectable;

  /**
   * Extract searchable text from an item.
   * Returns an array of strings that will be searched (case-insensitive).
   *
   * @example
   * ```typescript
   * getSearchText: (section) => [section.label, section.title, section.moduleName ?? ""]
   * ```
   */
  getSearchText: (item: TItem) => string[];

  /**
   * Optional function to apply filters to items.
   * Called before text search, receives the active filter values.
   *
   * @example
   * ```typescript
   * applyFilters: (items, filters) =>
   *   filters?.moduleId
   *     ? items.filter(s => s.moduleId === filters.moduleId)
   *     : items
   * ```
   */
  applyFilters?: (
    items: TItem[],
    filters: Record<string, string | undefined> | undefined
  ) => TItem[];

  /**
   * Maximum number of suggestions to return (default: all items).
   */
  maxSuggestions?: number;
}

/**
 * Return type for createLocalSearchFns.
 */
export interface LocalSearchFns<TSelectable extends SelectableRelation> {
  /**
   * Search function compatible with an app-local selector shell's `search` prop.
   * Filters items by query text (case-insensitive) and optional filters.
   */
  search: RelationSearchFn<TSelectable>;

  /**
   * Suggestions function compatible with an app-local selector shell's
   * `suggestions` prop.
   * Returns all items (optionally filtered and limited).
   */
  suggest: RelationSuggestionsFn<TSelectable>;
}

/**
 * Create local search and suggest functions for app-local selector shells.
 *
 * This is useful when you have all the data client-side and just need
 * to filter it based on user input. The functions returned are async
 * to match the retained helper-layer relation search interface.
 *
 * @param getItems - Function that returns the current list of items.
 *                   Using a getter allows the list to update reactively.
 * @param options - Configuration for searching and converting items.
 * @returns Object with search and suggest functions.
 *
 * @example
 * ```typescript
 * // Basic usage
 * const { search, suggest } = createLocalSearchFns(
 *   () => modules,
 *   {
 *     toSelectable: (m) => ({
 *       id: m.moduleId,
 *       label: `${m.pathwayName} ${m.code}`,
 *       description: m.title
 *     }),
 *     getSearchText: (m) => [m.code, m.title, m.pathwayName]
 *   }
 * );
 *
 * // With filters
 * const { search, suggest } = createLocalSearchFns(
 *   () => sections,
 *   {
 *     toSelectable: sectionToSelectable,
 *     getSearchText: (s) => [s.label, s.title, s.moduleName ?? ""],
 *     applyFilters: (items, filters) =>
 *       filters?.moduleId
 *         ? items.filter(s => s.moduleId === filters.moduleId)
 *         : items
 *   }
 * );
 * ```
 */
export function createLocalSearchFns<
  TItem,
  TSelectable extends SelectableRelation = SelectableRelation
>(
  getItems: () => TItem[],
  options: LocalSearchOptions<TItem, TSelectable>
): LocalSearchFns<TSelectable> {
  const { toSelectable, getSearchText, applyFilters, maxSuggestions } = options;

  const search: RelationSearchFn<TSelectable> = async (
    query: string,
    searchOptions?: SearchOptions
  ): Promise<SearchResult<TSelectable>> => {
    const items = getItems();
    const q = query.toLowerCase();

    // Apply filters first
    const filteredByFilters = applyFilters
      ? applyFilters(items, searchOptions?.filters)
      : items;

    // Then filter by search text
    const filtered = filteredByFilters.filter((item) =>
      getSearchText(item).some((text) => text.toLowerCase().includes(q))
    );

    // Convert to selectable format
    const selectables = filtered.map(toSelectable);

    return {
      items: selectables,
      total: selectables.length
    };
  };

  const suggest: RelationSuggestionsFn<TSelectable> = async (
    suggestionOptions?: SuggestionOptions
  ): Promise<TSelectable[]> => {
    const items = getItems();

    // Apply filters
    const filteredByFilters = applyFilters
      ? applyFilters(items, suggestionOptions?.filters)
      : items;

    // Limit if specified
    const limited = maxSuggestions
      ? filteredByFilters.slice(0, maxSuggestions)
      : filteredByFilters;

    return limited.map(toSelectable);
  };

  return { search, suggest };
}
