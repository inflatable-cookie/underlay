import * as z from "zod";
import { describe, expect, it, vi } from "vitest";

type ValidatedFormModule = typeof import("../../src/patterns/validated-form.svelte.ts");

const registerRequestSchema = z.object({
	email: z.string().trim().email(),
	password: z.string().min(12, "String must contain at least 12 character(s)"),
	displayName: z.string().trim().min(1, "String must contain at least 1 character(s)"),
});

async function loadValidatedFormModule(): Promise<ValidatedFormModule> {
	vi.resetModules();
	(globalThis as any).$state = <T>(initial: T) => initial;
	return await import("../../src/patterns/validated-form.svelte.ts");
}

describe("patterns/useValidatedForm", () => {
	it("collects schema errors and blocks submit until the values are valid", async () => {
		const mod = await loadValidatedFormModule();
		const onSubmit = vi.fn(async () => undefined);
		const form = mod.useValidatedForm({
			schema: registerRequestSchema,
			initialValues: {
				email: "not-an-email",
				password: "short",
				displayName: "",
			},
			onSubmit,
		});

		expect(form.validate()).toBe(false);
		expect(form.errors.email).toContain("Invalid email");
		expect(form.errors.password).toContain("at least 12 character(s)");
		expect(await form.submit()).toBe(false);
		expect(onSubmit).not.toHaveBeenCalled();
	});

	it("submits parsed values and supports validate-on-change updates", async () => {
		const mod = await loadValidatedFormModule();
		const onSubmit = vi.fn(async () => undefined);
		const form = mod.useValidatedForm({
			schema: registerRequestSchema,
			initialValues: {
				email: "person@example.com",
				password: "password-1234",
				displayName: " Person ",
			},
			onSubmit,
			validateOnChange: true,
		});

		form.setField("email", "bad-email");
		expect(form.errors.email).toContain("Invalid email");

		form.setField("email", "person@example.com");
		expect(form.errors.email).toBeUndefined();

		expect(await form.submit()).toBe(true);
		expect(onSubmit).toHaveBeenCalledWith({
			email: "person@example.com",
			password: "password-1234",
			displayName: "Person",
		});
	});

	it("ignores a second submit while one is already in flight", async () => {
		const mod = await loadValidatedFormModule();
		let release: () => void = () => {};
		const onSubmit = vi.fn(
			() =>
				new Promise<void>((resolve) => {
					release = resolve;
				}),
		);
		const form = mod.useValidatedForm({
			schema: registerRequestSchema,
			initialValues: {
				email: "person@example.com",
				password: "password-1234",
				displayName: "Person",
			},
			onSubmit,
		});

		const first = form.submit();
		expect(form.isSubmitting).toBe(true);
		expect(await form.submit()).toBe(false);
		expect(onSubmit).toHaveBeenCalledTimes(1);

		release();
		expect(await first).toBe(true);
		expect(form.isSubmitting).toBe(false);
	});

	it("clones plain and nested initial values so edits never leak back into the caller", async () => {
		const mod = await loadValidatedFormModule();
		const initialValues = {
			email: "person@example.com",
			password: "password-1234",
			displayName: "Person",
			tags: ["a"],
		};
		const schema = registerRequestSchema.extend({ tags: z.array(z.string()) });
		const form = mod.useValidatedForm({
			schema,
			initialValues,
			onSubmit: async () => undefined,
		});

		(form.values.tags as string[]).push("b");
		expect(initialValues.tags).toEqual(["a"]);

		form.reset();
		expect(form.values.tags).toEqual(["a"]);
	});

	it("ignores stale errors and results when reset() is called during a pending submit()", async () => {
		const mod = await loadValidatedFormModule();
		let rejectFirst!: (reason: Error) => void;
		let resolveSecond!: () => void;
		let callCount = 0;

		const onSubmit = vi.fn(() => {
			callCount += 1;
			if (callCount === 1) {
				return new Promise<void>((_, reject) => {
					rejectFirst = reject;
				});
			}
			return new Promise<void>((resolve) => {
				resolveSecond = resolve;
			});
		});
		const onError = vi.fn();

		const form = mod.useValidatedForm({
			schema: registerRequestSchema,
			initialValues: {
				email: "person@example.com",
				password: "password-1234",
				displayName: "Person",
			},
			onSubmit,
			onError,
		});

		const firstSubmit = form.submit();
		expect(form.isSubmitting).toBe(true);

		// User resets the form while the first submission is still awaiting onSubmit.
		form.reset();
		expect(form.isSubmitting).toBe(false);
		expect(form.submitError).toBeNull();

		// Start a second submission before the first stale submission settles.
		form.setField("displayName", "Updated Person");
		const secondSubmit = form.submit();
		expect(form.isSubmitting).toBe(true);

		// Stale first submission rejects: must not set submitError, call onError, or clear isSubmitting.
		rejectFirst(new Error("stale network error"));
		expect(await firstSubmit).toBe(false);
		expect(form.submitError).toBeNull();
		expect(onError).not.toHaveBeenCalled();
		expect(form.isSubmitting).toBe(true);
		expect(form.values.displayName).toBe("Updated Person");

		// Second submission succeeds cleanly.
		resolveSecond();
		expect(await secondSubmit).toBe(true);
		expect(form.isSubmitting).toBe(false);
		expect(form.submitError).toBeNull();
	});

	it("returns false when a pending submit() resolves after reset()", async () => {
		const mod = await loadValidatedFormModule();
		let resolvePending!: () => void;
		const form = mod.useValidatedForm({
			schema: registerRequestSchema,
			initialValues: {
				email: "person@example.com",
				password: "password-1234",
				displayName: "Person",
			},
			onSubmit: () =>
				new Promise<void>((resolve) => {
					resolvePending = resolve;
				}),
		});

		const pendingSubmit = form.submit();
		form.reset();
		resolvePending();

		expect(await pendingSubmit).toBe(false);
		expect(form.isSubmitting).toBe(false);
	});
});
