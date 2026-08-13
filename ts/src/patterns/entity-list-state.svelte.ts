import { page } from "$app/state";

import type { QueryParams } from "../client/query";

import type { NavigationContext } from "./navigation-types";
import { createPageListQueryState } from "./page-list-query.svelte";
import type { PageListQueryMode } from "./page-list-query.svelte";

export type EntityListFilterValueOptions = {
  /** Strip leading/trailing `%` wildcards (the `like`-operator search dialect). */
  stripLike?: boolean;
  /** Values treated as "no filter" (e.g. `"All"`); `""` is always empty. */
  emptyValues?: string[];
};

export type EntityListStateOptions = {
  /** Prop-driven mode: "url" syncs to the address bar, "local" stays in memory. */
  queryMode: () => PageListQueryMode;
  /** Prop-driven page title; used as the fallback sourceContext label. */
  title: () => string;
  /** Default page size applied when the query has no explicit limit. */
  pageSize?: number;
  /** Prop-driven navigation context override for detail-page back links. */
  sourceContext?: () => NavigationContext | undefined;
  /** Prop-driven reload scope (e.g. entity name + parent id) for reloadKey. */
  reloadScope?: () => string;
};

export type EntityListState = {
  /** The current query, always carrying an explicit limit. */
  readonly query: QueryParams;
  /** Replace the query; pass straight to EntityListPage's onQueryChange. */
  setQuery(next: QueryParams): void;
  /** Current refresh counter; increment to force a data reload. */
  readonly refreshVersion: number;
  /** Force the next reactive load to refetch. */
  refresh(): void;
  /** Derived `${reloadScope}:${refreshVersion}` key for EntityListPage. */
  readonly reloadKey: string;
  /** Provided context, or a fallback pointing at the current list URL. */
  readonly sourceContext: NavigationContext;
  /** Read the current query's filter value for a field, normalized per dialect. */
  filterValue(field: string, options?: EntityListFilterValueOptions): string | undefined;
  /** `{ backHref, backLabel }` in url mode, `{}` in local mode. */
  backHrefProps(href: string, label: string): { backHref?: string; backLabel?: string };
};

/**
 * Shared state factory for app-local EntityListPage wrappers.
 *
 * Absorbs the per-list plumbing every wrapper used to hand-write:
 * createPageListQueryState wiring, sourceContext fallback, filter-value
 * extraction (both the `like`-wildcard and the "All"-sentinel dialects),
 * refresh/reloadKey wiring, and backHref mode handling.
 *
 * Must be called during component init (composes rune-based state). Options
 * take thunks so prop-driven values stay reactive.
 */
export function createEntityListState(options: EntityListStateOptions): EntityListState {
  const base = createPageListQueryState({
    get mode() {
      return options.queryMode();
    },
    pageSize: options.pageSize,
  });

  function refresh(): void {
    base.refresh();
  }

  function filterValue(
    field: string,
    filterOptions?: EntityListFilterValueOptions,
  ): string | undefined {
    const entry = base.query.filters?.find((filter) => filter.field === field);
    if (!entry) return undefined;

    let value = entry.value;
    if (filterOptions?.stripLike) {
      value = value.replace(/^%|%$/g, "");
    }
    if (value === "" || filterOptions?.emptyValues?.includes(value)) {
      return undefined;
    }
    return value;
  }

  function backHrefProps(href: string, label: string): { backHref?: string; backLabel?: string } {
    if (options.queryMode() !== "url") return {};
    return { backHref: href, backLabel: label };
  }

  return {
    get query() {
      return base.query;
    },
    setQuery: base.setQuery,
    get refreshVersion() {
      return base.refreshVersion;
    },
    refresh,
    get reloadKey() {
      const scope = options.reloadScope?.();
      return scope ? `${scope}:${base.refreshVersion}` : `${base.refreshVersion}`;
    },
    get sourceContext() {
      return (
        options.sourceContext?.() ?? {
          label: options.title(),
          href: page.url.pathname + page.url.search,
          type: "list",
        }
      );
    },
    filterValue,
    backHrefProps,
  };
}
