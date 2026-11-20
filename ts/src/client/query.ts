/**
 * Query parameters for sorting and filtering
 *
 * This module provides types and utilities for building query parameters
 * for API endpoints that support sorting and filtering.
 *
 * ## Query Parameter Format
 *
 * ### Sorting
 * Sort parameters use the format: `sort=field1:asc,field2:desc`
 * - Comma-separated list of `field:direction` pairs
 * - Direction is `asc` or `desc` (defaults to `asc` if omitted)
 *
 * ### Filtering
 * Filter parameters use bracket notation: `filter[field]=value`
 * - Simple equality: `filter[pathwayId]=abc123`
 * - With operator: `filter[weight][gte]=10`
 * - Supported operators: eq, ne, gt, gte, lt, lte, like
 *
 * ### Variants
 * Variant parameters use `variant=name`
 * - Variants represent named baseline queries understood by the API
 * - Filters and sorts refine the selected variant
 */

/** Sort direction */
export type SortDirection = "asc" | "desc";

/** A single sort field with direction */
export interface SortField {
	/** The field name to sort by */
	field: string;
	/** The sort direction */
	direction: SortDirection;
}

/** Filter operator */
export type FilterOperator = "eq" | "ne" | "gt" | "gte" | "lt" | "lte" | "like";

const VALID_FILTER_OPERATORS = new Set<FilterOperator>([
	"eq",
	"ne",
	"gt",
	"gte",
	"lt",
	"lte",
	"like",
]);

function isFilterOperator(value: string): value is FilterOperator {
	return VALID_FILTER_OPERATORS.has(value as FilterOperator);
}

function parsePositiveInt(value: string | null): number | undefined {
	if (!value) return undefined;
	const trimmed = value.trim();
	if (!/^\d+$/.test(trimmed)) return undefined;
	const parsed = Number(trimmed);
	return Number.isSafeInteger(parsed) && parsed >= 1 ? parsed : undefined;
}

/** A single filter condition */
export interface FilterField {
	/** The field name to filter on */
	field: string;
	/** The filter operator (default: eq) */
	operator?: FilterOperator;
	/** The value to compare against */
	value: string;
}

/** Query parameters for API requests */
export interface QueryParams {
	/** Named baseline query variant understood by the API */
	variant?: string;
	/** Sort fields in order of priority */
	sort?: SortField[];
	/** Filter conditions */
	filters?: FilterField[];
	/** Page number (1-indexed) */
	page?: number;
	/** Items per page */
	limit?: number;
}

/**
 * Convert OrderByValue (from OrderBy component) to SortField array
 *
 * @example
 * ```ts
 * import { orderByToSortFields } from "@inflatable-cookie/underlay/client/query";
 * import type { OrderByValue } from "@inflatable-cookie/poodle-svelte";
 *
 * const orderBy: OrderByValue = [
 *   { key: "title", direction: "asc" },
 *   { key: "createdAt", direction: "desc" }
 * ];
 *
 * const sortFields = orderByToSortFields(orderBy);
 * // [{ field: "title", direction: "asc" }, { field: "createdAt", direction: "desc" }]
 * ```
 */
export function orderByToSortFields(
	orderBy: Array<{ key: string; direction: "asc" | "desc" }>,
): SortField[] {
	return orderBy.map(({ key, direction }) => ({
		field: key,
		direction,
	}));
}

/**
 * Serialize sort fields to query string format
 *
 * @example
 * ```ts
 * const sort = [
 *   { field: "title", direction: "asc" },
 *   { field: "createdAt", direction: "desc" }
 * ];
 * serializeSort(sort); // "title:asc,createdAt:desc"
 * ```
 */
export function serializeSort(sort: SortField[]): string {
	return sort.map((s) => `${s.field}:${s.direction}`).join(",");
}

/**
 * Parse sort string back to SortField array
 *
 * @example
 * ```ts
 * parseSort("title:asc,createdAt:desc");
 * // [{ field: "title", direction: "asc" }, { field: "createdAt", direction: "desc" }]
 * ```
 */
export function parseSort(sortString: string): SortField[] {
	if (!sortString) return [];

	const fields: SortField[] = [];
	for (const part of sortString.split(",")) {
		const trimmed = part.trim();
		if (!trimmed) continue;

		const [rawField, rawDir] = trimmed.split(":");
		const field = (rawField ?? "").trim();
		if (!field) continue;

		fields.push({
			field,
			direction: rawDir?.trim() === "desc" ? "desc" : "asc",
		});
	}
	return fields;
}

/**
 * Build query string from QueryParams
 *
 * @example
 * ```ts
 * const params: QueryParams = {
 *   variant: "pending",
 *   sort: [{ field: "title", direction: "asc" }],
 *   filters: [{ field: "pathwayId", value: "abc123" }],
 *   page: 2,
 *   limit: 20
 * };
 *
 * buildQueryString(params);
 * // "variant=pending&sort=title%3Aasc&filter%5BpathwayId%5D=abc123&page=2&limit=20"
 * ```
 */
export function buildQueryString(params: QueryParams): string {
	const searchParams = new URLSearchParams();

	// Add named baseline query variant
	if (params.variant) {
		searchParams.set("variant", params.variant);
	}

	// Add sort
	if (params.sort && params.sort.length > 0) {
		searchParams.set("sort", serializeSort(params.sort));
	}

	// Add filters
	if (params.filters) {
		for (const filter of params.filters) {
			const key =
				filter.operator && filter.operator !== "eq"
					? `filter[${filter.field}][${filter.operator}]`
					: `filter[${filter.field}]`;
			searchParams.set(key, filter.value);
		}
	}

	// Add pagination
	if (params.page !== undefined) {
		searchParams.set("page", String(params.page));
	}
	if (params.limit !== undefined) {
		searchParams.set("limit", String(params.limit));
	}

	return searchParams.toString();
}

/**
 * Append query parameters to a URL path
 *
 * @example
 * ```ts
 * const path = appendQueryParams("/v1/modules", {
 *   sort: [{ field: "title", direction: "asc" }],
 *   page: 1,
 *   limit: 20
 * });
 * // "/v1/modules?sort=title%3Aasc&page=1&limit=20"
 * ```
 */
export function appendQueryParams(path: string, params: QueryParams): string {
	const queryString = buildQueryString(params);
	if (!queryString) return path;

	const hashIndex = path.indexOf("#");
	const pathWithoutHash = hashIndex === -1 ? path : path.slice(0, hashIndex);
	const hash = hashIndex === -1 ? "" : path.slice(hashIndex);

	const queryIndex = pathWithoutHash.indexOf("?");
	if (queryIndex === -1) {
		return `${pathWithoutHash}?${queryString}${hash}`;
	}

	// Merge new params into existing query string to avoid duplicates
	const basePath = pathWithoutHash.slice(0, queryIndex);
	const existingQuery = pathWithoutHash.slice(queryIndex + 1);
	const merged = new URLSearchParams(existingQuery);
	const newParams = new URLSearchParams(queryString);
	for (const [key, value] of newParams) {
		merged.set(key, value);
	}
	return `${basePath}?${merged.toString()}${hash}`;
}

/**
 * Convert QueryParams to a flat Record<string, string> suitable for API commands.
 *
 * Cattle-grid API commands expect flat key-value queries (like URL search params).
 * This converts the structured QueryParams back to that format.
 *
 * @example
 * ```ts
 * const params: QueryParams = {
 *   variant: "pending",
 *   sort: [{ field: "title", direction: "asc" }],
 *   filters: [{ field: "status", operator: "eq", value: "active" }],
 *   page: 2,
 *   limit: 20
 * };
 *
 * queryParamsToFlatRecord(params);
 * // { variant: "pending", sort: "title:asc", "filter[status]": "active", page: "2", limit: "20" }
 * ```
 */
export function queryParamsToFlatRecord(params: QueryParams): Record<string, string> {
	const result: Record<string, string> = {};

	// Add named baseline query variant
	if (params.variant) {
		result.variant = params.variant;
	}

	// Add sort
	if (params.sort && params.sort.length > 0) {
		result.sort = serializeSort(params.sort);
	}

	// Add filters
	if (params.filters) {
		for (const filter of params.filters) {
			const key =
				filter.operator && filter.operator !== "eq"
					? `filter[${filter.field}][${filter.operator}]`
					: `filter[${filter.field}]`;
			result[key] = filter.value;
		}
	}

	// Add pagination
	if (params.page !== undefined) {
		result.page = String(params.page);
	}
	if (params.limit !== undefined) {
		result.limit = String(params.limit);
	}

	return result;
}

/**
 * Create a filter helper for building filter arrays
 *
 * @example
 * ```ts
 * const f = createFilterBuilder();
 *
 * const filters = [
 *   f.eq("pathwayId", "abc123"),
 *   f.gte("weight", "10"),
 *   f.like("title", "%intro%")
 * ];
 * ```
 */
export function createFilterBuilder() {
	return {
		eq: (field: string, value: string): FilterField => ({
			field,
			operator: "eq",
			value,
		}),
		ne: (field: string, value: string): FilterField => ({
			field,
			operator: "ne",
			value,
		}),
		gt: (field: string, value: string): FilterField => ({
			field,
			operator: "gt",
			value,
		}),
		gte: (field: string, value: string): FilterField => ({
			field,
			operator: "gte",
			value,
		}),
		lt: (field: string, value: string): FilterField => ({
			field,
			operator: "lt",
			value,
		}),
		lte: (field: string, value: string): FilterField => ({
			field,
			operator: "lte",
			value,
		}),
		like: (field: string, value: string): FilterField => ({
			field,
			operator: "like",
			value,
		}),
	};
}

/**
 * Parse query parameters from URLSearchParams
 *
 * @example
 * ```ts
 * const url = new URL("https://example.com/api?variant=pending&sort=title:asc&filter[status]=active&page=2");
 * const params = parseQueryParams(url.searchParams);
 * // { variant: "pending", sort: [{ field: "title", direction: "asc" }], filters: [{ field: "status", value: "active" }], page: 2 }
 * ```
 */
export function parseQueryParams(searchParams: URLSearchParams): QueryParams {
	const result: QueryParams = {};

	// Parse named baseline query variant
	const variant = searchParams.get("variant");
	if (variant) {
		result.variant = variant;
	}

	// Parse sort
	const sortParam = searchParams.get("sort");
	if (sortParam) {
		const sort = parseSort(sortParam);
		if (sort.length > 0) {
			result.sort = sort;
		}
	}

	// Parse filters
	const filters: FilterField[] = [];
	for (const [key, value] of searchParams.entries()) {
		// Match filter[field] or filter[field][op]
		const simpleMatch = key.match(/^filter\[([A-Za-z0-9_]+(?:[.-][A-Za-z0-9_]+)*)\]$/);
		if (simpleMatch) {
			filters.push({ field: simpleMatch[1], value });
			continue;
		}

		const opMatch = key.match(/^filter\[([A-Za-z0-9_]+(?:[.-][A-Za-z0-9_]+)*)\]\[(\w+)\]$/);
		if (opMatch && isFilterOperator(opMatch[2])) {
			filters.push({
				field: opMatch[1],
				operator: opMatch[2],
				value,
			});
		}
	}
	if (filters.length > 0) {
		result.filters = filters;
	}

	// Parse pagination
	const page = parsePositiveInt(searchParams.get("page"));
	if (page !== undefined) {
		result.page = page;
	}

	const limit = parsePositiveInt(searchParams.get("limit"));
	if (limit !== undefined) {
		result.limit = limit;
	}

	return result;
}
