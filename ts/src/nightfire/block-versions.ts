export interface BlockVersionSpec {
  current: string;
  supported: string[];
}

const DEFAULT_VERSIONS: BlockVersionSpec = {
  current: "initial",
  supported: ["initial"]
};

const blockVersions = new Map<string, BlockVersionSpec>();

export function registerBlockVersions(
  type: string,
  spec: BlockVersionSpec
): void {
  blockVersions.set(type, spec);
}

export function getBlockVersions(type: string): BlockVersionSpec {
  return blockVersions.get(type) ?? DEFAULT_VERSIONS;
}

/**
 * Resolve a stored block version to the current implementation.
 * Unknown versions fail closed (`null`).
 */
export function resolveBlockVersion(
  type: string,
  version: string
): string | null {
  const spec = getBlockVersions(type);
  return spec.supported.includes(version) ? spec.current : null;
}

export function coerceBlockVersion(
  type: string,
  version: string
): string | null {
  return resolveBlockVersion(type, version);
}
