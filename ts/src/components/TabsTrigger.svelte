<script lang="ts">
  import { Tabs as BitsTabs } from "bits-ui";
  import type { Snippet } from "svelte";
  import { getContext } from "svelte";
  import type { TabsVariant, TabsSize } from "./TabsRoot.svelte";

  interface Props {
    value: string;
    disabled?: boolean;
    children?: Snippet;
    class?: string;
    /** Optional count to display as a badge */
    count?: number | null;
  }

  let { value, disabled = false, children, class: className, count }: Props = $props();

  const getVariant = getContext<() => TabsVariant>("underlay-tabs-variant");
  const getSize = getContext<() => TabsSize>("underlay-tabs-size");
  let variant = $derived(getVariant?.() ?? "pills");
  let size = $derived(getSize?.() ?? "default");
</script>

<BitsTabs.Trigger
  {value}
  {disabled}
  class={`underlay-tabs-trigger underlay-tabs-trigger--${variant} underlay-tabs-trigger--${size} ${className ?? ""}`}
>
  {@render children?.()}
  {#if count != null}
    <span class="underlay-tabs-trigger__count">{count}</span>
  {/if}
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

  /* Underline variant - minimal with bottom border */
  :global(.underlay-tabs-trigger--underline) {
    border-radius: 0;
    border-bottom: 2px solid transparent;
    padding: 0.4rem 0.5rem;
  }

  :global(.underlay-tabs-trigger--underline[data-state="active"]) {
    color: var(--underlay-color-text, #e5e7eb);
    border-bottom-color: var(--underlay-color-primary, #3b82f6);
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

  /* Small size variant */
  :global(.underlay-tabs-trigger--sm) {
    font-size: 0.8rem;
    padding: 0.3rem 0.6rem;
  }

  :global(.underlay-tabs-trigger--sm.underlay-tabs-trigger--boxed) {
    padding: 0.4rem 0.75rem;
  }

  /* Count badge */
  .underlay-tabs-trigger__count {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    margin-left: 0.4rem;
    padding: 0.1rem 0.4rem;
    font-size: 0.75rem;
    font-weight: 500;
    line-height: 1;
    border-radius: 999px;
    background: rgba(148, 163, 184, 0.2);
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  :global(.underlay-tabs-trigger[data-state="active"]) .underlay-tabs-trigger__count {
    background: rgba(148, 163, 184, 0.3);
    color: var(--underlay-color-text, #e5e7eb);
  }

  /* Small size count badge */
  :global(.underlay-tabs-trigger--sm) .underlay-tabs-trigger__count {
    font-size: 0.65rem;
    padding: 0.05rem 0.3rem;
    margin-left: 0.3rem;
  }
</style>
