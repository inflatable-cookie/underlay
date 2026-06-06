import { writable, type Writable } from "svelte/store";
import { BROWSER } from "esm-env";
import { isStorageAvailable } from "./storage-availability.js";
import {
  defaultDeserialize,
  parseStoredItem,
  resolveExpiresAt,
  serializeForStorage,
} from "./storage-envelope.js";
import type { StorageOptions, StorageWrapper } from "./storage-types.js";

export function createStorageWrapper(
  type: "localStorage" | "sessionStorage",
): StorageWrapper {
  function getStorage(): Storage | null {
    if (!isStorageAvailable(type)) return null;
    return window[type];
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

  function store<T>(
    key: string,
    defaultValue: T,
    options?: StorageOptions,
  ): Writable<T> {
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
          // Ignore deserialization errors from external storage writes.
        }
      };

      window.addEventListener("storage", handleStorageEvent);
    }

    return {
      subscribe,
      set: persistingSet,
      update: persistingUpdate,
    };
  }

  return { get, set, remove, has, isExpired, store, clear };
}
