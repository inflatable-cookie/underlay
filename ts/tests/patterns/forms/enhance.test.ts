// @vitest-environment jsdom
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { createFormState } from "../../../src/patterns/forms";
import { createMockForm } from "./fixtures";

describe("createFormState.enhance", () => {
	const OriginalFormData = globalThis.FormData;
	const OriginalFetch = globalThis.fetch;
	const OriginalWindow = (globalThis as any).window;

	beforeEach(() => {
		vi.useFakeTimers();
		(globalThis as any).FormData = class {
			constructor(_form?: unknown) {}
		};
	});

	afterEach(() => {
		vi.useRealTimers();
		(globalThis as any).FormData = OriginalFormData;
		(globalThis as any).fetch = OriginalFetch;
		(globalThis as any).window = OriginalWindow;
	});

	it("handles success and failure action results", async () => {
		const onSuccess = vi.fn();
		const onError = vi.fn();
		const form = createFormState({ onSuccess, onError });
		const formEl = createMockForm();

		(globalThis as any).fetch = vi
			.fn()
			.mockResolvedValueOnce({ json: async () => ({ type: "success", data: { id: "1" } }) })
			.mockResolvedValueOnce({
				json: async () => ({
					type: "failure",
					data: { error: "Validation failed", fieldErrors: { email: "invalid" } }
				})
			});

		form.enhance(formEl as unknown as HTMLFormElement);
		await formEl.submit();
		await Promise.resolve();
		expect(onSuccess).toHaveBeenCalledWith({ id: "1" });
		expect(form.isSuccess).toBe(true);

		await formEl.submit();
		expect(onError).toHaveBeenCalledWith("Validation failed", { email: "invalid" });
		expect(form.error).toBe("Validation failed");
		expect(form.fieldErrors).toEqual({ email: "invalid" });
	});

	it("handles redirect, error, and unknown action result types", async () => {
		const form = createFormState();
		const formEl = createMockForm();
		(globalThis as any).window = { location: { href: "https://example.com/start" } };
		(globalThis as any).fetch = vi
			.fn()
			.mockResolvedValueOnce({ json: async () => ({ type: "redirect", location: "/next" }) })
			.mockResolvedValueOnce({ json: async () => ({ type: "error", error: new Error("boom") }) })
			.mockResolvedValueOnce({ json: async () => ({ type: "other" }) });

		form.enhance(formEl as unknown as HTMLFormElement);

		await formEl.submit();
		expect((globalThis as any).window.location.href).toBe("/next");
		expect(form.isSuccess).toBe(true);

		await formEl.submit();
		expect(form.error).toBe("boom");
		expect(form.isSuccess).toBe(false);

		await formEl.submit();
		expect(form.isSuccess).toBe(true);
	});

	it("handles non-JSON responses and network exceptions", async () => {
		const form = createFormState();
		const formEl = createMockForm();
		(globalThis as any).fetch = vi
			.fn()
			.mockResolvedValueOnce({ ok: true, json: async () => { throw new Error("not json"); } })
			.mockResolvedValueOnce({ ok: false, status: 503, json: async () => { throw new Error("not json"); } })
			.mockRejectedValueOnce(new Error("offline"));

		form.enhance(formEl as unknown as HTMLFormElement);

		await formEl.submit();
		expect(form.isSuccess).toBe(true);
		expect(form.error).toBeNull();

		await formEl.submit();
		expect(form.error).toBe("Request failed with status 503");
		expect(form.isSuccess).toBe(false);

		await formEl.submit();
		expect(form.error).toBe("offline");
	});

	it("handles method fallback, redirect without location, and non-Error thrown values", async () => {
		const form = createFormState();
		const formEl = createMockForm("https://example.com/form", "");
		(globalThis as any).fetch = vi
			.fn()
			.mockResolvedValueOnce({ json: async () => ({ type: "redirect" }) })
			.mockResolvedValueOnce({ json: async () => ({ type: "error" }) })
			.mockRejectedValueOnce("string-failure");

		form.enhance(formEl as unknown as HTMLFormElement);

		await formEl.submit();
		expect((globalThis as any).fetch).toHaveBeenNthCalledWith(
			1,
			"https://example.com/form",
			expect.objectContaining({ method: "POST" })
		);
		expect(form.isSuccess).toBe(true);

		await formEl.submit();
		expect(form.error).toBe("An unexpected error occurred");

		await formEl.submit();
		expect(form.error).toBe("An unexpected error occurred");
	});

	it("unsubscribes submit listener on destroy", () => {
		const form = createFormState();
		const formEl = createMockForm();

		const enhanced = form.enhance(formEl as unknown as HTMLFormElement);
		expect(formEl.addEventListener).toHaveBeenCalledWith("submit", expect.any(Function));

		enhanced.destroy?.();
		const submitHandler = (formEl.addEventListener as any).mock.calls[0][1];
		expect(formEl.removeEventListener).toHaveBeenCalledWith("submit", submitHandler);
	});
});
