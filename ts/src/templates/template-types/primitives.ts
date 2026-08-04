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

/// Fetch contract for template data loaders. Resolved against each project's
/// own ambient `fetch` type, so SvelteKit apps (whose ambient fetch carries
/// `preconnect` since kit 2.70) and plain DOM consumers both type-check
/// without casts. Do not re-declare the call signature literally — that pins
/// the type to DOM fetch and breaks under kit's ambient types.
export type FetchFn = typeof fetch;
