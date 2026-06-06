import type {
  SelectableRelation,
  SearchResult,
  FilterConfig,
} from "./types.js";

export interface DrillDownItem extends SelectableRelation {
  count?: number;
}

export type DrillDownContext = Record<string, string>;

export type DrillDownSearchFn = (
  query: string,
  context: DrillDownContext,
) => Promise<SearchResult<DrillDownItem>>;

export type DrillDownSuggestionsFn = (
  context: DrillDownContext,
) => Promise<DrillDownItem[]>;

export interface DrillDownLevel {
  key: string;
  label: string;
  search: DrillDownSearchFn;
  suggestions?: DrillDownSuggestionsFn;
  searchPlaceholder?: string;
  filters?: FilterConfig[];
}

/**
 * Full drill-down configuration.
 *
 * The final selection level uses the existing RelationSelector search/suggestions/filters.
 */
export interface DrillDownConfig {
  levels: DrillDownLevel[];

  /**
   * Optional callback to compute filters for the final selection level
   * based on the drill-down context (all prior level selections).
   * When provided, overrides the RelationSelector `filters` prop at the final level.
   * This allows filter options to be scoped by drill-down selections.
   */
  finalLevelFilters?: (context: DrillDownContext) => FilterConfig[];
}

export interface DrillDownState {
  depth: number;
  selections: Record<string, DrillDownItem>;
  suggestionItems: DrillDownItem[];
  suggestionsLoading: boolean;
  searchQuery: string;
  searchResults: DrillDownItem[];
  searchTotal: number;
  searching: boolean;
  searchError: string | null;
  activeFilters: Record<string, string | undefined>;
  slideDirection: "forward" | "back";
}

export interface DrillDownBreadcrumb {
  key: string;
  label: string;
  itemLabel: string;
  depth: number;
}
