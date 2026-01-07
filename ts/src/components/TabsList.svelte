<script lang="ts">
  import { Tabs as BitsTabs } from "bits-ui";
</script>

<BitsTabs.List
  {...$$restProps}
  class={`underlay-tabs-list ${$$restProps.class ?? ""}`}
>
  <slot />
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
