export interface SuggestionRequestOptions {
  suggestions?: boolean;
  recentHints?: string[];
}

export function formatHintsParam(hints: string[]): string {
  return hints.join(",");
}

export function parseHintsParam(value: string | null | undefined): string[] {
  if (!value) {
    return [];
  }
  return value
    .split(",")
    .map((entry) => entry.trim())
    .filter((entry) => entry.length > 0);
}

export function buildSuggestionParams(
  options?: SuggestionRequestOptions,
): URLSearchParams {
  const params = new URLSearchParams();

  if (options?.suggestions) {
    params.set("suggestions", "true");
  }
  if (options?.recentHints && options.recentHints.length > 0) {
    params.set("recentHints", formatHintsParam(options.recentHints));
  }

  return params;
}

export function appendSuggestionParams(
  basePath: string,
  options?: SuggestionRequestOptions,
): string {
  const queryString = buildSuggestionParams(options).toString();
  if (!queryString) {
    return basePath;
  }

  if (!basePath.includes("?")) {
    return `${basePath}?${queryString}`;
  }

  // Merge into existing query string to avoid duplicate keys
  const [path, existingQuery] = basePath.split("?", 2);
  const merged = new URLSearchParams(existingQuery);
  const newParams = new URLSearchParams(queryString);
  for (const [key, value] of newParams) {
    merged.set(key, value);
  }
  return `${path}?${merged.toString()}`;
}
