<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    class?: string;
    label?: string;
    children?: Snippet;
  }

  let { class: className = "", label, children }: Props = $props();
</script>

<div class={`underlay-control-bar-group ${className}`}>
  {#if label}
    <span class="underlay-control-bar-group__label">{label}</span>
  {/if}
  <div class="underlay-control-bar-group__items">
    {@render children?.()}
  </div>
</div>

<style>
  .underlay-control-bar-group {
    display: inline-flex;
    align-items: center;
    gap: 0.3rem;
    min-width: 0;
  }

  .underlay-control-bar-group__label {
    font-size: 0.7rem;
    font-weight: 600;
    color: var(--underlay-color-text-muted, #94a3b8);
    white-space: nowrap;
  }

  .underlay-control-bar-group__items {
    display: inline-flex;
    align-items: center;
    gap: 0.3rem;
    min-width: 0;
  }
</style>
