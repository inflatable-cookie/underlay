import type { Component, SvelteComponent } from "svelte";

export type BlockRendererComponent =
  | Component<any>
  | (new (...args: any[]) => SvelteComponent);

type RegistryKey = string;

function makeKey(schema: string | null, type: string): RegistryKey {
  return `${schema ?? "*"}|${type}`;
}

const registry = new Map<RegistryKey, BlockRendererComponent>();

export function registerBlockRenderer(
  schema: string | null,
  type: string,
  component: BlockRendererComponent
): void {
  registry.set(makeKey(schema, type), component);
}

export function getBlockRenderer(
  schema: string | null | undefined,
  type: string | undefined
): BlockRendererComponent | null {
  if (!type) return null;

  const schemaKey = schema ?? null;
  return (
    registry.get(makeKey(schemaKey, type)) ??
    registry.get(makeKey(null, type)) ??
    null
  );
}

// Registrations are defined in `*-registrations.ts` to keep this file
// focused purely on registry mechanics.
