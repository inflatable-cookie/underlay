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
 * import { orderByToSortFields } from "@decodelabs/underlay/client/query";
 * import type { OrderByValue } from "@poodle/svelte-primitives";
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
  orderBy: Array<{ key: string; direction: "asc" | "desc" }>
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

  return sortString.split(",").map((part) => {
    const [field, dir] = part.split(":");
    return {
      field: field.trim(),
      direction: (dir?.trim() === "desc" ? "desc" : "asc") as SortDirection,
    };
  });
}

/**
 * Build query string from QueryParams
 *
 * @example
 * ```ts
 * const params: QueryParams = {
 *   sort: [{ field: "title", direction: "asc" }],
 *   filters: [{ field: "pathwayId", value: "abc123" }],
 *   page: 2,
 *   limit: 20
 * };
 *
 * buildQueryString(params);
 * // "sort=title%3Aasc&filter%5BpathwayId%5D=abc123&page=2&limit=20"
 * ```
 */
export function buildQueryString(params: QueryParams): string {
  const searchParams = new URLSearchParams();

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

  const separator = path.includes("?") ? "&" : "?";
  return `${path}${separator}${queryString}`;
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
 * const url = new URL("https://example.com/api?sort=title:asc&filter[status]=active&page=2");
 * const params = parseQueryParams(url.searchParams);
 * // { sort: [{ field: "title", direction: "asc" }], filters: [{ field: "status", value: "active" }], page: 2 }
 * ```
 */
export function parseQueryParams(searchParams: URLSearchParams): QueryParams {
  const result: QueryParams = {};

  // Parse sort
  const sortParam = searchParams.get("sort");
  if (sortParam) {
    result.sort = parseSort(sortParam);
  }

  // Parse filters
  const filters: FilterField[] = [];
  for (const [key, value] of searchParams.entries()) {
    // Match filter[field] or filter[field][op]
    const simpleMatch = key.match(/^filter\[(\w+)\]$/);
    if (simpleMatch) {
      filters.push({ field: simpleMatch[1], value });
      continue;
    }

    const opMatch = key.match(/^filter\[(\w+)\]\[(\w+)\]$/);
    if (opMatch) {
      filters.push({
        field: opMatch[1],
        operator: opMatch[2] as FilterOperator,
        value,
      });
    }
  }
  if (filters.length > 0) {
    result.filters = filters;
  }

  // Parse pagination
  const page = searchParams.get("page");
  if (page) {
    result.page = parseInt(page, 10);
  }

  const limit = searchParams.get("limit");
  if (limit) {
    result.limit = parseInt(limit, 10);
  }

  return result;
}
