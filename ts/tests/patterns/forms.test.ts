import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { get } from "svelte/store";
import {
	createFormState,
	hasFieldErrors,
	extractErrorMessage,
	mergeFieldErrors
} from "../../src/patterns/forms";

function createMockForm(action = "https://example.com/form", method = "post") {
	let submitHandler: ((event: SubmitEvent) => Promise<void>) | undefined;
	return {
		action,
		method,
		addEventListener: vi.fn((event: string, cb: (event: SubmitEvent) => Promise<void>) => {
			if (event === "submit") submitHandler = cb;
		}),
		removeEventListener: vi.fn(),
		submit: async () => {
			const event = { preventDefault: vi.fn() } as unknown as SubmitEvent;
			await submitHandler?.(event);
			return event;
		}
	};
}

describe("createFormState", () => {
	beforeEach(() => {
		vi.useFakeTimers();
	});

	afterEach(() => {
		vi.useRealTimers();
		vi.restoreAllMocks();
	});

	it("initializes with provided initial errors", () => {
		const form = createFormState({
			initialError: "Initial error",
			initialFieldErrors: { email: "Invalid email" }
		});

		expect(form.error).toBe("Initial error");
		expect(form.fieldErrors).toEqual({ email: "Invalid email" });
		expect(form.isSubmitting).toBe(false);
		expect(form.isSuccess).toBe(false);
	});

	it("startSubmit sets submitting state and clears errors", () => {
		const onSubmit = vi.fn();
		const form = createFormState({
			onSubmit,
			initialError: "Old error",
			initialFieldErrors: { email: "Old field error" }
		});

		form.startSubmit();

		expect(onSubmit).toHaveBeenCalledOnce();
		expect(form.isSubmitting).toBe(true);
		expect(form.error).toBeNull();
		expect(form.fieldErrors).toEqual({});
		expect(form.isSuccess).toBe(false);
	});

	it("setSuccess completes submission and calls onSuccess", async () => {
		const onSuccess = vi.fn(async () => {});
		const payload = { id: "123" };
		const form = createFormState<typeof payload>({ onSuccess });

		form.startSubmit();
		form.setSuccess(payload);
		await vi.runAllTimersAsync();

		expect(onSuccess).toHaveBeenCalledWith(payload);
		expect(form.isSubmitting).toBe(false);
		expect(form.error).toBeNull();
		expect(form.fieldErrors).toEqual({});
		expect(form.isSuccess).toBe(true);
	});

	it("calls onSuccess with undefined payload when no data is provided", async () => {
		const onSuccess = vi.fn(async () => {});
		const form = createFormState<undefined>({ onSuccess });

		form.setSuccess();
		await vi.runAllTimersAsync();

		expect(onSuccess).toHaveBeenCalledWith(undefined);
	});

	it("setSuccess triggers async reset when resetOnSuccess is enabled", async () => {
		const form = createFormState({
			resetOnSuccess: true,
			initialError: "initial",
			initialFieldErrors: { name: "required" }
		});

		form.startSubmit();
		form.setSuccess();
		expect(form.isSuccess).toBe(true);

		await vi.runAllTimersAsync();
		expect(form.isSuccess).toBe(false);
		expect(form.error).toBe("initial");
		expect(form.fieldErrors).toEqual({ name: "required" });
	});

	it("setError and setFieldErrors update state and callbacks", () => {
		const onError = vi.fn();
		const form = createFormState({ onError });

		form.setError("Failed", { email: "bad" });
		expect(onError).toHaveBeenCalledWith("Failed", { email: "bad" });
		expect(form.isSubmitting).toBe(false);
		expect(form.error).toBe("Failed");
		expect(form.fieldErrors).toEqual({ email: "bad" });
		expect(form.isSuccess).toBe(false);

		form.setFieldErrors({ password: "too short" });
		expect(form.error).toBe("Failed");
		expect(form.fieldErrors).toEqual({ password: "too short" });
	});

	it("clearFieldError removes one key and reset restores initial state", () => {
		const form = createFormState({
			initialError: "Initial error",
			initialFieldErrors: { email: "required", password: "required" }
		});

		form.clearFieldError("email");
		expect(form.fieldErrors).toEqual({ password: "required" });

		form.setError("New error", { other: "x" });
		expect(form.error).toBe("New error");

		form.reset();
		const state = get(form.state);
		expect(state).toEqual({
			isSubmitting: false,
			error: "Initial error",
			fieldErrors: { email: "required", password: "required" },
			isSuccess: false
		});
	});
});

describe("createFormState.enhance", () => {
	const OriginalFormData = globalThis.FormData;
	const OriginalFetch = globalThis.fetch;
	const OriginalWindow = (globalThis as any).window;

	beforeEach(() => {
		(globalThis as any).FormData = class {
			constructor(_form?: unknown) {}
		};
	});

	afterEach(() => {
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
