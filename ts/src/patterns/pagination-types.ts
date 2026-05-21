/**
 * Cursor-pagination types for lower-level runtime flows.
 *
 * These mirror the Rust cursor pagination types in `underlay-db`.
 * They are not the preferred contract for `EntityListPage`-class admin browse
 * surfaces, which should use page-shaped `page + limit` routes plus
 * `PagedListResponse<T>`.
 */

/**
 * Query parameters for cursor-paginated endpoints.
 */
export interface CursorPaginationParams {
  /** Number of items per page (default: 50, max: 100) */
  limit?: number;
  /** Opaque cursor string for pagination position */
  cursor?: string | null;
  /** Direction of traversal: "forward" or "backward" */
  direction?: "forward" | "backward";
  /** Whether to include total count (default: true) */
  includeTotal?: boolean;
}

/**
 * Response format for cursor-paginated endpoints.
 */
export interface CursorPaginatedResponse<T> {
  /** The items for the current page */
  data: T[];
  /** Cursor to fetch the next page, or null if no more pages */
  nextCursor: string | null;
  /** Cursor to fetch the previous page, or null if at the start */
  prevCursor: string | null;
  /** Whether there are more items after this page */
  hasMore: boolean;
  /** Total count of items (null if not requested or unavailable) */
  total: number | null;
}

/**
 * Unified pagination controller interface.
 *
 * Both server-side (cursor-based) and client-side pagination controllers
 * implement this interface, allowing UI components to work with either.
 */
export interface PaginationController<T> {
  // Current state
  /** Items for the current page */
  readonly items: T[];
  /** Current page number (1-indexed) */
  readonly currentPage: number;
  /** Number of items per page */
  readonly pageSize: number;
  /** Whether there is a next page */
  readonly hasNextPage: boolean;
  /** Whether there is a previous page */
  readonly hasPrevPage: boolean;
  /** Total count of items (null if not available) */
  readonly total: number | null;
  /** Whether data is currently being fetched */
  readonly loading: boolean;
  /** Error message if fetch failed */
  readonly error: string | null;

  // Computed values
  /** First item index on current page (1-indexed) */
  readonly showingFrom: number;
  /** Last item index on current page (1-indexed) */
  readonly showingTo: number;
  /** Total number of pages (null if total unknown) */
  readonly totalPages: number | null;

  // Actions
  /** Navigate to the next page */
  nextPage(): void | Promise<void>;
  /** Navigate to the previous page */
  prevPage(): void | Promise<void>;
  /** Navigate to a specific page (only available for client-side pagination) */
  goToPage?(page: number): void;
  /** Change the page size */
  setPageSize(size: number): void;
  /** Refresh the current page */
  refresh(): Promise<void>;
  /** Reset to page 1 and refetch (use when filters change) */
  reset(): Promise<void>;
}

/**
 * Default page size for paginated endpoints.
 */
export const DEFAULT_PAGE_SIZE = 30;

/**
 * Maximum page size allowed by the API.
 */
export const MAX_PAGE_SIZE = 100;

/**
 * Build query string parameters for pagination.
 */
export function buildCursorPaginationQuery(
  params: CursorPaginationParams
): Record<string, string> {
  const query: Record<string, string> = {};

  if (params.limit !== undefined) {
    query.limit = String(params.limit);
  }
  if (params.cursor) {
    query.cursor = params.cursor;
  }
  if (params.direction && params.direction !== "forward") {
    query.direction = params.direction;
  }
  if (params.includeTotal === false) {
    query.includeTotal = "false";
  }

  return query;
}

/**
 * Append pagination parameters to a URL or path.
 */
export function appendCursorPaginationParams(
  path: string,
  params: CursorPaginationParams
): string {
  const query = buildCursorPaginationQuery(params);
  const entries = Object.entries(query);

  if (entries.length === 0) {
    return path;
  }

  const queryString = entries
    .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(value)}`)
    .join("&");

  if (!path.includes("?")) {
    return `${path}?${queryString}`;
  }

  // Merge into existing query string to avoid duplicate keys
  const [basePath, existingQuery] = path.split("?", 2);
  const merged = new URLSearchParams(existingQuery);
  for (const [key, value] of entries) {
    merged.set(key, value);
  }
  return `${basePath}?${merged.toString()}`;
}
