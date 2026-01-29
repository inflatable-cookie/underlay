<script lang="ts">
  import Button from "./Button.svelte";
  import SplitButton from "./SplitButton.svelte";

  type SaveIntent = "save" | "save-close";
  type SaveMode = "create" | "edit";

  interface Props {
    mode?: SaveMode;
    intent?: SaveIntent;
    type?: "button" | "submit";
    variant?: "primary" | "secondary";
    createLabel?: string;
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
    saveLabel = "Save changes",
    saveAndCloseLabel = "Save & close",
    disabled = false,
    onclick = undefined
  }: Props = $props();

  // In create mode, always use save-close intent (create and navigate away)
  $effect(() => {
    if (mode === "create") {
      intent = "save-close";
    }
  });

  let editOptions = $derived([
    { value: "save", label: saveLabel },
    { value: "save-close", label: saveAndCloseLabel }
  ]);
</script>

{#if mode === "create"}
  <Button {type} {variant} {disabled} {onclick}>{createLabel}</Button>
{:else}
  <SplitButton {type} {variant} options={editOptions} {disabled} {onclick} bind:value={intent} />
{/if}
