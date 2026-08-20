export interface NightfireBlock {
  id?: string;
  type: string;
  version: string;
  data: Record<string, unknown>;
}

export interface NightfireValue {
  schema: string;
  blocks: NightfireBlock[];
}

export type NightfireDraftValue = NightfireValue;

export function coerceNightfireBlock(
  value: unknown,
  fallbackType: string | null = null
): NightfireBlock | null {
  // Absent block must stay absent. Otherwise `coerceNightfireBlock(undefined, "markdown")`
  // fabricates a block, and `normaliseNightfireValue` fills an empty editor.
  if (value === undefined || value === null) {
    return null;
  }

  const record =
    value !== null && typeof value === "object" && !Array.isArray(value)
      ? (value as Record<string, unknown>)
      : null;

  const type =
    typeof record?.type === "string"
      ? record.type
      : fallbackType;

  if (!type) {
    return null;
  }

  return {
    id:
      typeof record?.id === "string" && record.id.length > 0
        ? record.id
        : undefined,
    type,
    version:
      typeof record?.version === "string"
        ? record.version
        : "initial",
    data:
      record?.data !== null &&
      typeof record?.data === "object" &&
      !Array.isArray(record.data)
        ? (record.data as Record<string, unknown>)
        : {}
  };
}
