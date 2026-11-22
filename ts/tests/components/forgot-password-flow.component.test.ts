// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import ForgotPasswordFlow from "../../src/patterns/auth-workflows/ForgotPasswordFlow.svelte";

function renderFlow(onRequestCode: () => Promise<void>) {
	return render(ForgotPasswordFlow, {
		onRequestCode,
		onVerifyCode: vi.fn(async () => ({ resetToken: "token" })),
		onResetPassword: vi.fn(async () => undefined),
		fetchRequirements: vi.fn(async () => ({}) as any),
	});
}

function statusError(status: number, message = "Request failed"): Error {
	return Object.assign(new Error(message), { status });
}

async function submitEmail(email: string) {
	const input = screen.getByLabelText(/^Email/) as HTMLInputElement;
	await fireEvent.input(input, { target: { value: email } });
	const form = input.closest("form") as HTMLFormElement;
	await fireEvent.submit(form);
}

describe("patterns/auth-workflows/ForgotPasswordFlow", () => {
	it("ignores a second submit while the reset code request is in flight", async () => {
		let release: () => void = () => {};
		const onRequestCode = vi.fn(
			() =>
				new Promise<void>((resolve) => {
					release = resolve;
				}),
		);
		renderFlow(onRequestCode);

		await submitEmail("person@example.com");
		const form = document.querySelector("form") as HTMLFormElement;
		await fireEvent.submit(form);
		expect(onRequestCode).toHaveBeenCalledTimes(1);
		expect(onRequestCode).toHaveBeenCalledWith("person@example.com");

		release();
		await waitFor(() => {
			expect(screen.getByRole("button", { name: "Change email" })).toBeTruthy();
		});
	});

	it("scopes the email field id to the instance rather than a fixed global id", async () => {
		renderFlow(vi.fn(async () => undefined));
		const email = screen.getByLabelText(/^Email/) as HTMLInputElement;
		expect(email.id).not.toBe("");
		expect(email.id).not.toBe("forgot-password-email");
	});

	it("sanitizes unsafe loginHref values to /login", () => {
		render(ForgotPasswordFlow, {
			onRequestCode: vi.fn(async () => undefined),
			onVerifyCode: vi.fn(async () => ({ resetToken: "token" })),
			onResetPassword: vi.fn(async () => undefined),
			fetchRequirements: vi.fn(async () => ({}) as any),
			loginHref: "javascript:alert(1)",
		});

		const link = screen.getByRole("link", { name: "Back to login" });
		expect(link.getAttribute("href")).toBe("/login");
	});

	it("advances to the verify step on an expected 404 so unknown accounts stay indistinguishable", async () => {
		const onRequestCode = vi.fn(async () => {
			throw statusError(404, "Not found");
		});
		renderFlow(onRequestCode);

		await submitEmail("missing@example.com");

		expect(await screen.findByRole("button", { name: "Change email" })).toBeTruthy();
		expect(screen.queryByText(/couldn't send a reset code/i)).toBeNull();
	});

	it.each([0, 429, 500, 503])(
		"surfaces a transport failure and stays on the email step for status %i",
		async (status) => {
			const onRequestCode = vi.fn(async () => {
				throw statusError(status, "upstream exploded");
			});
			renderFlow(onRequestCode);

			await submitEmail("person@example.com");

			expect(await screen.findByText(/couldn't send a reset code/i)).toBeTruthy();
			// The leak-prone message text is never echoed to the user.
			expect(screen.queryByText(/upstream exploded/)).toBeNull();
			expect(screen.getByRole("button", { name: "Send reset code" })).toBeTruthy();
			expect(screen.queryByRole("button", { name: "Change email" })).toBeNull();
		},
	);

	it("surfaces an error for a thrown error without a status instead of advancing", async () => {
		const onRequestCode = vi.fn(async () => {
			throw new Error("socket hang up");
		});
		renderFlow(onRequestCode);

		await submitEmail("person@example.com");

		expect(await screen.findByText(/couldn't send a reset code/i)).toBeTruthy();
		expect(screen.getByRole("button", { name: "Send reset code" })).toBeTruthy();
	});

	it("does not mutate state when unmounted while the reset code request is pending", async () => {
		let release: () => void = () => {};
		const onRequestCode = vi.fn(
			() =>
				new Promise<void>((resolve) => {
					release = resolve;
				}),
		);
		const { unmount } = renderFlow(onRequestCode);

		await submitEmail("person@example.com");
		expect(onRequestCode).toHaveBeenCalledTimes(1);

		unmount();
		release();
		await new Promise((resolve) => setTimeout(resolve, 10));

		expect(document.body.textContent?.trim()).toBe("");
	});

	it("does not mutate state when unmounted while the reset code request rejects", async () => {
		let reject: (error: unknown) => void = () => {};
		const onRequestCode = vi.fn(
			() =>
				new Promise<void>((_resolve, rejectPromise) => {
					reject = rejectPromise;
				}),
		);
		const { unmount } = renderFlow(onRequestCode);

		await submitEmail("person@example.com");
		unmount();
		reject(statusError(500));
		await new Promise((resolve) => setTimeout(resolve, 10));

		expect(document.body.textContent?.trim()).toBe("");
	});

	it("does not advance or error out when unmounted while code verification is pending", async () => {
		let release: (value: { resetToken: string }) => void = () => {};
		const { unmount, container } = render(ForgotPasswordFlow, {
			onRequestCode: vi.fn(async () => undefined),
			onVerifyCode: vi.fn(
				() =>
					new Promise<{ resetToken: string }>((resolve) => {
						release = resolve;
					}),
			),
			onResetPassword: vi.fn(async () => undefined),
			fetchRequirements: vi.fn(async () => ({}) as any),
		});

		await submitEmail("person@example.com");
		await waitFor(() => {
			expect(screen.getByRole("button", { name: "Change email" })).toBeTruthy();
		});

		const codeInputs = Array.from(container.querySelectorAll<HTMLInputElement>("input"));
		for (const input of codeInputs) {
			await fireEvent.input(input, { target: { value: "1" } });
		}
		const form = container.querySelector("form") as HTMLFormElement | null;
		if (form) await fireEvent.submit(form);

		unmount();
		release({ resetToken: "token" });
		await new Promise((resolve) => setTimeout(resolve, 10));

		expect(document.body.textContent?.trim()).toBe("");
	});
});
