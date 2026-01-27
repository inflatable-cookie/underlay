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
 * Usage:
 * ```svelte
 * <script lang="ts">
 *   import { useAuthenticatedData } from '@decodelabs/underlay/patterns';
 *   import { authLoading, currentUser, auth } from '$lib/stores/auth';
 *
 *   const pageData = useAuthenticatedData(
 *     async (fetch, token) => {
 *       const result = await someApiCall(fetch, token);
 *       return { items: result.items };
 *     },
 *     {
 *       getToken: () => auth.getToken(),
 *       defaultValue: { items: [] }
 *     }
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
   */
  getToken: () => string | null;

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
}

export interface AuthenticatedDataResult<T> {
  /** The fetched data (or default value if not yet fetched) */
  readonly data: T | undefined;

  /** Whether data is currently being fetched */
  readonly loading: boolean;

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
  let data = $state<T | undefined>(options.defaultValue);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let _fetched = false;

  const doFetch = async () => {
    const token = options.getToken();
    if (!token) {
      loading = false;
      return;
    }

    loading = true;
    error = null;

    try {
      const result = await fetcher(fetch, token);
      data = result;
      _fetched = true;
      options.onSuccess?.(result);
    } catch (e) {
      error = e instanceof Error ? e.message : "Failed to load data";
    } finally {
      loading = false;
    }
  };

  const tryFetch = async (authLoading: boolean, currentUser: unknown) => {
    if (_fetched || authLoading || !currentUser) {
      return;
    }
    await doFetch();
  };

  const refetch = async () => {
    _fetched = false;
    await doFetch();
  };

  return {
    get data() {
      return data;
    },
    get loading() {
      return loading;
    },
    get error() {
      return error;
    },
    tryFetch,
    refetch
  };
}
