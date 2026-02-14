import { getAuthConfig } from "./auth";

/**
 * Hook for fetching auth-protected data in SvelteKit page components.
 *
 * This pattern solves the auth initialization race condition that occurs when:
 * - SvelteKit page load functions run before the layout's onMount
 * - Auth initialization happens in onMount, so tokens aren't available during load
 *
 * Instead of fetching in +page.ts (which races with auth init), use this hook
 * in the component. It waits for auth to be ready, then fetches data.
 *
 * ## Setup (once per app)
 *
 * Configure global auth handlers in your app's +layout.svelte:
 * ```svelte
 * <script>
 *   import { configureAuth } from '@decodelabs/underlay/patterns';
 *   import { auth } from '$lib/stores/auth';
 *
 *   configureAuth({
 *     getToken: () => auth.getToken(),
 *     onRefresh: auth.getRefreshHandler()
 *   });
 * </script>
 * ```
 *
 * ## Usage
 *
 * ```svelte
 * <script lang="ts">
 *   import { useAuthenticatedData } from '@decodelabs/underlay/patterns';
 *   import { authLoading, currentUser } from '$lib/stores/auth';
 *
 *   const pageData = useAuthenticatedData(
 *     async (fetch, token) => {
 *       const result = await someApiCall(fetch, token);
 *       return { items: result.items };
 *     },
 *     { defaultValue: { items: [] } }
 *   );
 *
 *   // Trigger fetch when auth is ready
 *   $effect(() => {
 *     pageData.tryFetch($authLoading, $currentUser);
 *   });
 * </script>
 *
 * {#if pageData.loading}
 *   <PageLoading />
 * {:else if pageData.error}
 *   <FormError message={pageData.error} />
 * {:else}
 *   {pageData.data.items.length} items
 * {/if}
 * ```
 *
 * For +page.ts files in protected routes, keep them simple:
 * ```typescript
 * export const load: PageLoad = async () => {
 *   return {}; // Auth protection handled by layout
 * };
 * ```
 */

export interface AuthenticatedDataOptions<T> {
  /**
   * Function to get the current access token synchronously.
   * Should return null if not authenticated.
   *
   * If not provided, uses the global auth config set via configureAuth().
   */
  getToken?: () => string | null;

  /**
   * Default value before data is fetched.
   * Used as initial value and when auth fails.
   */
  defaultValue?: T;

  /**
   * Callback after successful fetch.
   * Useful for post-load actions like handling URL parameters.
   */
  onSuccess?: (data: T) => void;

  /**
   * Optional refresh function to call on 401 errors.
   * Should attempt to refresh the token and return the new token,
   * or null if refresh failed.
   *
   * If not provided, uses the global auth config set via configureAuth().
   */
  onRefresh?: (fetchFn: typeof fetch) => Promise<string | null>;
}

export interface AuthenticatedDataResult<T> {
  /** The fetched data (or default value if not yet fetched) */
  readonly data: T | undefined;

  /** Whether data is being fetched for the first time (no data yet) */
  readonly loading: boolean;

  /** Whether data is being refetched (data already exists) */
  readonly refetching: boolean;

  /** Error message if fetch failed */
  readonly error: string | null;

  /**
   * Attempt to fetch data if auth is ready.
   * Call this inside an $effect with your auth stores as dependencies.
   * Only fetches once unless refetch() is called.
   *
   * @param authLoading - Whether auth is still initializing
   * @param currentUser - Current user object (null if not authenticated)
   */
  tryFetch: (authLoading: boolean, currentUser: unknown) => Promise<void>;

  /** Force a refetch of the data */
  refetch: () => Promise<void>;
}

/**
 * Creates a reactive data fetcher that waits for auth to be ready.
 *
 * @param fetcher - Async function that fetches data using fetch and auth token
 * @param options - Configuration options including getToken function
 * @returns Reactive object with data, loading, error states and fetch methods
 */
export function useAuthenticatedData<T>(
  fetcher: (fetchFn: typeof fetch, token: string) => Promise<T>,
  options: AuthenticatedDataOptions<T>
): AuthenticatedDataResult<T> {
  // Resolve getToken and onRefresh from options or global config
  const globalConfig = getAuthConfig();
  const getToken = options.getToken ?? globalConfig?.getToken;
  const onRefresh = options.onRefresh ?? globalConfig?.onRefresh;

  if (!getToken) {
    throw new Error(
      "useAuthenticatedData: getToken is required. Either pass it in options or call configureAuth() at app startup."
    );
  }

  let data = $state<T | undefined>(options.defaultValue);
  let loading = $state(true);
  let refetching = $state(false);
  let error = $state<string | null>(null);
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
      const result = await fetcher(fetch, token);
      data = result;
      options.onSuccess?.(result);
    } catch (e) {
      // Check if this is a 401 error and we have a refresh handler
      const is401 = e && typeof e === 'object' && 'status' in e && (e as { status: number }).status === 401;

      if (is401 && onRefresh) {
        // Attempt to refresh the token
        const newToken = await onRefresh(fetch);
        if (newToken) {
          // Retry the fetch with the new token
          try {
            const result = await fetcher(fetch, newToken);
            data = result;
            options.onSuccess?.(result);
            return;
          } catch (retryError) {
            error = retryError instanceof Error ? retryError.message : "Failed to load data";
          }
        } else {
          // Refresh failed - propagate original error
          error = e instanceof Error ? e.message : "Session expired";
        }
      } else {
        error = e instanceof Error ? e.message : "Failed to load data";
      }
    } finally {
      // Mark as fetched even on error to prevent tryFetch from auto-retrying.
      // Users can still explicitly call refetch() to retry.
      _fetched = true;
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

  const refetch = async () => {
    _fetched = false;
    await doFetch(true);
  };

  return {
    get data() {
      return data;
    },
    get loading() {
      return loading;
    },
    get refetching() {
      return refetching;
    },
    get error() {
      return error;
    },
    tryFetch,
    refetch
  };
}
