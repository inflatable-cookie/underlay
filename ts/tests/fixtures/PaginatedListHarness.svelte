<script lang="ts">
  import PaginatedList from "../../src/components/PaginatedList.svelte";
  import type { PaginationController } from "../../src/patterns/pagination-types";

  type HarnessItem = { id: string; label: string };

  interface Props {
    controller?: PaginationController<HarnessItem>;
    layout?: "list" | "grid";
    gridMinWidth?: number;
    gap?: number;
    loadingMessage?: string;
    emptyMessage?: string;
    showPagination?: boolean;
    paginationVariant?: "full" | "simple";
    showLimitSelector?: boolean;
    className?: string;
    withEmptySnippet?: boolean;
    withLoadingSnippet?: boolean;
    withErrorSnippet?: boolean;
  }

  const defaultController: PaginationController<HarnessItem> = {
    items: [],
    currentPage: 1,
    pageSize: 20,
    hasNextPage: false,
    hasPrevPage: false,
    total: 0,
    loading: false,
    error: null,
    showingFrom: 0,
    showingTo: 0,
    totalPages: 1,
    nextPage: async () => {},
    prevPage: async () => {},
    goToPage: () => {},
    setPageSize: () => {},
    refresh: async () => {},
    reset: async () => {}
  };

  let {
    controller = defaultController,
    layout = "list",
    gridMinWidth = 20,
    gap = 1,
    loadingMessage = "Loading...",
    emptyMessage = "No items found.",
    showPagination = true,
    paginationVariant = "simple",
    showLimitSelector = false,
    className = "",
    withEmptySnippet = false,
    withLoadingSnippet = false,
    withErrorSnippet = false
  }: Props = $props();
</script>

{#snippet itemSnippet(item, index)}
  <div data-testid="paginated-item">{index}:{item.label}</div>
{/snippet}

{#snippet emptySnippet()}
  <p data-testid="paginated-empty">Custom empty</p>
{/snippet}

{#snippet loadingSnippet()}
  <p data-testid="paginated-loading">Custom loading</p>
{/snippet}

{#snippet errorSnippet(message)}
  <p data-testid="paginated-error">Custom error: {message}</p>
{/snippet}

<PaginatedList
  {controller}
  items={itemSnippet}
  empty={withEmptySnippet ? emptySnippet : undefined}
  loading={withLoadingSnippet ? loadingSnippet : undefined}
  error={withErrorSnippet ? errorSnippet : undefined}
  {layout}
  {gridMinWidth}
  {gap}
  {loadingMessage}
  {emptyMessage}
  {showPagination}
  {paginationVariant}
  {showLimitSelector}
  {className}
/>
