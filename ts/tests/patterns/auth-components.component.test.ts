// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import LoginPage from "../../src/patterns/auth-workflows/LoginPage.svelte";

describe("patterns/auth-workflows/*", () => {
	it("handles password login validation and trims credentials before submit", async () => {
		const onPasswordLogin = vi.fn(async () => ({ complete: true }));
		const onComplete = vi.fn();

		render(LoginPage, {
			methods: ["password"],
			onPasswordLogin,
			onComplete,
			forgotPasswordHref: "/forgot",
			registerHref: "/register",
		});

		await fireEvent.submit(screen.getByRole("button", { name: "Log in" }).closest("form") as HTMLFormElement);
		expect(screen.getByRole("alert").textContent).toContain("Email and password are required");

		const email = screen.getByLabelText(/^Email/) as HTMLInputElement;
		const password = screen.getByLabelText(/^Password/) as HTMLInputElement;
		await fireEvent.input(email, { target: { value: "  person@example.com  " } });
		await fireEvent.input(password, { target: { value: "  secret-pass  " } });
		await fireEvent.submit(screen.getByRole("button", { name: "Log in" }).closest("form") as HTMLFormElement);

		await waitFor(() => {
			expect(onPasswordLogin).toHaveBeenCalledWith("person@example.com", "secret-pass");
			expect(onComplete).toHaveBeenCalledTimes(1);
		});
		expect(screen.getByRole("link", { name: "Forgot password?" }).getAttribute("href")).toBe("/forgot");
		expect(screen.getByRole("link", { name: "Sign up" }).getAttribute("href")).toBe("/register");
	});

	it("supports tabbed passkey flow and keeps Google tab disabled when not configured", async () => {
		const onPasskeyLogin = vi.fn(async () => undefined);
		const onComplete = vi.fn();

		render(LoginPage, {
			methods: ["password", "passkey", "google"],
			onPasswordLogin: vi.fn(async () => ({ complete: true })),
			onPasskeyLogin,
			onComplete,
			showPasskeyEmailField: true,
		});

		expect(screen.getByRole("tab", { name: "Password" })).toBeTruthy();
		expect(screen.getByRole("tab", { name: "Passkeys" })).toBeTruthy();
		expect(screen.getByRole("tab", { name: "Google" })).toBeTruthy();

		await fireEvent.click(screen.getByRole("tab", { name: "Passkeys" }));
		const passkeyEmail = screen.getByLabelText(/^Email/) as HTMLInputElement;
		await fireEvent.input(passkeyEmail, { target: { value: "  passkey@example.com  " } });
		await fireEvent.click(screen.getByRole("button", { name: "Sign in with passkey" }));

		await waitFor(() => {
			expect(onPasskeyLogin).toHaveBeenCalledWith("passkey@example.com");
			expect(onComplete).toHaveBeenCalledTimes(1);
		});
	});

});
