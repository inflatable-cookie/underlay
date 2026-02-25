<script lang="ts">
  import ContainerGrid from "../../src/components/ContainerGrid.svelte";

  interface Props {
    breakpoint?: number;
    gap?: string;
    stretch?: boolean;
    className?: string;
  }

  let {
    breakpoint = 700,
    gap = "1.5rem",
    stretch = false,
    className = ""
  }: Props = $props();
</script>

<ContainerGrid {breakpoint} {gap} {stretch} class={className}>
  {#snippet children()}
    <div data-testid="grid-item-a">A</div>
    <div data-testid="grid-item-b">B</div>
  {/snippet}
</ContainerGrid>
