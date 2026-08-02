import { getAuthConfig } from "./auth";
import {
  runAuthenticatedFetch,
  shouldSkipAuthReadyFetch,
  tryResolveAuthFetchHandlers,
} from "./auth-fetch";

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
 * SSR note: auth handlers are resolved lazily on the fetch path, not at
 * setup, so SSR (where configureAuth must not run — a process-global token
 * getter would leak across requests) renders the loading state and the first
 * fetch happens after hydration. A missing getToken then surfaces as the
 * hook's error state instead of a setup-time throw.
 *
 * ## Setup (once per app)
 *
 * Configure global auth handlers in your app's +layout.svelte:
 * ```svelte
 * <script>
 *   import { configureAuth } from '@decodelabs/underlay/runtime/auth';
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
 * ### Basic (with global auth config)
 *
 * When configureAuth() includes getAuthLoading and getCurrentUser, the hook
 * auto-fetches — no manual $effect needed:
 *
 * ```svelte
 * <script lang="ts">
 *   import { useAuthenticatedData } from '@decodelabs/underlay/runtime/auth';
 *
 *   const pageData = useAuthenticatedData(
 *     async (fetch, token) => {
 *       const result = await someApiCall(fetch, token);
 *       return { data: result.data };
 *     },
 *     { defaultValue: { data: [] } }
 *   );
 * </script>
 * ```
 *
 * ### With URL-param-driven refetch (queryKey)
 *
 * For list components that refetch when URL search params change, pass a
 * queryKey getter. The hook internally tracks the previous key and only
 * refetches when data-relevant params genuinely change:
 *
 * ```svelte
 * <script lang="ts">
 *   import { useAuthenticatedData } from '@decodelabs/underlay/runtime/auth';
 *   import { dataSearchParams } from '$lib/utils/list-query';
 *   import { page } from '$app/stores';
 *
 *   const pageData = useAuthenticatedData(
 *     async (fetch, token) => listItemsAdmin(fetch, token, query),
 *     {
 *       defaultValue: { data: [], total: 0 },
 *       queryKey: () => dataSearchParams($page.url.searchParams).toString()
 *     }
 *   );
 * </script>
 * ```
 *
 * ### Manual wiring (legacy / no global auth config)
 *
 * ```svelte
 * <script lang="ts">
 *   import { useAuthenticatedData } from '@decodelabs/underlay/runtime/auth';
 *   import { authLoading, currentUser } from '$lib/stores/auth';
 *
 *   const pageData = useAuthenticatedData(
 *     async (fetch, token) => someApiCall(fetch, token),
 *     { defaultValue: { data: [] } }
 *   );
 *
 *   // Trigger fetch when auth is ready
 *   $effect(() => {
 *     pageData.tryFetch($authLoading, $currentUser);
 *   });
 * </script>
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
   *
   * When `queryKey` is also provided, onSuccess is called after the
   * internal query key is updated, so you don't need to track it yourself.
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

  /**
   * Reactive getter that returns a string key representing the current
   * query state (e.g. URL search params minus UI-only params like ?tab=).
   *
   * When provided, useAuthenticatedData will:
   * 1. Internally track the previous key value
   * 2. Set the initial key after the first successful fetch
   * 3. Create a $effect that compares current vs previous key
   * 4. Call refetch() only when the key genuinely changes
   *
   * This eliminates the manual previousQueryKey / previousUrl boilerplate.
   *
   * @example
   * queryKey: () => dataSearchParams($page.url.searchParams).toString()
   */
  queryKey?: () => string;

  /**
   * Reactive getter for whether auth is still initializing.
   * When both getAuthLoading and getCurrentUser are available (either
   * here or via configureAuth), the hook automatically creates a $effect
   * that calls tryFetch — no manual wiring needed.
   *
   * Must read a reactive source when called (e.g. () => $authLoading).
   * Resolves from: instance option → global configureAuth().
   */
  getAuthLoading?: () => boolean;

  /**
   * Reactive getter for the current user (null if not authenticated).
   * See getAuthLoading for details.
   *
   * @example () => $currentUser
   */
  getCurrentUser?: () => unknown;
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
  const globalConfig = getAuthConfig();

  let data = $state<T | undefined>(options.defaultValue);
  let loading = $state(true);
  let refetching = $state(false);
  let error = $state<string | null>(null);
  let _fetched = false;
  let _inFlight: Promise<void> | null = null;
  let _queuedRefetch = false;

  // Query key tracking: intercept onSuccess to capture the key after each fetch.
  // This must be set up before doFetch is defined since doFetch calls the hook.
  let _previousQueryKey: string | null = null;
  const _onSuccessHook: ((data: T) => void) | undefined = options.queryKey
    ? (result: T) => {
        _previousQueryKey = options.queryKey!();
        options.onSuccess?.(result);
      }
    : options.onSuccess;

  const doFetch = async (isRefetch = false) => {
    if (_inFlight) {
      if (isRefetch) {
        _queuedRefetch = true;
      }
      return _inFlight;
    }

    // Resolve auth handlers lazily: setup can run during SSR (no global
    // configureAuth there — a process-global token getter would leak across
    // requests), while doFetch only ever runs client-side. A resolution
    // failure surfaces as the hook's error state instead of crashing setup.
    const resolved = tryResolveAuthFetchHandlers("useAuthenticatedData", options);
    if (!resolved.handlers) {
      error = resolved.error;
      loading = false;
      refetching = false;
      return;
    }
    const { getToken, onRefresh } = resolved.handlers;

    if (isRefetch) {
      refetching = true;
    } else {
      loading = true;
    }
    error = null;

    let attemptedFetch = false;
    const run = (async () => {
      const attempted = await runAuthenticatedFetch({
        getToken,
        onRefresh,
        fetcher: (token) => fetcher(fetch, token),
        onSuccess: (result) => {
          data = result;
          _onSuccessHook?.(result);
        },
        onError: (fetchError) => {
          error = fetchError.message;
        },
        preserveRefreshFailureError: true,
      });

      if (!attempted) {
        loading = false;
      }
      attemptedFetch = attempted;
    })();

    _inFlight = run;
    try {
      await run;
    } finally {
      if (attemptedFetch) {
        // Mark as fetched even on error to prevent tryFetch from auto-retrying.
        // Users can still explicitly call refetch() to retry.
        _fetched = true;
      }
      loading = false;
      refetching = false;
      _inFlight = null;
      if (_queuedRefetch) {
        _queuedRefetch = false;
        await doFetch(true);
      }
    }
  };

  const tryFetch = async (authLoading: boolean, currentUser: unknown) => {
    if (shouldSkipAuthReadyFetch(_fetched, authLoading, currentUser)) {
      return;
    }
    await doFetch(false);
  };

  const refetch = async () => {
    _fetched = false;
    await doFetch(true);
  };

  // --- Auto-fetch: create $effect for tryFetch when auth getters are available ---
  const resolvedGetAuthLoading = options.getAuthLoading ?? globalConfig?.getAuthLoading;
  const resolvedGetCurrentUser = options.getCurrentUser ?? globalConfig?.getCurrentUser;

  if (resolvedGetAuthLoading && resolvedGetCurrentUser) {
    $effect(() => {
      tryFetch(resolvedGetAuthLoading(), resolvedGetCurrentUser());
    });
  }

  // --- Query key watching: create $effect that refetches on genuine key changes ---
  if (options.queryKey) {
    const queryKeyGetter = options.queryKey;

    $effect(() => {
      const currentKey = queryKeyGetter();
      if (_previousQueryKey !== null && _previousQueryKey !== currentKey) {
        _previousQueryKey = currentKey;
        refetch();
      }
    });
  }

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
