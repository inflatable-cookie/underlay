/**
 * Sequence utilities for generating next values in ordered lists.
 *
 * @example
 * ```typescript
 * import { getNextLetter, getNextNumber } from '@decodelabs/underlay/utils';
 *
 * // Get next available letter
 * const existingLabels = ['A', 'B', 'D'];
 * const nextLabel = getNextLetter(existingLabels); // 'C'
 *
 * // Get next available number
 * const existingNumbers = [1, 2, 3];
 * const nextNumber = getNextNumber(existingNumbers); // 4
 * ```
 *
 * @module
 */

/**
 * Options for getNextLetter.
 */
export interface GetNextLetterOptions {
  /** Return lowercase letters instead of uppercase (default: false) */
  lowercase?: boolean;
}

/**
 * Get the next available letter not in the existing set.
 *
 * Tries single letters first (A-Z or a-z), then double letters.
 * Returns the first gap found, or the next letter after the highest.
 *
 * @param existing - Array of existing letter labels
 * @param options - Configuration options
 * @returns The next available letter
 *
 * @example
 * ```typescript
 * getNextLetter(['A', 'B', 'C']);              // 'D'
 * getNextLetter(['A', 'C', 'D']);              // 'B' (fills gap)
 * getNextLetter(['a', 'b'], { lowercase: true }); // 'c'
 * getNextLetter([]);                           // 'A'
 * ```
 */
export function getNextLetter(
  existing: string[],
  options?: GetNextLetterOptions
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
      const upperCombo = String.fromCharCode(65 + i) + String.fromCharCode(65 + j);
      if (!used.has(upperCombo)) {
        return String.fromCharCode(baseCode + i) + String.fromCharCode(baseCode + j);
      }
    }
  }

  // Fallback (unlikely to be reached with 702 options)
  return lowercase ? "zz" : "ZZ";
}

/**
 * Get the next available positive integer.
 *
 * Returns max + 1 for the given array, or 1 if array is empty.
 *
 * @param existing - Array of existing numbers
 * @returns The next number in sequence
 *
 * @example
 * ```typescript
 * getNextNumber([1, 2, 3]); // 4
 * getNextNumber([1, 5, 3]); // 6 (max + 1)
 * getNextNumber([]);        // 1
 * ```
 */
export function getNextNumber(existing: number[]): number {
  if (existing.length === 0) return 1;
  return Math.max(...existing) + 1;
}
