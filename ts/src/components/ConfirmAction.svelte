<script lang="ts">
  import type { Snippet } from "svelte";
  import AlertDialog from "./AlertDialog.svelte";
  import TextButton from "./TextButton.svelte";

  interface Props {
    open?: boolean;
    triggerLabel?: string;
    triggerVariant?: "default" | "danger";
    title: string;
    description?: string | null;
    confirmLabel?: string;
    cancelLabel?: string;
    onConfirm?: () => void | Promise<void>;
    onCancel?: () => void;
    trigger?: Snippet;
  }

  let {
    open = $bindable(false),
    triggerLabel = "Open",
    triggerVariant = "default",
    title,
    description = null,
    confirmLabel = "Confirm",
    cancelLabel = "Cancel",
    onConfirm,
    onCancel,
    trigger
  }: Props = $props();
</script>

<TextButton type="button" variant={triggerVariant} onclick={() => (open = true)}>
  {#if trigger}
    {@render trigger()}
  {:else}
    {triggerLabel}
  {/if}
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
