import type { Snippet } from "svelte";

export type TemplateSurface = Snippet | ((...args: any[]) => any);

export interface PagedListResult<TItem> {
  data: TItem[];
  total?: number | null;
  hasMore?: boolean;
}

export type FetchFn = (
  input: RequestInfo | URL,
  init?: RequestInit,
) => Promise<Response>;
