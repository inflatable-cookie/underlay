import type { ComponentType } from "svelte";

export interface RestoreResolutionOrderPreview {
  prefixText?: string;
  previousOrderText?: string | null;
  currentOrderText: string;
  title?: string | null;
}

export interface RestoreResolutionPlannerItem {
  id: string;
  label: string;
  subtitle?: string | null;
  accent?: string | null;
  mediaIcon?: ComponentType;
  preview?: RestoreResolutionOrderPreview | null;
}

export function normalizeRestoreResolutionOrder(
  nextOrderIds: string[],
  canonicalIds: string[],
): string[] {
  if (canonicalIds.length === 0) {
    return [];
  }

  const canonicalIdSet = new Set(canonicalIds);
  const normalized = Array.from(new Set(nextOrderIds)).filter((id) =>
    canonicalIdSet.has(id)
  );
  const missingIds = canonicalIds.filter((id) => !normalized.includes(id));

  return [...normalized, ...missingIds];
}
