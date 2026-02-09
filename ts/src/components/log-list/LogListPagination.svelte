<script lang="ts">
  import Button from "../Button.svelte";
  import ChevronLeft from "lucide-svelte/icons/chevron-left";
  import ChevronRight from "lucide-svelte/icons/chevron-right";

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

<div class="log-list__pagination">
  <span class="log-list__pagination-info">
    {#if total}
      Showing {(page - 1) * pageSize + 1}–{Math.min(page * pageSize, total)} of {total}
    {/if}
  </span>
  <div class="log-list__pagination-controls">
    <Button
      variant="subtle"
      size="sm"
      onclick={onPrevPage}
      disabled={page <= 1 || loading}
    >
      <ChevronLeft size={16} />
    </Button>
    <span class="log-list__pagination-page">
      Page {page} of {totalPages}
    </span>
    <Button
      variant="subtle"
      size="sm"
      onclick={onNextPage}
      disabled={page >= totalPages || loading}
    >
      <ChevronRight size={16} />
    </Button>
  </div>
</div>

<style>
  .log-list__pagination {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.75rem 1rem;
    background: var(--underlay-color-surface-raised, #283548);
    border-top: 1px solid var(--underlay-color-border-subtle, #334155);
    font-size: 0.8125rem;
  }

  .log-list__pagination-info {
    color: var(--underlay-color-text-muted, #94a3b8);
  }

  .log-list__pagination-controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .log-list__pagination-page {
    color: var(--underlay-color-text-secondary, #cbd5e1);
    min-width: 100px;
    text-align: center;
  }
</style>
