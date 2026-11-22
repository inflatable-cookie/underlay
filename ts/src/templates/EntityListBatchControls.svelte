<script lang="ts">
  import { AlertDialog, BulkActionBar, Dialog } from "@inflatable-cookie/poodle-svelte";
  import type { BulkAction } from "@inflatable-cookie/poodle-svelte";
  import type { BatchAction } from "../patterns/batch-actions.svelte";
  import type { BatchActionConfig } from "./template.types";

  interface Props {
    showBar: boolean;
    selectionCount: number;
    totalCount: number;
    selectedIds: string[];
    bulkActions: BulkAction[];
    executing: boolean;
    pendingAction: BatchAction<string> | null;
    confirmDescription: string;
    pendingDialogAction: BatchActionConfig | null;
    onClearSelection: () => void;
    onSelectAll: () => void;
    onAction: (actionId: string) => void;
    onConfirmPendingAction: () => Promise<void>;
    onCancelPendingAction: () => void;
    onDialogSubmit: (values: Record<string, unknown>) => Promise<void>;
    onDialogCancel: () => void;
  }

  let {
    showBar,
    selectionCount,
    totalCount,
    selectedIds,
    bulkActions,
    executing,
    pendingAction,
    confirmDescription,
    pendingDialogAction,
    onClearSelection,
    onSelectAll,
    onAction,
    onConfirmPendingAction,
    onCancelPendingAction,
    onDialogSubmit,
    onDialogCancel
  }: Props = $props();
</script>

<!-- Batch action bar -->
{#if showBar}
  <BulkActionBar
    {selectionCount}
    {totalCount}
    actions={bulkActions}
    loading={executing}
    showSelectAll
    allSelected={selectionCount > 0 && selectionCount === totalCount}
    onClear={onClearSelection}
    onSelectAll={onSelectAll}
    onAction={onAction}
  />
{/if}

<!-- Batch action confirmation dialog -->
{#if pendingAction}
  <AlertDialog
    open={true}
    title={pendingAction.confirm?.title ?? "Confirm"}
    description={confirmDescription}
    confirmLabel={pendingAction.confirm?.confirmLabel ?? "Confirm"}
    cancelLabel={pendingAction.confirm?.cancelLabel ?? "Cancel"}
    tone={pendingAction.variant === "danger" ? "danger" : "warning"}
    onConfirm={onConfirmPendingAction}
    onCancel={onCancelPendingAction}
  />
{/if}

<!-- Batch action custom dialog -->
{#if pendingDialogAction?.dialog}
  <Dialog
    open={true}
    title={pendingDialogAction.dialog.title}
    onRequestClose={onDialogCancel}
  >
    {@render pendingDialogAction.dialog.content({
      ids: selectedIds,
      onSubmit: onDialogSubmit,
      onCancel: onDialogCancel
    })}
  </Dialog>
{/if}
