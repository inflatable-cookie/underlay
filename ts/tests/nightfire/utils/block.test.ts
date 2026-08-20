import { describe, expect, it } from "vitest";
import {
	normaliseNightfireBlock,
	type NightfireBlockDefinition,
	type NightfireTypeOption,
} from "../../../src/nightfire/utils";

describe("normaliseNightfireBlock", () => {
	const typeOptions: NightfireTypeOption[] = [
		{ type: "markdown", label: "Markdown" },
		{ type: "custom", label: "Custom" },
	];

	const definition: NightfireBlockDefinition = {
		schema: "test:schema",
		mode: "single",
		defaultType: "markdown",
	};

	describe("valid block normalization", () => {
		it("preserves valid block with allowed type and drops hash", () => {
			const block = {
				type: "markdown",
				version: "v1",
				hash: "abc123",
				data: { text: "Hello" },
			};
			const result = normaliseNightfireBlock(block, typeOptions, definition);
			expect(result).toEqual({
				type: "markdown",
				version: "v1",
				data: { text: "Hello" },
			});
			expect((result as { hash?: string }).hash).toBeUndefined();
		});

		it("preserves custom block type if in options", () => {
			const block = {
				type: "custom",
				version: "v2",
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
				data: {},
			};
			const result = normaliseNightfireBlock(block, typeOptions, definition);
			expect(result.type).toBe("markdown");
		});

		it("falls back to definition defaultType if no type options", () => {
			const block = { type: "unknown" };
			const emptyOptions: NightfireTypeOption[] = [];
			const result = normaliseNightfireBlock(block, emptyOptions, definition);
			expect(result.type).toBe("markdown");
		});

		it("falls back to markdown when no options and no definition defaultType", () => {
			const block = { type: "unknown" };
			const emptyOptions: NightfireTypeOption[] = [];
			const noDefaultDefinition = {
				schema: "test:schema",
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
				data: {},
			});
		});

		it("creates default block for undefined", () => {
			const result = normaliseNightfireBlock(undefined, typeOptions, definition);
			expect(result).toEqual({
				type: "markdown",
				version: "initial",
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
					schema: "acow:content/description",
					mode: "single",
					defaultType: "markdown",
				},
			);

			expect(result).toEqual({
				type: "markdown",
				version: "initial",
				data: {},
			});
		});
	});

	describe("partial block handling", () => {
		it("fills missing version", () => {
			const block = { type: "markdown", data: { text: "Hi" } };
			const result = normaliseNightfireBlock(block, typeOptions, definition);
			expect(result.version).toBe("initial");
		});

		it("fills missing data", () => {
			const block = { type: "markdown", version: "v1" };
			const result = normaliseNightfireBlock(block, typeOptions, definition);
			expect(result.data).toEqual({});
		});

		it("replaces null data with empty object", () => {
			const block = { type: "markdown", version: "v1", data: null };
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
