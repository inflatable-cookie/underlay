<script lang="ts">
  type SummaryPanelBlock = {
    data?: {
      panels?: {
        title?: string;
        body?: string;
      }[];
    };
  };

  export let block: SummaryPanelBlock;

  const panels = Array.isArray(block?.data?.panels)
    ? block.data!.panels!
    : [];
</script>

{#if panels.length > 0}
  <div data-nightfire-summary="panel">
    {#each panels as panel}
      <section>
        {#if panel.title}
          <h3>{panel.title}</h3>
        {/if}
        {#if panel.body}
          <p>{panel.body}</p>
        {/if}
      </section>
    {/each}
  </div>
{/if}
