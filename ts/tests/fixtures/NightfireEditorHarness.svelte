<script lang="ts">
  import { untrack } from "svelte";
  import {
    NightfireEditor,
    type NightfireBlockOptionInput,
    type NightfireSlashCommandsConfig,
    type NightfireValue
  } from "@inflatable-cookie/underlay/nightfire/editor";

  interface Props {
    schema?: string;
    initialValue?: NightfireValue;
    modeOverride?: "single" | "multi" | null;
    defaultTypeOverride?: string | null;
    blockOptions?: NightfireBlockOptionInput[] | null;
    slashCommands?: NightfireSlashCommandsConfig | null;
  }

  let {
    schema = "acow:content/markup",
    initialValue = {
      schema: "acow:content/markup",
      blocks: [
        {
          type: "markdown",
          version: "initial",
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
