import type { FilterConfig } from "./types.js";

export function getInitialFilterValues(
  filters?: FilterConfig[],
): Record<string, string | undefined> {
  const values: Record<string, string | undefined> = {};
  if (!filters) return values;

  for (const filter of filters) {
    values[filter.key] = filter.defaultValue;
  }
  return values;
}
