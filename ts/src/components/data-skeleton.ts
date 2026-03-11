export type DataSkeletonType = "list" | "grid" | "table" | "detail";
export type DataSkeletonListPattern = "default" | "avatar-text" | "card";
export type DataSkeletonGridPattern = "default" | "product-card";
export type DataSkeletonDetailSection = "header" | "stats" | "description" | "actions";

export interface DataSkeletonPreset {
  type: DataSkeletonType;
  pattern?: string | null;
  count?: number;
  columns?: number;
  rows?: number;
  header?: boolean;
  sections?: DataSkeletonDetailSection[];
}

const presetRegistry = new Map<string, DataSkeletonPreset>();

export function registerDataSkeletonPreset(name: string, preset: DataSkeletonPreset): void {
  presetRegistry.set(name, {
    ...preset,
    sections: preset.sections ? [...preset.sections] : undefined
  });
}

export function unregisterDataSkeletonPreset(name: string): void {
  presetRegistry.delete(name);
}

export function getDataSkeletonPreset(name: string): DataSkeletonPreset | null {
  const preset = presetRegistry.get(name);
  if (!preset) {
    return null;
  }

  return {
    ...preset,
    sections: preset.sections ? [...preset.sections] : undefined
  };
}

export function normaliseDataSkeletonSections(
  sections?: DataSkeletonDetailSection[] | null
): DataSkeletonDetailSection[] {
  const resolved = sections?.length
    ? sections
    : ["header", "stats", "description", "actions"] satisfies DataSkeletonDetailSection[];

  return Array.from(new Set(resolved));
}
