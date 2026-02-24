import { describe, it, expect } from "vitest";
import { resolveActionFailureResult } from "../../src/patterns/forms-action-result";

describe("resolveActionFailureResult", () => {
	it("prefers error string for message", () => {
		const result = resolveActionFailureResult({
			error: "Explicit error",
			message: "Fallback message"
		});
		expect(result).toEqual({
			message: "Explicit error",
			fieldErrors: {}
		});
	});

	it("uses message when error is missing", () => {
		const result = resolveActionFailureResult({
			message: "Message value"
		});
		expect(result).toEqual({
			message: "Message value",
			fieldErrors: {}
		});
	});

	it("defaults message when no known fields are present", () => {
		const result = resolveActionFailureResult({});
		expect(result.message).toBe("Validation failed");
		expect(result.fieldErrors).toEqual({});
	});

	it("collects only string field errors", () => {
		const result = resolveActionFailureResult({
			fieldErrors: {
				email: "Invalid email",
				count: 3,
				active: true,
				password: "Too short"
			}
		});
		expect(result.fieldErrors).toEqual({
			email: "Invalid email",
			password: "Too short"
		});
	});

	it("ignores invalid fieldErrors shapes", () => {
		expect(resolveActionFailureResult({ fieldErrors: ["x"] as unknown as Record<string, unknown> }).fieldErrors).toEqual({});
		expect(resolveActionFailureResult({ fieldErrors: null as unknown as Record<string, unknown> }).fieldErrors).toEqual({});
	});
});
