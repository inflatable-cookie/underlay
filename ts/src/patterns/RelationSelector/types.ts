import type { Snippet } from "svelte";
import type { SelectionHistory } from "../selection-history.js";

/**
 * A single option within a filter.
 */
export interface FilterOption {
  /** Unique identifier for this option */
  id: string;
  /** Display label */
  label: string;
}

/**
 * Configuration for a filter that can be applied to the selector.
 */
export interface FilterConfig {
  /** Unique key for this filter (used in activeFilters map) */
  key: string;
  /** Display label shown above the filter dropdown */
  label: string;
  /** Available filter options */
  options: FilterOption[];
  /** Default selected option ID (undefined = all items) */
  defaultValue?: string;
  /** Whether to include an "All" option at the start (default: true) */
  includeAll?: boolean;
  /** Custom label for the "All" option (default: "All") */
  allLabel?: string;
}

/**
 * Base interface for any selectable relation item.
 * Consumers should extend this with their own metadata.
 */
export interface SelectableRelation {
  /** Unique identifier for the item */
  id: string;
  /** Primary display text */
  label: string;
  /** Optional secondary text (shown below label) */
  description?: string | null;
  /** Whether this item cannot be selected */
  disabled?: boolean;
  /** Additional data for custom rendering */
  metadata?: Record<string, unknown>;
}

/**
 * Result from a server-side search operation.
 */
export interface SearchResult<T> {
  /** The matching items */
  items: T[];
  /** Total count of matches (for pagination info) */
  total: number;
}

/**
 * Options passed to the search function.
 */
export interface SearchOptions {
  /** Maximum number of results to return */
  limit?: number;
  /** Offset for pagination */
  offset?: number;
  /** Active filter values (filter key -> selected option id, undefined = all) */
  filters?: Record<string, string | undefined>;
}

/**
 * Function signature for server-side search.
 * Implementations should debounce on the component side.
 */
export type RelationSearchFn<T extends SelectableRelation> = (
  query: string,
  options?: SearchOptions
) => Promise<SearchResult<T>>;

/**
 * Options passed to the suggestions function.
 */
export interface SuggestionOptions {
  /** Recent selection IDs as hints for server-side ordering */
  recentHints?: string[];
  /** Active filter values (filter key -> selected option id, undefined = all) */
  filters?: Record<string, string | undefined>;
}

/**
 * Function signature for fetching suggestions.
 * Called once when the selector opens, not on every keystroke.
 *
 * @param options Optional hints for server-side suggestion ordering
 */
export type RelationSuggestionsFn<T extends SelectableRelation> = (
  options?: SuggestionOptions
) => Promise<T[]>;

/**
 * Props for the RelationSelector component.
 */
export interface RelationSelectorProps<T extends SelectableRelation> {
  // === Selection (single-select) ===
  /** Currently selected value ID (single-select mode) */
  value?: string | null;
  /** Callback when selection changes (single-select mode) */
  onchange?: (value: string | null) => void;
  /**
   * Initial selection data (single-select mode).
   * Use this to display the correct label when the suggestions list
   * hasn't loaded yet (e.g., when editing an existing record).
   */
  initialSelection?: T | null;

  // === Selection (multi-select) ===
  /** Currently selected value IDs (multi-select mode) */
  values?: string[];
  /** Callback when selection changes (multi-select mode) */
  onchangeMulti?: (values: string[]) => void;
  /**
   * Initial selections data (multi-select mode).
   * Use this to display correct labels when the suggestions list
   * hasn't loaded yet.
   */
  initialSelections?: T[];

  // === Mode ===
  /** Selection mode - single or multi */
  mode?: "single" | "multi";

  // === Data Fetching ===
  /** Server-side search function (required) */
  search: RelationSearchFn<T>;
  /** Optional function to fetch suggestions/recent items */
  suggestions?: RelationSuggestionsFn<T>;
  /**
   * Optional selection history tracker for tracking recent selections.
   * When provided:
   * - Selections are automatically tracked
   * - Recent IDs are passed as hints to the suggestions function
   */
  selectionHistory?: SelectionHistory;

  // === Filters ===
  /**
   * Optional filter configurations for narrowing down results.
   * Each filter appears as a dropdown in the selector UI.
   * Filter values are passed to search and suggestions functions.
   */
  filters?: FilterConfig[];

  // === Labels & Text ===
  /** Modal title, e.g., "Select Level" */
  label: string;
  /** Trigger button placeholder when nothing selected */
  placeholder?: string;
  /** Search input placeholder */
  searchPlaceholder?: string;
  /** Message shown when search returns no results */
  emptyMessage?: string;
  /** Label for suggestions section, e.g., "Recent", "Suggested" */
  suggestionsLabel?: string;

  // === State ===
  /** Whether the selector is disabled */
  disabled?: boolean;
  /** Whether a selection is required */
  required?: boolean;
  /** Error message to display */
  error?: string;

  // === Create Form Integration ===
  /** Whether to show "Add new" button */
  allowCreate?: boolean;
  /** Label for create button, e.g., "Add new level" */
  createLabel?: string;
  /** Callback when a new item is created */
  onCreate?: (item: T) => void;

  // === Customization Snippets ===
  /** Custom rendering for list items */
  renderItem?: Snippet<[item: T, selected: boolean]>;
  /** Custom rendering for trigger button */
  renderTrigger?: Snippet<[selected: T | T[] | null, open: () => void]>;
  /** Custom rendering for selected pills (multi-select) */
  renderSelectedPill?: Snippet<[item: T, remove: () => void]>;
  /** Create form content */
  createForm?: Snippet<[onSuccess: (item: T) => void, onCancel: () => void]>;
}

/**
 * Internal state for the RelationSelector.
 */
export interface RelationSelectorState<T extends SelectableRelation> {
  /** Whether the popover is open (for quick selection) */
  popoverOpen: boolean;
  /** Whether the full modal is open (for create form) */
  modalOpen: boolean;
  /** Current search query */
  searchQuery: string;
  /** Whether search is in progress */
  isSearching: boolean;
  /** Search results */
  searchResults: T[];
  /** Total count from search */
  searchTotal: number;
  /** Whether suggestions are loading */
  isSuggestionsLoading: boolean;
  /** Loaded suggestions */
  suggestionItems: T[];
  /** Whether the create form is expanded */
  createFormOpen: boolean;
  /** Search error message */
  searchError: string | null;
  /** Active filter values (filter key -> selected option id, undefined = all) */
  activeFilters: Record<string, string | undefined>;
}
