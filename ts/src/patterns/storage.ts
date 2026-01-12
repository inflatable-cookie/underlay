/**
 * SSR-safe storage wrappers for localStorage and sessionStorage.
 *
 * These wrappers prevent runtime errors when accessing browser storage APIs
 * during server-side rendering, and provide reactive Svelte stores for
 * persisted state.
 *
 * @example
 * ```typescript
 * import { storage } from '@decodelabs/underlay/patterns';
 *
 * // Simple get/set (SSR-safe)
 * const theme = storage.local.get('theme', 'light');
 * storage.local.set('theme', 'dark');
 *
 * // Reactive store with persistence
 * const preferences = storage.local.store('preferences', {
 *   darkMode: false,
 *   notifications: true
 * });
 *
 * // In Svelte component:
 * // $preferences.darkMode
 * // $preferences = { ...defaults, darkMode: true };
 * ```
 */

import { writable, type Writable } from "svelte/store";
import { browser } from "$app/environment";

// ============================================================================
// Types
// ============================================================================

/** Options for storage operations */
export interface StorageOptions {
  /** Custom serializer (default: JSON.stringify) */
  serialize?: (value: unknown) => string;
  /** Custom deserializer (default: JSON.parse) */
  deserialize?: (value: string) => unknown;
}

/** A storage wrapper with SSR-safe methods */
export interface StorageWrapper {
  /**
   * Get a value from storage.
   * Returns the default value if not found or during SSR.
   */
  get<T>(key: string, defaultValue: T, options?: StorageOptions): T;

  /**
   * Set a value in storage.
   * No-op during SSR.
   */
  set<T>(key: string, value: T, options?: StorageOptions): void;

  /**
   * Remove a value from storage.
   * No-op during SSR.
   */
  remove(key: string): void;

  /**
   * Check if a key exists in storage.
   * Returns false during SSR.
   */
  has(key: string): boolean;

  /**
   * Create a reactive Svelte store backed by storage.
   * Changes are automatically persisted.
   */
  store<T>(key: string, defaultValue: T, options?: StorageOptions): Writable<T>;

  /**
   * Clear all items from this storage type.
   * No-op during SSR.
   */
  clear(): void;
}

// ============================================================================
// Implementation
// ============================================================================

/**
 * Check if we're in a browser environment with storage available.
 */
function isStorageAvailable(type: "localStorage" | "sessionStorage"): boolean {
  if (!browser) return false;

  try {
    const storage = window[type];
    const testKey = "__underlay_storage_test__";
    storage.setItem(testKey, "test");
    storage.removeItem(testKey);
    return true;
  } catch {
    // Storage might be disabled (private browsing, quota exceeded, etc.)
    return false;
  }
}

/**
 * Create a storage wrapper for the given storage type.
 */
function createStorageWrapper(
  type: "localStorage" | "sessionStorage"
): StorageWrapper {
  const defaultSerialize = (value: unknown): string => JSON.stringify(value);
  const defaultDeserialize = (value: string): unknown => JSON.parse(value);

  function getStorage(): Storage | null {
    if (!isStorageAvailable(type)) return null;
    return window[type];
  }

  function get<T>(key: string, defaultValue: T, options?: StorageOptions): T {
    const storage = getStorage();
    if (!storage) return defaultValue;

    try {
      const item = storage.getItem(key);
      if (item === null) return defaultValue;

      const deserialize = options?.deserialize ?? defaultDeserialize;
      return deserialize(item) as T;
    } catch {
      // Parsing failed, return default
      return defaultValue;
    }
  }

  function set<T>(key: string, value: T, options?: StorageOptions): void {
    const storage = getStorage();
    if (!storage) return;

    try {
      const serialize = options?.serialize ?? defaultSerialize;
      storage.setItem(key, serialize(value));
    } catch (err) {
      // Quota exceeded or other error - log but don't throw
      console.warn(`Failed to set storage key "${key}":`, err);
    }
  }

  function remove(key: string): void {
    const storage = getStorage();
    if (!storage) return;

    storage.removeItem(key);
  }

  function has(key: string): boolean {
    const storage = getStorage();
    if (!storage) return false;

    return storage.getItem(key) !== null;
  }

  function clear(): void {
    const storage = getStorage();
    if (!storage) return;

    storage.clear();
  }

  /**
   * Create a Svelte store that persists to storage.
   *
   * The store automatically:
   * - Loads initial value from storage (or uses default)
   * - Saves changes back to storage
   * - Optionally syncs across browser tabs
   */
  function store<T>(
    key: string,
    defaultValue: T,
    options?: StorageOptions
  ): Writable<T> {
    // Get initial value from storage
    const initial = get(key, defaultValue, options);
    const { subscribe, set: setStore, update } = writable<T>(initial);

    // Wrap set/update to persist changes
    function persistingSet(value: T): void {
      set(key, value, options);
      setStore(value);
    }

    function persistingUpdate(updater: (value: T) => T): void {
      update((current) => {
        const next = updater(current);
        set(key, next, options);
        return next;
      });
    }

    // Listen for storage events from other tabs (localStorage only)
    if (browser && type === "localStorage") {
      const handleStorageEvent = (event: StorageEvent) => {
        if (event.key !== key) return;
        if (event.storageArea !== window.localStorage) return;

        try {
          const deserialize = options?.deserialize ?? defaultDeserialize;
          const newValue =
            event.newValue === null
              ? defaultValue
              : (deserialize(event.newValue) as T);
          setStore(newValue);
        } catch {
          // Ignore deserialization errors
        }
      };

      window.addEventListener("storage", handleStorageEvent);

      // Return a store with cleanup on unsubscribe
      // Note: Svelte stores don't have built-in cleanup, so this listener
      // persists for the app lifetime. This is generally fine for SPAs.
    }

    return {
      subscribe,
      set: persistingSet,
      update: persistingUpdate
    };
  }

  return { get, set, remove, has, store, clear };
}

// ============================================================================
// Exports
// ============================================================================

/**
 * SSR-safe storage utilities.
 *
 * Access localStorage and sessionStorage without SSR errors.
 *
 * @example
 * ```typescript
 * import { storage } from '@decodelabs/underlay/patterns';
 *
 * // Get with default value (SSR-safe)
 * const theme = storage.local.get('theme', 'light');
 *
 * // Set value (no-op during SSR)
 * storage.local.set('theme', 'dark');
 *
 * // Reactive store with auto-persistence
 * const prefs = storage.local.store('prefs', { notifications: true });
 * ```
 */
export const storage = {
  /** SSR-safe localStorage wrapper */
  local: createStorageWrapper("localStorage"),

  /** SSR-safe sessionStorage wrapper */
  session: createStorageWrapper("sessionStorage")
};

/**
 * Create a persisted Svelte store backed by localStorage.
 *
 * Shorthand for `storage.local.store()`.
 *
 * @example
 * ```typescript
 * const theme = createPersistedStore('theme', 'light');
 * // Use in component: $theme
 * ```
 */
export function createPersistedStore<T>(
  key: string,
  defaultValue: T,
  options?: StorageOptions
): Writable<T> {
  return storage.local.store(key, defaultValue, options);
}

/**
 * Create a session-persisted Svelte store backed by sessionStorage.
 *
 * Shorthand for `storage.session.store()`.
 *
 * @example
 * ```typescript
 * const formDraft = createSessionStore('form-draft', {});
 * // Persists only for this browser tab session
 * ```
 */
export function createSessionStore<T>(
  key: string,
  defaultValue: T,
  options?: StorageOptions
): Writable<T> {
  return storage.session.store(key, defaultValue, options);
}
