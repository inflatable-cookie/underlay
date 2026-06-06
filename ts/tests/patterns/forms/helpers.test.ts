import { describe, expect, it } from "vitest";
import {
	extractErrorMessage,
	hasFieldErrors,
	mergeFieldErrors
} from "../../../src/patterns/forms";

describe("forms helpers", () => {
	it("detects field errors shape", () => {
		expect(hasFieldErrors({ fieldErrors: { email: "bad" } })).toBe(true);
		expect(hasFieldErrors({})).toBe(false);
		expect(hasFieldErrors(null)).toBe(false);
		expect(hasFieldErrors("x")).toBe(false);
	});

	it("extracts error messages with fallback", () => {
		expect(extractErrorMessage("raw")).toBe("raw");
		expect(extractErrorMessage(new Error("boom"))).toBe("boom");
		expect(extractErrorMessage({ message: "msg" })).toBe("msg");
		expect(extractErrorMessage({ error: "err" })).toBe("err");
		expect(extractErrorMessage({})).toBe("An error occurred");
		expect(extractErrorMessage(undefined, "fallback")).toBe("fallback");
	});

	it("merges field errors from multiple sources", () => {
		expect(
			mergeFieldErrors(
				{ email: "required", shared: "a" },
				undefined,
				{ password: "short", shared: "b" },
				null
			)
		).toEqual({
			email: "required",
			password: "short",
			shared: "b"
		});
	});
});
