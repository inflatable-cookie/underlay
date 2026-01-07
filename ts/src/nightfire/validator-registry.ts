import type { NightfireValue } from "./index";

type ValidatorKey = string;

type BlockValidator = (block: unknown) => unknown;

function makeKey(schema: string | null, type: string): ValidatorKey {
  return `${schema ?? "*"}|${type}`;
}

const validators = new Map<ValidatorKey, BlockValidator>();

export function registerBlockValidator(
  schema: string | null,
  type: string,
  fn: BlockValidator
): void {
  validators.set(makeKey(schema, type), fn);
}

function validateBlock(schema: string | null, block: any): any {
  if (!block || typeof block !== "object" || !block.type) {
    return block;
  }

  const key = makeKey(schema, block.type as string);

  const fn =
    validators.get(key) ??
    validators.get(makeKey(null, block.type as string));

  if (!fn) return block;

  return fn(block);
}

export function validateNightfireValue(
  value: NightfireValue
): NightfireValue {
  if (!value || typeof value !== "object") return value;

  const schema = value.schema ?? null;

  if (value.block) {
    const nextBlock = validateBlock(schema, value.block);
    return {
      ...value,
      block: nextBlock
    };
  }

  if (Array.isArray(value.blocks)) {
    const nextBlocks = value.blocks.map((b) => validateBlock(schema, b));
    return {
      ...value,
      blocks: nextBlocks
    };
  }

  return value;
}

export function prepareNightfireForSave(
  value: NightfireValue
): NightfireValue {
  return validateNightfireValue(value);
}
