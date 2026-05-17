/**
 * Batch selection state management for multi-select list operations.
 *
 * Provides reactive state and handlers for selecting multiple items
 * in a list, with support for:
 * - Individual item toggle
 * - Select all / deselect all
 * - Selection count tracking
 * - Integration with Poodle BulkActionBar
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { useBatchSelection } from '@decodelabs/underlay/patterns';
 *   import { AlertDialog, BulkActionBar } from '@poodle/svelte';
 *
 *   const items = $derived(data.projects);
 *   const selection = useBatchSelection<string>();
 *
 *   async function handleBatchDelete() {
 *     const ids = selection.selectedIds;
 *     await deleteItems(ids);
 *     selection.clear();
 *   }
 * </script>
 *
 * {#each items as item}
 *   <input
 *     type="checkbox"
 *     checked={selection.isSelected(item.id)}
 *     onchange={(e) => selection.toggle(item.id, e.currentTarget.checked)}
 *   />
 * {/each}
 *
 * let showBatchDeleteConfirm = $state(false);
 *
 * <BulkActionBar
 *   selectionCount={selection.count}
 *   totalCount={items.length}
 *   actions={[{ id: "delete", label: "Delete", icon: "trash-2", tone: "danger" }]}
 *   showSelectAll
 *   onClear={selection.clear}
 *   onSelectAll={() => selection.selectAll(items.map(i => i.id))}
 *   onAction={() => { showBatchDeleteConfirm = true; }}
 * />
 *
 * <AlertDialog
 *   open={showBatchDeleteConfirm}
 *   title="Delete selected items"
 *   description={`Delete ${selection.count} selected item${selection.count === 1 ? "" : "s"}?`}
 *   confirmLabel={`Delete ${selection.count} item${selection.count === 1 ? "" : "s"}`}
 *   tone="danger"
 *   onConfirm={handleBatchDelete}
 *   onCancel={() => { showBatchDeleteConfirm = false; }}
 * />
 * ```
 *
 * @module
 */

export interface BatchSelectionResult<T> {
  /** Current selected IDs as an array (reactive) */
  readonly selectedIds: T[];

  /** Number of selected items (reactive) */
  readonly count: number;

  /** Whether any items are selected (reactive) */
  readonly hasSelection: boolean;

  /**
   * Check if an item is selected.
   * @param id - The item ID to check
   * @returns true if the item is selected
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
}

/**
 * Create a batch selection state manager.
 *
 * @returns Reactive batch selection state with selection methods
 */
export function useBatchSelection<T = string>(): BatchSelectionResult<T> {
  let selectedSet = $state(new Set<T>());

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

  return {
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
    set
  };
}
