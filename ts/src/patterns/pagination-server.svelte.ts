import { getAuthConfig } from "./auth";
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
  const globalConfig = getAuthConfig();
  const getToken = options.getToken ?? globalConfig?.getToken;
  const onRefresh = options.onRefresh ?? globalConfig?.onRefresh;

  if (!getToken) {
    throw new Error(
      "createPaginationController: getToken is required. Either pass it in options or call configureAuth() at app startup.",
    );
  }

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

    const run = (async () => {
      const token = getToken();
      if (!token) {
        loading = false;
        return;
      }

      loading = true;
      error = null;

      const params: CursorPaginationParams = {
        limit: pageSize,
        cursor,
        direction,
        includeTotal: options.includeTotal ?? true,
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
        const is401 =
          e &&
          typeof e === "object" &&
          "status" in e &&
          (e as { status: number }).status === 401;

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
              const err =
                retryError instanceof Error
                  ? retryError
                  : new Error("Failed to load data");
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
