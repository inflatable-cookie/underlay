import type { SelectableRelation } from "./types.js";

export function mergeResolvedItem<T extends SelectableRelation>(
  resolvedItems: Map<string, T>,
  item: T | null | undefined,
): Map<string, T> | null {
  if (!item || resolvedItems.has(item.id)) return null;
  const next = new Map(resolvedItems);
  next.set(item.id, item);
  return next;
}

export function mergeResolvedItems<T extends SelectableRelation>(
  resolvedItems: Map<string, T>,
  items: T[] | null | undefined,
): Map<string, T> | null {
  if (!items?.length) return null;

  let next: Map<string, T> | null = null;
  for (const item of items) {
    if (!resolvedItems.has(item.id)) {
      next ??= new Map(resolvedItems);
      next.set(item.id, item);
    }
  }
  return next;
}

export function resolveSelectedItem<T extends SelectableRelation>(
  resolvedItems: Map<string, T>,
  value: string | null | undefined,
): T | null {
  return value ? (resolvedItems.get(value) ?? null) : null;
}

export function resolveSelectedItems<T extends SelectableRelation>(
  resolvedItems: Map<string, T>,
  values: string[] | undefined,
): T[] {
  if (!values) return [];
  return values
    .map((id) => resolvedItems.get(id))
    .filter((item): item is T => item !== undefined);
}

export function toggleSelectionValue(
  values: string[],
  itemId: string,
): string[] {
  return values.includes(itemId)
    ? values.filter((id) => id !== itemId)
    : [...values, itemId];
}
