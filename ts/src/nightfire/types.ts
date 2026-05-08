export interface NightfireBlock extends Record<string, unknown> {
  id?: string;
  type: string;
  version?: string;
  hash?: string;
  data?: Record<string, unknown>;
}

export interface NightfireSingleValue {
  schema: string;
  block: NightfireBlock;
  blocks?: undefined;
}

export interface NightfireMultiValue {
  schema: string;
  blocks: NightfireBlock[];
  block?: undefined;
}

export interface NightfireEmptyValue {
  schema: string;
  block?: undefined;
  blocks?: undefined;
}

export interface NightfireValue {
  schema: string;
  block?: NightfireBlock;
  blocks?: NightfireBlock[];
}

export type NightfireDraftValue = NightfireValue;

export function coerceNightfireBlock(
  value: unknown,
  fallbackType: string | null = null
): NightfireBlock | null {
  // Absent block must stay absent. Otherwise `coerceNightfireBlock(undefined, "markdown")`
  // fabricates a block, and `normaliseNightfireValue` prefers that synthetic "single"
  // over a real `blocks` array — multi-block values collapse to one empty editor.
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
    hash:
      typeof record?.hash === "string"
        ? record.hash
        : "",
    data:
      record?.data !== null &&
      typeof record?.data === "object" &&
      !Array.isArray(record.data)
        ? (record.data as Record<string, unknown>)
        : {}
  };
}
