/**
 * Local search helpers for RelationSelector drill-down levels.
 *
 * Creates search and suggest functions for drill-down levels
 * that filter client-side data, similar to createLocalSearchFns
 * but using the DrillDownContext interface.
 *
 * @example
 * ```typescript
 * import { createLocalDrillDownSearchFns } from '@decodelabs/underlay/patterns';
 *
 * const { search, suggest } = createLocalDrillDownSearchFns(
 *   () => sections,
 *   {
 *     toItem: (s) => ({ id: s.sectionId, label: s.label, description: s.title }),
 *     getSearchText: (s) => [s.label, s.title],
 *     applyContext: (items, ctx) =>
 *       ctx.module ? items.filter(s => s.moduleId === ctx.module) : items
 *   }
 * );
 * ```
 *
 * @module
 */

import type { SearchResult } from "./RelationSelector/types.js";
import type {
  DrillDownItem,
  DrillDownContext,
  DrillDownSearchFn,
  DrillDownSuggestionsFn
} from "./RelationSelector/drilldown-types.js";

/**
 * Options for createLocalDrillDownSearchFns.
 */
export interface LocalDrillDownSearchOptions<TItem> {
  /**
   * Convert a source item to a DrillDownItem.
   */
  toItem: (item: TItem) => DrillDownItem;

  /**
   * Extract searchable text from an item.
   * Returns an array of strings that will be searched (case-insensitive).
   */
  getSearchText: (item: TItem) => string[];

  /**
   * Optional function to filter items based on drill-down context.
   * Receives selections from all prior levels.
   *
   * @example
   * ```typescript
   * applyContext: (items, ctx) =>
   *   ctx.module ? items.filter(s => s.moduleId === ctx.module) : items
   * ```
   */
  applyContext?: (items: TItem[], context: DrillDownContext) => TItem[];

  /**
   * Maximum number of suggestions to return (default: all items).
   */
  maxSuggestions?: number;
}

/**
 * Return type for createLocalDrillDownSearchFns.
 */
export interface LocalDrillDownSearchFns {
  /** Search function compatible with DrillDownLevel's search prop */
  search: DrillDownSearchFn;
  /** Suggestions function compatible with DrillDownLevel's suggestions prop */
  suggest: DrillDownSuggestionsFn;
}

/**
 * Create local search and suggest functions for a drill-down level.
 *
 * @param getItems - Function that returns the current list of source items.
 * @param options - Configuration for searching and converting items.
 * @returns Object with search and suggest functions for the drill-down level.
 */
export function createLocalDrillDownSearchFns<TItem>(
  getItems: () => TItem[],
  options: LocalDrillDownSearchOptions<TItem>
): LocalDrillDownSearchFns {
  const { toItem, getSearchText, applyContext, maxSuggestions } = options;

  const search: DrillDownSearchFn = async (
    query: string,
    context: DrillDownContext
  ): Promise<SearchResult<DrillDownItem>> => {
    const items = getItems();
    const q = query.toLowerCase();

    // Apply context filter first
    const contextFiltered = applyContext ? applyContext(items, context) : items;

    // Then filter by search text
    const filtered = contextFiltered.filter((item) =>
      getSearchText(item).some((text) => text.toLowerCase().includes(q))
    );

    const drillDownItems = filtered.map(toItem);

    return {
      items: drillDownItems,
      total: drillDownItems.length
    };
  };

  const suggest: DrillDownSuggestionsFn = async (
    context: DrillDownContext
  ): Promise<DrillDownItem[]> => {
    const items = getItems();

    // Apply context filter
    const contextFiltered = applyContext ? applyContext(items, context) : items;

    // Limit if specified
    const limited = maxSuggestions
      ? contextFiltered.slice(0, maxSuggestions)
      : contextFiltered;

    return limited.map(toItem);
  };

  return { search, suggest };
}
