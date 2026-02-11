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
    container-type: inline-size;
    margin-top: 0.75rem;
  }

  /* Remove top margin from first child to avoid double spacing */
  :global(.underlay-tabs-content > *:first-child) {
    margin-top: 0;
  }

  /* Pills variant - uses default styles, no overrides needed */

  /* Underline variant - tight spacing */
  :global(.underlay-tabs-content--underline) {
    margin-top: 0.5rem;
  }

  /* Plain variant - same spacing as underline */
  :global(.underlay-tabs-content--plain) {
    margin-top: 0.5rem;
  }

  /* Boxed variant - bordered content area */
  :global(.underlay-tabs-content--boxed) {
    margin-top: 0;
    padding: 1.25rem 0 0 0;
    border: none;
    border-top: 0.3em solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25));
    border-radius: 0;
  }

  /* Form variant - generous spacing below tabs */
  :global(.underlay-tabs-content--form) {
    margin-top: 2rem;
  }

  /*
   * Form variant: override the hidden attribute so inactive panels stay in the
   * layout flow. Editors (CodeMirror / EasyMDE) that rely on measuring their
   * container on mount break under display:none.  Using height:0 + overflow
   * keeps them initialised while remaining invisible and non-interactive.
   */
  :global(.underlay-tabs-content--form[hidden]) {
    display: block !important;
    height: 0;
    overflow: hidden;
    visibility: hidden;
    margin: 0;
    padding: 0;
  }

  /* Collapsed mode - simpler styling */
  :global(.underlay-tabs--collapsed .underlay-tabs-content--boxed) {
    border-top: none;
    padding-top: 1rem;
  }
</style>
