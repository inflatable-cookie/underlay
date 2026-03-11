<script lang="ts">
  import { untrack } from "svelte";
  import NightfireEditor from "../../src/nightfire/NightfireEditor.svelte";
  import type { NightfireSlashCommandsConfig, NightfireValue } from "../../src/nightfire";

  interface Props {
    initialValue?: NightfireValue;
    slashCommands?: NightfireSlashCommandsConfig | null;
  }

  let {
    initialValue = {
      schema: "acow:content/markup@1",
      blocks: [
        {
          type: "markdown",
          version: "initial",
          hash: "",
          data: {
            text: ""
          }
        }
      ]
    },
    slashCommands = null
  }: Props = $props();

  let value = $state(untrack(() => structuredClone(initialValue)));
</script>

<NightfireEditor
  name="body"
  schema="acow:content/markup@1"
  bind:value
  {slashCommands}
/>

<pre data-testid="nightfire-value">{JSON.stringify(value, null, 2)}</pre>
