/**
 * Pagination controllers for Svelte 5.
 *
 * Provides unified pagination state management for both server-side
 * (cursor-based) and client-side pagination with the same interface.
 */

import { getAuthConfig } from "./auth";
import type {
  PaginatedResponse,
  PaginationController,
  PaginationParams
} from "./pagination-types";
import { DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE } from "./pagination-types";

// ============================================================================
// Server-Side Pagination Controller
// ============================================================================

export interface ServerPaginationOptions<T> {
  /**
   * Function to get the current access token synchronously.
   * If not provided, uses the global auth config set via configureAuth().
   */
  getToken?: () => string | null;

  /**
   * Optional refresh function to call on 401 errors.
   * If not provided, uses the global auth config set via configureAuth().
   */
  onRefresh?: (fetchFn: typeof fetch) => Promise<string | null>;

  /**
   * Initial page size (default: 30).
   */
  pageSize?: number;

  /**
   * Whether to include total count in API requests (default: true).
   */
  includeTotal?: boolean;

  /**
   * Key for persisting page size preference in localStorage.
   * If provided, the user's page size choice will be remembered.
   * Example: "pagination:summaries" or "pagination:modules"
   */
  persistKey?: string;

  /**
   * Callback after successful fetch.
   */
  onSuccess?: (response: PaginatedResponse<T>) => void;

  /**
   * Callback when an error occurs.
   */
  onError?: (error: Error) => void;
}

export interface ServerPaginationResult<T> extends PaginationController<T> {
  /**
   * Attempt to fetch the first page if auth is ready.
   * Call this inside an $effect with your auth stores as dependencies.
   * Only fetches once unless refresh() is called.
   *
   * @param authLoading - Whether auth is still initializing
   * @param currentUser - Current user object (null if not authenticated)
   */
  tryFetch: (authLoading: boolean, currentUser: unknown) => Promise<void>;
}

/**
 * Creates a server-side pagination controller that manages cursor-based navigation.
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { createPaginationController } from '@decodelabs/underlay/patterns';
 *
 *   const pagination = createPaginationController(
 *     async (fetch, token, params) => {
 *       return await getBundleActivitiesPaginated(bundleId, fetch, token, params);
 *     },
 *     { pageSize: 25 }
 *   );
 *
 *   $effect(() => {
 *     pagination.tryFetch($authLoading, $currentUser);
 *   });
 * </script>
 *
 * {#each pagination.items as item}
 *   <div>{item.name}</div>
 * {/each}
 *
 * <Pagination controller={pagination} />
 * ```
 */
export function createPaginationController<T>(
  fetcher: (
    fetchFn: typeof fetch,
    token: string,
    params: PaginationParams
  ) => Promise<PaginatedResponse<T>>,
  options: ServerPaginationOptions<T> = {}
): ServerPaginationResult<T> {
  // Resolve auth config
  const globalConfig = getAuthConfig();
  const getToken = options.getToken ?? globalConfig?.getToken;
  const onRefresh = options.onRefresh ?? globalConfig?.onRefresh;

  if (!getToken) {
    throw new Error(
      "createPaginationController: getToken is required. Either pass it in options or call configureAuth() at app startup."
    );
  }

  // Helper to get initial page size (checks localStorage if persistKey provided)
  const getInitialPageSize = (): number => {
    if (options.persistKey && typeof localStorage !== "undefined") {
      const stored = localStorage.getItem(options.persistKey);
      if (stored) {
        const parsed = parseInt(stored, 10);
        if (!isNaN(parsed) && parsed > 0 && parsed <= MAX_PAGE_SIZE) {
          return parsed;
        }
      }
    }
    return options.pageSize ?? DEFAULT_PAGE_SIZE;
  };

  // State
  let items = $state<T[]>([]);
  let currentPage = $state(1);
  let pageSize = $state(Math.min(getInitialPageSize(), MAX_PAGE_SIZE));
  let loading = $state(true);
  let error = $state<string | null>(null);
  let total = $state<number | null>(null);

  // Cursor state
  let nextCursor = $state<string | null>(null);
  let prevCursor = $state<string | null>(null);
  let hasMore = $state(false);

  // Cursor history for backward navigation
  let cursorHistory: string[] = [];
  let _fetched = false;
  let _inFlight: Promise<void> | null = null;

  // Derived values
  const hasNextPage = $derived(hasMore || nextCursor !== null);
  const hasPrevPage = $derived(currentPage > 1);
  const showingFrom = $derived(items.length > 0 ? (currentPage - 1) * pageSize + 1 : 0);
  const showingTo = $derived(showingFrom > 0 ? showingFrom + items.length - 1 : 0);
  const totalPages = $derived(total !== null ? Math.ceil(total / pageSize) : null);

  const doFetch = async (cursor: string | null = null, direction: "forward" | "backward" = "forward") => {
    if (_inFlight) {
      return _inFlight;
    }

    const run = (async () => {
    const token = getToken();
    if (!token) {
      loading = false;
      return;
    }

    loading = true;
    error = null;

    const params: PaginationParams = {
      limit: pageSize,
      cursor,
      direction,
      includeTotal: options.includeTotal ?? true
    };

    try {
      const response = await fetcher(fetch, token, params);

      items = response.data;
      nextCursor = response.nextCursor;
      prevCursor = response.prevCursor;
      hasMore = response.hasMore;

      if (response.total !== null) {
        total = response.total;
      }

      options.onSuccess?.(response);
    } catch (e) {
      // Check for 401 error
      const is401 = e && typeof e === 'object' && 'status' in e && (e as { status: number }).status === 401;

      if (is401 && onRefresh) {
        const newToken = await onRefresh(fetch);
        if (newToken) {
          try {
            const response = await fetcher(fetch, newToken, params);
            items = response.data;
            nextCursor = response.nextCursor;
            prevCursor = response.prevCursor;
            hasMore = response.hasMore;
            if (response.total !== null) {
              total = response.total;
            }
            options.onSuccess?.(response);
            return;
          } catch (retryError) {
            const err = retryError instanceof Error ? retryError : new Error("Failed to load data");
            error = err.message;
            options.onError?.(err);
          }
        } else {
          const err = new Error("Session expired");
          error = err.message;
          options.onError?.(err);
        }
      } else {
        const err = e instanceof Error ? e : new Error("Failed to load data");
        error = err.message;
        options.onError?.(err);
      }
    } finally {
      // Mark as fetched even on error to prevent tryFetch from auto-retrying.
      // Users can still explicitly call refetch()/refresh()/reset() to retry.
      _fetched = true;
      loading = false;
    }
    })();

    _inFlight = run;
    try {
      await run;
    } finally {
      _inFlight = null;
    }
  };

  const tryFetch = async (authLoading: boolean, currentUser: unknown) => {
    if (_fetched || authLoading || !currentUser) {
      return;
    }
    await doFetch(null, "forward");
  };

  const nextPage = async () => {
    if (!hasNextPage || loading) return;

    // Save current cursor for backward navigation
    if (nextCursor) {
      cursorHistory.push(prevCursor ?? "");
    }

    currentPage++;
    await doFetch(nextCursor, "forward");
  };

  const prevPage = async () => {
    if (!hasPrevPage || loading) return;

    currentPage--;

    // Use cursor history for backward navigation
    const historyCursor = cursorHistory.pop();
    if (historyCursor !== undefined) {
      await doFetch(historyCursor || null, "forward");
    } else if (prevCursor) {
      await doFetch(prevCursor, "backward");
    } else {
      // Fallback: go back to first page
      cursorHistory = [];
      await doFetch(null, "forward");
    }
  };

  const setPageSize = (size: number) => {
    const newSize = Math.min(Math.max(1, size), MAX_PAGE_SIZE);
    if (newSize === pageSize) return;

    pageSize = newSize;
    currentPage = 1;
    cursorHistory = [];

    // Persist to localStorage if key provided
    if (options.persistKey && typeof localStorage !== "undefined") {
      localStorage.setItem(options.persistKey, String(newSize));
    }

    doFetch(null, "forward");
  };

  const refresh = async () => {
    currentPage = 1;
    cursorHistory = [];
    _fetched = false;
    await doFetch(null, "forward");
  };

  // reset() is an alias for refresh() - use when filters change
  const reset = refresh;

  return {
    get items() { return items; },
    get currentPage() { return currentPage; },
    get pageSize() { return pageSize; },
    get hasNextPage() { return hasNextPage; },
    get hasPrevPage() { return hasPrevPage; },
    get total() { return total; },
    get loading() { return loading; },
    get error() { return error; },
    get showingFrom() { return showingFrom; },
    get showingTo() { return showingTo; },
    get totalPages() { return totalPages; },
    tryFetch,
    nextPage,
    prevPage,
    setPageSize,
    refresh,
    reset
  };
}

// ============================================================================
// Client-Side Pagination Controller
// ============================================================================

export interface ClientPaginationOptions {
  /**
   * Initial page size (default: 30).
   */
  pageSize?: number;

  /**
   * Initial page number (1-indexed, default: 1).
   */
  initialPage?: number;

  /**
   * Key for persisting page size preference in localStorage.
   * If provided, the user's page size choice will be remembered.
   */
  persistKey?: string;
}

/**
 * Creates a client-side pagination controller for pre-loaded data.
 *
 * Use this when you already have all items loaded and want to paginate locally.
 * Provides instant page switching and supports random page access via goToPage().
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { createClientPagination } from '@decodelabs/underlay/patterns';
 *
 *   let { activities } = $props<{ activities: Activity[] }>();
 *
 *   const pagination = createClientPagination(() => activities, { pageSize: 25 });
 * </script>
 *
 * {#each pagination.items as item}
 *   <div>{item.name}</div>
 * {/each}
 *
 * <Pagination controller={pagination} />
 * ```
 */
export function createClientPagination<T>(
  getAllItems: () => T[],
  options: ClientPaginationOptions = {}
): PaginationController<T> {
  // Helper to get initial page size (checks localStorage if persistKey provided)
  const getInitialPageSize = (): number => {
    if (options.persistKey && typeof localStorage !== "undefined") {
      const stored = localStorage.getItem(options.persistKey);
      if (stored) {
        const parsed = parseInt(stored, 10);
        if (!isNaN(parsed) && parsed > 0 && parsed <= MAX_PAGE_SIZE) {
          return parsed;
        }
      }
    }
    return options.pageSize ?? DEFAULT_PAGE_SIZE;
  };

  let currentPage = $state(options.initialPage ?? 1);
  let pageSize = $state(Math.min(getInitialPageSize(), MAX_PAGE_SIZE));

  // Derived from source array
  const allItems = $derived(getAllItems());
  const total = $derived(allItems.length);
  const totalPages = $derived(Math.max(1, Math.ceil(total / pageSize)));

  // Ensure current page is valid when data changes
  const validPage = $derived(Math.min(Math.max(1, currentPage), totalPages));

  // Slice items for current page
  const items = $derived.by(() => {
    const start = (validPage - 1) * pageSize;
    const end = start + pageSize;
    return allItems.slice(start, end);
  });

  // Navigation state
  const hasNextPage = $derived(validPage < totalPages);
  const hasPrevPage = $derived(validPage > 1);
  const showingFrom = $derived(total > 0 ? (validPage - 1) * pageSize + 1 : 0);
  const showingTo = $derived(showingFrom > 0 ? Math.min(showingFrom + pageSize - 1, total) : 0);

  const nextPage = () => {
    if (hasNextPage) {
      currentPage = validPage + 1;
    }
  };

  const prevPage = () => {
    if (hasPrevPage) {
      currentPage = validPage - 1;
    }
  };

  const goToPage = (page: number) => {
    currentPage = Math.min(Math.max(1, page), totalPages);
  };

  const setPageSize = (size: number) => {
    const newSize = Math.min(Math.max(1, size), MAX_PAGE_SIZE);
    if (newSize === pageSize) return;

    // Adjust page to keep roughly the same position
    const firstItemIndex = (validPage - 1) * pageSize;
    pageSize = newSize;
    currentPage = Math.floor(firstItemIndex / newSize) + 1;

    // Persist to localStorage if key provided
    if (options.persistKey && typeof localStorage !== "undefined") {
      localStorage.setItem(options.persistKey, String(newSize));
    }
  };

  const refresh = async () => {
    // For client-side, refresh is a no-op since data is already loaded
    // But we keep the method for interface compatibility
  };

  const reset = async () => {
    // For client-side, reset goes back to page 1
    currentPage = 1;
  };

  return {
    get items() { return items; },
    get currentPage() { return validPage; },
    get pageSize() { return pageSize; },
    get hasNextPage() { return hasNextPage; },
    get hasPrevPage() { return hasPrevPage; },
    get total() { return total; },
    get loading() { return false; }, // Client-side is never loading
    get error() { return null; }, // Client-side has no errors
    get showingFrom() { return showingFrom; },
    get showingTo() { return showingTo; },
    get totalPages() { return totalPages; },
    nextPage,
    prevPage,
    goToPage,
    setPageSize,
    refresh,
    reset
  };
}
