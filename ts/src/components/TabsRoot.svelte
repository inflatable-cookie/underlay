<script lang="ts">
  import { Tabs as BitsTabs } from "bits-ui";
  import type { Snippet } from "svelte";

  interface Props {
    value: string;
    children?: Snippet;
  }

  let { value = $bindable(), children }: Props = $props();
</script>

<BitsTabs.Root bind:value>
  {@render children?.()}
</BitsTabs.Root>
