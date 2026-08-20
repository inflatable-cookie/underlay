import type { NightfireBlock, NightfireValue } from "./types";

export interface NightfireMediaLocator {
  blockId: string;
  dataPointer: string;
}

function isValidDataPointer(dataPointer: string): boolean {
  return dataPointer === "" || dataPointer.startsWith("/");
}

function decodePointerSegment(segment: string): string {
  return segment.replace(/~1/g, "/").replace(/~0/g, "~");
}

function resolveJsonPointer(value: unknown, pointer: string): unknown {
  if (pointer === "") {
    return value;
  }

  return pointer
    .slice(1)
    .split("/")
    .map(decodePointerSegment)
    .reduce<unknown>((current, segment) => {
      if (current === null || typeof current !== "object") {
        return undefined;
      }

      if (Array.isArray(current)) {
        const index = Number(segment);
        return Number.isInteger(index) ? current[index] : undefined;
      }

      return (current as Record<string, unknown>)[segment];
    }, value);
}

export function formatNightfireMediaLocator(locator: NightfireMediaLocator): string {
  const blockId = locator.blockId.trim();
  if (!blockId) {
    throw new Error("Nightfire media locator requires a blockId");
  }

  if (!isValidDataPointer(locator.dataPointer)) {
    throw new Error("Nightfire media locator requires an empty or slash-prefixed JSON Pointer");
  }

  return `${blockId}#${locator.dataPointer}`;
}

export function parseNightfireMediaLocator(locatorKey: string): NightfireMediaLocator {
  const separator = locatorKey.indexOf("#");
  if (separator < 0) {
    throw new Error("Nightfire media locator must contain '#'");
  }

  const blockId = locatorKey.slice(0, separator).trim();
  const dataPointer = locatorKey.slice(separator + 1);

  if (!blockId) {
    throw new Error("Nightfire media locator requires a blockId");
  }

  if (!isValidDataPointer(dataPointer)) {
    throw new Error("Nightfire media locator requires an empty or slash-prefixed JSON Pointer");
  }

  return { blockId, dataPointer };
}

export function findNightfireBlockById(
  value: NightfireValue,
  blockId: string
): NightfireBlock | null {
  if (!Array.isArray(value.blocks)) {
    return null;
  }

  return value.blocks.find((block) => block.id === blockId) ?? null;
}

export function resolveNightfireMediaLocator(
  value: NightfireValue,
  locator: NightfireMediaLocator
): unknown {
  const block = findNightfireBlockById(value, locator.blockId);
  if (!block) {
    return undefined;
  }

  return resolveJsonPointer(block.data ?? {}, locator.dataPointer);
}
