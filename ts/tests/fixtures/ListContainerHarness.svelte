<script lang="ts">
  import ListContainer from "../../src/components/ListContainer.svelte";

  interface Props {
    title?: string;
    variant?: "page" | "tab";
    loading?: boolean;
    error?: string | null;
    count?: number;
    emptyMessage?: string;
    hasItems?: boolean;
    backHref?: string | null;
    backLabel?: string;
    backIsContextual?: boolean;
    withActions?: boolean;
    withFilters?: boolean;
    withContent?: boolean;
    withBatchBar?: boolean;
    withPagination?: boolean;
  }

  let {
    title = "Users",
    variant = "page",
    loading = false,
    error = null,
    count = 3,
    emptyMessage = "Nothing here.",
    hasItems = true,
    backHref = null,
    backLabel = "Back",
    backIsContextual = false,
    withActions = false,
    withFilters = false,
    withContent = false,
    withBatchBar = false,
    withPagination = false
  }: Props = $props();
</script>

{#snippet actionsSnippet()}
  <button type="button" data-testid="list-actions">Action</button>
{/snippet}

{#snippet filtersSnippet()}
  <div data-testid="list-filters">Filters</div>
{/snippet}

{#snippet contentSnippet()}
  <div data-testid="list-content">Content</div>
{/snippet}

{#snippet batchSnippet()}
  <div data-testid="list-batch">Batch</div>
{/snippet}

{#snippet paginationSnippet()}
  <div data-testid="list-pagination">Pagination</div>
{/snippet}

<ListContainer
  {title}
  {variant}
  {loading}
  {error}
  {count}
  {emptyMessage}
  {hasItems}
  {backHref}
  {backLabel}
  {backIsContextual}
  actions={withActions ? actionsSnippet : undefined}
  filters={withFilters ? filtersSnippet : undefined}
  content={withContent ? contentSnippet : undefined}
  batchBar={withBatchBar ? batchSnippet : undefined}
  pagination={withPagination ? paginationSnippet : undefined}
/>
