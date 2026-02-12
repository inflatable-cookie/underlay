<script lang="ts">
  /**
   * BatchActionBar - A fixed toolbar that appears when items are selected.
   *
   * Shows selection count and action buttons for batch operations.
   * Supports both static actions (via props) and dynamic actions (via registeredActions).
   */
  import type { Component, Snippet } from "svelte";
  import Button from "./Button.svelte";
  import AlertDialog from "./AlertDialog.svelte";
  import Dialog from "./Dialog.svelte";
  import X from "lucide-svelte/icons/x";
  import Trash2 from "lucide-svelte/icons/trash-2";
  import CheckCheck from "lucide-svelte/icons/check-check";
  import CheckSquare from "lucide-svelte/icons/check-square";
  import Square from "lucide-svelte/icons/square";

  /**
   * Dynamic action configuration for integration with useBatchActions.
   */
  interface DynamicAction {
    /** Unique identifier */
    id: string;
    /** Display label */
    label: string;
    /** Optional icon component */
    icon?: Component;
    /** Visual variant */
    variant?: "default" | "danger" | "warning";
  }

  interface Props {
    /** Number of selected items */
    selectedCount: number;
    /** Total number of items available for selection */
    totalCount?: number;
    /** Whether a batch operation is in progress */
    loading?: boolean;
    /** Show batch delete action */
    showDelete?: boolean;
    /** Show batch status update action */
    showStatusUpdate?: boolean;
    /** Available status options for status update */
    statusOptions?: { value: string; label: string }[];
    /** Item label (singular) - defaults to "item" */
    itemLabel?: string;
    /** Item label (plural) - defaults to "items" */
    itemLabelPlural?: string;
    /**
     * Dynamic actions from useBatchActions controller.
     * These are rendered as buttons before the static actions.
     */
    registeredActions?: DynamicAction[];
    /** Callback when clear selection is clicked */
    onClearSelection: () => void;
    /** Callback when select all is clicked */
    onSelectAll?: () => void;
    /** Callback when batch delete is confirmed */
    onBatchDelete?: () => void;
    /** Callback when batch status update is confirmed */
    onBatchStatusUpdate?: (status: string) => void;
    /**
     * Callback when a dynamic action is requested.
     * The action may need confirmation before executing.
     */
    onAction?: (actionId: string) => void;
    /** Custom actions slot */
    actions?: Snippet;
  }

  let {
    selectedCount,
    totalCount = 0,
    loading = false,
    showDelete = true,
    showStatusUpdate = false,
    statusOptions = [],
    itemLabel = "item",
    itemLabelPlural = "items",
    registeredActions = [],
    onClearSelection,
    onSelectAll,
    onBatchDelete,
    onBatchStatusUpdate,
    onAction,
    actions
  }: Props = $props();

  /**
   * Map action variant to Button variant.
   */
  function getButtonVariant(actionVariant: DynamicAction["variant"]): "subtle" | "danger" | "danger-subtle" {
    switch (actionVariant) {
      case "danger":
        return "danger";
      case "warning":
        return "danger-subtle";
      default:
        return "subtle";
    }
  }

  const allSelected = $derived(totalCount > 0 && selectedCount === totalCount);
  const itemText = $derived(selectedCount === 1 ? itemLabel : itemLabelPlural);

  let showDeleteConfirm = $state(false);
  let showStatusModal = $state(false);
  let selectedStatus = $state("");

  function handleDeleteClick() {
    showDeleteConfirm = true;
  }

  function confirmDelete() {
    showDeleteConfirm = false;
    onBatchDelete?.();
  }

  function handleStatusClick() {
    showStatusModal = true;
  }

  function confirmStatusUpdate() {
    if (selectedStatus) {
      showStatusModal = false;
      onBatchStatusUpdate?.(selectedStatus);
      selectedStatus = "";
    }
  }
</script>

{#if selectedCount > 0}
  <div class="underlay-batch-action-bar">
    <div class="underlay-selection-info">
      <span class="underlay-count">{selectedCount}</span>
      <span class="underlay-label">{itemText} selected</span>
    </div>

    <div class="underlay-actions">
      {#if onSelectAll && totalCount > 0}
        <Button
          type="button"
          variant="subtle"
          size="sm"
          onclick={allSelected ? onClearSelection : onSelectAll}
          disabled={loading}
        >
          {#if allSelected}
            <Square size={16} />
            Deselect All
          {:else}
            <CheckSquare size={16} />
            Select All ({totalCount})
          {/if}
        </Button>
      {/if}

      {#if showStatusUpdate && statusOptions.length > 0}
        <Button
          type="button"
          variant="subtle"
          size="sm"
          onclick={handleStatusClick}
          disabled={loading}
        >
          <CheckCheck size={16} />
          Update Status
        </Button>
      {/if}

      {#each registeredActions as action (action.id)}
        {@const ActionIcon = action.icon}
        <Button
          type="button"
          variant={getButtonVariant(action.variant)}
          size="sm"
          onclick={() => onAction?.(action.id)}
          disabled={loading}
        >
          {#if ActionIcon}
            <ActionIcon size={16} />
          {/if}
          {action.label}
        </Button>
      {/each}

      {#if actions}
        {@render actions()}
      {/if}

      {#if showDelete}
        <Button
          type="button"
          variant="danger"
          size="sm"
          onclick={handleDeleteClick}
          disabled={loading}
        >
          <Trash2 size={16} />
          Delete
        </Button>
      {/if}

      <Button
        type="button"
        variant="subtle"
        size="sm"
        onclick={onClearSelection}
        disabled={loading}
      >
        <X size={16} />
        Clear
      </Button>
    </div>
  </div>
{/if}

<!-- Delete confirmation -->
<AlertDialog
  bind:open={showDeleteConfirm}
  showTrigger={false}
  title="Confirm Delete"
  description={`Are you sure you want to delete ${selectedCount} ${itemText}? This action cannot be undone.`}
  confirmLabel={`Delete ${selectedCount} ${itemText}`}
  onConfirm={confirmDelete}
/>

<!-- Status update dialog -->
{#if showStatusUpdate}
  <Dialog
    bind:open={showStatusModal}
    title="Update Status"
  >
    <p class="underlay-confirm-text">
      Update status for <strong>{selectedCount}</strong> {itemText}:
    </p>
    <div class="underlay-status-options">
      {#each statusOptions as option}
        <label class="underlay-status-option">
          <input
            type="radio"
            name="status"
            value={option.value}
            bind:group={selectedStatus}
          />
          <span>{option.label}</span>
        </label>
      {/each}
    </div>
    <div class="underlay-dialog-actions">
      <Button
        type="button"
        variant="subtle"
        onclick={() => showStatusModal = false}
      >
        Cancel
      </Button>
      <Button
        type="button"
        variant="primary"
        onclick={confirmStatusUpdate}
        disabled={!selectedStatus}
      >
        Update Status
      </Button>
    </div>
  </Dialog>
{/if}

<style>
  .underlay-batch-action-bar {
    position: fixed;
    bottom: 1.5rem;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    align-items: center;
    gap: 1.5rem;
    padding: 0.75rem 1.25rem;
    background: var(--surface-elevated, #1f2937);
    border: 1px solid var(--border-color, #374151);
    border-radius: 0.75rem;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
    z-index: 100;
  }

  .underlay-selection-info {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .underlay-count {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 1.75rem;
    height: 1.75rem;
    padding: 0 0.5rem;
    background: var(--accent-color, #3b82f6);
    color: white;
    font-weight: 600;
    font-size: 0.875rem;
    border-radius: 9999px;
  }

  .underlay-label {
    color: var(--text-secondary, #9ca3af);
    font-size: 0.875rem;
  }

  .underlay-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .underlay-confirm-text {
    margin: 0 0 1rem;
  }

  .underlay-status-options {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .underlay-status-option {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    border-radius: 0.375rem;
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .underlay-status-option:hover {
    background: var(--surface-hover, #374151);
  }

  .underlay-status-option input {
    accent-color: var(--accent-color, #3b82f6);
  }

  .underlay-dialog-actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    margin-top: 1.5rem;
  }
</style>
