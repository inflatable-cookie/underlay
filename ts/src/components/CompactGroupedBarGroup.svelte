<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    class?: string;
    label?: string;
    joined?: boolean;
    /**
     * Lower values are laid out earlier in wrapped rows.
     * Use higher values for less important groups that should wrap later.
     */
    priority?: number;
    children?: Snippet;
  }

  let {
    class: className = "",
    label,
    joined = false,
    priority = 100,
    children
  }: Props = $props();
</script>

<div class={`underlay-compact-grouped-bar-group ${className}`} style={`order:${priority};`}>
  {#if label}
    <span class="underlay-compact-grouped-bar-group__label">{label}</span>
  {/if}
  <div
    class="underlay-compact-grouped-bar-group__items"
    class:underlay-compact-grouped-bar-group__items--joined={joined}
  >
    {@render children?.()}
  </div>
</div>

<style>
  .underlay-compact-grouped-bar-group {
    display: inline-flex;
    align-items: center;
    gap: 0.3rem;
    min-width: 0;
  }

  .underlay-compact-grouped-bar-group__label {
    font-size: 0.69rem;
    font-weight: 600;
    color: var(--underlay-color-text-muted, #94a3b8);
    white-space: nowrap;
  }

  .underlay-compact-grouped-bar-group__items {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    min-width: 0;
  }

  .underlay-compact-grouped-bar-group__items--joined {
    gap: 0.2rem;
    padding: 0.1rem;
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.24));
    border-radius: var(--underlay-radius-sm, 0.4rem);
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.1));
  }
</style>
