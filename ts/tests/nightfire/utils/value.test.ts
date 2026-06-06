import { describe, expect, it } from "vitest";
import { normaliseNightfireValue } from "../../../src/nightfire/utils";

describe("normaliseNightfireValue", () => {
	const testSchema = "test:content/article@1";

	describe("valid NightfireValue objects", () => {
		it("passes through valid NightfireValue unchanged", () => {
			const value = {
				schema: testSchema,
				block: {
					type: "markdown",
					version: "initial",
					hash: "abc123",
					data: { text: "Hello world" },
				},
			};
			const result = normaliseNightfireValue(value, testSchema);
			expect(result).toEqual(value);
		});

		it("passes through NightfireValue with different schema", () => {
			const value = {
				schema: "other:schema@1",
				block: { type: "custom", version: "v1", hash: "", data: {} },
			};
			const result = normaliseNightfireValue(value, testSchema);
			expect(result.schema).toBe("other:schema@1");
		});

		it("passes through NightfireValue with blocks array", () => {
			const value = {
				schema: testSchema,
				blocks: [
					{ type: "markdown", version: "initial", hash: "", data: { text: "One" } },
					{ type: "markdown", version: "initial", hash: "", data: { text: "Two" } },
				],
			};
			const result = normaliseNightfireValue(value, testSchema);
			expect(result).toEqual(value);
		});
	});

	describe("raw string conversion", () => {
		it("converts non-empty string to markdown block", () => {
			const result = normaliseNightfireValue("Hello world", testSchema);
			expect(result).toEqual({
				schema: testSchema,
				block: {
					type: "markdown",
					version: "initial",
					hash: "",
					data: { text: "Hello world" },
				},
			});
		});

		it("converts string with markdown content", () => {
			const markdown = "# Title\n\nSome **bold** text.";
			const result = normaliseNightfireValue(markdown, testSchema);
			expect(result.block?.data).toEqual({ text: markdown });
		});

		it("does not convert string if markdown not in allowed types", () => {
			const result = normaliseNightfireValue("Hello", testSchema, ["custom", "other"]);
			expect(result).toEqual({
				schema: testSchema,
				block: {
					type: "custom",
					version: "initial",
					hash: "",
					data: {},
				},
			});
		});

		it("falls back to markdown type when allowed block types list is empty", () => {
			const result = normaliseNightfireValue("Hello", testSchema, []);
			expect(result).toEqual({
				schema: testSchema,
				block: {
					type: "markdown",
					version: "initial",
					hash: "",
					data: {},
				},
			});
		});

		it("converts string if markdown is in allowed types", () => {
			const result = normaliseNightfireValue("Hello", testSchema, ["markdown", "custom"]);
			expect(result.block?.type).toBe("markdown");
			expect(result.block?.data).toEqual({ text: "Hello" });
		});
	});

	describe("null and undefined handling", () => {
		it("returns minimal value for null", () => {
			const result = normaliseNightfireValue(null, testSchema);
			expect(result).toEqual({
				schema: testSchema,
				block: undefined,
			});
		});

		it("returns minimal value for undefined", () => {
			const result = normaliseNightfireValue(undefined, testSchema);
			expect(result).toEqual({
				schema: testSchema,
				block: undefined,
			});
		});

		it("returns minimal value for empty string", () => {
			const result = normaliseNightfireValue("", testSchema);
			expect(result).toEqual({
				schema: testSchema,
				block: undefined,
			});
		});
	});

	describe("edge cases", () => {
		it("handles number input", () => {
			const result = normaliseNightfireValue(123 as any, testSchema);
			expect(result).toEqual({
				schema: testSchema,
				block: undefined,
			});
		});

		it("handles array input", () => {
			const result = normaliseNightfireValue(["a", "b"] as any, testSchema);
			expect(result).toEqual({
				schema: testSchema,
				block: undefined,
			});
		});

		it("handles object without schema property", () => {
			const result = normaliseNightfireValue({ foo: "bar" } as any, testSchema);
			expect(result).toEqual({
				schema: testSchema,
				block: undefined,
			});
		});
	});
});
