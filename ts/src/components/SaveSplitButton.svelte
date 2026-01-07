<script lang="ts">
  import SplitButton from "./SplitButton.svelte";

  type SaveIntent = "save" | "save-close";
  type SaveMode = "create" | "edit";

  export let mode: SaveMode = "edit";
  export let intent: SaveIntent = "save-close";
  export let type: "button" | "submit" = "submit";
  export let variant: "primary" | "secondary" = "primary";

  export let createLabel = "Create";
  export let createAndCloseLabel = "Create & close";
  export let saveLabel = "Save changes";
  export let saveAndCloseLabel = "Save & close";

  $: options =
    mode === "create"
      ? [
          { value: "save", label: createLabel },
          { value: "save-close", label: createAndCloseLabel }
        ]
      : [
          { value: "save", label: saveLabel },
          { value: "save-close", label: saveAndCloseLabel }
        ];
</script>

<SplitButton {type} {variant} {options} bind:value={intent} />
