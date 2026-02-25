<script lang="ts">
  import StatGrid from "../../src/components/StatGrid.svelte";

  interface Props {
    columns?: 2 | 3 | 4 | 5;
    minItemWidth?: number;
  }

  let {
    columns = 4,
    minItemWidth = 250
  }: Props = $props();
</script>

<StatGrid {columns} {minItemWidth}>
  <div data-testid="stat-grid-item-a">A</div>
  <div data-testid="stat-grid-item-b">B</div>
</StatGrid>
