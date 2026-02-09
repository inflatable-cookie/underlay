import type { NightfireValue } from "../index";
import { normaliseNightfireValue } from "../utils";
import type { NightfireFieldMode } from "./field-lifecycle";

export interface StrategyNormalisationResult {
  coerced: NightfireValue;
  schemaMismatch: string | null;
}

export function normaliseForStrategy(
  value: NightfireValue,
  schema: string,
  mode: NightfireFieldMode
): StrategyNormalisationResult {
  const normalised = normaliseNightfireValue(value, schema);
  const actualSchema = (() => {
    if (!value || typeof value !== "object") return null;
    const current = (value as Record<string, unknown>).schema;
    return typeof current === "string" ? current : null;
  })();

  let coerced: NightfireValue = { ...normalised, schema } as NightfireValue;
  const record = coerced as unknown as Record<string, unknown>;
  const single = record.block ?? null;
  const multi = Array.isArray(record.blocks) ? (record.blocks as unknown[]) : undefined;

  if (mode === "single") {
    if (!single && multi && multi.length > 0) {
      coerced = { ...coerced, block: multi[0], blocks: undefined } as NightfireValue;
    } else if (single && multi) {
      coerced = { ...coerced, blocks: undefined } as NightfireValue;
    }
  } else {
    if (!multi && single) {
      coerced = { ...coerced, block: undefined, blocks: [single] } as NightfireValue;
    } else if (multi && single) {
      coerced = { ...coerced, block: undefined } as NightfireValue;
    }
  }

  return {
    coerced,
    schemaMismatch: actualSchema && actualSchema !== schema ? actualSchema : null
  };
}
