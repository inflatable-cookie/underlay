import type { NightfireValue } from "../index";
import { addBlockToList, moveBlockInList, removeBlockFromList } from "./block-list";
import { SUMMARY_SCHEMA_ID, transformSummaryBlockOnLayoutChange } from "./summary-transform";

type TypeLabelFn = (type: string) => string;

export function asSingleBlockValue(schema: string, block: any): NightfireValue {
  return {
    schema,
    block,
    blocks: undefined
  };
}

export function asMultiBlockValue(schema: string, blocks: any[]): NightfireValue {
  return {
    schema,
    block: undefined,
    blocks
  };
}

export function replaceBlockAtIndex(blocks: any[], index: number, nextBlock: any): any[] {
  const nextBlocks = blocks.slice();
  nextBlocks[index] = nextBlock;
  return nextBlocks;
}

export function changeSingleBlockType(
  schema: string,
  currentBlock: any,
  nextType: string,
  getLabelForType: TypeLabelFn
): { block: any; warning: string | null } {
  const current = currentBlock ?? {};
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

export function changeBlockType(currentBlock: any, nextType: string): any {
  return {
    ...(currentBlock ?? {}),
    type: nextType
  };
}

export function addBlock(blocks: any[], defaultType: string): any[] {
  return addBlockToList(blocks, defaultType);
}

export function removeBlock(blocks: any[], index: number): any[] {
  return removeBlockFromList(blocks, index);
}

export function moveBlock(blocks: any[], from: number, to: number): any[] | null {
  return moveBlockInList(blocks, from, to);
}
