<script lang="ts">
  import Loader from "lucide-svelte/icons/loader-circle";

  interface Props {
    error?: string;
    onRetry?: () => void;
    showLoading: boolean;
    showEmpty: boolean;
    searchQuery: string;
    emptyMessage: string;
  }

  let {
    error,
    onRetry,
    showLoading,
    showEmpty,
    searchQuery,
    emptyMessage
  }: Props = $props();
</script>

{#if error}
  <div class="relation-picker-dialog__error">
    <span>{error}</span>
    {#if onRetry}
      <button
        type="button"
        class="relation-picker-dialog__error-retry"
        onclick={onRetry}
      >
        Retry
      </button>
    {/if}
  </div>
{/if}

{#if showLoading}
  <div class="relation-picker-dialog__loading">
    <Loader size="1.5em" class="relation-picker-dialog__loading-spinner" />
    <span>Loading...</span>
  </div>
{:else if showEmpty}
  <div class="relation-picker-dialog__empty">
    {searchQuery.trim() ? "No matches found." : emptyMessage}
  </div>
{/if}

<style>
  @keyframes relation-picker-spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .relation-picker-dialog__empty {
    padding: 2rem 1rem;
    text-align: center;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.9rem;
  }

  .relation-picker-dialog__loading {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 2rem 1rem;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.9rem;
  }

  :global(.relation-picker-dialog__loading-spinner) {
    animation: relation-picker-spin 1s linear infinite;
  }

  .relation-picker-dialog__error {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    padding: 0.75rem;
    margin-bottom: 1rem;
    border-radius: 0.35rem;
    background: var(--underlay-color-danger, #ef4444);
    color: white;
    font-size: 0.85rem;
  }

  .relation-picker-dialog__error-retry {
    flex-shrink: 0;
    padding: 0.35rem 0.65rem;
    border: 1px solid rgba(255, 255, 255, 0.5);
    border-radius: 0.25rem;
    background: transparent;
    color: white;
    font-size: 0.8rem;
    cursor: pointer;
    transition: background-color 0.15s ease;
  }

  .relation-picker-dialog__error-retry:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .relation-picker-dialog__error-retry:focus-visible {
    outline: 2px solid white;
    outline-offset: 2px;
  }
</style>
