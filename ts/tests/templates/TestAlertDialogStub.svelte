<script lang="ts">
  interface Props {
    open?: boolean;
    onConfirm?: () => Promise<void> | void;
    onCancel?: () => void;
    confirmLabel?: string;
  }

  let { open = $bindable(false), onConfirm, onCancel, confirmLabel = "Confirm" }: Props = $props();
</script>

{#if open}
  <div role="alertdialog">
    <button type="button" data-testid="stub-confirm" onclick={() => onConfirm?.()}>
      {confirmLabel}
    </button>
    <button type="button" onclick={() => onCancel?.()}>Cancel</button>
  </div>
{/if}
