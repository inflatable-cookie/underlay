/**
 * Tests for sequence utility functions.
 */

import { describe, it, expect } from "vitest";
import { getNextLetter, getNextNumber } from "../../src/utils/sequence";

// ============================================================================
// getNextLetter
// ============================================================================

describe("getNextLetter", () => {
	describe("basic behavior", () => {
		it("returns A for empty array", () => {
			expect(getNextLetter([])).toBe("A");
		});

		it("returns next letter in sequence", () => {
			expect(getNextLetter(["A"])).toBe("B");
			expect(getNextLetter(["A", "B"])).toBe("C");
			expect(getNextLetter(["A", "B", "C"])).toBe("D");
		});

		it("returns Z after Y", () => {
			const letters = Array.from({ length: 25 }, (_, i) =>
				String.fromCharCode(65 + i)
			);
			expect(getNextLetter(letters)).toBe("Z");
		});
	});

	describe("gap filling", () => {
		it("fills gaps in sequence", () => {
			expect(getNextLetter(["A", "C"])).toBe("B");
			expect(getNextLetter(["A", "C", "D"])).toBe("B");
			expect(getNextLetter(["B", "C", "D"])).toBe("A");
		});

		it("returns first missing letter", () => {
			expect(getNextLetter(["B"])).toBe("A");
			expect(getNextLetter(["A", "D", "E"])).toBe("B");
		});
	});

	describe("case handling", () => {
		it("is case insensitive when checking existing", () => {
			expect(getNextLetter(["a"])).toBe("B");
			expect(getNextLetter(["A"])).toBe("B");
			expect(getNextLetter(["a", "b"])).toBe("C");
		});

		it("returns uppercase by default", () => {
			expect(getNextLetter(["a", "b"])).toBe("C");
		});

		it("returns lowercase when option is set", () => {
			expect(getNextLetter([], { lowercase: true })).toBe("a");
			expect(getNextLetter(["a"], { lowercase: true })).toBe("b");
			expect(getNextLetter(["A", "B"], { lowercase: true })).toBe("c");
		});
	});

	describe("double letters", () => {
		it("returns AA after all single letters are used", () => {
			const allSingle = Array.from({ length: 26 }, (_, i) =>
				String.fromCharCode(65 + i)
			);
			expect(getNextLetter(allSingle)).toBe("AA");
		});

		it("returns AB after AA", () => {
			const allSingle = Array.from({ length: 26 }, (_, i) =>
				String.fromCharCode(65 + i)
			);
			expect(getNextLetter([...allSingle, "AA"])).toBe("AB");
		});

		it("handles mixed single and double letters", () => {
			const existing = ["A", "B", "C", "AA"];
			expect(getNextLetter(existing)).toBe("D");
		});

		it("returns lowercase double letters when option is set", () => {
			const allSingle = Array.from({ length: 26 }, (_, i) =>
				String.fromCharCode(65 + i)
			);
			expect(getNextLetter(allSingle, { lowercase: true })).toBe("aa");
		});

		it("returns ZZ fallback when all single and double labels are used", () => {
			const allSingle = Array.from({ length: 26 }, (_, i) =>
				String.fromCharCode(65 + i)
			);
			const allDouble = Array.from({ length: 26 }, (_, i) =>
				Array.from(
					{ length: 26 },
					(_, j) => `${String.fromCharCode(65 + i)}${String.fromCharCode(65 + j)}`
				)
			).flat();
			expect(getNextLetter([...allSingle, ...allDouble])).toBe("ZZ");
		});
	});

	describe("edge cases", () => {
		it("handles duplicate entries in existing array", () => {
			expect(getNextLetter(["A", "A", "B", "B"])).toBe("C");
		});

		it("handles non-letter strings gracefully", () => {
			// Non-letters won't match any letter checks, so should return A
			expect(getNextLetter(["1", "2", "3"])).toBe("A");
		});

		it("handles empty strings in array", () => {
			expect(getNextLetter(["", "A"])).toBe("B");
		});
	});
});

// ============================================================================
// getNextNumber
// ============================================================================

describe("getNextNumber", () => {
	describe("basic behavior", () => {
		it("returns 1 for empty array", () => {
			expect(getNextNumber([])).toBe(1);
		});

		it("returns max + 1", () => {
			expect(getNextNumber([1])).toBe(2);
			expect(getNextNumber([1, 2])).toBe(3);
			expect(getNextNumber([1, 2, 3])).toBe(4);
		});

		it("handles non-sequential numbers", () => {
			expect(getNextNumber([1, 5, 3])).toBe(6);
			expect(getNextNumber([10, 20, 30])).toBe(31);
		});
	});

	describe("does not fill gaps", () => {
		it("returns max + 1 even with gaps", () => {
			expect(getNextNumber([1, 3])).toBe(4);
			expect(getNextNumber([1, 5, 10])).toBe(11);
		});
	});

	describe("edge cases", () => {
		it("handles single element", () => {
			expect(getNextNumber([1])).toBe(2);
			expect(getNextNumber([100])).toBe(101);
		});

		it("handles large numbers", () => {
			expect(getNextNumber([999999])).toBe(1000000);
		});

		it("handles negative numbers", () => {
			expect(getNextNumber([-5, -3, -1])).toBe(0);
		});

		it("handles zero", () => {
			expect(getNextNumber([0])).toBe(1);
			expect(getNextNumber([-1, 0, 1])).toBe(2);
		});

		it("handles duplicate entries", () => {
			expect(getNextNumber([1, 1, 2, 2, 3, 3])).toBe(4);
		});

		it("handles unsorted array", () => {
			expect(getNextNumber([5, 1, 3, 4, 2])).toBe(6);
		});
	});
});
