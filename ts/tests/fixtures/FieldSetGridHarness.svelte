<script lang="ts">
  import FieldSetGrid from "../../src/components/FieldSetGrid.svelte";

  interface Props {
    columns?: 1 | 2 | 3 | 4 | 6 | "auto";
    full?: boolean;
    className?: string;
  }

  let {
    columns = 1,
    full = false,
    className = ""
  }: Props = $props();
</script>

<FieldSetGrid {columns} {full} class={className}>
  {#snippet children()}
    <div data-testid="grid-child-a">A</div>
    <div data-testid="grid-child-b">B</div>
  {/snippet}
</FieldSetGrid>
