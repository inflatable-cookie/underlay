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
import { BROWSER } from "esm-env";

// ============================================================================
// Types
// ============================================================================

/** Options for storage operations */
export interface StorageOptions {
  /** Custom serializer (default: JSON.stringify) */
  serialize?: (value: unknown) => string;
  /** Custom deserializer (default: JSON.parse) */
  deserialize?: (value: string) => unknown;
  /** Time-to-live in seconds from the moment the value is written */
  ttl?: number;
  /** Absolute expiration time for the stored value */
  expiresAt?: Date | number;
}

interface StoredEnvelope {
  __underlay: true;
  version: 1;
  value: string;
  expiresAt?: number;
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
   * Check whether a key exists but is expired.
   * Expired keys are removed lazily when checked.
   */
  isExpired(key: string): boolean;

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
  if (!BROWSER) return false;

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
  const UNDERLAY_STORAGE_VERSION = 1;

  function getStorage(): Storage | null {
    if (!isStorageAvailable(type)) return null;
    return window[type];
  }

  function resolveExpiresAt(options?: StorageOptions): number | undefined {
    if (!options) return undefined;

    if (options.expiresAt instanceof Date) {
      return options.expiresAt.getTime();
    }

    if (typeof options.expiresAt === "number" && Number.isFinite(options.expiresAt)) {
      return options.expiresAt;
    }

    if (typeof options.ttl === "number" && Number.isFinite(options.ttl)) {
      return Date.now() + (options.ttl * 1000);
    }

    return undefined;
  }

  function isStoredEnvelope(value: unknown): value is StoredEnvelope {
    return (
      value !== null &&
      typeof value === "object" &&
      (value as Record<string, unknown>).__underlay === true &&
      (value as Record<string, unknown>).version === UNDERLAY_STORAGE_VERSION &&
      typeof (value as Record<string, unknown>).value === "string"
    );
  }

  function parseStoredItem(raw: string | null): {
    serialized: string | null;
    expiresAt?: number;
    expired: boolean;
  } {
    if (raw === null) {
      return {
        serialized: null,
        expired: false
      };
    }

    try {
      const parsed = JSON.parse(raw) as unknown;
      if (isStoredEnvelope(parsed)) {
        const expiresAt = typeof parsed.expiresAt === "number" ? parsed.expiresAt : undefined;
        const expired = typeof expiresAt === "number" && expiresAt <= Date.now();

        return {
          serialized: parsed.value,
          expiresAt,
          expired
        };
      }
    } catch {
      // Raw values that are not valid JSON should still be passed through
      // to the configured deserializer for backward compatibility.
    }

    return {
      serialized: raw,
      expired: false
    };
  }

  function serializeForStorage(value: unknown, options?: StorageOptions): string {
    const serialize = options?.serialize ?? defaultSerialize;
    const serialized = serialize(value);
    const expiresAt = resolveExpiresAt(options);

    if (expiresAt === undefined) {
      return serialized;
    }

    const envelope: StoredEnvelope = {
      __underlay: true,
      version: UNDERLAY_STORAGE_VERSION,
      value: serialized,
      expiresAt
    };

    return JSON.stringify(envelope);
  }

  function get<T>(key: string, defaultValue: T, options?: StorageOptions): T {
    const storage = getStorage();
    if (!storage) return defaultValue;

    try {
      const item = parseStoredItem(storage.getItem(key));
      if (item.serialized === null) return defaultValue;
      if (item.expired) {
        storage.removeItem(key);
        return defaultValue;
      }

      const deserialize = options?.deserialize ?? defaultDeserialize;
      return deserialize(item.serialized) as T;
    } catch {
      // Parsing failed, return default
      return defaultValue;
    }
  }

  function set<T>(key: string, value: T, options?: StorageOptions): void {
    const storage = getStorage();
    if (!storage) return;

    try {
      const expiresAt = resolveExpiresAt(options);
      if (typeof expiresAt === "number" && expiresAt <= Date.now()) {
        storage.removeItem(key);
        return;
      }

      storage.setItem(key, serializeForStorage(value, options));
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

    const item = parseStoredItem(storage.getItem(key));
    if (item.expired) {
      storage.removeItem(key);
      return false;
    }

    return item.serialized !== null;
  }

  function isExpired(key: string): boolean {
    const storage = getStorage();
    if (!storage) return false;

    const item = parseStoredItem(storage.getItem(key));
    if (!item.expired) {
      return false;
    }

    storage.removeItem(key);
    return true;
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
    let expiryTimer: ReturnType<typeof setTimeout> | null = null;

    function clearExpiryTimer() {
      if (expiryTimer !== null) {
        clearTimeout(expiryTimer);
        expiryTimer = null;
      }
    }

    function scheduleExpiry(expiresAt?: number) {
      clearExpiryTimer();
      if (!BROWSER || typeof expiresAt !== "number") {
        return;
      }

      const delay = expiresAt - Date.now();
      if (delay <= 0) {
        remove(key);
        setStore(defaultValue);
        return;
      }

      expiryTimer = setTimeout(() => {
        remove(key);
        setStore(defaultValue);
        expiryTimer = null;
      }, delay);
    }

    function refreshExpiryFromStorage() {
      const storage = getStorage();
      if (!storage) {
        clearExpiryTimer();
        return;
      }

      const parsed = parseStoredItem(storage.getItem(key));
      if (parsed.expired) {
        storage.removeItem(key);
        setStore(defaultValue);
        clearExpiryTimer();
        return;
      }

      scheduleExpiry(parsed.expiresAt);
    }

    refreshExpiryFromStorage();

    // Wrap set/update to persist changes
    function persistingSet(value: T): void {
      set(key, value, options);
      setStore(value);
      refreshExpiryFromStorage();
    }

    function persistingUpdate(updater: (value: T) => T): void {
      update((current) => {
        const next = updater(current);
        set(key, next, options);
        return next;
      });
      refreshExpiryFromStorage();
    }

    // Listen for storage events from other tabs (localStorage only)
    if (BROWSER && type === "localStorage") {
      const handleStorageEvent = (event: StorageEvent) => {
        if (event.key !== key) return;
        if (event.storageArea !== window.localStorage) return;

        try {
          const parsed = parseStoredItem(event.newValue);
          const deserialize = options?.deserialize ?? defaultDeserialize;
          const newValue =
            parsed.serialized === null || parsed.expired
              ? defaultValue
              : (deserialize(parsed.serialized) as T);
          setStore(newValue);
          scheduleExpiry(parsed.expired ? undefined : parsed.expiresAt);
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

  return { get, set, remove, has, isExpired, store, clear };
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
