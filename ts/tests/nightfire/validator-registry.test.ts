import { describe, expect, it } from "vitest";
import {
	prepareNightfireForSave,
	registerBlockValidator,
	validateNightfireValue,
	writePreparedNightfireToFormData,
} from "../../src/nightfire/validator-registry";

describe("nightfire/validator-registry", () => {
	it("validates block and blocks using schema-specific and wildcard validators", () => {
		registerBlockValidator(null, "markdown", (block: any) => ({ ...block, wildcard: true }));
		registerBlockValidator("schema-1", "markdown", (block: any) => ({ ...block, scoped: true }));

		expect(
			validateNightfireValue({ schema: "schema-1", block: { type: "markdown", data: {} } } as any)
		).toEqual({
			schema: "schema-1",
			block: { id: undefined, type: "markdown", version: "initial", hash: "", data: {} },
			blocks: undefined,
		});

		expect(
			validateNightfireValue({ schema: "schema-x", block: { type: "markdown", data: {} } } as any)
		).toEqual({
			schema: "schema-x",
			block: { id: undefined, type: "markdown", version: "initial", hash: "", data: {} },
			blocks: undefined,
		});

		expect(
			validateNightfireValue({
				schema: "schema-x",
				blocks: [{ type: "markdown", data: {} }, { type: "unknown", data: {} }],
			} as any)
		).toEqual({
			schema: "schema-x",
			blocks: [
				{ id: undefined, type: "markdown", version: "initial", hash: "", data: {} },
				{ id: undefined, type: "unknown", version: "initial", hash: "", data: {} },
			],
			block: undefined,
		});
	});

	it("preserves invalid values and prepare helper delegates to validation", () => {
		expect(validateNightfireValue(null as any)).toBeNull();
		const value = { schema: "s", block: { type: "x" } } as any;
		const prepared = prepareNightfireForSave(value);
		expect(prepared?.schema).toBe("s");
		expect(prepared?.block).toMatchObject({
			type: "x",
			version: "initial",
			hash: "",
			data: {},
		});
		expect(prepared?.block?.id).toMatch(/^nf_/);

		const noBlockValue = { schema: "s" } as any;
		expect(validateNightfireValue(noBlockValue)).toBe(noBlockValue);

			expect(
				validateNightfireValue({
					schema: "schema-x",
					blocks: [null, { data: {} }],
				} as any)
		).toEqual({
			schema: "schema-x",
			});

			expect(
				validateNightfireValue({
					block: { type: "markdown", data: {} },
				} as any)
			).toEqual({
				block: { id: undefined, type: "markdown", version: "initial", hash: "", data: {} },
				blocks: undefined,
			});
	});

	it("writes prepared Nightfire JSON to FormData with stable block ids", () => {
		const formData = new FormData();

		writePreparedNightfireToFormData(formData, "content", {
			schema: "schema-x",
			block: {
				type: "markdown",
				data: {
					imageId: "media-1",
				},
			},
		});

		const raw = formData.get("content");
		expect(typeof raw).toBe("string");

		const parsed = JSON.parse(raw as string);
		expect(parsed).toMatchObject({
			schema: "schema-x",
			block: {
				type: "markdown",
				data: {
					imageId: "media-1",
				},
			},
		});
		expect(parsed.block.id).toMatch(/^nf_/);
	});

	it("writes empty string when prepared Nightfire content is empty", () => {
		const formData = new FormData();
		writePreparedNightfireToFormData(formData, "content", { schema: "schema-x" });
		expect(formData.get("content")).toBe("");
	});
});
