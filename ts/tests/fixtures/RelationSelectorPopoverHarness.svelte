<script lang="ts">
  import { Popover as BitsPopover } from "bits-ui";
  import RelationSelectorPopover from "../../src/patterns/RelationSelector/RelationSelectorPopover.svelte";
</script>

<BitsPopover.Root open={true}>
  <BitsPopover.Trigger>
    {#snippet child({ props })}
      <button type="button" {...props}>Open</button>
    {/snippet}
  </BitsPopover.Trigger>

  <RelationSelectorPopover />
</BitsPopover.Root>
