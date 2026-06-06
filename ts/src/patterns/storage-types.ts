import type { Writable } from "svelte/store";

export interface StorageOptions {
  serialize?: (value: unknown) => string;
  deserialize?: (value: string) => unknown;
  ttl?: number;
  expiresAt?: Date | number;
}

export interface StoredEnvelope {
  __underlay: true;
  version: 1;
  value: string;
  expiresAt?: number;
}

export interface ParsedStoredItem {
  serialized: string | null;
  expiresAt?: number;
  expired: boolean;
}

export interface StorageWrapper {
  get<T>(key: string, defaultValue: T, options?: StorageOptions): T;
  set<T>(key: string, value: T, options?: StorageOptions): void;
  remove(key: string): void;
  has(key: string): boolean;
  isExpired(key: string): boolean;
  store<T>(key: string, defaultValue: T, options?: StorageOptions): Writable<T>;
  clear(): void;
}
