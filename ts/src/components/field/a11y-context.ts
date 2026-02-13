export const FIELD_A11Y_CONTEXT_KEY = "underlayFieldA11y";

export interface FieldA11yContext {
  controlId: () => string;
  matchesControl: (controlId: string | undefined) => boolean;
  errorId: () => string | undefined;
  hasError: () => boolean;
}

export function mergeAriaDescribedBy(
  ...values: Array<string | null | undefined>
): string | undefined {
  const tokens = values
    .flatMap((value) => (value ?? "").split(/\s+/))
    .map((token) => token.trim())
    .filter(Boolean);

  if (tokens.length === 0) {
    return undefined;
  }

  return Array.from(new Set(tokens)).join(" ");
}
