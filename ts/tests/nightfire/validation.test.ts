import { describe, expect, it } from "vitest";
import {
	registerBlockValidator,
	validateNightfireValue,
	prepareNightfireForSave,
} from "../../src/nightfire/validation";

describe("nightfire/validation", () => {
	it("re-exports validator helpers", () => {
		registerBlockValidator("schema-v", "markdown", (block: any) => ({
			...block,
			validated: true,
		}));

		const value = {
			schema: "schema-v",
			blocks: [{ type: "markdown", data: {} }],
		} as any;

		expect(validateNightfireValue(value)).toEqual({
			schema: "schema-v",
			blocks: [{ id: undefined, type: "markdown", version: "initial", data: {} }],
		});
		const prepared = prepareNightfireForSave(value);
		expect(prepared?.schema).toBe("schema-v");
		expect(prepared?.blocks[0]).toMatchObject({
			type: "markdown",
			version: "initial",
			data: {},
		});
		expect(prepared?.blocks[0]?.id).toMatch(/^nf_/);
	});
});
