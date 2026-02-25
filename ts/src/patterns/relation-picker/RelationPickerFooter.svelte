<script lang="ts">
  import type { Snippet } from "svelte";
  import ActionArea from "../../components/ActionArea.svelte";
  import Button from "../../components/Button.svelte";
  import TextButton from "../../components/TextButton.svelte";

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
  <ActionArea class="relation-picker-dialog__footer">
    <Button variant="primary" onclick={() => onConfirm?.()}>
      Confirm ({selectedCount})
    </Button>
    {#snippet aside()}
      <TextButton type="button" onclick={() => onCancel?.()}>
        Cancel
      </TextButton>
    {/snippet}
  </ActionArea>
{/if}

<style>
  .relation-picker-dialog__footer {
    gap: 0.5rem;
    padding: 0.75rem 1rem 1rem;
    border-top: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.3));
    flex-shrink: 0;
  }
</style>
