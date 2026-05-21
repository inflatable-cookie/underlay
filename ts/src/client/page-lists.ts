/**
 * Page-shaped list query helpers for admin/resource browse surfaces.
 *
 * Use this with `PagedListResponse<T>` and `EntityListPage`-class consumers.
 * This is intentionally separate from cursor/runtime pagination helpers.
 */

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

  const queryString = entries
    .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(value)}`)
    .join("&");

  if (!path.includes("?")) {
    return `${path}?${queryString}`;
  }

  const [basePath, existingQuery] = path.split("?", 2);
  const merged = new URLSearchParams(existingQuery);
  for (const [key, value] of entries) {
    merged.set(key, value);
  }
  return `${basePath}?${merged.toString()}`;
}
