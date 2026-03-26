/**
 * List controller for autonomous list components.
 *
 * Provides unified state management for list components that:
 * - Fetch their own data based on filter props
 * - Manage loading/error states
 * - Support filter state with URL sync (optional)
 * - Integrate with batch selection
 *
 * This is the foundation for the Autonomous List Component pattern,
 * enabling list components to work both as standalone pages and
 * embedded tabs with identical behavior.
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { createListController } from '@decodelabs/underlay/patterns';
 *   import { authLoading, currentUser } from '$lib/stores/auth';
 *
 *   interface AreaFilters {
 *     moduleId?: string;
 *     sectionId?: string;
 *     search?: string;
 *   }
 *
 *   let { moduleId, sectionId } = $props<{ moduleId?: string; sectionId?: string }>();
 *
 *   const list = createListController<Area, AreaFilters>(
 *     async (fetch, token, filters) => {
 *       return await getAreas(fetch, token, filters);
 *     },
 *     {
 *       initialFilters: { moduleId, sectionId },
 *       getToken: () => auth.getToken()
 *     }
 *   );
 *
 *   // Trigger fetch when auth is ready
 *   $effect(() => {
 *     list.tryFetch($authLoading, $currentUser);
 *   });
 *
 *   // Re-fetch when filter props change
 *   $effect(() => {
 *     list.setFilters({ moduleId, sectionId });
 *   });
 * </script>
 *
 * {#if list.loading}
 *   <PageLoading />
 * {:else if list.error}
 *   <Callout tone="danger" message={list.error} announceMode="polite" />
 * {:else}
 *   {#each list.items as item}
 *     <ListCard title={item.title} />
 *   {/each}
 * {/if}
 * ```
 *
 * @module
 */

import { getAuthConfig } from "./auth";

/**
 * Options for creating a list controller.
 */
export interface ListControllerOptions<T, F extends Record<string, unknown>> {
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
   * Initial filter values.
   * These are merged with any filters passed to setFilters().
   */
  initialFilters?: Partial<F>;

  /**
   * Whether to fetch automatically when filters change (default: true).
   * Set to false if you want to control fetching manually.
   */
  autoFetchOnFilterChange?: boolean;

  /**
   * Callback after successful fetch.
   */
  onSuccess?: (items: T[]) => void;

  /**
   * Callback when an error occurs.
   */
  onError?: (error: Error) => void;

  /**
   * Callback when items change (after fetch or local update).
   * Useful for notifying parent components.
   */
  onItemsChange?: (items: T[]) => void;
}

/**
 * Result interface for the list controller.
 */
export interface ListControllerResult<T, F extends Record<string, unknown>> {
  /** Current items in the list (reactive) */
  readonly items: T[];

  /** Whether data is being fetched for the first time (no data yet) */
  readonly loading: boolean;

  /** Whether data is being refetched (data already exists) */
  readonly refetching: boolean;

  /** Error message if fetch failed */
  readonly error: string | null;

  /** Current filter values (reactive) */
  readonly filters: F;

  /** Whether data has been fetched at least once */
  readonly fetched: boolean;

  /**
   * Attempt to fetch data if auth is ready.
   * Call this inside an $effect with your auth stores as dependencies.
   * Only fetches once unless refresh() is called.
   *
   * @param authLoading - Whether auth is still initializing
   * @param currentUser - Current user object (null if not authenticated)
   */
  tryFetch: (authLoading: boolean, currentUser: unknown) => Promise<void>;

  /**
   * Force a refetch of the data with current filters.
   */
  refresh: () => Promise<void>;

  /**
   * Update one or more filter values.
   * If autoFetchOnFilterChange is true (default), triggers a refetch.
   *
   * @param newFilters - Partial filter values to merge
   */
  setFilters: (newFilters: Partial<F>) => void;

  /**
   * Set a single filter value.
   * Convenience method for updating one filter at a time.
   *
   * @param key - Filter key
   * @param value - New value
   */
  setFilter: <K extends keyof F>(key: K, value: F[K]) => void;

  /**
   * Reset filters to initial values and refetch.
   */
  resetFilters: () => Promise<void>;

  /**
   * Update items locally without refetching.
   * Useful for optimistic updates after mutations.
   *
   * @param updater - Function that receives current items and returns new items
   */
  updateItems: (updater: (items: T[]) => T[]) => void;

  /**
   * Remove an item locally by ID.
   * Useful for optimistic removal after delete.
   *
   * @param id - ID of item to remove
   * @param idField - Field name containing the ID (default: 'id')
   */
  removeItem: (id: string, idField?: string) => void;
}

/**
 * Creates a list controller for autonomous list components.
 *
 * @param fetcher - Async function that fetches items using fetch, token, and filters
 * @param options - Configuration options
 * @returns Reactive list controller with items, filters, and fetch methods
 */
export function createListController<T, F extends Record<string, unknown> = Record<string, unknown>>(
  fetcher: (fetchFn: typeof fetch, token: string, filters: F) => Promise<T[]>,
  options: ListControllerOptions<T, F> = {}
): ListControllerResult<T, F> {
  // Resolve auth config
  const globalConfig = getAuthConfig();
  const getToken = options.getToken ?? globalConfig?.getToken;
  const onRefresh = options.onRefresh ?? globalConfig?.onRefresh;

  if (!getToken) {
    throw new Error(
      "createListController: getToken is required. Either pass it in options or call configureAuth() at app startup."
    );
  }

  const autoFetchOnFilterChange = options.autoFetchOnFilterChange ?? true;

  // State
  let items = $state<T[]>([]);
  let loading = $state(true);
  let refetching = $state(false);
  let error = $state<string | null>(null);
  let filters = $state<F>((options.initialFilters ?? {}) as F);
  let _fetched = false;

  const doFetch = async (isRefetch = false) => {
    const token = getToken();
    if (!token) {
      loading = false;
      return;
    }

    // On initial load, show loading. On refetch, show refetching (keeps existing data visible)
    if (isRefetch) {
      refetching = true;
    } else {
      loading = true;
    }
    error = null;

    try {
      const result = await fetcher(fetch, token, filters);
      items = result;
      _fetched = true;
      options.onSuccess?.(result);
      options.onItemsChange?.(result);
    } catch (e) {
      // Check for 401 error and attempt refresh
      const is401 = e && typeof e === 'object' && 'status' in e && (e as { status: number }).status === 401;

      if (is401 && onRefresh) {
        const newToken = await onRefresh(fetch);
        if (newToken) {
          try {
            const result = await fetcher(fetch, newToken, filters);
            items = result;
            _fetched = true;
            options.onSuccess?.(result);
            options.onItemsChange?.(result);
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
      loading = false;
      refetching = false;
    }
  };

  const tryFetch = async (authLoading: boolean, currentUser: unknown) => {
    if (_fetched || authLoading || !currentUser) {
      return;
    }
    await doFetch(false);
  };

  const refresh = async () => {
    await doFetch(true);
  };

  const setFilters = (newFilters: Partial<F>) => {
    filters = { ...filters, ...newFilters };
    if (autoFetchOnFilterChange && _fetched) {
      doFetch(true);
    }
  };

  const setFilter = <K extends keyof F>(key: K, value: F[K]) => {
    setFilters({ [key]: value } as unknown as Partial<F>);
  };

  const resetFilters = async () => {
    filters = (options.initialFilters ?? {}) as F;
    await doFetch(true);
  };

  const updateItems = (updater: (items: T[]) => T[]) => {
    items = updater(items);
    options.onItemsChange?.(items);
  };

  const removeItem = (id: string, idField = 'id') => {
    items = items.filter((item) => {
      const itemId = (item as Record<string, unknown>)[idField];
      return itemId !== id;
    });
    options.onItemsChange?.(items);
  };

  return {
    get items() { return items; },
    get loading() { return loading; },
    get refetching() { return refetching; },
    get error() { return error; },
    get filters() { return filters; },
    get fetched() { return _fetched; },
    tryFetch,
    refresh,
    setFilters,
    setFilter,
    resetFilters,
    updateItems,
    removeItem
  };
}
