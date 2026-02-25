<script lang="ts">
  import AlertDialog from "../../src/components/AlertDialog.svelte";

  interface Props {
    initialOpen?: boolean;
    title?: string;
    description?: string | null;
    showTrigger?: boolean;
    triggerLabel?: string;
    confirmLabel?: string;
    cancelLabel?: string;
    withChildren?: boolean;
    onConfirm?: () => void | Promise<void>;
    onCancel?: () => void;
  }

  let {
    initialOpen = false,
    title = "Delete item?",
    description = "This action cannot be undone.",
    showTrigger = true,
    triggerLabel = "Open alert",
    confirmLabel = "Delete",
    cancelLabel = "Cancel",
    withChildren = true,
    onConfirm = undefined,
    onCancel = undefined
  }: Props = $props();

  let open = $state(false);
  let initialized = $state(false);
  $effect(() => {
    if (!initialized) {
      open = initialOpen;
      initialized = true;
    }
  });
</script>

{#snippet childSnippet()}
  <p data-testid="alert-dialog-child">Danger zone</p>
{/snippet}

<p data-testid="alert-open-state">{open ? "open" : "closed"}</p>

<AlertDialog
  bind:open
  {title}
  {description}
  {showTrigger}
  {triggerLabel}
  {confirmLabel}
  {cancelLabel}
  {onConfirm}
  {onCancel}
  children={withChildren ? childSnippet : undefined}
/>
