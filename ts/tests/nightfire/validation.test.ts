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
			block: { type: "markdown", data: {} },
		} as any;

		expect(validateNightfireValue(value)).toEqual({
			schema: "schema-v",
			block: { id: undefined, type: "markdown", version: "initial", hash: "", data: {} },
			blocks: undefined,
		});
		const prepared = prepareNightfireForSave(value);
		expect(prepared?.schema).toBe("schema-v");
		expect(prepared?.block).toMatchObject({
			type: "markdown",
			version: "initial",
			hash: "",
			data: {},
		});
		expect(prepared?.block?.id).toMatch(/^nf_/);
	});
});
