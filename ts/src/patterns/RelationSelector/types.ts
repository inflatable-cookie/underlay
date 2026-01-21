import type { Snippet } from "svelte";

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
 * Function signature for server-side search.
 * Implementations should debounce on the component side.
 */
export type RelationSearchFn<T extends SelectableRelation> = (
  query: string,
  options?: {
    limit?: number;
    offset?: number;
  }
) => Promise<SearchResult<T>>;

/**
 * Function signature for fetching suggestions.
 * Called once when the selector opens, not on every keystroke.
 */
export type RelationSuggestionsFn<T extends SelectableRelation> = () => Promise<T[]>;

/**
 * Props for the RelationSelector component.
 */
export interface RelationSelectorProps<T extends SelectableRelation> {
  // === Selection (single-select) ===
  /** Currently selected value ID (single-select mode) */
  value?: string | null;
  /** Callback when selection changes (single-select mode) */
  onchange?: (value: string | null) => void;

  // === Selection (multi-select) ===
  /** Currently selected value IDs (multi-select mode) */
  values?: string[];
  /** Callback when selection changes (multi-select mode) */
  onchangeMulti?: (values: string[]) => void;

  // === Mode ===
  /** Selection mode - single or multi */
  mode?: "single" | "multi";

  // === Data Fetching ===
  /** Server-side search function (required) */
  search: RelationSearchFn<T>;
  /** Optional function to fetch suggestions/recent items */
  suggestions?: RelationSuggestionsFn<T>;

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
}
