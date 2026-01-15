import type { NightfireValue } from "./index";

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
 * Normalises a value that may be a raw string (legacy data) into a proper
 * NightfireValue with a markdown block.
 *
 * This handles cases where:
 * - The database stored a plain string instead of a NightfireValue object
 * - The value is already a valid NightfireValue (passed through unchanged)
 * - The value is null/undefined (returns a minimal NightfireValue with empty block)
 *
 * @param value - The raw value from the database (may be string, object, or null)
 * @param schema - The schema identifier to use for the NightfireValue
 * @param allowedBlockTypes - Optional list of allowed block types; if provided and
 *                            "markdown" is not in the list, raw strings won't be converted
 * @returns A properly structured NightfireValue
 */
export function normaliseNightfireValue(
  value: unknown,
  schema: string,
  allowedBlockTypes?: string[] | null
): NightfireValue {
  // Already a valid NightfireValue object
  if (
    value !== null &&
    typeof value === "object" &&
    "schema" in (value as object)
  ) {
    return value as NightfireValue;
  }

  // Raw string - convert to markdown block if markdown is allowed
  if (typeof value === "string" && value.length > 0) {
    // If allowedBlockTypes is provided, check if markdown is allowed
    if (allowedBlockTypes && !allowedBlockTypes.includes("markdown")) {
      // Can't convert to markdown, return empty value with schema
      return {
        schema,
        block: {
          type: allowedBlockTypes[0] ?? "markdown",
          version: "initial",
          hash: "",
          data: {}
        }
      };
    }

    // Convert raw string to markdown block
    return {
      schema,
      block: {
        type: "markdown",
        version: "initial",
        hash: "",
        data: {
          text: value
        }
      }
    };
  }

  // Null, undefined, or empty - return minimal NightfireValue
  return {
    schema,
    block: undefined
  };
}

export function normaliseNightfireBlock(
  block: any,
  typeOptions: NightfireTypeOption[],
  definition: NightfireBlockDefinition
): {
  type: string;
  version: string;
  hash: string;
  data: unknown;
} {
  const allowed = typeOptions.map((o) => o.type);
  const defaultType =
    typeOptions[0]?.type ?? definition.defaultType ?? "markdown";

  let next = block ?? null;

  if (!next || typeof next !== "object") {
    next = null;
  }

  const type =
    next && typeof (next as any).type === "string" &&
    allowed.includes((next as any).type as string)
      ? ((next as any).type as string)
      : defaultType;

  return {
    type,
    version:
      next && typeof (next as any).version === "string"
        ? ((next as any).version as string)
        : "initial",
    hash:
      next && typeof (next as any).hash === "string"
        ? ((next as any).hash as string)
        : "",
    data:
      next &&
      typeof (next as any).data === "object" &&
      (next as any).data !== null
        ? (next as any).data
        : {}
  };
}

export function isEmptyNightfire(
  value: NightfireValue | null | undefined
): boolean {
  if (!value || typeof value !== "object") return true;

  if ((value as any).block && (value as any).block !== null) {
    return false;
  }

  const blocks = (value as any).blocks as unknown[] | undefined;
  if (Array.isArray(blocks) && blocks.length > 0) {
    return false;
  }

  return true;
}

export function writeNightfireToFormData(
  formData: FormData,
  name: string,
  value: NightfireValue | null | undefined
): void {
  if (isEmptyNightfire(value)) {
    formData.set(name, "");
    return;
  }

  formData.set(name, JSON.stringify(value));
}
