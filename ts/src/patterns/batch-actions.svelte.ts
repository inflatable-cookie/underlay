/**
 * Batch actions state management for multi-select list operations.
 *
 * Extends `useBatchSelection` with action registration and execution,
 * providing a complete solution for batch operations in autonomous list components.
 *
 * Features:
 * - Action registration with labels, icons, and variants
 * - Confirmation dialog state management
 * - Action execution with result handling
 * - Conditional action availability based on selection
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { useBatchActions, type BatchAction } from '@decodelabs/underlay/patterns';
 *   import Trash2 from 'lucide-svelte/icons/trash-2';
 *
 *   const batch = useBatchActions<string>();
 *
 *   // Register actions
 *   batch.registerAction({
 *     id: 'delete',
 *     label: 'Delete',
 *     icon: Trash2,
 *     variant: 'danger',
 *     confirm: {
 *       title: 'Delete Items',
 *       description: (count) => `Are you sure you want to delete ${count} items?`,
 *       confirmLabel: 'Delete',
 *     },
 *     execute: async (ids) => {
 *       await deleteItems(ids);
 *       return { success: true, affected: ids.length };
 *     },
 *   });
 * </script>
 *
 * {#each items as item}
 *   <ListCard
 *     selected={batch.isSelected(item.id)}
 *     onSelectionChange={(selected) => batch.toggle(item.id, selected)}
 *   />
 * {/each}
 *
 * <BatchActionBar
 *   selectedCount={batch.count}
 *   actions={batch.actions}
 *   onAction={(actionId) => batch.requestAction(actionId)}
 * />
 *
 * {#if batch.pendingAction}
 *   <AlertDialog
 *     title={batch.pendingAction.confirm?.title}
 *     description={batch.getConfirmDescription()}
 *     onConfirm={batch.confirmPendingAction}
 *     onCancel={batch.cancelPendingAction}
 *   />
 * {/if}
 * ```
 *
 * @module
 */

import type { Component } from "svelte";

/**
 * Result of executing a batch action.
 */
export interface BatchActionResult {
  /** Whether the action succeeded */
  success: boolean;
  /** Number of items affected */
  affected: number;
  /** Optional message to display */
  message?: string;
}

/**
 * Configuration for a batch action's confirmation dialog.
 */
export interface BatchActionConfirm {
  /** Dialog title */
  title: string;
  /** Dialog description - can be static or function of selection count */
  description: string | ((count: number) => string);
  /** Confirm button label (default: "Confirm") */
  confirmLabel?: string;
  /** Cancel button label (default: "Cancel") */
  cancelLabel?: string;
}

/**
 * Definition of a batch action.
 */
export interface BatchAction<T = string> {
  /** Unique identifier for the action */
  id: string;
  /** Display label for the action */
  label: string;
  /** Optional icon component */
  icon?: Component;
  /** Visual variant (affects button styling) */
  variant?: "default" | "danger" | "warning";
  /**
   * Function to check if action is available for current selection.
   * Receives the selected items (if available) to allow item-based filtering.
   * Default: always available.
   */
  isAvailable?: (selectedIds: T[]) => boolean;
  /**
   * Execute the action on the selected items.
   * @param selectedIds - Array of selected item IDs
   * @returns Result of the action
   */
  execute: (selectedIds: T[]) => Promise<BatchActionResult>;
  /**
   * Optional confirmation dialog configuration.
   * If provided, shows confirmation before executing.
   */
  confirm?: BatchActionConfirm;
}

/**
 * State and methods for batch actions.
 */
export interface BatchActionsResult<T = string> {
  // === Selection state (from useBatchSelection) ===

  /** Current selected IDs as an array (reactive) */
  readonly selectedIds: T[];

  /** Number of selected items (reactive) */
  readonly count: number;

  /** Whether any items are selected (reactive) */
  readonly hasSelection: boolean;

  /**
   * Check if an item is selected.
   * @param id - The item ID to check
   */
  isSelected: (id: T) => boolean;

  /**
   * Toggle an item's selection state.
   * @param id - The item ID to toggle
   * @param selected - Whether to select (true) or deselect (false)
   */
  toggle: (id: T, selected: boolean) => void;

  /**
   * Select a single item.
   * @param id - The item ID to select
   */
  select: (id: T) => void;

  /**
   * Deselect a single item.
   * @param id - The item ID to deselect
   */
  deselect: (id: T) => void;

  /**
   * Select all items from the provided list.
   * @param ids - Array of all item IDs to select
   */
  selectAll: (ids: T[]) => void;

  /**
   * Clear all selections.
   */
  clear: () => void;

  /**
   * Replace the selection with a new set of IDs.
   * @param ids - Array of item IDs to set as selected
   */
  set: (ids: T[]) => void;

  // === Action state ===

  /** Registered actions (reactive) */
  readonly actions: BatchAction<T>[];

  /** Currently available actions based on selection (reactive) */
  readonly availableActions: BatchAction<T>[];

  /** Action pending confirmation (reactive) */
  readonly pendingAction: BatchAction<T> | null;

  /** Whether an action is currently executing (reactive) */
  readonly executing: boolean;

  /** Error message from last failed action (reactive) */
  readonly error: string | null;

  // === Action methods ===

  /**
   * Register a batch action.
   * @param action - Action definition
   */
  registerAction: (action: BatchAction<T>) => void;

  /**
   * Unregister a batch action.
   * @param actionId - ID of action to remove
   */
  unregisterAction: (actionId: string) => void;

  /**
   * Request an action to be executed.
   * If the action has a confirmation, sets it as pending.
   * Otherwise, executes immediately.
   * @param actionId - ID of action to execute
   */
  requestAction: (actionId: string) => Promise<BatchActionResult | null>;

  /**
   * Get the confirmation description for the pending action.
   * Resolves function-based descriptions with current count.
   */
  getConfirmDescription: () => string;

  /**
   * Confirm and execute the pending action.
   */
  confirmPendingAction: () => Promise<BatchActionResult | null>;

  /**
   * Cancel the pending action.
   */
  cancelPendingAction: () => void;

  /**
   * Clear the error state.
   */
  clearError: () => void;
}

/**
 * Create a batch actions state manager.
 *
 * @returns Reactive batch actions state with selection and action methods
 */
export function useBatchActions<T = string>(): BatchActionsResult<T> {
  // Selection state
  let selectedSet = $state(new Set<T>());

  // Action state
  let actions = $state<BatchAction<T>[]>([]);
  let pendingAction = $state<BatchAction<T> | null>(null);
  let executing = $state(false);
  let error = $state<string | null>(null);

  // === Selection methods ===

  const isSelected = (id: T): boolean => {
    return selectedSet.has(id);
  };

  const toggle = (id: T, selected: boolean): void => {
    const newSet = new Set(selectedSet);
    if (selected) {
      newSet.add(id);
    } else {
      newSet.delete(id);
    }
    selectedSet = newSet;
  };

  const select = (id: T): void => {
    if (!selectedSet.has(id)) {
      const newSet = new Set(selectedSet);
      newSet.add(id);
      selectedSet = newSet;
    }
  };

  const deselect = (id: T): void => {
    if (selectedSet.has(id)) {
      const newSet = new Set(selectedSet);
      newSet.delete(id);
      selectedSet = newSet;
    }
  };

  const selectAll = (ids: T[]): void => {
    selectedSet = new Set(ids);
  };

  const clear = (): void => {
    selectedSet = new Set();
  };

  const set = (ids: T[]): void => {
    selectedSet = new Set(ids);
  };

  // === Action methods ===

  const registerAction = (action: BatchAction<T>): void => {
    // Replace if exists, otherwise add
    const existingIndex = actions.findIndex((a) => a.id === action.id);
    if (existingIndex >= 0) {
      actions = [...actions.slice(0, existingIndex), action, ...actions.slice(existingIndex + 1)];
    } else {
      actions = [...actions, action];
    }
  };

  const unregisterAction = (actionId: string): void => {
    actions = actions.filter((a) => a.id !== actionId);
  };

  const executeAction = async (action: BatchAction<T>): Promise<BatchActionResult | null> => {
    const ids = Array.from(selectedSet);
    if (ids.length === 0) {
      return null;
    }

    executing = true;
    error = null;

    try {
      const result = await action.execute(ids);

      if (result.success) {
        // Clear selection after successful action
        clear();
      } else if (result.message) {
        error = result.message;
      }

      return result;
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : "Action failed";
      error = errorMessage;
      return { success: false, affected: 0, message: errorMessage };
    } finally {
      executing = false;
      pendingAction = null;
    }
  };

  const requestAction = async (actionId: string): Promise<BatchActionResult | null> => {
    const action = actions.find((a) => a.id === actionId);
    if (!action) {
      return null;
    }

    // Check availability
    const ids = Array.from(selectedSet);
    if (action.isAvailable && !action.isAvailable(ids)) {
      return null;
    }

    // If confirmation required, set pending
    if (action.confirm) {
      pendingAction = action;
      return null;
    }

    // Execute immediately
    return executeAction(action);
  };

  const getConfirmDescription = (): string => {
    if (!pendingAction?.confirm) {
      return "";
    }

    const desc = pendingAction.confirm.description;
    if (typeof desc === "function") {
      return desc(selectedSet.size);
    }
    return desc;
  };

  const confirmPendingAction = async (): Promise<BatchActionResult | null> => {
    if (!pendingAction) {
      return null;
    }
    return executeAction(pendingAction);
  };

  const cancelPendingAction = (): void => {
    pendingAction = null;
  };

  const clearError = (): void => {
    error = null;
  };

  // Compute available actions based on selection
  const getAvailableActions = (): BatchAction<T>[] => {
    const ids = Array.from(selectedSet);
    return actions.filter((action) => {
      if (!action.isAvailable) {
        return true;
      }
      return action.isAvailable(ids);
    });
  };

  return {
    // Selection
    get selectedIds() {
      return Array.from(selectedSet);
    },
    get count() {
      return selectedSet.size;
    },
    get hasSelection() {
      return selectedSet.size > 0;
    },
    isSelected,
    toggle,
    select,
    deselect,
    selectAll,
    clear,
    set,

    // Actions
    get actions() {
      return actions;
    },
    get availableActions() {
      return getAvailableActions();
    },
    get pendingAction() {
      return pendingAction;
    },
    get executing() {
      return executing;
    },
    get error() {
      return error;
    },
    registerAction,
    unregisterAction,
    requestAction,
    getConfirmDescription,
    confirmPendingAction,
    cancelPendingAction,
    clearError,
  };
}
