import type { Writable } from "svelte/store";
import { createStorageWrapper } from "./storage-wrapper.js";
import type { StorageOptions } from "./storage-types.js";

export type { StorageOptions, StorageWrapper } from "./storage-types.js";

export const storage = {
  local: createStorageWrapper("localStorage"),
  session: createStorageWrapper("sessionStorage"),
};

export function createPersistedStore<T>(
  key: string,
  defaultValue: T,
  options?: StorageOptions,
): Writable<T> {
  return storage.local.store(key, defaultValue, options);
}

export function createSessionStore<T>(
  key: string,
  defaultValue: T,
  options?: StorageOptions,
): Writable<T> {
  return storage.session.store(key, defaultValue, options);
}
