import {
  runAuthenticatedFetch,
  shouldSkipAuthReadyFetch,
  tryResolveAuthFetchHandlers,
} from "./auth-fetch";
import type {
  CursorPaginatedResponse,
  CursorPaginationParams,
  PaginationController,
} from "./pagination-types";
import { DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE } from "./pagination-types";

export interface ServerPaginationOptions<T> {
  getToken?: () => string | null;
  onRefresh?: (fetchFn: typeof fetch) => Promise<string | null>;
  pageSize?: number;
  includeTotal?: boolean;
  persistKey?: string;
  onSuccess?: (response: CursorPaginatedResponse<T>) => void;
  onError?: (error: Error) => void;
}

export interface ServerPaginationResult<T> extends PaginationController<T> {
  tryFetch: (authLoading: boolean, currentUser: unknown) => Promise<void>;
}

export function createPaginationController<T>(
  fetcher: (
    fetchFn: typeof fetch,
    token: string,
    params: CursorPaginationParams,
  ) => Promise<CursorPaginatedResponse<T>>,
  options: ServerPaginationOptions<T> = {},
): ServerPaginationResult<T> {
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

  let items = $state<T[]>([]);
  let currentPage = $state(1);
  let pageSize = $state(Math.min(getInitialPageSize(), MAX_PAGE_SIZE));
  let loading = $state(true);
  let error = $state<string | null>(null);
  let total = $state<number | null>(null);

  let nextCursor = $state<string | null>(null);
  let prevCursor = $state<string | null>(null);
  let hasMore = $state(false);

  let cursorHistory: string[] = [];
  let _fetched = false;
  let _inFlight: Promise<void> | null = null;

  const hasNextPage = $derived(hasMore || nextCursor !== null);
  const hasPrevPage = $derived(currentPage > 1);
  const showingFrom = $derived(
    items.length > 0 ? (currentPage - 1) * pageSize + 1 : 0,
  );
  const showingTo = $derived(
    showingFrom > 0 ? showingFrom + items.length - 1 : 0,
  );
  const totalPages = $derived(
    total !== null ? Math.ceil(total / pageSize) : null,
  );

  const doFetch = async (
    cursor: string | null = null,
    direction: "forward" | "backward" = "forward",
  ) => {
    if (_inFlight) {
      return _inFlight;
    }

    // Resolve auth lazily (fetch path only) so setup stays SSR-safe; a
    // misconfiguration surfaces as the controller's error state.
    const resolved = tryResolveAuthFetchHandlers("createPaginationController", options);
    if (!resolved.handlers) {
      error = resolved.error;
      loading = false;
      return;
    }
    const { getToken, onRefresh } = resolved.handlers;

    let attemptedFetch = false;
    const run = (async () => {
      loading = true;
      error = null;

      const params: CursorPaginationParams = {
        limit: pageSize,
        cursor,
        direction,
        includeTotal: options.includeTotal ?? true,
      };

      const attempted = await runAuthenticatedFetch({
        getToken,
        onRefresh,
        fetcher: (token) => fetcher(fetch, token, params),
        onSuccess: (response) => {
          items = response.data;
          nextCursor = response.nextCursor;
          prevCursor = response.prevCursor;
          hasMore = response.hasMore;

          if (response.total !== null) {
            total = response.total;
          }

          options.onSuccess?.(response);
        },
        onError: (fetchError) => {
          error = fetchError.message;
          options.onError?.(fetchError);
        },
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
        _fetched = true;
      }
      loading = false;
      _inFlight = null;
    }
  };

  const tryFetch = async (authLoading: boolean, currentUser: unknown) => {
    if (shouldSkipAuthReadyFetch(_fetched, authLoading, currentUser)) {
      return;
    }
    await doFetch(null, "forward");
  };

  const nextPage = async () => {
    if (!hasNextPage || loading) return;

    if (nextCursor) {
      cursorHistory.push(prevCursor ?? "");
    }

    currentPage++;
    await doFetch(nextCursor, "forward");
  };

  const prevPage = async () => {
    if (!hasPrevPage || loading) return;

    currentPage--;

    const historyCursor = cursorHistory.pop();
    if (historyCursor !== undefined) {
      await doFetch(historyCursor || null, "forward");
    } else if (prevCursor) {
      await doFetch(prevCursor, "backward");
    } else {
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

  const reset = refresh;

  return {
    get items() {
      return items;
    },
    get currentPage() {
      return currentPage;
    },
    get pageSize() {
      return pageSize;
    },
    get hasNextPage() {
      return hasNextPage;
    },
    get hasPrevPage() {
      return hasPrevPage;
    },
    get total() {
      return total;
    },
    get loading() {
      return loading;
    },
    get error() {
      return error;
    },
    get showingFrom() {
      return showingFrom;
    },
    get showingTo() {
      return showingTo;
    },
    get totalPages() {
      return totalPages;
    },
    tryFetch,
    nextPage,
    prevPage,
    setPageSize,
    refresh,
    reset,
  };
}
