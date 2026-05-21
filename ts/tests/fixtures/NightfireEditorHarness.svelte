<script lang="ts">
  import { untrack } from "svelte";
  import NightfireEditor from "../../src/nightfire/NightfireEditor.svelte";
  import type {
    NightfireBlockOptionInput,
    NightfireSlashCommandsConfig,
    NightfireValue
  } from "../../src/nightfire";

  interface Props {
    schema?: string;
    initialValue?: NightfireValue;
    modeOverride?: "single" | "multi" | null;
    defaultTypeOverride?: string | null;
    blockOptions?: NightfireBlockOptionInput[] | null;
    slashCommands?: NightfireSlashCommandsConfig | null;
  }

  let {
    schema = "acow:content/markup@1",
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
    modeOverride = null,
    defaultTypeOverride = null,
    blockOptions = null,
    slashCommands = null
  }: Props = $props();

  let value = $state(untrack(() => structuredClone(initialValue)));
</script>

<NightfireEditor
  name="body"
  {schema}
  bind:value
  {modeOverride}
  {defaultTypeOverride}
  {blockOptions}
  {slashCommands}
/>

<pre data-testid="nightfire-value">{JSON.stringify(value, null, 2)}</pre>
