<script lang="ts">
  import Select from "../Select.svelte";

  interface Props {
    page: number;
    limit: number;
    total: number;
    totalPages: number;
    showLimitSelector: boolean;
    limitOptions: number[];
    onLimitChange: (limit: number) => void;
    onPageChange: (page: number) => void;
  }

  let {
    page,
    limit,
    total,
    totalPages,
    showLimitSelector,
    limitOptions,
    onLimitChange,
    onPageChange
  }: Props = $props();
</script>

<div class="table-footer">
  <div class="pagination-info">
    Showing {(page - 1) * limit + 1} to {Math.min(page * limit, total)}
    of {total}
  </div>

  <div class="pagination-right">
    {#if showLimitSelector && limitOptions.length > 0}
      <div class="limit-selector">
        <span>Show</span>
        <Select
          value={String(limit)}
          onchange={(value) => onLimitChange(Number(value))}
          items={limitOptions.map((opt) => ({ value: String(opt), label: String(opt) }))}
        />
        <span>per page</span>
      </div>
    {/if}

    {#if totalPages > 1}
      <div class="pagination-controls">
        <button
          type="button"
          class="pagination-button"
          disabled={page <= 1}
          onclick={() => onPageChange(1)}
          aria-label="First page"
        >
          ««
        </button>
        <button
          type="button"
          class="pagination-button"
          disabled={page <= 1}
          onclick={() => onPageChange(page - 1)}
          aria-label="Previous page"
        >
          «
        </button>

        <span class="pagination-page">
          Page {page} of {totalPages}
        </span>

        <button
          type="button"
          class="pagination-button"
          disabled={page >= totalPages}
          onclick={() => onPageChange(page + 1)}
          aria-label="Next page"
        >
          »
        </button>
        <button
          type="button"
          class="pagination-button"
          disabled={page >= totalPages}
          onclick={() => onPageChange(totalPages)}
          aria-label="Last page"
        >
          »»
        </button>
      </div>
    {/if}
  </div>
</div>

<style>
  .table-footer {
    grid-column: 1 / -1;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--dt-gap);
    background: var(--dt-header-bg);
    border-top: var(--dt-border);
  }

  .pagination-info {
    color: var(--color-text-muted, #64748b);
  }

  .pagination-right {
    display: flex;
    align-items: center;
    gap: 1.5rem;
  }

  .limit-selector {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--color-text-muted, #64748b);
  }

  .limit-selector :global(.underlay-select-trigger) {
    min-width: 4.5rem;
    padding: 0.25rem 0.5rem;
    font-size: inherit;
  }

  .pagination-controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .pagination-button {
    padding: 0.25rem 0.5rem;
    border: 1px solid var(--color-border, #e2e8f0);
    border-radius: var(--radius-sm, 0.25rem);
    background: var(--color-surface, #fff);
    cursor: pointer;
    font-size: inherit;
  }

  .pagination-button:hover:not(:disabled) {
    background: var(--dt-row-hover);
  }

  .pagination-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .pagination-page {
    padding: 0 0.5rem;
  }

  @media (max-width: 900px) {
    .table-footer {
      flex-direction: column;
      gap: 0.75rem;
      align-items: stretch;
    }

    .pagination-right {
      flex-direction: column;
      gap: 0.75rem;
    }

    .limit-selector {
      justify-content: center;
    }

    .pagination-controls {
      justify-content: center;
    }
  }
</style>
