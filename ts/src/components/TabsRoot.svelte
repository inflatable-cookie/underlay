<script lang="ts">
  import { Tabs as BitsTabs } from "bits-ui";

  export let value: string;
</script>

<BitsTabs.Root bind:value>
  <slot />
</BitsTabs.Root>
