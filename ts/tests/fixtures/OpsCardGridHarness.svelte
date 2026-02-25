<script lang="ts">
  import OpsCardGrid from "../../src/patterns/OpsCardGrid.svelte";

  interface Props {
    minColumnWidth?: string;
    class?: string;
    withChildren?: boolean;
  }

  let {
    minColumnWidth = "18rem",
    class: className = "",
    withChildren = true
  }: Props = $props();
</script>

{#snippet gridChildren()}
  <div data-testid="ops-grid-child-a">A</div>
  <div data-testid="ops-grid-child-b">B</div>
{/snippet}

<OpsCardGrid {minColumnWidth} class={className} children={withChildren ? gridChildren : undefined} />
