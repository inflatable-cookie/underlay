import {
  coerceNightfireBlock,
  type NightfireBlock,
  type NightfireValue
} from "../types";
import {
  addBlockToList,
  insertBlockIntoList,
  moveBlockInList,
  removeBlockFromList
} from "./block-list";
import { SUMMARY_SCHEMA_ID, transformSummaryBlockOnLayoutChange } from "./summary-transform";

type TypeLabelFn = (type: string) => string;

function asBlockObject(value: unknown): NightfireBlock {
  return coerceNightfireBlock(value, "markdown")!;
}

export function asSingleBlockValue(schema: string, block: unknown): NightfireValue {
  return {
    schema,
    block: asBlockObject(block),
    blocks: undefined
  };
}

export function asMultiBlockValue(schema: string, blocks: unknown[]): NightfireValue {
  return {
    schema,
    block: undefined,
    blocks: blocks.map((block) => asBlockObject(block))
  };
}

export function replaceBlockAtIndex(blocks: unknown[], index: number, nextBlock: unknown): unknown[] {
  const nextBlocks = blocks.slice();
  nextBlocks[index] = asBlockObject(nextBlock);
  return nextBlocks;
}

export function changeSingleBlockType(
  schema: string,
  currentBlock: unknown,
  nextType: string,
  getLabelForType: TypeLabelFn
): { block: NightfireBlock; warning: string | null } {
  const current = asBlockObject(currentBlock);
  if (schema !== SUMMARY_SCHEMA_ID) {
    return {
      block: { ...current, type: nextType },
      warning: null
    };
  }

  const transformed = transformSummaryBlockOnLayoutChange(current, nextType, getLabelForType);
  return {
    block: transformed.block,
    warning: transformed.warning
  };
}

export function changeBlockType(currentBlock: unknown, nextType: string): NightfireBlock {
  return {
    ...asBlockObject(currentBlock),
    type: nextType
  };
}

export function addBlock(blocks: unknown[], defaultType: string): unknown[] {
  return addBlockToList(blocks, defaultType);
}

export function insertBlockAfter(blocks: unknown[], index: number, defaultType: string): unknown[] {
  return insertBlockIntoList(blocks, index, defaultType);
}

export function removeBlock(blocks: unknown[], index: number): unknown[] {
  return removeBlockFromList(blocks, index);
}

export function moveBlock(blocks: unknown[], from: number, to: number): unknown[] | null {
  return moveBlockInList(blocks, from, to);
}
