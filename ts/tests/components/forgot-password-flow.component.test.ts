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

		const email = screen.getByLabelText(/^Email/) as HTMLInputElement;
		await fireEvent.input(email, { target: { value: "person@example.com" } });
		const form = email.closest("form") as HTMLFormElement;

		await fireEvent.submit(form);
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
});
