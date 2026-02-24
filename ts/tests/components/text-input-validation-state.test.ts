import { describe, expect, it } from "vitest";
import { hasInputValue, isValidationStatusValid } from "../../src/components/text-input/validation-state";

describe("components/text-input/validation-state", () => {
	it("detects meaningful input values", () => {
		expect(hasInputValue("hello")).toBe(true);
		expect(hasInputValue("   ")).toBe(false);
		expect(hasInputValue(42)).toBe(true);
		expect(hasInputValue(Number.NaN)).toBe(false);
	});

	it("treats idle/valid states as valid", () => {
		expect(isValidationStatusValid("idle")).toBe(true);
		expect(isValidationStatusValid("valid")).toBe(true);
		expect(isValidationStatusValid("validating")).toBe(false);
		expect(isValidationStatusValid("invalid")).toBe(false);
	});
});
