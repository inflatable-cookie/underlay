import {
  coerceNightfireBlock,
  type NightfireBlock,
  type NightfireDraftValue,
  type NightfireValue
} from "./types";
import { ensureNightfireBlockIds } from "./block-ids";
import { coerceBlockVersion } from "./block-versions";

type ValidatorKey = string;

export type BlockValidator = (block: unknown) => unknown;
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

function validateBlock(schema: string | null, block: unknown): NightfireBlock | null {
  const typedBlock = coerceNightfireBlock(block);
  if (!typedBlock) {
    return null;
  }
  const currentVersion = coerceBlockVersion(typedBlock.type, typedBlock.version);
  if (!currentVersion) {
    return null;
  }
  typedBlock.version = currentVersion;

  const key = makeKey(schema, typedBlock.type);

  const fn =
    validators.get(key) ??
    validators.get(makeKey(null, typedBlock.type));

  if (!fn) return typedBlock;

  return coerceNightfireBlock(fn(typedBlock), typedBlock.type);
}

export function validateNightfireValue(
  value: NightfireDraftValue
): NightfireDraftValue {
  if (!value || typeof value !== "object") return value;

  const schema = value.schema ?? "";
  const source = Array.isArray(value.blocks) ? value.blocks : [];
  const nextBlocks = source
    .map((block) => validateBlock(schema, block))
    .filter((block): block is NightfireBlock => block !== null);

  return {
    schema,
    blocks: nextBlocks
  };
}

export function prepareNightfireForSave(
  value: NightfireDraftValue
): NightfireValue | null {
  const validated = validateNightfireValue(value);
  if (!Array.isArray(validated.blocks) || validated.blocks.length === 0) {
    return null;
  }
  return ensureNightfireBlockIds({
    schema: validated.schema,
    blocks: validated.blocks
  });
}

/**
 * Write a Nightfire field into FormData using the canonical save boundary.
 *
 * This keeps the outer form field contract simple while ensuring the inner
 * Nightfire JSON is validated, block-id-stable, and serialized verbatim.
 *
 * Caller note:
 * - map surrounding DTO field names at the API boundary if needed
 * - do not rewrite keys inside block `data` objects; shared extractors match
 *   the stored Nightfire JSON exactly
 */
export function writePreparedNightfireToFormData(
  formData: FormData,
  name: string,
  value: NightfireDraftValue | null | undefined
): void {
  const prepared = value ? prepareNightfireForSave(value) : null;
  formData.set(name, prepared ? JSON.stringify(prepared) : "");
}
