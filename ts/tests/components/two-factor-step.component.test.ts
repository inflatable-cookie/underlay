import { fireEvent, render, screen } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";
import TwoFactorStep from "../../src/patterns/auth-workflows/TwoFactorStep.svelte";

describe("TwoFactorStep", () => {
	it("shows request errors before an email code is sent", async () => {
		const onRequestEmailCode = vi.fn(async () => false);
		const { rerender } = render(TwoFactorStep, {
			hasTotpSetup: true,
			onRequestEmailCode,
		});

		await fireEvent.click(screen.getByRole("tab", { name: "Email" }));
		await fireEvent.click(screen.getByRole("button", { name: "Send code" }));
		expect(onRequestEmailCode).toHaveBeenCalledTimes(1);

		await rerender({ error: "Could not send a code" });
		expect(screen.getByText("Could not send a code")).toBeDefined();
		expect(screen.getByRole("button", { name: "Send code" })).toBeDefined();
	});

	it("handles a rejected email request and resets the method when TOTP setup changes", async () => {
		const { rerender } = render(TwoFactorStep, {
			hasTotpSetup: true,
			onRequestEmailCode: async () => {
				throw new Error("Email is unavailable");
			},
		});

		await fireEvent.click(screen.getByRole("tab", { name: "Email" }));
		await fireEvent.click(screen.getByRole("button", { name: "Send code" }));
		expect(await screen.findByText("Email is unavailable")).toBeDefined();

		await rerender({ hasTotpSetup: false });
		expect(screen.queryByRole("tablist")).toBeNull();
		expect(screen.getByText("We've sent a verification code to your email.")).toBeDefined();

		await rerender({ hasTotpSetup: true });
		expect(screen.getByRole("tab", { name: "Authenticator" }).getAttribute("aria-selected")).toBe(
			"true",
		);
		await fireEvent.click(screen.getByRole("tab", { name: "Email" }));
		expect(screen.getByRole("button", { name: "Send code" })).toBeDefined();
		expect(screen.queryByText("Email is unavailable")).toBeNull();
	});
});
