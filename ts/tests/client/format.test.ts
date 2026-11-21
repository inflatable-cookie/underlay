/**
 * Tests for formatting utilities.
 */

import { describe, it, expect } from "vitest";
import { toRomanNumerals } from "../../src/client/format";

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

		it("returns string for non-finite numbers", () => {
			expect(toRomanNumerals(Number.NaN)).toBe("NaN");
			expect(toRomanNumerals(Number.POSITIVE_INFINITY)).toBe("Infinity");
			expect(toRomanNumerals(Number.NEGATIVE_INFINITY)).toBe("-Infinity");
		});

		it("returns string for non-integer numbers", () => {
			expect(toRomanNumerals(1.5)).toBe("1.5");
			expect(toRomanNumerals(3999.5)).toBe("3999.5");
			expect(toRomanNumerals(2.0)).toBe("ii");
		});
	});
});
