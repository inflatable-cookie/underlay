import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { get } from "svelte/store";
import { createFormState } from "../../../src/patterns/forms";

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
