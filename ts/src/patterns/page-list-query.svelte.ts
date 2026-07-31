import { goto } from "$app/navigation";
import { page } from "$app/state";

import {
  buildQueryString,
  parseQueryParams,
  type QueryParams,
} from "../client/query";

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

/**
 * Canonical query state for admin list pages.
 *
 * Replaces the per-list boilerplate of parse-from-URL, default-limit
 * merging, and goto-on-change that every EntityListPage consumer used to
 * hand-write (see docs/guides plus `ModulesListPage` in the reference apps).
 */
export function createPageListQueryState(
  options: PageListQueryStateOptions,
): PageListQueryState {
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
    url.search = buildQueryString(next);
    goto(url.toString(), { replaceState: true, keepFocus: true });
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
