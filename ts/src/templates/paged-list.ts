import type { PagedListResult } from "./template.types";

export interface PagedListLike<TItem> {
  data: TItem[];
  total: number;
  hasMore?: boolean;
}

export function toPagedListResult<TItem>(
  response: PagedListLike<TItem>
): PagedListResult<TItem> {
  return {
    data: response.data,
    total: response.total,
    hasMore: response.hasMore
  };
}
