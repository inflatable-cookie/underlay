<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    align?: "start" | "end";
    children?: Snippet;
    class?: string;
  }

  let { align = "end", children, class: className }: Props = $props();
</script>

<div
  class={`underlay-card-actions underlay-card-actions--${align} ${className ?? ""}`}
>
  {@render children?.()}
</div>

<style>
  .underlay-card-actions {
    display: flex;
    padding: 0 0.1rem;
  }

  .underlay-card-actions--end {
    justify-content: flex-end;
  }

  .underlay-card-actions--start {
    justify-content: flex-start;
  }
</style>
