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
  _mode: NightfireFieldMode
): StrategyNormalisationResult {
  const normalised = normaliseNightfireValue(value, schema);
  const actualSchema = value?.schema ?? null;

  return {
    coerced: {
      schema,
      blocks: Array.isArray(normalised.blocks) ? normalised.blocks : []
    },
    schemaMismatch: actualSchema && actualSchema !== schema ? actualSchema : null
  };
}
