<script lang="ts">
  import SplitButton from "./SplitButton.svelte";

  type SaveIntent = "save" | "save-close";
  type SaveMode = "create" | "edit";

  interface Props {
    mode?: SaveMode;
    intent?: SaveIntent;
    type?: "button" | "submit";
    variant?: "primary" | "secondary";
    createLabel?: string;
    createAndCloseLabel?: string;
    saveLabel?: string;
    saveAndCloseLabel?: string;
    disabled?: boolean;
    onclick?: () => void;
  }

  let {
    mode = "edit",
    intent = $bindable("save-close"),
    type = "submit",
    variant = "primary",
    createLabel = "Create",
    createAndCloseLabel = "Create & close",
    saveLabel = "Save changes",
    saveAndCloseLabel = "Save & close",
    disabled = false,
    onclick = undefined
  }: Props = $props();

  let options = $derived(
    mode === "create"
      ? [
          { value: "save", label: createLabel },
          { value: "save-close", label: createAndCloseLabel }
        ]
      : [
          { value: "save", label: saveLabel },
          { value: "save-close", label: saveAndCloseLabel }
        ]
  );
</script>

<SplitButton {type} {variant} {options} {disabled} {onclick} bind:value={intent} />
