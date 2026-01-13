<script lang="ts">
  import { Tabs as BitsTabs } from "bits-ui";
  import type { Snippet } from "svelte";

  interface Props {
    value: string;
    children?: Snippet;
    class?: string;
  }

  let { value, children, class: className }: Props = $props();
</script>

<BitsTabs.Content
  {value}
  class={`underlay-tabs-content ${className ?? ""}`}
>
  {@render children?.()}
</BitsTabs.Content>

<style>
  :global(.underlay-tabs-content) {
    margin-top: 0.75rem;
  }
</style>
