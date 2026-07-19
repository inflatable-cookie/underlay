import type { Snippet } from "svelte";

/// A renderable surface a consumer hands to a template: a Svelte snippet or a
/// render function. Snippets of any arity (`Snippet<[]>`, `Snippet<[T]>`, ...)
/// are structurally assignable to this rest-args function type; the previous
/// `Snippet | fn` union meant bare `Snippet<[]>` only, which forced consumers
/// to cast every argument-taking snippet (`as never`), and a union here
/// collapses `{@render}` call signatures to `never`.
export type TemplateSurface = (...args: any[]) => any;

export interface PagedListResult<TItem> {
  data: TItem[];
  total?: number | null;
  hasMore?: boolean;
}

export type FetchFn = (
  input: RequestInfo | URL,
  init?: RequestInit,
) => Promise<Response>;
