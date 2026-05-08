import type { NightfireDraftValue } from "../types";
import { normaliseNightfireValue } from "../utils";
import type { NightfireFieldMode } from "./field-lifecycle";

export interface StrategyNormalisationResult {
  coerced: NightfireDraftValue;
  schemaMismatch: string | null;
}

export function normaliseForStrategy(
  value: NightfireDraftValue,
  schema: string,
  mode: NightfireFieldMode
): StrategyNormalisationResult {
  const normalised = normaliseNightfireValue(value, schema);
  const actualSchema = value?.schema ?? null;

  let coerced: NightfireDraftValue = { ...normalised, schema };
  const single = coerced.block ?? null;
  const multi = Array.isArray(coerced.blocks) ? coerced.blocks : undefined;

  if (mode === "single") {
    if (!single && multi && multi.length > 0) {
      coerced = { schema, block: multi[0] };
    } else if (single && multi) {
      coerced = { schema, block: single };
    }
  } else {
    if (!multi && single) {
      coerced = { schema, blocks: [single] };
    } else if (multi && single) {
      coerced = { schema, blocks: multi };
    }
  }

  return {
    coerced,
    schemaMismatch: actualSchema && actualSchema !== schema ? actualSchema : null
  };
}
