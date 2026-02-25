// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import AuthLayoutHarness from "../fixtures/AuthLayoutHarness.svelte";
import LoginPage from "../../src/components/auth/LoginPage.svelte";
import GoogleSignInButton from "../../src/components/auth/GoogleSignInButton.svelte";

describe("components/auth/*", () => {
	it("renders AuthLayout as a standalone wrapper with logo/footer snippets", () => {
		const view = render(AuthLayoutHarness, {
			title: "Sign in",
			maxWidth: "32rem",
			className: "custom-auth-layout",
		});

		expect(screen.getByRole("heading", { name: "Sign in" })).toBeTruthy();
		expect(screen.getByTestId("logo").textContent).toContain("Underlay");
		expect(screen.getByTestId("content").textContent).toContain("Auth content");
		expect(screen.getByTestId("footer-link").getAttribute("href")).toBe("/terms");

		const layoutRoot = view.container.querySelector(".underlay-auth-layout") as HTMLElement;
		expect(layoutRoot.classList.contains("custom-auth-layout")).toBe(true);

		const card = view.container.querySelector(".underlay-auth-layout__card") as HTMLElement;
		expect(card.getAttribute("style")).toContain("max-width: 32rem;");
	});

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

		const email = screen.getByLabelText("Email") as HTMLInputElement;
		const password = screen.getByLabelText("Password") as HTMLInputElement;
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
		const googleTab = screen.getByRole("tab", { name: "Google" });
		expect(googleTab.getAttribute("aria-disabled") === "true" || googleTab.hasAttribute("data-disabled")).toBe(true);

		await fireEvent.click(screen.getByRole("tab", { name: "Passkeys" }));
		const passkeyEmail = screen.getByLabelText("Email (optional)") as HTMLInputElement;
		await fireEvent.input(passkeyEmail, { target: { value: "  passkey@example.com  " } });
		await fireEvent.click(screen.getByRole("button", { name: "Sign in with passkey" }));

		await waitFor(() => {
			expect(onPasskeyLogin).toHaveBeenCalledWith("passkey@example.com");
			expect(onComplete).toHaveBeenCalledTimes(1);
		});
	});

	it("runs GoogleSignInButton click-handler and missing-url error paths", async () => {
		const onNavigate = vi.fn();
		const onError = vi.fn();
		const onclick = vi.fn(async () => undefined);

		render(GoogleSignInButton, {
			onclick,
			onNavigate,
			onError,
			label: "Continue with Google",
		});

		await fireEvent.click(screen.getByRole("button", { name: "Continue with Google" }));
		await waitFor(() => {
			expect(onclick).toHaveBeenCalledTimes(1);
			expect(onNavigate).not.toHaveBeenCalled();
		});

		const missingUrl = render(GoogleSignInButton, {
			onError,
			label: "No url",
		});
		await fireEvent.click(screen.getByRole("button", { name: "No url" }));
		await waitFor(() => {
			expect(onError).toHaveBeenCalledWith({ message: "missing authorization url" });
		});
		missingUrl.unmount();
	});
});
