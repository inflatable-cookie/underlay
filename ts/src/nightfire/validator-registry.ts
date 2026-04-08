import type { NightfireValue } from "./types";

type ValidatorKey = string;

type BlockValidator = (block: unknown) => unknown;
type TypedBlock = { type: string } & Record<string, unknown>;

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

function validateBlock(schema: string | null, block: unknown): unknown {
  if (!block || typeof block !== "object") {
    return block;
  }

  const candidate = block as Partial<TypedBlock>;
  if (typeof candidate.type !== "string") {
    return block;
  }

  const typedBlock = block as TypedBlock;
  const key = makeKey(schema, typedBlock.type);

  const fn =
    validators.get(key) ??
    validators.get(makeKey(null, typedBlock.type));

  if (!fn) return block;

  return fn(typedBlock);
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
    const nextBlocks = value.blocks.map((block) => validateBlock(schema, block));
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
