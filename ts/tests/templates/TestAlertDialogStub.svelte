<script lang="ts">
  interface Props {
    open?: boolean;
    title?: string;
    description?: string;
    onConfirm?: () => Promise<void> | void;
    onCancel?: () => void;
    confirmLabel?: string;
  }

  let {
    open = $bindable(false),
    title = "",
    description = "",
    onConfirm,
    onCancel,
    confirmLabel = "Confirm"
  }: Props = $props();
</script>

{#if open}
  <div role="alertdialog" aria-label={title}>
    <p data-testid="stub-description">{description}</p>
    <button type="button" data-testid="stub-confirm" onclick={() => onConfirm?.()}>
      {confirmLabel}
    </button>
    <button type="button" onclick={() => onCancel?.()}>Cancel</button>
  </div>
{/if}
