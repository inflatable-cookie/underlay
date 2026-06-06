import type {
  ParsedStoredItem,
  StorageOptions,
  StoredEnvelope,
} from "./storage-types.js";

const UNDERLAY_STORAGE_VERSION = 1;

export const defaultSerialize = (value: unknown): string =>
  JSON.stringify(value);

export const defaultDeserialize = (value: string): unknown => JSON.parse(value);

export function resolveExpiresAt(options?: StorageOptions): number | undefined {
  if (!options) return undefined;

  if (options.expiresAt instanceof Date) {
    return options.expiresAt.getTime();
  }

  if (
    typeof options.expiresAt === "number" &&
    Number.isFinite(options.expiresAt)
  ) {
    return options.expiresAt;
  }

  if (typeof options.ttl === "number" && Number.isFinite(options.ttl)) {
    return Date.now() + options.ttl * 1000;
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

export function parseStoredItem(raw: string | null): ParsedStoredItem {
  if (raw === null) {
    return {
      serialized: null,
      expired: false,
    };
  }

  try {
    const parsed = JSON.parse(raw) as unknown;
    if (isStoredEnvelope(parsed)) {
      const expiresAt =
        typeof parsed.expiresAt === "number" ? parsed.expiresAt : undefined;
      const expired = typeof expiresAt === "number" && expiresAt <= Date.now();

      return {
        serialized: parsed.value,
        expiresAt,
        expired,
      };
    }
  } catch {
    // Invalid JSON can still be a legacy raw value for custom deserializers.
  }

  return {
    serialized: raw,
    expired: false,
  };
}

export function serializeForStorage(
  value: unknown,
  options?: StorageOptions,
): string {
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
    expiresAt,
  };

  return JSON.stringify(envelope);
}
