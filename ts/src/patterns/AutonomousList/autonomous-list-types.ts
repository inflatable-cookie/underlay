import type { Component, Snippet } from "svelte";
import type { BatchAction } from "../batch-actions.svelte";
import type { NavigationContext } from "../navigation-types";
import type { PageHeaderLevel, BreadcrumbItem } from "../types";

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
export interface ListReorderConfig {
  /** Async function to persist the new order */
  execute: (orderedIds: string[], fetchFn: typeof fetch, token: string) => Promise<void>;
  /** When reordering is available (e.g., only when no filters active) */
  condition?: (filters: Record<string, unknown>) => boolean;
}

/**
 * Props for the AutonomousList component.
 */
export interface AutonomousListProps<T> {
  /** Page/section title */
  title: string;
  /** Heading level */
  level?: PageHeaderLevel;
  /** Breadcrumbs */
  breadcrumbs?: BreadcrumbItem[];

  /** Data fetcher function */
  fetcher: (fetchFn: typeof fetch, token: string, filters: Record<string, unknown>) => Promise<T[]>;
  /** Field name containing the item ID (default: "id") */
  idField?: string;

  /** Filter field definitions */
  filters?: ListFilterField[];

  /** Batch action definitions */
  batchActions?: BatchAction<string>[];

  /** Reorder configuration (omit for non-reorderable lists) */
  reorderable?: ListReorderConfig;

  /** URL for the "Add" button */
  addHref?: string;
  /** Label for the "Add" button */
  addLabel?: string;

  /** Message when list is empty */
  emptyMessage?: string;
  /** Icon for empty state */
  emptyIcon?: Component<{ size?: number }>;

  /** Navigation source context for child navigation */
  sourceContext?: NavigationContext;

  /** Snippet for rendering each item */
  item?: Snippet<[T, ListItemContext]>;
  /** Snippet for rendering each item in reorder mode */
  reorderItem?: Snippet<[T]>;

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
