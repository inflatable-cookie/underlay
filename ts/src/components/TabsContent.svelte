<script lang="ts">
  import { Tabs as BitsTabs } from "bits-ui";

  export let value: string;
</script>

<BitsTabs.Content
  {...$$restProps}
  {value}
  class={`underlay-tabs-content ${$$restProps.class ?? ""}`}
>
  <slot />
</BitsTabs.Content>

<style>
  :global(.underlay-tabs-content) {
    margin-top: 0.75rem;
  }
</style>
