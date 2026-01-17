<script lang="ts">
  import { Tabs as BitsTabs } from "bits-ui";
  import type { Snippet } from "svelte";
  import { getContext } from "svelte";
  import type { TabsVariant } from "./TabsRoot.svelte";

  interface Props {
    children?: Snippet;
    class?: string;
  }

  let { children, class: className }: Props = $props();

  const getVariant = getContext<() => TabsVariant>("underlay-tabs-variant");
  let variant = $derived(getVariant?.() ?? "pills");
</script>

<BitsTabs.List
  class={`underlay-tabs-list underlay-tabs-list--${variant} ${className ?? ""}`}
>
  {@render children?.()}
</BitsTabs.List>

<style>
  /* Base styles */
  :global(.underlay-tabs-list) {
    display: inline-flex;
    gap: 0.25rem;
  }

  /* Pills variant (default) */
  :global(.underlay-tabs-list--pills) {
    padding: 0.25rem;
    border-radius: 999px;
    border: 1px solid rgba(148, 163, 184, 0.35);
    background: rgba(255, 255, 255, 0.03);
  }

  /* Boxed variant - traditional tabs */
  :global(.underlay-tabs-list--boxed) {
    gap: 0.35rem;
    padding: 0.35rem 0.5rem 0;
    border: none;
    background: rgba(0, 0, 0, 0.25);
    border-radius: 0.5rem 0.5rem 0 0;
  }
</style>
