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

// Support both Svelte 4 class components and Svelte 5 function components
type AnyComponent =
  | (new (...args: any[]) => SvelteComponent)
  | Component<any, any, any>;

function makeKey(schema: string, type: string): RegistryKey {
  return `${schema}|${type}`;
}

const schemaDefs = new Map<string, SchemaDefinition>();
const blockEditors = new Map<RegistryKey, AnyComponent>();
const blockTypeOptions = new Map<string, BlockTypeOption[]>();

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

// Registrations are defined in `*-registrations.ts` to keep this file
// focused purely on registry mechanics.
