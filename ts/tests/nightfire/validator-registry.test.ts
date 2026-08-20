import { describe, expect, it } from "vitest";
import {
	prepareNightfireForSave,
	registerBlockValidator,
	validateNightfireValue,
	writePreparedNightfireToFormData,
} from "../../src/nightfire/validator-registry";
import { registerBlockVersions } from "../../src/nightfire/block-versions";

describe("nightfire/validator-registry", () => {
	it("validates blocks using schema-specific and wildcard validators", () => {
		registerBlockValidator(null, "markdown", (block: any) => ({ ...block, wildcard: true }));
		registerBlockValidator("schema-1", "markdown", (block: any) => ({ ...block, scoped: true }));

		expect(
			validateNightfireValue({
				schema: "schema-1",
				blocks: [{ type: "markdown", data: {} }],
			} as any)
		).toEqual({
			schema: "schema-1",
			blocks: [{ id: undefined, type: "markdown", version: "initial", data: {} }],
		});

		expect(
			validateNightfireValue({
				schema: "schema-x",
				blocks: [{ type: "markdown", data: {} }],
			} as any)
		).toEqual({
			schema: "schema-x",
			blocks: [{ id: undefined, type: "markdown", version: "initial", data: {} }],
		});

		expect(
			validateNightfireValue({
				schema: "schema-x",
				blocks: [{ type: "markdown", data: {} }, { type: "unknown", data: {} }],
			} as any)
		).toEqual({
			schema: "schema-x",
			blocks: [
				{ id: undefined, type: "markdown", version: "initial", data: {} },
				{ id: undefined, type: "unknown", version: "initial", data: {} },
			],
		});
	});

	it("preserves invalid values and prepare helper delegates to validation", () => {
		expect(validateNightfireValue(null as any)).toBeNull();
		const value = { schema: "s", blocks: [{ type: "x" }] } as any;
		const prepared = prepareNightfireForSave(value);
		expect(prepared?.schema).toBe("s");
		expect(prepared?.blocks[0]).toMatchObject({
			type: "x",
			version: "initial",
			data: {},
		});
		expect(prepared?.blocks[0]?.id).toMatch(/^nf_/);

		const noBlockValue = { schema: "s", blocks: [] } as any;
		expect(validateNightfireValue(noBlockValue)).toEqual({ schema: "s", blocks: [] });

		expect(
			validateNightfireValue({
				schema: "schema-x",
				blocks: [null, { data: {} }],
			} as any)
		).toEqual({
			schema: "schema-x",
			blocks: [],
		});

		expect(
			validateNightfireValue({
				blocks: [{ type: "markdown", data: {} }],
			} as any)
		).toEqual({
			schema: "",
			blocks: [{ id: undefined, type: "markdown", version: "initial", data: {} }],
		});
	});

	it("writes prepared Nightfire JSON to FormData with stable block ids", () => {
		const formData = new FormData();

		writePreparedNightfireToFormData(formData, "content", {
			schema: "schema-x",
			blocks: [
				{
					type: "markdown",
					data: {
						image_id: "media-1",
					},
				},
			],
		});

		const raw = formData.get("content");
		expect(typeof raw).toBe("string");

		const parsed = JSON.parse(raw as string);
		expect(parsed).toMatchObject({
			schema: "schema-x",
			blocks: [
				{
					type: "markdown",
					data: {
						image_id: "media-1",
					},
				},
			],
		});
		expect(parsed.block).toBeUndefined();
		expect(parsed.blocks[0].id).toMatch(/^nf_/);
		expect(parsed.blocks[0].hash).toBeUndefined();
	});

	it("writes empty string when prepared Nightfire content is empty", () => {
		const formData = new FormData();
		writePreparedNightfireToFormData(formData, "content", { schema: "schema-x", blocks: [] });
		expect(formData.get("content")).toBe("");
	});

	it("rejects leftover v1 block envelopes on save", () => {
		expect(
			prepareNightfireForSave({
				schema: "schema-x",
				block: { type: "markdown", data: {} },
			} as any)
		).toBeNull();
	});

	it("coerces a supported older version and rejects unknown versions", () => {
		registerBlockVersions("callout", { current: "2", supported: ["1", "2"] });

		expect(
			validateNightfireValue({
				schema: "schema-x",
				blocks: [{ type: "callout", version: "1", data: {} }],
			} as any)
		).toEqual({
			schema: "schema-x",
			blocks: [{ id: undefined, type: "callout", version: "2", data: {} }],
		});

		expect(
			validateNightfireValue({
				schema: "schema-x",
				blocks: [{ type: "callout", version: "9", data: {} }],
			} as any)
		).toEqual({
			schema: "schema-x",
			blocks: [],
		});
	});
});
