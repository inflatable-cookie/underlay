import { onDestroy } from "svelte";
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
		if (options.persistKey) {
			try {
				if (typeof localStorage !== "undefined") {
					const stored = localStorage.getItem(options.persistKey);
					if (stored) {
						const parsed = parseInt(stored, 10);
						if (!isNaN(parsed) && parsed > 0 && parsed <= MAX_PAGE_SIZE) {
							return parsed;
						}
					}
				}
			} catch {
				// Ignore storage access/read errors (e.g. opaque origin, private browsing, or restricted storage).
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
	let disposed = false;
	let requestSeq = 0;
	let resetSeq = 0;
	let _inFlight: Promise<boolean> | null = null;
	let pendingFetch: { cursor: string | null; direction: "forward" | "backward" } | null = null;

	const markDisposed = () => {
		disposed = true;
		pendingFetch = null;
	};
	try {
		onDestroy(markDisposed);
	} catch {
		// Outside component initialization (e.g. $effect.root or unit tests); fall back to effect teardown.
	}
	$effect(() => markDisposed);

	const hasNextPage = $derived(hasMore || nextCursor !== null);
	const hasPrevPage = $derived(currentPage > 1);
	const showingFrom = $derived(items.length > 0 ? (currentPage - 1) * pageSize + 1 : 0);
	const showingTo = $derived(showingFrom > 0 ? showingFrom + items.length - 1 : 0);
	const totalPages = $derived(total !== null ? Math.ceil(total / pageSize) : null);

	const doFetch = async (
		cursor: string | null = null,
		direction: "forward" | "backward" = "forward",
		fetchOptions: { supersede?: boolean } = {},
	): Promise<boolean> => {
		if (disposed) {
			return false;
		}

		if (_inFlight) {
			if (fetchOptions.supersede) {
				requestSeq++;
				pendingFetch = { cursor, direction };
				return false;
			}
			return _inFlight;
		}

		// Resolve auth lazily (fetch path only) so setup stays SSR-safe; a
		// misconfiguration surfaces as the controller's error state.
		const resolved = tryResolveAuthFetchHandlers("createPaginationController", options);
		if (!resolved.handlers) {
			error = resolved.error;
			loading = false;
			return false;
		}
		const { getToken, onRefresh } = resolved.handlers;

		const seq = ++requestSeq;
		let attemptedFetch = false;
		let succeeded = false;
		const run = (async (): Promise<boolean> => {
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
				onRefresh:
					onRefresh &&
					(async (fetchFn) => {
						if (disposed || seq !== requestSeq) return null;
						const token = await onRefresh(fetchFn);
						return disposed || seq !== requestSeq ? null : token;
					}),
				fetcher: (token) => fetcher(fetch, token, params),
				onSuccess: (response) => {
					if (disposed || seq !== requestSeq) return;
					succeeded = true;
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
					if (disposed || seq !== requestSeq) return;
					error = fetchError.message;
					options.onError?.(fetchError);
				},
			});

			attemptedFetch = attempted;
			return succeeded;
		})();

		_inFlight = run;
		try {
			return await run;
		} finally {
			if (_inFlight === run) {
				_inFlight = null;
			}
			if (!disposed) {
				if (seq === requestSeq) {
					if (attemptedFetch) {
						_fetched = true;
					}
					if (!pendingFetch) {
						loading = false;
					}
				}
				if (pendingFetch && !_inFlight) {
					const next = pendingFetch;
					pendingFetch = null;
					void doFetch(next.cursor, next.direction).catch(() => {});
				}
			}
		}
	};

	const tryFetch = async (authLoading: boolean, currentUser: unknown) => {
		if (shouldSkipAuthReadyFetch(_fetched, authLoading, currentUser)) {
			return;
		}
		await doFetch(null, "forward");
	};

	const nextPage = async () => {
		if (!hasNextPage || loading || disposed) return;

		const prevPageNumber = currentPage;
		const prevHistory = [...cursorHistory];
		const navResetSeq = resetSeq;
		const targetCursor = nextCursor;

		if (targetCursor) {
			cursorHistory.push(prevCursor ?? "");
		}

		currentPage++;
		let succeeded = false;
		try {
			succeeded = await doFetch(targetCursor, "forward");
		} finally {
			if (!succeeded && !disposed && resetSeq === navResetSeq) {
				currentPage = prevPageNumber;
				cursorHistory = prevHistory;
			}
		}
	};

	const prevPage = async () => {
		if (!hasPrevPage || loading || disposed) return;

		const prevPageNumber = currentPage;
		const prevHistory = [...cursorHistory];
		const navResetSeq = resetSeq;

		currentPage--;

		const historyCursor = cursorHistory.pop();
		let succeeded = false;
		try {
			if (historyCursor !== undefined) {
				succeeded = await doFetch(historyCursor || null, "forward");
			} else if (prevCursor) {
				succeeded = await doFetch(prevCursor, "backward");
			} else {
				cursorHistory = [];
				succeeded = await doFetch(null, "forward");
			}
		} finally {
			if (!succeeded && !disposed && resetSeq === navResetSeq) {
				currentPage = prevPageNumber;
				cursorHistory = prevHistory;
			}
		}
	};

	const setPageSize = (size: number) => {
		if (disposed || !Number.isFinite(size)) return;
		const newSize = Math.min(Math.max(1, size), MAX_PAGE_SIZE);
		if (newSize === pageSize) return;

		pageSize = newSize;
		currentPage = 1;
		cursorHistory = [];
		resetSeq++;

		if (options.persistKey) {
			try {
				if (typeof localStorage !== "undefined") {
					localStorage.setItem(options.persistKey, String(newSize));
				}
			} catch {
				// Ignore storage access/write errors (e.g. opaque origin, quota exceeded, or private browsing).
			}
		}

		void doFetch(null, "forward", { supersede: true }).catch(() => {});
	};

	const refresh = async () => {
		if (disposed) return;
		currentPage = 1;
		cursorHistory = [];
		resetSeq++;
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
