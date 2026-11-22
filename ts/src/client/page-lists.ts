/**
 * Page-shaped list query helpers for admin/resource browse surfaces.
 *
 * Use this with `PagedListResponse<T>` and `EntityListPage`-class consumers.
 * This is intentionally separate from cursor/runtime pagination helpers.
 */

import type { PagedListResponse } from "./envelopes";

export type PageListWireResponse<T> = Omit<PagedListResponse<T>, "hasMore"> & { has_more: boolean };

/** Convert a Rust `PageList<T>` response to the client pagination shape. */
export function normalizePageListResponse<T>({
	data,
	total,
	has_more,
}: PageListWireResponse<T>): PagedListResponse<T> {
	return { data, total, hasMore: has_more };
}

export interface PageListParams {
	/** Page number (1-indexed). */
	page?: number;
	/** Number of items per page. */
	limit?: number;
}

export function buildPageListQuery(params: PageListParams): Record<string, string> {
	const query: Record<string, string> = {};

	if (params.page !== undefined) {
		query.page = String(params.page);
	}
	if (params.limit !== undefined) {
		query.limit = String(params.limit);
	}

	return query;
}

export function appendPageListParams(path: string, params: PageListParams): string {
	const query = buildPageListQuery(params);
	const entries = Object.entries(query);

	if (entries.length === 0) {
		return path;
	}

	const hashIndex = path.indexOf("#");
	const pathWithoutHash = hashIndex === -1 ? path : path.slice(0, hashIndex);
	const hash = hashIndex === -1 ? "" : path.slice(hashIndex);

	const queryString = entries
		.map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(value)}`)
		.join("&");

	const queryIndex = pathWithoutHash.indexOf("?");
	if (queryIndex === -1) {
		return `${pathWithoutHash}?${queryString}${hash}`;
	}

	const basePath = pathWithoutHash.slice(0, queryIndex);
	const existingQuery = pathWithoutHash.slice(queryIndex + 1);
	const merged = new URLSearchParams(existingQuery);
	for (const [key, value] of entries) {
		merged.set(key, value);
	}
	return `${basePath}?${merged.toString()}${hash}`;
}
