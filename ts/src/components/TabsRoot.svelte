<script lang="ts">
  import { Tabs as BitsTabs } from "bits-ui";
  import type { Snippet } from "svelte";
  import { setContext } from "svelte";

  export type TabsVariant = "pills" | "boxed";

  interface Props {
    value: string;
    variant?: TabsVariant;
    children?: Snippet;
  }

  let { value = $bindable(), variant = "pills", children }: Props = $props();

  setContext("underlay-tabs-variant", () => variant);
</script>

<BitsTabs.Root bind:value class={`underlay-tabs underlay-tabs--${variant}`}>
  {@render children?.()}
</BitsTabs.Root>
