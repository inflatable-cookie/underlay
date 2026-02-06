/**
 * Tests for formatting utilities.
 */

import { describe, it, expect } from "vitest";
import {
	toRomanNumerals,
	formatRomanRange,
	formatOutcomeLabel,
} from "../../src/client/format";

// ============================================================================
// toRomanNumerals
// ============================================================================

describe("toRomanNumerals", () => {
	describe("basic numerals", () => {
		it("converts 1-10", () => {
			expect(toRomanNumerals(1)).toBe("i");
			expect(toRomanNumerals(2)).toBe("ii");
			expect(toRomanNumerals(3)).toBe("iii");
			expect(toRomanNumerals(4)).toBe("iv");
			expect(toRomanNumerals(5)).toBe("v");
			expect(toRomanNumerals(6)).toBe("vi");
			expect(toRomanNumerals(7)).toBe("vii");
			expect(toRomanNumerals(8)).toBe("viii");
			expect(toRomanNumerals(9)).toBe("ix");
			expect(toRomanNumerals(10)).toBe("x");
		});

		it("converts multiples of 10", () => {
			expect(toRomanNumerals(10)).toBe("x");
			expect(toRomanNumerals(20)).toBe("xx");
			expect(toRomanNumerals(30)).toBe("xxx");
			expect(toRomanNumerals(40)).toBe("xl");
			expect(toRomanNumerals(50)).toBe("l");
			expect(toRomanNumerals(60)).toBe("lx");
			expect(toRomanNumerals(70)).toBe("lxx");
			expect(toRomanNumerals(80)).toBe("lxxx");
			expect(toRomanNumerals(90)).toBe("xc");
		});

		it("converts multiples of 100", () => {
			expect(toRomanNumerals(100)).toBe("c");
			expect(toRomanNumerals(200)).toBe("cc");
			expect(toRomanNumerals(300)).toBe("ccc");
			expect(toRomanNumerals(400)).toBe("cd");
			expect(toRomanNumerals(500)).toBe("d");
			expect(toRomanNumerals(600)).toBe("dc");
			expect(toRomanNumerals(700)).toBe("dcc");
			expect(toRomanNumerals(800)).toBe("dccc");
			expect(toRomanNumerals(900)).toBe("cm");
		});

		it("converts multiples of 1000", () => {
			expect(toRomanNumerals(1000)).toBe("m");
			expect(toRomanNumerals(2000)).toBe("mm");
			expect(toRomanNumerals(3000)).toBe("mmm");
		});
	});

	describe("compound numbers", () => {
		it("converts common years", () => {
			expect(toRomanNumerals(1999)).toBe("mcmxcix");
			expect(toRomanNumerals(2000)).toBe("mm");
			expect(toRomanNumerals(2024)).toBe("mmxxiv");
		});

		it("converts various compound numbers", () => {
			expect(toRomanNumerals(14)).toBe("xiv");
			expect(toRomanNumerals(49)).toBe("xlix");
			expect(toRomanNumerals(99)).toBe("xcix");
			expect(toRomanNumerals(444)).toBe("cdxliv");
			expect(toRomanNumerals(888)).toBe("dccclxxxviii");
		});

		it("handles maximum value (3999)", () => {
			expect(toRomanNumerals(3999)).toBe("mmmcmxcix");
		});
	});

	describe("edge cases", () => {
		it("returns string for numbers < 1", () => {
			expect(toRomanNumerals(0)).toBe("0");
			expect(toRomanNumerals(-1)).toBe("-1");
			expect(toRomanNumerals(-100)).toBe("-100");
		});

		it("returns string for numbers > 3999", () => {
			expect(toRomanNumerals(4000)).toBe("4000");
			expect(toRomanNumerals(10000)).toBe("10000");
		});
	});

	describe("returns lowercase", () => {
		it("always returns lowercase numerals", () => {
			const result = toRomanNumerals(1994);
			expect(result).toBe("mcmxciv");
			expect(result).toBe(result.toLowerCase());
		});
	});
});

// ============================================================================
// formatRomanRange
// ============================================================================

describe("formatRomanRange", () => {
	describe("both from and to specified", () => {
		it("formats simple range", () => {
			expect(formatRomanRange(1, 3)).toBe("i-iii");
			expect(formatRomanRange(5, 10)).toBe("v-x");
		});

		it("formats single value when from equals to", () => {
			expect(formatRomanRange(7, 7)).toBe("vii");
			expect(formatRomanRange(1, 1)).toBe("i");
		});
	});

	describe("only from specified", () => {
		it("formats as open-ended range", () => {
			expect(formatRomanRange(5, null)).toBe("v+");
			expect(formatRomanRange(10, undefined)).toBe("x+");
		});
	});

	describe("only to specified", () => {
		it("formats as range from 1", () => {
			expect(formatRomanRange(null, 10)).toBe("i-x");
			expect(formatRomanRange(undefined, 5)).toBe("i-v");
		});
	});

	describe("neither specified", () => {
		it("returns null", () => {
			expect(formatRomanRange(null, null)).toBeNull();
			expect(formatRomanRange(undefined, undefined)).toBeNull();
		});
	});

	describe("edge cases", () => {
		it("handles large numbers", () => {
			expect(formatRomanRange(100, 200)).toBe("c-cc");
		});

		it("handles numbers outside roman range", () => {
			// Numbers > 3999 are returned as-is
			expect(formatRomanRange(1, 5000)).toBe("i-5000");
		});
	});
});

// ============================================================================
// formatOutcomeLabel
// ============================================================================

describe("formatOutcomeLabel", () => {
	describe("with range", () => {
		it("returns key and range", () => {
			const result = formatOutcomeLabel("A1a", 1, 3);
			expect(result).toEqual({ key: "A1a", range: "i-iii" });
		});

		it("returns key and single value for equal from/to", () => {
			const result = formatOutcomeLabel("B2b", 5, 5);
			expect(result).toEqual({ key: "B2b", range: "v" });
		});
	});

	describe("with partial range", () => {
		it("returns open-ended range for from only", () => {
			const result = formatOutcomeLabel("C3c", 7, null);
			expect(result).toEqual({ key: "C3c", range: "vii+" });
		});

		it("returns range from 1 for to only", () => {
			const result = formatOutcomeLabel("D4d", null, 10);
			expect(result).toEqual({ key: "D4d", range: "i-x" });
		});
	});

	describe("without range", () => {
		it("returns key with null range", () => {
			const result = formatOutcomeLabel("E5e", null, null);
			expect(result).toEqual({ key: "E5e", range: null });
		});
	});

	describe("preserves key exactly", () => {
		it("does not modify the key", () => {
			const result = formatOutcomeLabel("ABC-123_test", 1, 2);
			expect(result.key).toBe("ABC-123_test");
		});
	});
});
