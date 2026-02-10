<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    minColumnWidth?: string;
    class?: string;
    children?: Snippet;
  }

  let {
    minColumnWidth = "18rem",
    class: className,
    children,
  }: Props = $props();

  let gridStyle = $derived(`--underlay-ops-card-grid-min: ${minColumnWidth};`);
</script>

<section class={`underlay-ops-card-grid ${className ?? ""}`} style={gridStyle}>
  {@render children?.()}
</section>

<style>
  .underlay-ops-card-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(var(--underlay-ops-card-grid-min), 1fr));
    gap: 0.75rem;
    margin-bottom: 1rem;
  }
  
  @media (max-width: 640px) {
    .underlay-ops-card-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
