<script lang="ts">
  import type { Snippet } from "svelte";
  import Button from "../../components/Button.svelte";

  interface Props {
    footer?: Snippet;
    multiSelect: boolean;
    createFormOpen: boolean;
    selectedCount: number;
    onCancel?: () => void;
    onConfirm?: () => void;
  }

  let {
    footer,
    multiSelect,
    createFormOpen,
    selectedCount,
    onCancel,
    onConfirm
  }: Props = $props();
</script>

{#if footer}
  <div class="relation-picker-dialog__footer">
    {@render footer()}
  </div>
{:else if multiSelect && !createFormOpen}
  <div class="relation-picker-dialog__footer">
    <Button variant="subtle" onclick={() => onCancel?.()}>
      Cancel
    </Button>
    <Button variant="primary" onclick={() => onConfirm?.()}>
      Confirm ({selectedCount})
    </Button>
  </div>
{/if}

<style>
  .relation-picker-dialog__footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    padding: 0.75rem 1rem 1rem;
    border-top: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.3));
    flex-shrink: 0;
  }
</style>
