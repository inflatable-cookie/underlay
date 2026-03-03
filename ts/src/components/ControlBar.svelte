<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    class?: string;
    density?: "default" | "compact";
    children?: Snippet;
  }

  let { class: className = "", density = "default", children }: Props = $props();
</script>

<div class={`underlay-control-bar underlay-control-bar--${density} ${className}`}>
  {@render children?.()}
</div>

<style>
  .underlay-control-bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
  }

  .underlay-control-bar--compact {
    gap: 0.3rem;
  }
</style>
