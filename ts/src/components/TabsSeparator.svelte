<script lang="ts">
  import { getContext } from "svelte";
  import type { TabsVariant, TabsSize } from "./TabsRoot.svelte";

  const getVariant = getContext<() => TabsVariant>("underlay-tabs-variant");
  const getSize = getContext<() => TabsSize>("underlay-tabs-size");

  let variant = $derived(getVariant?.() ?? "pills");
  let size = $derived(getSize?.() ?? "default");
</script>

<div
  class="underlay-tabs-separator underlay-tabs-separator--{variant} underlay-tabs-separator--{size}"
  role="separator"
  aria-orientation="vertical"
></div>

<style>
  .underlay-tabs-separator {
    width: 1px;
    align-self: stretch;
    background: rgba(148, 163, 184, 0.35);
    margin: 0 0.25rem;
  }

  /* Pills variant */
  .underlay-tabs-separator--pills {
    margin: 0.15rem 0.35rem;
  }

  /* Boxed variant */
  .underlay-tabs-separator--boxed {
    margin: 0.35rem 0.25rem;
    margin-bottom: 0.5rem;
  }

  /* Small size */
  .underlay-tabs-separator--sm.underlay-tabs-separator--pills {
    margin: 0.1rem 0.25rem;
  }

  .underlay-tabs-separator--sm.underlay-tabs-separator--boxed {
    margin: 0.25rem 0.15rem;
    margin-bottom: 0.35rem;
  }
</style>
