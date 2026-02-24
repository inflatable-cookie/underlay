import { describe, expect, it, vi } from "vitest";
import {
	registerFormValidationField,
	updateFormValidationField,
} from "../../src/components/text-input/form-validation";

describe("components/text-input/form-validation", () => {
	it("registers field with derived hasValue and validation flags", () => {
		const registerField = vi.fn();
		const ctx = { registerField, unregisterField: vi.fn(), updateField: vi.fn() } as any;

		registerFormValidationField(ctx, "name", true, " Clay ", "valid");
		expect(registerField).toHaveBeenCalledWith("name", true, true, "valid", true);

		registerFormValidationField(ctx, "age", false, "   ", "invalid");
		expect(registerField).toHaveBeenCalledWith("age", false, false, "invalid", false);
	});

	it("updates field with derived state", () => {
		const updateField = vi.fn();
		const ctx = { registerField: vi.fn(), unregisterField: vi.fn(), updateField } as any;

		updateFormValidationField(ctx, "name", "", "validating");
		expect(updateField).toHaveBeenCalledWith("name", false, "validating", false);

		updateFormValidationField(ctx, "score", "99", "valid");
		expect(updateField).toHaveBeenCalledWith("score", true, "valid", true);
	});
});
