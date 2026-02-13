import type { SelectableRelation, SearchResult, FilterConfig } from "./types.js";

/**
 * A single item in a drill-down level.
 * Extends SelectableRelation with an optional child count.
 */
export interface DrillDownItem extends SelectableRelation {
  /** Number of children (shown as count badge) */
  count?: number;
}

/**
 * Context passed to drill-down level functions.
 * Maps each prior level's key to the selected item's ID.
 */
export type DrillDownContext = Record<string, string>;

/**
 * Search function for a drill-down level.
 * Receives the search query and selections from all prior levels.
 */
export type DrillDownSearchFn = (
  query: string,
  context: DrillDownContext
) => Promise<SearchResult<DrillDownItem>>;

/**
 * Suggestions function for a drill-down level.
 * Called once when the level is first displayed.
 * Receives selections from all prior levels.
 */
export type DrillDownSuggestionsFn = (
  context: DrillDownContext
) => Promise<DrillDownItem[]>;

/**
 * Configuration for one drill-down level.
 *
 * Each level represents one step in the hierarchy that the user
 * clicks through before reaching the final selection.
 */
export interface DrillDownLevel {
  /** Unique key — also used as the filter key injected into activeFilters */
  key: string;
  /** Display label for this level (shown in breadcrumbs and header) */
  label: string;
  /** Search function for this level */
  search: DrillDownSearchFn;
  /** Optional suggestions function (loaded when level first shown) */
  suggestions?: DrillDownSuggestionsFn;
  /** Optional search placeholder for this level */
  searchPlaceholder?: string;
  /** Optional filters to show at this level */
  filters?: FilterConfig[];
}

/**
 * Full drill-down configuration.
 *
 * The levels array defines the hierarchy steps the user clicks through.
 * The final selection level uses the existing RelationSelector search/suggestions/filters.
 *
 * @example
 * ```typescript
 * const drillDown: DrillDownConfig = {
 *   levels: [
 *     {
 *       key: "module",
 *       label: "Module",
 *       search: async (query, ctx) => searchModules(query),
 *       suggestions: async (ctx) => allModules
 *     },
 *     {
 *       key: "section",
 *       label: "Section",
 *       search: async (query, ctx) => searchSections(query, ctx.module),
 *       suggestions: async (ctx) => getSectionsForModule(ctx.module)
 *     }
 *   ]
 * };
 * ```
 */
export interface DrillDownConfig {
  /** The hierarchy levels, in order. Final selection uses the existing RelationSelector props. */
  levels: DrillDownLevel[];

  /**
   * Optional callback to compute filters for the final selection level
   * based on the drill-down context (all prior level selections).
   * When provided, overrides the RelationSelector `filters` prop at the final level.
   * This allows filter options to be scoped by drill-down selections.
   */
  finalLevelFilters?: (context: DrillDownContext) => FilterConfig[];
}

/**
 * Internal state for drill-down navigation.
 */
export interface DrillDownState {
  /** Current depth (0 = first level, levels.length = final selection level) */
  depth: number;
  /** Selections made at each drill-down level (level key -> selected item) */
  selections: Record<string, DrillDownItem>;
  /** Suggestion items for the current drill-down level */
  suggestionItems: DrillDownItem[];
  /** Whether suggestions are loading for the current level */
  suggestionsLoading: boolean;
  /** Search query for the current drill-down level */
  searchQuery: string;
  /** Search results for the current drill-down level */
  searchResults: DrillDownItem[];
  /** Total count from drill-down search */
  searchTotal: number;
  /** Whether drill-down search is in progress */
  searching: boolean;
  /** Search error for the current drill-down level */
  searchError: string | null;
  /** Active filters for the current drill-down level */
  activeFilters: Record<string, string | undefined>;
  /** Slide animation direction */
  slideDirection: "forward" | "back";
}

/** Breadcrumb entry for the drill-down trail */
export interface DrillDownBreadcrumb {
  /** Level key */
  key: string;
  /** Level display label */
  label: string;
  /** Selected item label at this level */
  itemLabel: string;
  /** Depth index for this breadcrumb */
  depth: number;
}
