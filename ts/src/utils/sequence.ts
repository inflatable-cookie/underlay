export interface GetNextLetterOptions {
  lowercase?: boolean;
}

/**
 * Get the next available letter not in the existing set.
 *
 * Tries single letters first (A-Z or a-z), then double letters.
 * Returns the first gap found, or the next letter after the highest.
 *
 * Tries single letters first, then double letters.
 */
export function getNextLetter(
  existing: string[],
  options?: GetNextLetterOptions,
): string {
  const lowercase = options?.lowercase ?? false;
  const baseCode = lowercase ? 97 : 65; // 'a' = 97, 'A' = 65
  const used = new Set(existing.map((s) => s.toUpperCase()));

  // Try single letters first (A-Z)
  for (let i = 0; i < 26; i++) {
    const upperLetter = String.fromCharCode(65 + i);
    if (!used.has(upperLetter)) {
      return String.fromCharCode(baseCode + i);
    }
  }

  // Then double letters (AA-ZZ)
  for (let i = 0; i < 26; i++) {
    for (let j = 0; j < 26; j++) {
      const upperCombo =
        String.fromCharCode(65 + i) + String.fromCharCode(65 + j);
      if (!used.has(upperCombo)) {
        return (
          String.fromCharCode(baseCode + i) + String.fromCharCode(baseCode + j)
        );
      }
    }
  }

  return lowercase ? "zz" : "ZZ";
}

/**
 * Get the next available positive integer.
 */
export function getNextNumber(existing: number[]): number {
  if (existing.length === 0) return 1;
  return Math.max(...existing) + 1;
}
