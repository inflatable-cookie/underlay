import type { NightfireDraftValue, NightfireValue } from "../types";
import { createDefaultBlock } from "./block-list";
import { writePreparedNightfireToFormData } from "../validation";

export type NightfireFieldMode = "single" | "multi";

export function createRequiredInitialValue(
  schema: string,
  mode: NightfireFieldMode,
  defaultType: string
): NightfireValue {
  const defaultBlock = createDefaultBlock(defaultType);
  if (mode === "multi") {
    return {
      schema,
      blocks: [defaultBlock]
    };
  }

  return {
    schema,
    block: defaultBlock
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
