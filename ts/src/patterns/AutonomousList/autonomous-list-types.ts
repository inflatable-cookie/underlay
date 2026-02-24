import type { Component, Snippet } from "svelte";
import type { SvelteComponent } from "svelte";
import type { BatchAction } from "../batch-actions.svelte";
import type { NavigationContext } from "../navigation-types";
import type { PageHeaderLevel, BreadcrumbItem } from "../types";
import type { PaginatedResponse, PaginationParams } from "../pagination-types";
import type { ReorderController, ReorderableItem } from "../reorder-controller.svelte";

/**
 * Filter field definition for the AutonomousList FilterBar.
 */
export interface ListFilterField {
  /** Unique key for this filter */
  key: string;
  /** Display label */
  label: string;
  /** Filter type */
  type: "text" | "select";
  /** Placeholder text */
  placeholder?: string;
  /** Options for select type */
  options?: Array<{ value: string; label: string }>;
  /** Whether to include an "All" option for selects (default: true) */
  includeAll?: boolean;
  /** Label for the "All" option (default: "All") */
  allLabel?: string;
  /** Default value */
  defaultValue?: string;
  /** Debounce time in ms for text inputs (default: 400) */
  debounce?: number;
}

/**
 * Reorder configuration.
 */
export interface ListReorderConfig<T = unknown> {
  /** Async function to persist the new order */
  execute: (orderedIds: string[], fetchFn: typeof fetch, token: string) => Promise<void>;
  /** When reordering is available (e.g., only when no filters active) */
  condition?: (filters: Record<string, unknown>) => boolean;
  /** Optional submit-error hook for conflict recovery and custom error copy. */
  onSubmitError?: (
    error: unknown,
    context: ListReorderSubmitErrorContext<T>
  ) => void | string | Promise<void | string>;
}

export interface ListReorderSubmitErrorContext<T> {
  controller: ReorderController<ReorderableItem>;
  latestItems: readonly T[];
}

/**
 * Server-side pagination fetcher signature.
 * Called with current filter values and pagination params.
 */
export type ServerFetcher<T> = (
  fetchFn: typeof fetch,
  token: string,
  params: PaginationParams,
  filters: Record<string, unknown>
) => Promise<PaginatedResponse<T>>;

/** Icon component — accepts both Svelte 5 and Svelte 4 (lucide-svelte) components */
type IconComponent =
  | Component<{ size?: number }>
  | (new (...args: any[]) => SvelteComponent);

/**
 * Props for the AutonomousList component.
 */
export interface AutonomousListProps<T> {
  /** Page/section title */
  title: string;
  /** Section label (shown above title in "page" variant) */
  section?: string;
  /** Heading level */
  level?: PageHeaderLevel;
  /** Breadcrumbs */
  breadcrumbs?: BreadcrumbItem[];

  /**
   * Display variant:
   * - "page": root page with section header (level defaults to 1)
   * - "tab": embedded in tabs/modals (level defaults to 3)
   */
  variant?: "page" | "tab";

  // ── Data fetching ──

  /**
   * Client-side data fetcher — returns all items at once.
   * Use this for small datasets where client-side filtering/pagination is fine.
   * Mutually exclusive with `serverFetcher`.
   */
  fetcher?: (fetchFn: typeof fetch, token: string, filters: Record<string, unknown>) => Promise<T[]>;

  /**
   * Server-side paginated fetcher — returns one page at a time.
   * Use this for large datasets where the server handles pagination.
   * Mutually exclusive with `fetcher`.
   */
  serverFetcher?: ServerFetcher<T>;

  /** Field name containing the item ID (default: "id") */
  idField?: string;

  /** Page size for pagination (default: 30) */
  pageSize?: number;

  /** LocalStorage key for persisting page size preference */
  persistKey?: string;

  // ── Filtering ──

  /** Filter field definitions */
  filters?: ListFilterField[];

  /** Custom filter bar snippet — replaces the default filter rendering */
  filterBar?: Snippet<[FilterBarContext]>;

  // ── Batch actions ──

  /** Batch action definitions */
  batchActions?: BatchAction<string>[];

  // ── Reordering ──

  /** Reorder configuration (omit for non-reorderable lists) */
  reorderable?: ListReorderConfig<T>;

  // ── Navigation ──

  /** URL for the "Add" button */
  addHref?: string;
  /** Label for the "Add" button */
  addLabel?: string;
  /** Navigation source context for child navigation */
  sourceContext?: NavigationContext;
  /** Additional header actions snippet */
  headerActions?: Snippet;

  // ── Empty state ──

  /** Message when list is empty */
  emptyMessage?: string;
  /** Icon for empty state */
  emptyIcon?: IconComponent;

  // ── Content rendering ──

  /** Snippet for rendering each item */
  item?: Snippet<[T, ListItemContext]>;
  /** Snippet for rendering each item in reorder mode */
  reorderItem?: Snippet<[T]>;

  // ── Callbacks ──

  /** Called when data changes (fetch, delete, reorder) — for parent coordination */
  onDataChange?: () => void;

  /** Additional CSS class */
  class?: string;
}

/**
 * Context passed to item snippets.
 */
export interface ListItemContext {
  /** Whether the item is currently selected */
  selected: boolean;
  /** Callback to toggle selection */
  onSelectionChange: (selected: boolean) => void;
  /** Whether selection mode is active */
  selectionMode: boolean;
}

/**
 * Context passed to custom filterBar snippets.
 */
export interface FilterBarContext {
  /** Current filter values */
  filters: Record<string, string>;
  /** Set a filter value (triggers refetch) */
  setFilter: (key: string, value: string | undefined) => void;
  /** Clear a specific filter */
  clearFilter: (key: string) => void;
  /** Clear all filters */
  clearAll: () => void;
  /** Refresh data with current filters */
  refresh: () => void;
}
