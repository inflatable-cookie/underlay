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

/**
 * Normalises a value that may be a raw string (legacy data) into Nightfire
 * draft state with a durable block envelope when content exists.
 *
 * This handles cases where:
 * - The database stored a plain string instead of a NightfireValue object
 * - The value is already a valid NightfireValue (passed through unchanged)
 * - The value is null/undefined (returns an editor-local empty draft value)
 *
 * @param value - The raw value from the database (may be string, object, or null)
 * @param schema - The schema identifier to use for the Nightfire draft value
 * @param allowedBlockTypes - Optional list of allowed block types; if provided and
 *                            "markdown" is not in the list, raw strings won't be converted
 * @returns A normalized draft value suitable for editor state
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
    const single = coerceNightfireBlock(record.block, defaultType);
    const multi = Array.isArray(record.blocks)
      ? record.blocks
          .map((block) => coerceNightfireBlock(block, defaultType))
          .filter((block): block is NightfireBlock => block !== null)
      : [];

    if (single) {
      return {
        schema: nextSchema,
        block: single
      };
    }

    if (multi.length > 0) {
      return {
        schema: nextSchema,
        blocks: multi
      };
    }

    return { schema: nextSchema };
  }

  // Raw string - convert to markdown block if markdown is allowed
  if (typeof value === "string" && value.length > 0) {
    // If allowedBlockTypes is provided, check if markdown is allowed
    if (allowedBlockTypes && !allowedBlockTypes.includes("markdown")) {
      // Can't convert to markdown, return empty value with schema
      return {
        schema,
        block: coerceNightfireBlock({}, defaultType)!
      };
    }

    // Convert raw string to markdown block
    return {
      schema,
      block: coerceNightfireBlock(
        { type: "markdown", data: { text: value } },
        "markdown"
      )!
    };
  }

  // Null, undefined, or empty - return editor-local empty draft state
  return { schema };
}

export function normaliseNightfireBlock(
  block: unknown,
  typeOptions: NightfireTypeOption[],
  definition: NightfireBlockDefinition
): NightfireBlock {
  const allowed = typeOptions.map((o) => o.type);
  const defaultType =
    typeOptions[0]?.type ?? definition.defaultType ?? "markdown";
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
 * (i.e. does a block/blocks array exist at all).
 */
export function isEmptyNightfire(
  value: NightfireDraftValue | null | undefined,
  contentLevel: boolean = true
): boolean {
  if (!value || typeof value !== "object") return true;

  const block = (value as any).block;
  if (block && block !== null) {
    if (!contentLevel) return false;
    return isBlockContentEmpty(block);
  }

  const blocks = (value as any).blocks as unknown[] | undefined;
  if (Array.isArray(blocks) && blocks.length > 0) {
    if (!contentLevel) return false;
    return blocks.every((b) => isBlockContentEmpty(b));
  }

  return true;
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
