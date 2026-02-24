import { describe, expect, it } from "vitest";
import {
	getBlockEditor,
	getBlockTypeLabel,
	getBlockTypeOptionsForSchema,
	getSchemaDefinition,
	isBlockContentEmpty,
	registerBlockEditor,
	registerBlockEmptyChecker,
	registerSchema,
} from "../../src/nightfire/editor-registry";

describe("nightfire/editor-registry", () => {
	it("registers and resolves schema definitions", () => {
		registerSchema({ schema: "s1", mode: "single", defaultType: "markdown" });
		expect(getSchemaDefinition("s1")).toEqual({ schema: "s1", mode: "single", defaultType: "markdown" });
		expect(getSchemaDefinition("missing")).toBeNull();
	});

	it("registers block editors and type options without duplicates", () => {
		const EditorA = Symbol("EditorA");
		const EditorB = Symbol("EditorB");

		registerBlockEditor("s2", "markdown", "Markdown", EditorA as any);
		registerBlockEditor("s2", "callout", "Callout", EditorB as any);
		registerBlockEditor("s2", "markdown", "Markdown v2", EditorA as any); // duplicate type option should not duplicate

		expect(getBlockEditor("s2", "markdown")).toBe(EditorA as any);
		expect(getBlockEditor("s2", "callout")).toBe(EditorB as any);
		expect(getBlockEditor("s2", "missing")).toBeNull();

		expect(getBlockTypeOptionsForSchema("s2")).toEqual([
			{ type: "markdown", label: "Markdown" },
			{ type: "callout", label: "Callout" },
		]);
		expect(getBlockTypeLabel("s2", "markdown")).toBe("Markdown");
		expect(getBlockTypeLabel("s2", "missing")).toBeNull();
		expect(getBlockTypeLabel("missing", "markdown")).toBeNull();
	});

	it("checks block emptiness via registered checker with safe defaults", () => {
		expect(isBlockContentEmpty(null)).toBe(true);
		expect(isBlockContentEmpty({})).toBe(true);
		expect(isBlockContentEmpty({ type: "unknown", data: {} })).toBe(false);

		registerBlockEmptyChecker("markdown", (block) => {
			const text = block?.data?.text;
			return !text || typeof text !== "string" || text.trim().length === 0;
		});

		expect(isBlockContentEmpty({ type: "markdown", data: { text: "" } })).toBe(true);
		expect(isBlockContentEmpty({ type: "markdown", data: { text: "  " } })).toBe(true);
		expect(isBlockContentEmpty({ type: "markdown", data: { text: "hello" } })).toBe(false);
	});
});
