<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    class?: string;
    density?: "default" | "compact";
    align?: "start" | "end";
    wrap?: boolean;
    children?: Snippet;
  }

  let {
    class: className = "",
    density = "default",
    align = "start",
    wrap = true,
    children
  }: Props = $props();
</script>

<div
  class={`underlay-inline-action-group underlay-inline-action-group--${density} underlay-inline-action-group--${align} ${className}`}
  class:underlay-inline-action-group--nowrap={!wrap}
>
  {@render children?.()}
</div>

<style>
  .underlay-inline-action-group {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .underlay-inline-action-group--compact {
    gap: 0.35rem;
  }

  .underlay-inline-action-group--end {
    justify-content: flex-end;
  }

  .underlay-inline-action-group--nowrap {
    flex-wrap: nowrap;
  }
</style>
