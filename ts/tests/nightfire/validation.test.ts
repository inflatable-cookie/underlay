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
			block: { type: "markdown", data: {}, validated: true },
		});
		expect(prepareNightfireForSave(value)).toEqual(validateNightfireValue(value));
	});
});
