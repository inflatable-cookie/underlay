<script lang="ts">
  import { Tabs as BitsTabs } from "bits-ui";
  import type { Snippet } from "svelte";
  import { getContext } from "svelte";
  import type { TabsVariant } from "./TabsRoot.svelte";

  interface Props {
    value: string;
    disabled?: boolean;
    children?: Snippet;
    class?: string;
  }

  let { value, disabled = false, children, class: className }: Props = $props();

  const getVariant = getContext<() => TabsVariant>("underlay-tabs-variant");
  let variant = $derived(getVariant?.() ?? "pills");
</script>

<BitsTabs.Trigger
  {value}
  {disabled}
  class={`underlay-tabs-trigger underlay-tabs-trigger--${variant} ${className ?? ""}`}
>
  {@render children?.()}
</BitsTabs.Trigger>

<style>
  /* Base styles */
  :global(.underlay-tabs-trigger) {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border: none;
    padding: 0.4rem 0.75rem;
    cursor: pointer;
    background: transparent;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.9rem;
    line-height: 1;
  }

  :global(.underlay-tabs-trigger:hover) {
    color: var(--underlay-color-text, #e5e7eb);
  }

  :global(.underlay-tabs-trigger:focus-visible) {
    outline: 2px solid rgba(59, 130, 246, 0.9);
    outline-offset: 2px;
  }

  :global(.underlay-tabs-trigger[data-disabled]) {
    opacity: 0.55;
    cursor: default;
  }

  /* Pills variant */
  :global(.underlay-tabs-trigger--pills) {
    border-radius: 999px;
  }

  :global(.underlay-tabs-trigger--pills[data-state="active"]) {
    background: rgba(148, 163, 184, 0.18);
    color: var(--underlay-color-text, #e5e7eb);
  }

  /* Boxed variant - traditional tabs */
  :global(.underlay-tabs-trigger--boxed) {
    border-radius: 0.4rem 0.4rem 0 0;
    padding: 0.55rem 1rem;
    background: var(--underlay-color-button-neutral-bg, #1f2933);
    border: 1px solid transparent;
    border-bottom: none;
  }

  :global(.underlay-tabs-trigger--boxed:hover) {
    background: var(--underlay-color-button-neutral-hover, #374151);
  }

  :global(.underlay-tabs-trigger--boxed[data-state="active"]) {
    background: var(--underlay-color-button-neutral-hover, #374151);
    color: var(--underlay-color-text, #e5e7eb);
  }
</style>
