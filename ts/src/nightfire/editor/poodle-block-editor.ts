import type { BlockTypeDefinition, BlockTypeGroup } from "@inflatable-cookie/poodle-svelte";

import type { NightfireTypeOption } from "../utils";
import type { GroupedOptions, NightfireBlockOptionInput } from "./grouped-options";

/**
 * Shape adapters between Nightfire's block model and Poodle's `BlockEditor`
 * props. Poodle's component API is consumed directly - these only reshape
 * Nightfire data into what it already accepts.
 */

const DEFAULT_BLOCK_ICON = "file-text";

export type NightfireEditorBlock = Record<string, unknown> & {
  id?: string;
  type: string;
  version?: string | number;
  hash?: string | null;
  data?: Record<string, unknown>;
  content?: string;
};

export type ToEditorBlockOptions = {
  fallbackType?: string;
  createId?: () => string;
};

function defaultCreateId(): string {
  const cryptoApi = globalThis.crypto;
  if (cryptoApi?.randomUUID) {
    return cryptoApi.randomUUID();
  }

  return `nightfire-${Math.random().toString(36).slice(2, 10)}`;
}

/** Poodle requires a stable `id` and a concrete `type` on every block. */
export function toEditorBlock(
  block: NightfireEditorBlock,
  options: ToEditorBlockOptions = {},
): NightfireEditorBlock & { id: string; type: string } {
  const fallbackType = options.fallbackType ?? "markdown";
  const createId = options.createId ?? defaultCreateId;
  const type = typeof block.type === "string" && block.type.length > 0 ? block.type : fallbackType;

  return {
    ...block,
    id: typeof block.id === "string" && block.id.length > 0 ? block.id : createId(),
    type,
  };
}

export function toEditorBlocks(
  blocks: NightfireEditorBlock[],
  options: ToEditorBlockOptions = {},
): Array<NightfireEditorBlock & { id: string; type: string }> {
  return blocks.map((block) => toEditorBlock(block, options));
}

export function toBlockTypes(
  options: Array<NightfireTypeOption | NightfireBlockOptionInput>,
): BlockTypeDefinition[] {
  return options.map((option) => ({
    type: option.type,
    label: option.label,
    icon: DEFAULT_BLOCK_ICON,
  })) as BlockTypeDefinition[];
}

/**
 * Poodle groups its type picker when given `BlockTypeGroup[]`; Nightfire only
 * has groups to give when the caller supplied them.
 */
export function toBlockTypeGroups(groups: GroupedOptions[]): BlockTypeGroup[] {
  return groups.map((group) => ({
    label: group.label,
    options: toBlockTypes(group.options),
  })) as BlockTypeGroup[];
}
