/**
 * Convert a number to lowercase roman numerals.
 * Supports integers 1-3999; anything else is returned as its string form.
 */
export function toRomanNumerals(num: number): string {
	if (!Number.isInteger(num) || num < 1 || num > 3999) {
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
		[1, "i"],
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
