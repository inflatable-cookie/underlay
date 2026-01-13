<script lang="ts">
  import { Tabs as BitsTabs } from "bits-ui";
  import type { Snippet } from "svelte";

  interface Props {
    value: string;
    disabled?: boolean;
    children?: Snippet;
    class?: string;
  }

  let { value, disabled = false, children, class: className }: Props = $props();
</script>

<BitsTabs.Trigger
  {value}
  {disabled}
  class={`underlay-tabs-trigger ${className ?? ""}`}
>
  {@render children?.()}
</BitsTabs.Trigger>

<style>
  :global(.underlay-tabs-trigger) {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border: none;
    border-radius: 999px;
    padding: 0.4rem 0.75rem;
    cursor: pointer;
    background: transparent;
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, #9ca3af));
    font-size: 0.9rem;
    line-height: 1;
  }

  :global(.underlay-tabs-trigger[data-state="active"]) {
    background: rgba(148, 163, 184, 0.18);
    color: var(--underlay-color-text, var(--underlay-color-text, #e5e7eb));
  }

  :global(.underlay-tabs-trigger:hover) {
    color: var(--underlay-color-text, var(--underlay-color-text, #e5e7eb));
  }

  :global(.underlay-tabs-trigger:focus-visible) {
    outline: 2px solid rgba(59, 130, 246, 0.9);
    outline-offset: 2px;
  }

  :global(.underlay-tabs-trigger[data-disabled]) {
    opacity: 0.55;
    cursor: default;
  }
</style>
