<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    class?: string;
    minItemWidth?: string;
    columns?: number;
    gap?: string;
    children?: Snippet;
  }

  let {
    class: className = "",
    minItemWidth = "150px",
    columns = 4,
    gap = "0.6rem",
    children
  }: Props = $props();
</script>

<div
  class={`underlay-filter-toolbar ${className}`}
  style={`--underlay-filter-toolbar-min-width:${minItemWidth};--underlay-filter-toolbar-columns:${columns};--underlay-filter-toolbar-gap:${gap};`}
>
  {@render children?.()}
</div>

<style>
  .underlay-filter-toolbar {
    width: 100%;
    max-width: 1120px;
    display: grid;
    grid-template-columns: repeat(
      var(--underlay-filter-toolbar-columns, 4),
      minmax(var(--underlay-filter-toolbar-min-width, 150px), 1fr)
    );
    gap: var(--underlay-filter-toolbar-gap, 0.6rem);
  }

  @media (max-width: 960px) {
    .underlay-filter-toolbar {
      grid-template-columns: repeat(2, minmax(var(--underlay-filter-toolbar-min-width, 150px), 1fr));
    }
  }

  @media (max-width: 640px) {
    .underlay-filter-toolbar {
      grid-template-columns: 1fr;
    }
  }
</style>
