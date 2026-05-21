/**
 * Tests for Nightfire utility functions.
 */

import { describe, it, expect } from "vitest";
import {
	normaliseNightfireValue,
	normaliseNightfireBlock,
	isEmptyNightfire,
	writeNightfireToFormData,
	type NightfireBlockDefinition,
	type NightfireTypeOption,
} from "../../src/nightfire/utils";

// ============================================================================
// normaliseNightfireValue
// ============================================================================

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
			// Keeps original schema from value, not the provided one
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

// ============================================================================
// normaliseNightfireBlock
// ============================================================================

describe("normaliseNightfireBlock", () => {
	const typeOptions: NightfireTypeOption[] = [
		{ type: "markdown", label: "Markdown" },
		{ type: "custom", label: "Custom" },
	];

	const definition: NightfireBlockDefinition = {
		schema: "test:schema@1",
		mode: "single",
		defaultType: "markdown",
	};

	describe("valid block normalization", () => {
		it("preserves valid block with allowed type", () => {
			const block = {
				type: "markdown",
				version: "v1",
				hash: "abc123",
				data: { text: "Hello" },
			};
			const result = normaliseNightfireBlock(block, typeOptions, definition);
			expect(result).toEqual(block);
		});

		it("preserves custom block type if in options", () => {
			const block = {
				type: "custom",
				version: "v2",
				hash: "def456",
				data: { foo: "bar" },
			};
			const result = normaliseNightfireBlock(block, typeOptions, definition);
			expect(result.type).toBe("custom");
		});
	});

	describe("type fallback", () => {
		it("falls back to first type option if type not allowed", () => {
			const block = {
				type: "unknown",
				version: "v1",
				hash: "",
				data: {},
			};
			const result = normaliseNightfireBlock(block, typeOptions, definition);
			expect(result.type).toBe("markdown"); // First in typeOptions
		});

		it("falls back to definition defaultType if no type options", () => {
			const block = { type: "unknown" };
			const emptyOptions: NightfireTypeOption[] = [];
			const result = normaliseNightfireBlock(block, emptyOptions, definition);
			expect(result.type).toBe("markdown"); // From definition.defaultType
		});

		it("falls back to markdown when no options and no definition defaultType", () => {
			const block = { type: "unknown" };
			const emptyOptions: NightfireTypeOption[] = [];
			const noDefaultDefinition = {
				schema: "test:schema@1",
				mode: "single",
				defaultType: undefined,
			} as any;
			const result = normaliseNightfireBlock(
				block,
				emptyOptions,
				noDefaultDefinition
			);
			expect(result.type).toBe("markdown");
		});
	});

	describe("null and undefined handling", () => {
		it("creates default block for null", () => {
			const result = normaliseNightfireBlock(null, typeOptions, definition);
			expect(result).toEqual({
				type: "markdown",
				version: "initial",
				hash: "",
				data: {},
			});
		});

		it("creates default block for undefined", () => {
			const result = normaliseNightfireBlock(undefined, typeOptions, definition);
			expect(result).toEqual({
				type: "markdown",
				version: "initial",
				hash: "",
				data: {},
			});
		});

		it("uses schema defaultType before the first registered type option", () => {
			const result = normaliseNightfireBlock(
				null,
				[
					{ type: "content.list", label: "Content List" },
					{ type: "markdown", label: "Markdown" },
				],
				{
					schema: "acow:content/description@1",
					mode: "single",
					defaultType: "markdown",
				},
			);

			expect(result).toEqual({
				type: "markdown",
				version: "initial",
				hash: "",
				data: {},
			});
		});
	});

	describe("partial block handling", () => {
		it("fills missing version", () => {
			const block = { type: "markdown", hash: "abc", data: { text: "Hi" } };
			const result = normaliseNightfireBlock(block, typeOptions, definition);
			expect(result.version).toBe("initial");
		});

		it("fills missing hash", () => {
			const block = { type: "markdown", version: "v1", data: { text: "Hi" } };
			const result = normaliseNightfireBlock(block, typeOptions, definition);
			expect(result.hash).toBe("");
		});

		it("fills missing data", () => {
			const block = { type: "markdown", version: "v1", hash: "abc" };
			const result = normaliseNightfireBlock(block, typeOptions, definition);
			expect(result.data).toEqual({});
		});

		it("replaces null data with empty object", () => {
			const block = { type: "markdown", version: "v1", hash: "", data: null };
			const result = normaliseNightfireBlock(block, typeOptions, definition);
			expect(result.data).toEqual({});
		});
	});

	describe("non-object handling", () => {
		it("creates default for string input", () => {
			const result = normaliseNightfireBlock("invalid" as any, typeOptions, definition);
			expect(result.type).toBe("markdown");
			expect(result.data).toEqual({});
		});

		it("creates default for number input", () => {
			const result = normaliseNightfireBlock(123 as any, typeOptions, definition);
			expect(result.type).toBe("markdown");
		});

		it("creates default for array input", () => {
			const result = normaliseNightfireBlock([] as any, typeOptions, definition);
			expect(result.type).toBe("markdown");
		});
	});
});

// ============================================================================
// isEmptyNightfire
// ============================================================================

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

// ============================================================================
// writeNightfireToFormData
// ============================================================================

describe("writeNightfireToFormData", () => {
	it("writes empty string for null value", () => {
		const formData = new FormData();
		writeNightfireToFormData(formData, "body", null);
		expect(formData.get("body")).toBe("");
	});

	it("writes empty string for undefined value", () => {
		const formData = new FormData();
		writeNightfireToFormData(formData, "body", undefined);
		expect(formData.get("body")).toBe("");
	});

	it("writes empty string for empty NightfireValue", () => {
		const formData = new FormData();
		writeNightfireToFormData(formData, "body", { schema: "test@1" });
		expect(formData.get("body")).toBe("");
	});

	it("writes JSON string for non-empty value", () => {
		const formData = new FormData();
		const value = {
			schema: "test@1",
			block: { type: "markdown", version: "initial", hash: "", data: { text: "Hello" } },
		};
		writeNightfireToFormData(formData, "body", value);

		const written = formData.get("body") as string;
		expect(JSON.parse(written)).toEqual(value);
	});

	it("overwrites existing form field", () => {
		const formData = new FormData();
		formData.set("body", "old value");

		const value = {
			schema: "test@1",
			block: { type: "markdown", version: "initial", hash: "", data: {} },
		};
		writeNightfireToFormData(formData, "body", value);

		expect(formData.get("body")).not.toBe("old value");
	});

	it("uses provided field name", () => {
		const formData = new FormData();
		const value = {
			schema: "test@1",
			block: { type: "markdown", version: "initial", hash: "", data: {} },
		};
		writeNightfireToFormData(formData, "customField", value);

		expect(formData.get("customField")).not.toBeNull();
		expect(formData.get("body")).toBeNull();
	});
});
