<script lang="ts">
  import ConfirmAction from "../../src/components/ConfirmAction.svelte";

  interface Props {
    title?: string;
    description?: string | null;
    triggerLabel?: string;
    triggerVariant?: "default" | "danger";
    confirmLabel?: string;
    cancelLabel?: string;
    onConfirm?: () => void | Promise<void>;
    onCancel?: () => void;
  }

  let {
    title = "Confirm change",
    description = "Are you sure?",
    triggerLabel = "Remove",
    triggerVariant = "danger",
    confirmLabel = "Yes",
    cancelLabel = "No",
    onConfirm = undefined,
    onCancel = undefined
  }: Props = $props();
</script>

<ConfirmAction
  {title}
  {description}
  {triggerLabel}
  {triggerVariant}
  {confirmLabel}
  {cancelLabel}
  {onConfirm}
  {onCancel}
/>
