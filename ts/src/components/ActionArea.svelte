<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    align?: "start" | "end";
    density?: "default" | "compact";
    class?: string;
    children?: Snippet;
    aside?: Snippet;
  }

  let { align = "start", density = "default", class: className = "", children, aside }: Props = $props();
  let hasChildren = $derived(typeof children === "function");
  let hasAside = $derived(typeof aside === "function");
</script>

<div class={`underlay-action-area underlay-action-area--${align} underlay-action-area--${density} ${className}`}>
  {#if hasChildren}
    {@render children?.()}
  {/if}
  {#if hasAside}
    <div class="underlay-action-area__aside">
      {@render aside?.()}
    </div>
  {/if}
</div>

<style>
  .underlay-action-area {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--underlay-form-actions-gap, 1.5rem);
  }

  .underlay-action-area--start {
    justify-content: flex-start;
  }

  .underlay-action-area--end {
    justify-content: flex-end;
  }

  .underlay-action-area--compact {
    --underlay-form-actions-gap: var(--underlay-space-1, 0.5rem);
  }

  .underlay-action-area__aside {
    margin-left: auto;
    display: flex;
    align-items: center;
    gap: var(--underlay-form-actions-gap, 1.5rem);
  }
</style>
