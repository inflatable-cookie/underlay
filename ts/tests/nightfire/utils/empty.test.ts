import { describe, expect, it } from "vitest";
import { isEmptyNightfire } from "../../../src/nightfire/utils";

describe("isEmptyNightfire", () => {
	describe("empty values", () => {
		it("returns true for null", () => {
			expect(isEmptyNightfire(null)).toBe(true);
		});

		it("returns true for undefined", () => {
			expect(isEmptyNightfire(undefined)).toBe(true);
		});

		it("returns true for non-object", () => {
			expect(isEmptyNightfire("string" as any)).toBe(true);
			expect(isEmptyNightfire(123 as any)).toBe(true);
		});

		it("returns true for object with no blocks", () => {
			expect(isEmptyNightfire({ schema: "test" } as any)).toBe(true);
		});

		it("returns true for object with empty blocks array", () => {
			expect(isEmptyNightfire({ schema: "test", blocks: [] })).toBe(true);
		});

		it("returns true for leftover v1 block field", () => {
			expect(isEmptyNightfire({ schema: "test", block: { type: "markdown" } } as any)).toBe(true);
		});
	});

	describe("non-empty values", () => {
		it("returns false for object with non-empty blocks array", () => {
			const value = {
				schema: "test",
				blocks: [{ type: "markdown", version: "initial", data: {} }],
			};
			expect(isEmptyNightfire(value)).toBe(false);
		});

		it("returns false for block with empty data", () => {
			const value = {
				schema: "test",
				blocks: [{ type: "markdown", version: "initial", data: {} }],
			};
			expect(isEmptyNightfire(value)).toBe(false);
		});

		it("treats structurally present blocks as non-empty when contentLevel is false", () => {
			const value = {
				schema: "test",
				blocks: [{ type: "markdown", version: "initial", data: { text: "" } }],
			};
			expect(isEmptyNightfire(value, false)).toBe(false);
		});
	});
});
