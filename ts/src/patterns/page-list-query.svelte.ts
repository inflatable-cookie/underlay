import { goto } from "$app/navigation";
import { page } from "$app/state";

import { buildQueryString, parseQueryParams, type QueryParams } from "../client/query";

export type PageListQueryMode = "url" | "local";

export type PageListQueryStateOptions = {
	/** "url" syncs query state to the address bar; "local" keeps it in memory. */
	mode: PageListQueryMode;
	/** Default page size applied when the query has no explicit limit. */
	pageSize?: number;
	/** Initial query for local mode (url mode reads the current address). */
	initialQuery?: QueryParams;
};

export type PageListQueryState = {
	/** The current query, always carrying an explicit limit. */
	readonly query: QueryParams;
	/** Replace the query (writes to the URL in url mode, state in local mode). */
	setQuery(next: QueryParams): void;
	/** Current refresh counter; increment to force a data reload. */
	readonly refreshVersion: number;
	/** Force the next reactive load to refetch. */
	refresh(): void;
};

const LIST_QUERY_KEYS = new Set(["variant", "sort", "page", "limit"]);
const FILTER_PARAM_RE =
	/^filter\[[A-Za-z0-9_]+(?:[.-][A-Za-z0-9_]+)*\](?:\[(?:eq|ne|gt|gte|lt|lte|like)\])?$/;

function isListQueryParam(key: string): boolean {
	return LIST_QUERY_KEYS.has(key) || FILTER_PARAM_RE.test(key);
}

/**
 * Canonical query state for admin list pages.
 *
 * Replaces the per-list boilerplate of parse-from-URL, default-limit
 * merging, and goto-on-change that every EntityListPage consumer used to
 * hand-write (see docs/guides plus `ModulesListPage` in the reference apps).
 */
export function createPageListQueryState(options: PageListQueryStateOptions): PageListQueryState {
	const pageSize = options.pageSize ?? 30;
	let localQuery = $state<QueryParams>(options.initialQuery ?? { page: 1 });
	let refreshVersion = $state(0);

	const currentQuery: QueryParams = $derived.by(() => {
		if (options.mode === "local") {
			return { ...localQuery, limit: localQuery.limit ?? pageSize };
		}

		const parsed = parseQueryParams(page.url.searchParams);
		return { ...parsed, limit: parsed.limit ?? pageSize };
	});

	function setQuery(next: QueryParams): void {
		if (options.mode === "local") {
			localQuery = { ...next, limit: next.limit ?? pageSize };
			return;
		}

		const url = new URL(page.url);
		for (const key of Array.from(url.searchParams.keys())) {
			if (isListQueryParam(key)) {
				url.searchParams.delete(key);
			}
		}
		const nextParams = new URLSearchParams(buildQueryString(next));
		for (const [key, value] of nextParams.entries()) {
			url.searchParams.set(key, value);
		}

		try {
			void Promise.resolve(goto(url.toString(), { replaceState: true, keepFocus: true })).catch(
				() => {
					// Ignore cancelled or rejected navigations so callers never see unhandled rejections.
				},
			);
		} catch {
			// Ignore synchronous navigation errors.
		}
	}

	function refresh(): void {
		refreshVersion += 1;
	}

	return {
		get query() {
			return currentQuery;
		},
		setQuery,
		get refreshVersion() {
			return refreshVersion;
		},
		refresh,
	};
}
