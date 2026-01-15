/// <reference path="../svelte.d.ts" />

export { default as NightfireRenderer } from "./NightfireRenderer.svelte";
export { default as NightfireBlockEditor } from "./NightfireBlockEditor.svelte";
export { default as NightfireEditor } from "./NightfireEditor.svelte";

export type NightfireValue = {
  schema: string;
  block?: unknown;
  blocks?: unknown[];
};

export { prepareNightfireForSave, validateNightfireValue } from "./validation";
export { isEmptyNightfire, normaliseNightfireValue, writeNightfireToFormData } from "./utils";

export {
  registerSchema,
  registerBlockEditor,
  getSchemaDefinition,
  getBlockEditor,
  getBlockTypeOptionsForSchema,
  getBlockTypeLabel
} from "./editor-registry";

export { registerBlockRenderer, getBlockRenderer } from "./render-registry";

export { registerBlockValidator } from "./validator-registry";

export type { FieldMode, SchemaDefinition, BlockTypeOption } from "./editor-registry";
