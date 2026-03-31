/// <reference path="../svelte.d.ts" />

export { default as NightfireRenderer } from "./NightfireRenderer.svelte";
export { default as NightfireBlockEditor } from "./NightfireBlockEditor.svelte";
export { default as NightfireEditor } from "./NightfireEditor.svelte";
export { default as SlashCommandPalette } from "./SlashCommandPalette.svelte";
export { default as MarkdownEditor } from "./markup/MarkdownEditor.svelte";
export { default as MarkdownEditorSurface } from "./markup/MarkdownEditorSurface.svelte";

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
  registerBlockEmptyChecker,
  isBlockContentEmpty,
  getSchemaDefinition,
  getBlockEditor,
  getBlockTypeOptionsForSchema,
  getBlockTypeLabel
} from "./editor-registry";

export { registerBlockRenderer, getBlockRenderer } from "./render-registry";

export { registerBlockValidator } from "./validator-registry";

export type { FieldMode, SchemaDefinition, BlockTypeOption, BlockEmptyChecker } from "./editor-registry";

// Nightfire strategies (lazy loading and caching)
export {
  configureNightfireStrategies,
  createNightfireStrategiesContext,
  useNightfireStrategies,
  getStrategy
} from "./strategies";

export type {
  NightfireStrategy,
  NightfireStrategyCardinality,
  NightfireBlockOption,
  NightfireStrategiesConfig,
  NightfireStrategiesStore
} from "./strategies";

// Nightfire media context (media picker injection for block editors)
export {
  createNightfireMediaContext,
  useNightfireMedia
} from "./media/context";

export type {
  NightfireMediaContext,
  NightfireMediaPickResult
} from "./media/context";

// Re-export SchemaMismatchInfo from editor for convenience
export type { SchemaMismatchInfo } from "./NightfireEditor.svelte";
export type {
  NightfireSlashCommand,
  NightfireSlashCommandInput,
  NightfireSlashCommandsConfig
} from "./slash-commands";
