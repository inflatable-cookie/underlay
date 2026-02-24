import { describe, expect, it } from "vitest";
import {
	isAuthError,
	isErrorEnvelope,
	toUserMessage,
	UnderlayHttpError,
} from "../../src/client/errors";

describe("client/errors", () => {
	it("UnderlayHttpError exposes envelope accessors", () => {
		const error = new UnderlayHttpError(401, "Unauthorized", {
			error: {
				code: "auth.session_expired",
				message: "Session expired",
				fieldErrors: { email: "required" },
			},
		});
		expect(error.name).toBe("UnderlayHttpError");
		expect(error.isAuthError()).toBe(true);
		expect(error.code).toBe("auth.session_expired");
		expect(error.fieldErrors).toEqual({ email: "required" });
	});

	it("isAuthError supports UnderlayHttpError and status-like objects", () => {
		expect(isAuthError(new UnderlayHttpError(401, "x"))).toBe(true);
		expect(isAuthError(new UnderlayHttpError(403, "x"))).toBe(false);
		expect(isAuthError({ status: 401 })).toBe(true);
		expect(isAuthError({ status: 500 })).toBe(false);
		expect(isAuthError("not-an-error")).toBe(false);
	});

	it("isErrorEnvelope validates expected shape", () => {
		expect(
			isErrorEnvelope({
				error: { code: "x", message: "y" },
			})
		).toBe(true);
		expect(isErrorEnvelope({ error: { code: 123, message: "y" } })).toBe(false);
		expect(isErrorEnvelope(null)).toBe(false);
		expect(isErrorEnvelope({})).toBe(false);
	});

	it("toUserMessage handles codes/statuses and fallback messaging", () => {
		expect(toUserMessage({ code: "timeout" })).toBe(
			"The server took too long to respond."
		);
		expect(toUserMessage({ status: 401, code: "auth.session_expired" })).toBe(
			"Your session has expired. Please sign in again."
		);
		expect(toUserMessage({ status: 401, message: "custom 401" })).toBe(
			"custom 401"
		);
		expect(toUserMessage({ status: 403 })).toBe(
			"You do not have permission to do that."
		);
		expect(toUserMessage({ status: 404 })).toBe("That resource was not found.");
		expect(toUserMessage({ status: 400 })).toBe("That request wasn't accepted.");
		expect(toUserMessage({ status: 500 })).toBe(
			"The server is unavailable right now."
		);
		expect(toUserMessage({ status: 0 })).toBe(
			"The server is unavailable right now."
		);
		expect(toUserMessage({ status: 418, message: "teapot" })).toBe("teapot");
		expect(toUserMessage(new Error("boom"))).toBe("boom");
		expect(toUserMessage({})).toBe(
			"Something went wrong while talking to the server."
		);
	});
});
