import {
  coerceNightfireBlock,
  type NightfireBlock,
  type NightfireDraftValue
} from "./types";
import { isBlockContentEmpty } from "./editor-registry";

export interface NightfireBlockDefinition {
  schema: string;
  mode: "single" | "multi";
  defaultType: string;
}

export interface NightfireTypeOption {
  type: string;
  label: string;
}

function readBlocks(record: Record<string, unknown>, defaultType: string): NightfireBlock[] {
  // v1 `{ block }` is not accepted. Dual-shape conversion is gone.
  if ("block" in record && record.block != null) {
    return [];
  }

  if (!Array.isArray(record.blocks)) {
    return [];
  }

  return record.blocks
    .map((block) => coerceNightfireBlock(block, defaultType))
    .filter((block): block is NightfireBlock => block !== null);
}

/**
 * Normalises a value that may be a raw string (legacy data) into Nightfire
 * draft state with a durable `{ schema, blocks }` envelope when content exists.
 *
 * v1 `{ schema, block }` values are rejected, not converted.
 */
export function normaliseNightfireValue(
  value: unknown,
  schema: string,
  allowedBlockTypes?: string[] | null
): NightfireDraftValue {
  const defaultType = allowedBlockTypes?.[0] ?? "markdown";

  if (value !== null && typeof value === "object" && !Array.isArray(value)) {
    const record = value as Record<string, unknown>;
    const nextSchema =
      typeof record.schema === "string" ? record.schema : schema;
    return {
      schema: nextSchema,
      blocks: readBlocks(record, defaultType)
    };
  }

  // Raw string - convert to markdown block if markdown is allowed
  if (typeof value === "string" && value.length > 0) {
    if (allowedBlockTypes && !allowedBlockTypes.includes("markdown")) {
      return {
        schema,
        blocks: [coerceNightfireBlock({}, defaultType)!]
      };
    }

    return {
      schema,
      blocks: [
        coerceNightfireBlock(
          { type: "markdown", data: { text: value } },
          "markdown"
        )!
      ]
    };
  }

  return { schema, blocks: [] };
}

export function normaliseNightfireBlock(
  block: unknown,
  typeOptions: NightfireTypeOption[],
  definition: NightfireBlockDefinition
): NightfireBlock {
  const allowed = typeOptions.map((o) => o.type);
  const defaultType =
    definition.defaultType ?? typeOptions[0]?.type ?? "markdown";
  const source = block === undefined || block === null ? {} : block;
  const normalized = coerceNightfireBlock(source, defaultType)!;
  if (allowed.includes(normalized.type)) {
    return normalized;
  }

  return {
    ...normalized,
    type: defaultType
  };
}

/**
 * Checks whether a NightfireValue is empty.
 *
 * When `contentLevel` is true (default) the check delegates to
 * per-block-type empty checkers registered via `registerBlockEmptyChecker`.
 * This means a markdown block with blank text is treated as empty even
 * though the block structure exists. When no checker is registered for a
 * block type the block is assumed non-empty (conservative default).
 *
 * When `contentLevel` is false only structural presence is checked
 * (i.e. does a blocks array contain any items).
 */
export function isEmptyNightfire(
  value: NightfireDraftValue | null | undefined,
  contentLevel: boolean = true
): boolean {
  if (!value || typeof value !== "object") return true;

  const blocks = Array.isArray(value.blocks) ? value.blocks : [];
  if (blocks.length === 0) return true;
  if (!contentLevel) return false;
  return blocks.every((b) => isBlockContentEmpty(b));
}

export function writeNightfireToFormData(
  formData: FormData,
  name: string,
  value: NightfireDraftValue | null | undefined
): void {
  if (isEmptyNightfire(value)) {
    formData.set(name, "");
    return;
  }

  formData.set(name, JSON.stringify(value));
}
