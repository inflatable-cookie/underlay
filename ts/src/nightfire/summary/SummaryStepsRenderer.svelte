<script lang="ts">
  type SummaryStep = {
    title: string;
    body: string;
  };

  export let steps: SummaryStep[] = [];
</script>

{#if steps.length > 0}
  <ol class="summary-steps">
    {#each steps as step, index}
      <li class="summary-steps__item">
        <span class="summary-steps__index">{index + 1}</span>
        <div>
          <div class="summary-steps__title">{step.title}</div>
          <div class="summary-steps__body">
            {@html step.body}
          </div>
        </div>
      </li>
    {/each}
  </ol>
{/if}

<style>
  .summary-steps {
    list-style: none;
    padding: 0;
    margin: 0;
    display: grid;
    gap: var(--froyo-space-3);
  }

  .summary-steps__item {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr);
    gap: var(--froyo-space-3);
    align-items: flex-start;
  }

  .summary-steps__index {
    width: var(--froyo-summary-step-index-size);
    height: var(--froyo-summary-step-index-size);
    border-radius: var(--froyo-radius-pill);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: var(--froyo-color-primary);
    color: var(--froyo-color-on-primary);
    font-size: var(--froyo-font-size-sm);
    font-weight: 600;
    margin-top: calc(var(--froyo-space-1) * 0.8);
  }

  .summary-steps__title {
    font-size: 0.9rem;
    font-weight: 600;
    margin-bottom: calc(var(--froyo-space-1) * 0.6);
  }

  .summary-steps__body :global(p) {
    margin-block: calc(var(--froyo-space-1) * 0.6);
  }
</style>
