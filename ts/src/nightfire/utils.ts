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
