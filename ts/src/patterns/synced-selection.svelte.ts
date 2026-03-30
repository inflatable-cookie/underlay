/**
 * Helper for managing selection state that syncs from multiple sources.
 *
 * This pattern is common in form pages that need to:
 * 1. Initialize selection from URL parameters or loaded data (one-time)
 * 2. Restore selection from form values after validation failure
 * 3. Allow user to change the selection
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { useSyncedSelection } from '@decodelabs/underlay/patterns';
 *
 *   const preselectedId = $derived(pageData.data?.areaId);
 *
 *   const selection = useSyncedSelection<string>();
 *
 *   // Initialize from loaded data (runs once when data loads)
 *   $effect(() => {
 *     selection.initializeFrom(preselectedId);
 *   });
 *
 *   // Sync from form values (after validation failure)
 *   $effect(() => {
 *     if (typeof formValues?.areaId === 'string') {
 *       selection.syncFrom(formValues.areaId);
 *     }
 *   });
 * </script>
 *
 * <AreaSelector
 *   value={selection.value}
 *   onchange={(val) => { selection.value = val; }}
 * />
 * ```
 *
 * @module
 */

export interface SyncedSelectionResult<T> {
  /** Current selected value (reactive) */
  value: T | null;

  /** Whether the selection has been initialized from data */
  readonly hasInitialized: boolean;

  /**
   * Initialize the selection from a source value (one-time).
   * Only sets the value if not already initialized and source is non-null.
   * Call this in an $effect with your data source as a dependency.
   */
  initializeFrom: (source: T | null | undefined) => void;

  /**
   * Sync the selection from another source (e.g., form values).
   * Always updates if the source is non-null.
   * Call this in an $effect with your form values as a dependency.
   */
  syncFrom: (source: T | null | undefined) => void;

  /**
   * Reset the selection state, allowing re-initialization.
   */
  reset: () => void;
}

/**
 * Create a synced selection state manager.
 *
 * @param initialValue - Optional initial value (defaults to null)
 * @returns Reactive selection state with initialization and sync methods
 */
export function useSyncedSelection<T>(
  initialValue: T | null = null
): SyncedSelectionResult<T> {
  let value = $state<T | null>(initialValue);
  let hasInitialized = $state(initialValue !== null);

  const initializeFrom = (source: T | null | undefined) => {
    if (!hasInitialized && source != null) {
      hasInitialized = true;
      value = source;
    }
  };

  const syncFrom = (source: T | null | undefined) => {
    if (source != null) {
      value = source;
    }
  };

  const reset = () => {
    hasInitialized = false;
    value = null;
  };

  return {
    get value() {
      return value;
    },
    set value(v: T | null) {
      value = v;
    },
    get hasInitialized() {
      return hasInitialized;
    },
    initializeFrom,
    syncFrom,
    reset
  };
}
