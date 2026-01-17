<script lang="ts">
  import { Tabs as BitsTabs } from "bits-ui";
  import type { Snippet } from "svelte";
  import { getContext } from "svelte";
  import type { TabsVariant } from "./TabsRoot.svelte";

  interface Props {
    value: string;
    children?: Snippet;
    class?: string;
  }

  let { value, children, class: className }: Props = $props();

  const getVariant = getContext<() => TabsVariant>("underlay-tabs-variant");
  let variant = $derived(getVariant?.() ?? "pills");
</script>

<BitsTabs.Content
  {value}
  class={`underlay-tabs-content underlay-tabs-content--${variant} ${className ?? ""}`}
>
  {@render children?.()}
</BitsTabs.Content>

<style>
  /* Base styles */
  :global(.underlay-tabs-content) {
    margin-top: 0.75rem;
  }

  /* Pills variant - minimal spacing */
  :global(.underlay-tabs-content--pills) {
    /* default styles */
  }

  /* Boxed variant - bordered content area */
  :global(.underlay-tabs-content--boxed) {
    margin-top: 0;
    padding: 1.25rem 0 0 0;
    border: none;
    border-top: 2px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25));
    border-radius: 0;
  }
</style>
