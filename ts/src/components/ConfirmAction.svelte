<script lang="ts">
  import AlertDialog from "./AlertDialog.svelte";
  import TextButton from "./TextButton.svelte";

  export let open = false;

  export let triggerLabel = "Open";
  export let triggerVariant: "default" | "danger" = "default";

  export let title: string;
  export let description: string | null = null;

  export let confirmLabel = "Confirm";
  export let cancelLabel = "Cancel";

  export let onConfirm: (() => void | Promise<void>) | undefined = undefined;
  export let onCancel: (() => void) | undefined = undefined;
</script>

<TextButton type="button" variant={triggerVariant} on:click={() => (open = true)}>
  <slot name="trigger">{triggerLabel}</slot>
</TextButton>

<AlertDialog
  bind:open
  showTrigger={false}
  {title}
  {description}
  {confirmLabel}
  {cancelLabel}
  {onConfirm}
  {onCancel}
/>
