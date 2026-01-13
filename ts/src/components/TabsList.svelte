<script lang="ts">
  import { Tabs as BitsTabs } from "bits-ui";
  import type { Snippet } from "svelte";

  interface Props {
    children?: Snippet;
    class?: string;
  }

  let { children, class: className }: Props = $props();
</script>

<BitsTabs.List
  class={`underlay-tabs-list ${className ?? ""}`}
>
  {@render children?.()}
</BitsTabs.List>

<style>
  :global(.underlay-tabs-list) {
    display: inline-flex;
    gap: 0.25rem;
    padding: 0.25rem;
    border-radius: 999px;
    border: 1px solid rgba(148, 163, 184, 0.35);
    background: rgba(255, 255, 255, 0.03);
  }
</style>
