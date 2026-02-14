<script lang="ts">
  import type { Snippet } from "svelte";
  import AlertDialog from "../components/AlertDialog.svelte";
  import CopyActionsMenu from "./CopyActionsMenu.svelte";
  import { useToasts } from "./useToasts";
  import type { ToastStore } from "./toasts";

  type CopyItem = {
    label: string;
    text: string;
    successMessage: string;
    failureMessage?: string;
  };

  type DropdownMenuItem = {
    label?: string;
    onSelect?: (() => void) | undefined;
    disabled?: boolean;
    destructive?: boolean;
    separator?: boolean;
  };

  interface DeleteConfig {
    /** Label shown in the confirmation dialog (e.g., the entity title) */
    entityLabel: string;
    /** Dialog title */
    title: string;
    /** Dialog description */
    description: string;
    /** Confirm button label */
    confirmLabel?: string;
    /** Async function to execute the delete */
    execute: () => Promise<void>;
  }

  interface Props {
    /** Toast store for clipboard and error notifications */
    toastStore?: ToastStore;
    /** Copy-to-clipboard items */
    copies?: CopyItem[];
    /** Edit action label (default: "Edit") */
    editLabel?: string;
    /** Callback when edit is selected */
    onEdit?: () => void;
    /** Soft-delete configuration (omit to hide delete action) */
    deleteConfig?: DeleteConfig;
    /** Callback after successful delete */
    onDeleteSuccess?: () => void;
    /** Custom actions inserted between edit and delete */
    customActions?: DropdownMenuItem[];
    /** Custom trigger snippet */
    trigger?: Snippet;
    /** Dropdown alignment */
    align?: "start" | "center" | "end";
    /** Dropdown side */
    side?: "top" | "right" | "bottom" | "left";
    /** Additional CSS class */
    class?: string;
  }

  let {
    toastStore: externalToastStore = undefined,
    copies = [],
    editLabel = "Edit",
    onEdit = undefined,
    deleteConfig = undefined,
    onDeleteSuccess = undefined,
    customActions = [],
    trigger,
    align = "end",
    side = "bottom",
    class: className = undefined
  }: Props = $props();

  // Use provided toast store or fall back to context-based one
  const internalToastStore = useToasts();
  const toast = $derived(externalToastStore ?? internalToastStore);

  // Soft delete dialog state
  let deleteOpen = $state(false);

  function requestDelete() {
    deleteOpen = true;
  }

  async function confirmDelete() {
    if (!deleteConfig) return;

    try {
      await deleteConfig.execute();
      deleteOpen = false;
      onDeleteSuccess?.();
    } catch (e) {
      console.error("Entity delete failed", e);
      toast.push({
        variant: "error",
        message: e instanceof Error ? e.message : "Delete failed. Please try again."
      });
      throw e;
    }
  }

  function cancelDelete() {
    deleteOpen = false;
  }

  // Build actions array
  const actions = $derived.by((): DropdownMenuItem[] => {
    const items: DropdownMenuItem[] = [];

    // Edit action (first)
    if (onEdit) {
      items.push({
        label: editLabel,
        onSelect: () => onEdit()
      });
    }

    // Custom actions (middle)
    if (customActions.length > 0) {
      items.push(...customActions);
    }

    // Delete action (last, destructive)
    if (deleteConfig) {
      items.push({
        label: deleteConfig.confirmLabel ?? "Delete",
        destructive: true,
        onSelect: () => requestDelete()
      });
    }

    return items;
  });
</script>

<CopyActionsMenu
  class={className}
  toastStore={toast}
  {trigger}
  {copies}
  {actions}
  {align}
  {side}
/>

{#if deleteConfig}
  <AlertDialog
    bind:open={deleteOpen}
    showTrigger={false}
    title={deleteConfig.title}
    description={deleteConfig.description}
    confirmLabel={deleteConfig.confirmLabel ?? "Delete"}
    cancelLabel="Cancel"
    onConfirm={confirmDelete}
    onCancel={cancelDelete}
  >
    <p>
      <strong>{deleteConfig.entityLabel}</strong>
    </p>
  </AlertDialog>
{/if}
