/**
 * Selection History Tracker
 *
 * Tracks user selections in localStorage for use as suggestion hints.
 * When users select items in a RelationSelector, their recent choices
 * are stored and can be sent to the server to influence suggestion ordering.
 *
 * @example
 * ```typescript
 * import { createSelectionHistory } from '@decodelabs/underlay/patterns';
 *
 * // Create a history tracker for a specific entity type
 * const levelHistory = createSelectionHistory('learning.levels');
 *
 * // Track a selection (called automatically by RelationSelector)
 * levelHistory.track('level-123');
 *
 * // Get recent IDs to send as hints to the server
 * const hints = levelHistory.getRecentIds(); // ['level-123', 'level-456', ...]
 *
 * // Use in API call
 * const suggestions = await api.getLevels({
 *   suggestions: true,
 *   recentHints: hints.join(',')
 * });
 * ```
 */

import { storage } from "./storage.js";

// ============================================================================
// Types
// ============================================================================

/** Options for creating a selection history tracker */
export interface SelectionHistoryOptions {
  /** Maximum number of items to track (default: 20) */
  maxItems?: number;
  /** Storage type: 'local' persists across sessions, 'session' for current tab only (default: 'local') */
  storageType?: "local" | "session";
  /** Optional namespace prefix for the storage key */
  namespace?: string;
}

/** A tracked selection entry */
interface SelectionEntry {
  /** The selected item ID */
  id: string;
  /** Timestamp when selected (ms since epoch) */
  timestamp: number;
}

/** The stored history data */
interface SelectionHistoryData {
  /** Version for future migrations */
  version: 1;
  /** List of selection entries, most recent first */
  entries: SelectionEntry[];
}

/** A selection history tracker instance */
export interface SelectionHistory {
  /**
   * Track a selection. Call this when the user selects an item.
   * The item is moved to the front of the history (or added if new).
   */
  track: (id: string) => void;

  /**
   * Track multiple selections at once.
   */
  trackMultiple: (ids: string[]) => void;

  /**
   * Get the list of recently selected IDs, most recent first.
   * @param limit Optional limit on how many to return (default: all)
   */
  getRecentIds: (limit?: number) => string[];

  /**
   * Check if an ID exists in the recent history.
   */
  hasRecent: (id: string) => boolean;

  /**
   * Remove an ID from the history (e.g., if the item was deleted).
   */
  remove: (id: string) => void;

  /**
   * Clear all history for this tracker.
   */
  clear: () => void;

  /**
   * Get the storage key used by this tracker.
   * Useful for debugging or direct storage access.
   */
  getStorageKey: () => string;
}

// ============================================================================
// Implementation
// ============================================================================

const STORAGE_KEY_PREFIX = "underlay:selection-history:";
const DEFAULT_MAX_ITEMS = 20;

/**
 * Create a selection history tracker for a specific entity type.
 *
 * @param entityKey A unique key identifying the entity type, e.g., 'learning.levels'
 * @param options Configuration options
 * @returns A SelectionHistory instance
 *
 * @example
 * ```typescript
 * const levelHistory = createSelectionHistory('learning.levels', {
 *   maxItems: 10,
 *   storageType: 'local'
 * });
 *
 * // In RelationSelector onchange handler:
 * levelHistory.track(selectedId);
 *
 * // When fetching suggestions:
 * const hints = levelHistory.getRecentIds(5);
 * ```
 */
export function createSelectionHistory(
  entityKey: string,
  options: SelectionHistoryOptions = {}
): SelectionHistory {
  const {
    maxItems = DEFAULT_MAX_ITEMS,
    storageType = "local",
    namespace
  } = options;

  const storageKey = namespace
    ? `${STORAGE_KEY_PREFIX}${namespace}:${entityKey}`
    : `${STORAGE_KEY_PREFIX}${entityKey}`;

  const storageWrapper = storageType === "local" ? storage.local : storage.session;

  function getData(): SelectionHistoryData {
    return storageWrapper.get<SelectionHistoryData>(storageKey, {
      version: 1,
      entries: []
    });
  }

  function setData(data: SelectionHistoryData): void {
    storageWrapper.set(storageKey, data);
  }

  function track(id: string): void {
    const data = getData();
    const now = Date.now();

    // Remove existing entry for this ID (if any)
    const filtered = data.entries.filter((e) => e.id !== id);

    // Add to front
    const updated: SelectionEntry[] = [{ id, timestamp: now }, ...filtered];

    // Trim to max size
    const trimmed = updated.slice(0, maxItems);

    console.log("[SelectionHistory] track:", id, "storage:", storageKey, "entries:", trimmed.length);
    setData({ ...data, entries: trimmed });
  }

  function trackMultiple(ids: string[]): void {
    if (ids.length === 0) return;

    const data = getData();
    const now = Date.now();
    const idSet = new Set(ids);

    // Remove existing entries for these IDs
    const filtered = data.entries.filter((e) => !idSet.has(e.id));

    // Add new entries at front (in order provided)
    const newEntries: SelectionEntry[] = ids.map((id, index) => ({
      id,
      // Slight timestamp offset to preserve order
      timestamp: now - index
    }));

    const updated = [...newEntries, ...filtered];
    const trimmed = updated.slice(0, maxItems);

    setData({ ...data, entries: trimmed });
  }

  function getRecentIds(limit?: number): string[] {
    const data = getData();
    const ids = data.entries.map((e) => e.id);
    console.log("[SelectionHistory] getRecentIds:", storageKey, "ids:", ids);
    return limit ? ids.slice(0, limit) : ids;
  }

  function hasRecent(id: string): boolean {
    const data = getData();
    return data.entries.some((e) => e.id === id);
  }

  function remove(id: string): void {
    const data = getData();
    const filtered = data.entries.filter((e) => e.id !== id);
    setData({ ...data, entries: filtered });
  }

  function clear(): void {
    storageWrapper.remove(storageKey);
  }

  function getStorageKey(): string {
    return storageKey;
  }

  return {
    track,
    trackMultiple,
    getRecentIds,
    hasRecent,
    remove,
    clear,
    getStorageKey
  };
}

/**
 * Format recent IDs as a comma-separated string for use in query parameters.
 *
 * @example
 * ```typescript
 * const hints = formatHintsParam(history.getRecentIds(5));
 * // "id1,id2,id3,id4,id5"
 *
 * fetch(`/api/items?suggestions=true&recentHints=${hints}`);
 * ```
 */
export function formatHintsParam(ids: string[]): string {
  return ids.join(",");
}

/**
 * Parse a hints parameter string back to an array of IDs.
 *
 * @example
 * ```typescript
 * const ids = parseHintsParam(request.query.recentHints);
 * // ['id1', 'id2', 'id3']
 * ```
 */
export function parseHintsParam(param: string | null | undefined): string[] {
  if (!param || param.trim() === "") return [];
  return param.split(",").filter((id) => id.trim() !== "");
}
