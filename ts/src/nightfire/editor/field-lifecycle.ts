import type { NightfireDraftValue, NightfireValue } from "../types";
import { createDefaultBlock } from "./block-list";
import { writePreparedNightfireToFormData } from "../validation";

export type NightfireFieldMode = "single" | "multi";

export function createRequiredInitialValue(
  schema: string,
  _mode: NightfireFieldMode,
  defaultType: string
): NightfireValue {
  return {
    schema,
    blocks: [createDefaultBlock(defaultType)]
  };
}

export function createPrepareWriter(
  getValue: () => NightfireDraftValue,
  getName: () => string
): (formData: FormData) => void {
  return (formData: FormData) => {
    writePreparedNightfireToFormData(formData, getName(), getValue());
  };
}
