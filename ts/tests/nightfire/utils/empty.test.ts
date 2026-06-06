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

		it("returns true for object with no block or blocks", () => {
			expect(isEmptyNightfire({ schema: "test@1" })).toBe(true);
		});

		it("returns true for object with null block", () => {
			expect(isEmptyNightfire({ schema: "test@1", block: null } as any)).toBe(true);
		});

		it("returns true for object with empty blocks array", () => {
			expect(isEmptyNightfire({ schema: "test@1", blocks: [] } as any)).toBe(true);
		});

		it("returns true for object with undefined block", () => {
			expect(isEmptyNightfire({ schema: "test@1", block: undefined })).toBe(true);
		});
	});

	describe("non-empty values", () => {
		it("returns false for object with block", () => {
			const value = {
				schema: "test@1",
				block: { type: "markdown", version: "initial", hash: "", data: {} },
			};
			expect(isEmptyNightfire(value)).toBe(false);
		});

		it("returns false for object with non-empty blocks array", () => {
			const value = {
				schema: "test@1",
				blocks: [{ type: "markdown", version: "initial", hash: "", data: {} }],
			} as any;
			expect(isEmptyNightfire(value)).toBe(false);
		});

		it("returns false for block with empty data", () => {
			const value = {
				schema: "test@1",
				block: { type: "markdown", version: "initial", hash: "", data: {} },
			};
			expect(isEmptyNightfire(value)).toBe(false);
		});

		it("treats structurally present block as non-empty when contentLevel is false", () => {
			const value = {
				schema: "test@1",
				block: { type: "markdown", version: "initial", hash: "", data: { text: "" } },
			};
			expect(isEmptyNightfire(value, false)).toBe(false);
		});

		it("treats structurally present blocks array as non-empty when contentLevel is false", () => {
			const value = {
				schema: "test@1",
				blocks: [{ type: "markdown", version: "initial", hash: "", data: { text: "" } }],
			} as any;
			expect(isEmptyNightfire(value, false)).toBe(false);
		});
	});
});
