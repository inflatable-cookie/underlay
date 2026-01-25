/**
 * Convert a number to lowercase roman numerals.
 * Supports numbers 1-3999.
 */
export function toRomanNumerals(num: number): string {
  if (num < 1 || num > 3999) {
    return String(num);
  }

  const romanNumerals: [number, string][] = [
    [1000, "m"],
    [900, "cm"],
    [500, "d"],
    [400, "cd"],
    [100, "c"],
    [90, "xc"],
    [50, "l"],
    [40, "xl"],
    [10, "x"],
    [9, "ix"],
    [5, "v"],
    [4, "iv"],
    [1, "i"]
  ];

  let result = "";
  let remaining = num;

  for (const [value, numeral] of romanNumerals) {
    while (remaining >= value) {
      result += numeral;
      remaining -= value;
    }
  }

  return result;
}

/**
 * Format a number range as roman numerals.
 * Returns null if both from and to are null/undefined.
 *
 * Examples:
 * - formatRomanRange(1, 3) => "i-iii"
 * - formatRomanRange(5, null) => "v+"
 * - formatRomanRange(null, 10) => "i-x"
 * - formatRomanRange(7, 7) => "vii"
 */
export function formatRomanRange(
  from: number | null | undefined,
  to: number | null | undefined
): string | null {
  if (from == null && to == null) {
    return null;
  }

  if (from != null && to != null) {
    if (from === to) {
      return toRomanNumerals(from);
    }
    return `${toRomanNumerals(from)}-${toRomanNumerals(to)}`;
  }

  if (from != null) {
    return `${toRomanNumerals(from)}+`;
  }

  // to != null, from == null - assume starting from 1
  return `i-${toRomanNumerals(to!)}`;
}

/**
 * Format an outcome label with optional roman numeral range.
 *
 * Examples:
 * - formatOutcomeLabel("A1a", 1, 3) => { key: "A1a", range: "i-iii" }
 * - formatOutcomeLabel("B2b", null, null) => { key: "B2b", range: null }
 */
export function formatOutcomeLabel(
  key: string,
  numberFrom: number | null | undefined,
  numberTo: number | null | undefined
): { key: string; range: string | null } {
  return {
    key,
    range: formatRomanRange(numberFrom, numberTo)
  };
}
