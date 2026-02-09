import type { NightfireValue } from "../index";
import { createDefaultBlock } from "./block-list";
import { writeNightfireToFormData } from "../utils";

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
    } as NightfireValue;
  }

  return {
    schema,
    block: defaultBlock
  } as NightfireValue;
}

export function createPrepareWriter(
  getValue: () => NightfireValue,
  getName: () => string
): (formData: FormData) => void {
  return (formData: FormData) => {
    writeNightfireToFormData(formData, getName(), getValue());
  };
}
