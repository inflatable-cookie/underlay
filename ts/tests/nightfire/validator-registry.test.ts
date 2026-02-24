import { describe, expect, it } from "vitest";
import {
	prepareNightfireForSave,
	registerBlockValidator,
	validateNightfireValue,
} from "../../src/nightfire/validator-registry";

describe("nightfire/validator-registry", () => {
	it("validates block and blocks using schema-specific and wildcard validators", () => {
		registerBlockValidator(null, "markdown", (block: any) => ({ ...block, wildcard: true }));
		registerBlockValidator("schema-1", "markdown", (block: any) => ({ ...block, scoped: true }));

		expect(
			validateNightfireValue({ schema: "schema-1", block: { type: "markdown", data: {} } } as any)
		).toEqual({ schema: "schema-1", block: { type: "markdown", data: {}, scoped: true } });

		expect(
			validateNightfireValue({ schema: "schema-x", block: { type: "markdown", data: {} } } as any)
		).toEqual({ schema: "schema-x", block: { type: "markdown", data: {}, wildcard: true } });

		expect(
			validateNightfireValue({
				schema: "schema-x",
				blocks: [{ type: "markdown", data: {} }, { type: "unknown", data: {} }],
			} as any)
		).toEqual({
			schema: "schema-x",
			blocks: [
				{ type: "markdown", data: {}, wildcard: true },
				{ type: "unknown", data: {} },
			],
		});
	});

	it("preserves invalid values and prepare helper delegates to validation", () => {
		expect(validateNightfireValue(null as any)).toBeNull();
		const value = { schema: "s", block: { type: "x" } } as any;
		expect(prepareNightfireForSave(value)).toEqual(validateNightfireValue(value));
	});
});
