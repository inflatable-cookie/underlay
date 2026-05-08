import type { SvelteComponent, Component } from "svelte";

export type FieldMode = "single" | "multi";

export interface SchemaDefinition {
  schema: string;
  mode: FieldMode;
  defaultType: string;
}

export interface BlockTypeOption {
  type: string;
  label: string;
}

type RegistryKey = string;

/**
 * Function that determines whether a block's content is meaningfully empty.
 * Returns `true` when the block has no user-entered content.
 */
export type BlockEmptyChecker = (block: any) => boolean;

// Support both Svelte 4 class components and Svelte 5 function components
type AnyComponent =
  | (new (...args: any[]) => SvelteComponent)
  | Component<any, any, any>;

export type BlockEditorComponent = AnyComponent;

function makeKey(schema: string, type: string): RegistryKey {
  return `${schema}|${type}`;
}

const schemaDefs = new Map<string, SchemaDefinition>();
const blockEditors = new Map<RegistryKey, AnyComponent>();
const blockTypeOptions = new Map<string, BlockTypeOption[]>();
const blockEmptyCheckers = new Map<string, BlockEmptyChecker>();

export function registerSchema(def: SchemaDefinition): void {
  schemaDefs.set(def.schema, def);
}

export function registerBlockEditor(
  schema: string,
  type: string,
  label: string,
  component: AnyComponent
): void {
  blockEditors.set(makeKey(schema, type), component);

  const key = schema;
  const existing = blockTypeOptions.get(key) ?? [];

  if (!existing.some((opt) => opt.type === type)) {
    blockTypeOptions.set(key, [...existing, { type, label }]);
  }
}

export function getSchemaDefinition(
  schema: string
): SchemaDefinition | null {
  return schemaDefs.get(schema) ?? null;
}

export function getBlockEditor(
  schema: string,
  type: string
): AnyComponent | null {
  return blockEditors.get(makeKey(schema, type)) ?? null;
}

export function getBlockTypeOptionsForSchema(
  schema: string
): BlockTypeOption[] {
  return blockTypeOptions.get(schema) ?? [];
}

export function getBlockTypeLabel(
  schema: string,
  type: string
): string | null {
  const options = blockTypeOptions.get(schema);
  if (!options) return null;
  const match = options.find((opt) => opt.type === type);
  return match?.label ?? null;
}

/**
 * Register a function that checks whether a block of the given type
 * is content-empty (i.e. has no meaningful user input).
 *
 * When no checker is registered for a block type the default behaviour
 * is to treat any present block as non-empty.
 */
export function registerBlockEmptyChecker(
  type: string,
  checker: BlockEmptyChecker
): void {
  blockEmptyCheckers.set(type, checker);
}

/**
 * Check whether a block is content-empty using the registered checker
 * for its type. Returns `true` if the block is empty, `false` if it
 * has meaningful content. When no checker is registered the block is
 * assumed to be non-empty (conservative default).
 */
export function isBlockContentEmpty(block: any): boolean {
  if (!block || typeof block !== "object") return true;
  const type = block.type as string | undefined;
  if (!type) return true;
  const checker = blockEmptyCheckers.get(type);
  if (!checker) return false; // no checker → assume non-empty
  return checker(block);
}

// Registrations are defined in `*-registrations.ts` to keep this file
// focused purely on registry mechanics.
