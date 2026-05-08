import type { NightfireBlock, NightfireDraftValue, NightfireValue } from "./types";

export function generateNightfireBlockId(): string {
  const uuid =
    typeof globalThis.crypto?.randomUUID === "function"
      ? globalThis.crypto.randomUUID().replace(/-/g, "")
      : `${Date.now().toString(36)}${Math.random().toString(36).slice(2, 10)}`;

  return `nf_${uuid}`;
}

export function ensureNightfireBlockId(block: NightfireBlock): NightfireBlock {
  if (typeof block.id === "string" && block.id.trim().length > 0) {
    return block;
  }

  return {
    ...block,
    id: generateNightfireBlockId()
  };
}

export function ensureNightfireBlockIds<T extends NightfireDraftValue | NightfireValue>(value: T): T {
  if (value.block) {
    return {
      ...value,
      block: ensureNightfireBlockId(value.block),
      blocks: undefined
    } as T;
  }

  if (Array.isArray(value.blocks)) {
    return {
      ...value,
      block: undefined,
      blocks: value.blocks.map((block) => ensureNightfireBlockId(block))
    } as T;
  }

  return value;
}
