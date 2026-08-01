import type { ReorderController, ReorderableItem } from "./reorder-controller.svelte";

export interface ReorderConflictDetails {
  addedIds: string[];
  removedIds: string[];
  message: string;
}

export interface ReorderConflictResolution {
  addedCount: number;
  removedCount: number;
  unresolvedAddedIds: string[];
}

type UnknownRecord = Record<string, unknown>;

function asRecord(value: unknown): UnknownRecord | null {
  return typeof value === "object" && value !== null ? (value as UnknownRecord) : null;
}

function toStringArray(value: unknown): string[] {
  if (!Array.isArray(value)) return [];
  return value.filter((entry): entry is string => typeof entry === "string" && entry.length > 0);
}

function parseConflictContext(value: unknown): ReorderConflictDetails | null {
  const obj = asRecord(value);
  if (!obj) return null;

  const addedIds = toStringArray(obj.added_ids ?? obj.addedIds);
  const removedIds = toStringArray(obj.removed_ids ?? obj.removedIds);

  if (addedIds.length === 0 && removedIds.length === 0) {
    return null;
  }

  return {
    addedIds,
    removedIds,
    message: "Items have changed since you started reordering."
  };
}

/**
 * Extract reorder conflict details from a transport/application error.
 *
 * Supports Underlay/Cattle-Grid style errors where the response envelope is
 * attached at `error.raw` and context lives in either:
 * - `raw.context`
 * - `raw.error.context`
 * - `raw.error.details`
 */
export function extractReorderConflict(error: unknown): ReorderConflictDetails | null {
  const top = asRecord(error);
  if (!top) return null;

  const raw = asRecord(top.raw);
  const status =
    typeof top.status === "number"
      ? top.status
      : typeof raw?.status === "number"
        ? raw.status
        : null;
  if (status !== 409) return null;

  const topMessage = typeof top.message === "string" ? top.message : null;
  const rawError = asRecord(raw?.error);
  const envelope = asRecord(top.envelope);
  const envelopeError = asRecord(envelope?.error);

  const candidates: unknown[] = [
    top.context,
    raw?.context,
    raw?.details,
    rawError?.context,
    rawError?.details,
    envelope?.context,
    envelope?.details,
    envelopeError?.context,
    envelopeError?.details,
    top.details
  ];

  for (const candidate of candidates) {
    const parsed = parseConflictContext(candidate);
    if (parsed) {
      return {
        ...parsed,
        message:
          (typeof rawError?.message === "string" && rawError.message) ||
          (typeof envelopeError?.message === "string" && envelopeError.message) ||
          topMessage ||
          parsed.message
      };
    }
  }

  return null;
}

/**
 * Apply a reorder conflict to the current pending reorder state.
 *
 * - Removes IDs that were deleted by another actor.
 * - Appends newly-added IDs using the latest list snapshot.
 */
export function applyReorderConflict<T extends ReorderableItem>(
  controller: ReorderController<T>,
  conflict: ReorderConflictDetails,
  latestItems: readonly T[]
): ReorderConflictResolution {
  const latestById = new Map(latestItems.map((item) => [item.id, item]));
  const pendingIds = new Set(controller.pending.map((item) => item.id));

  const removedCount = conflict.removedIds.filter((id) => pendingIds.has(id)).length;
  if (conflict.removedIds.length > 0) {
    controller.removeItems(conflict.removedIds);
  }

  const unresolvedAddedIds: string[] = [];
  const addedItems: T[] = [];

  for (const id of conflict.addedIds) {
    const item = latestById.get(id);
    if (item) {
      addedItems.push(item);
    } else {
      unresolvedAddedIds.push(id);
    }
  }

  if (addedItems.length > 0) {
    controller.mergeNewItems(addedItems);
  }

  return {
    addedCount: addedItems.length,
    removedCount,
    unresolvedAddedIds
  };
}

export interface ReorderConflictRecoveryResult {
  handled: boolean;
  message: string;
  addedIds: string[];
}

export interface RecoverReorderConflictOptions<T extends ReorderableItem> {
  controller: ReorderController<T>;
  error: unknown;
  latestItems: readonly T[];
  entityLabel: string;
}

/**
 * Apply server reorder-conflict context to pending controller state.
 * Keeps users in reorder mode so they can review and submit again.
 */
export function recoverReorderConflict<T extends ReorderableItem>({
  controller,
  error,
  latestItems,
  entityLabel
}: RecoverReorderConflictOptions<T>): ReorderConflictRecoveryResult {
  const conflict = extractReorderConflict(error);
  if (!conflict) {
    return { handled: false, message: "", addedIds: [] };
  }

  const resolution = applyReorderConflict(controller, conflict, latestItems);
  const changes: string[] = [];

  if (resolution.addedCount > 0) {
    changes.push(`${resolution.addedCount} ${entityLabel} added`);
  }
  if (resolution.removedCount > 0) {
    changes.push(`${resolution.removedCount} ${entityLabel} removed`);
  }
  if (resolution.unresolvedAddedIds.length > 0) {
    changes.push(`${resolution.unresolvedAddedIds.length} new item(s) need refresh`);
  }

  const suffix =
    changes.length > 0
      ? ` Applied updates: ${changes.join(", ")}.`
      : " Applied latest server state.";

  return {
    handled: true,
    message: `${conflict.message}${suffix} Review the order and save again.`,
    addedIds: conflict.addedIds
  };
}
