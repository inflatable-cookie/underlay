<script lang="ts">
  import Activity from "lucide-svelte/icons/activity";

  interface Props {
    loading?: boolean;
    error?: string | null;
    entriesCount?: number;
    emptyMessage?: string;
  }

  let {
    loading = false,
    error = null,
    entriesCount = 0,
    emptyMessage = "No log entries found"
  }: Props = $props();
</script>

{#if loading && entriesCount === 0}
  <div class="log-list__status">
    <span class="log-list__status-icon log-list__status-icon--spinning">
      <Activity size={24} />
    </span>
    <p>Loading log entries...</p>
  </div>
{:else if error}
  <div class="log-list__status log-list__status--error">
    <p>{error}</p>
  </div>
{:else if entriesCount === 0}
  <div class="log-list__status">
    <span class="log-list__status-icon">
      <Activity size={24} />
    </span>
    <p>{emptyMessage}</p>
  </div>
{/if}

<style>
  .log-list__status {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    padding: 3rem 1rem;
    color: var(--underlay-color-text-muted, #94a3b8);
  }

  .log-list__status p {
    margin: 0;
    font-size: 0.875rem;
  }

  .log-list__status--error {
    color: var(--underlay-color-danger, #ef4444);
  }

  .log-list__status-icon {
    opacity: 0.5;
  }

  .log-list__status-icon--spinning {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>
