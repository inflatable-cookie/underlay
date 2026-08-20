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
  const blocks = Array.isArray(value.blocks) ? value.blocks : [];
  return {
    ...value,
    blocks: blocks.map((block) => ensureNightfireBlockId(block))
  } as T;
}
