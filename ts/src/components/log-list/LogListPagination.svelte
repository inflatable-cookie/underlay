<script lang="ts">
  import { IconButton, PaginationSummary } from "@poodle/svelte-primitives";

  interface Props {
    page?: number;
    pageSize?: number;
    total?: number;
    totalPages?: number;
    loading?: boolean;
    onPrevPage?: () => void;
    onNextPage?: () => void;
  }

  let {
    page = 1,
    pageSize = 50,
    total,
    totalPages = 1,
    loading = false,
    onPrevPage,
    onNextPage
  }: Props = $props();
</script>

<div class="underlay-log-list__pagination">
  <span class="underlay-log-list__pagination-info">
    {#if total}
      <PaginationSummary currentPage={page} totalPages={totalPages} totalItems={total} {pageSize} />
    {/if}
  </span>
  <div class="underlay-log-list__pagination-controls">
    <IconButton
      icon="chevron-left"
      variant="ghost"
      size="sm"
      ariaLabel="Previous page"
      on:click={() => onPrevPage?.()}
      disabled={page <= 1 || loading}
    />
    <span class="underlay-log-list__pagination-page">
      Page {page} of {totalPages}
    </span>
    <IconButton
      icon="chevron-right"
      variant="ghost"
      size="sm"
      ariaLabel="Next page"
      on:click={() => onNextPage?.()}
      disabled={page >= totalPages || loading}
    />
  </div>
</div>

<style>
  .underlay-log-list__pagination {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.75rem 1rem;
    background: var(--underlay-color-surface-raised, #283548);
    border-top: 1px solid var(--underlay-color-border-subtle, #334155);
    font-size: 0.8125rem;
  }

  .underlay-log-list__pagination-info {
    color: var(--underlay-color-text-muted, #94a3b8);
  }

  .underlay-log-list__pagination-info :global(.pagination-summary) {
    color: inherit;
  }

  .underlay-log-list__pagination-controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .underlay-log-list__pagination-page {
    color: var(--underlay-color-text-secondary, #cbd5e1);
    min-width: 100px;
    text-align: center;
  }
</style>
