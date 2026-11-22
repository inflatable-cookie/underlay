// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";

vi.mock("@inflatable-cookie/poodle-svelte", async (importOriginal) => {
	const actual = await importOriginal<typeof import("@inflatable-cookie/poodle-svelte")>();
	const { default: TestAlertDialogStub } = await import("./TestAlertDialogStub.svelte");
	return {
		...actual,
		AlertDialog: TestAlertDialogStub,
	};
});

import AdminUserMenu from "../../src/templates/AdminUserMenu.svelte";

describe("templates/AdminUserMenu confirmLogout re-entrancy", () => {
	it("guards confirmLogout directly when onConfirm is invoked twice while logout is pending", async () => {
		let resolveLogout: () => void = () => {};
		const onLogout = vi.fn(
			() =>
				new Promise<void>((resolve) => {
					resolveLogout = resolve;
				}),
		);

		render(AdminUserMenu, {
			displayName: "Ada Lovelace",
			roles: ["admin"],
			appName: "Underlay",
			onLogout,
		});

		await fireEvent.click(screen.getByRole("button", { name: /Ada Lovelace/i }));
		const menu = await screen.findByRole("navigation", { name: "User menu" });
		await fireEvent.click(within(menu).getByRole("button", { name: "Log out" }));

		const confirmButton = await screen.findByTestId("stub-confirm");
		// Stub calls onConfirm unconditionally without Poodle's working guard,
		// so only AdminUserMenu's own `if (loggingOut) return;` prevents a second call.
		await fireEvent.click(confirmButton);
		await fireEvent.click(confirmButton);
		expect(onLogout).toHaveBeenCalledTimes(1);

		resolveLogout();
		await waitFor(() => {
			expect(screen.queryByRole("alertdialog")).toBeNull();
		});
	});

	it("surfaces a rejected onLogout instead of silently closing the dialog", async () => {
		const onLogout = vi.fn(async () => {
			throw new Error("Session service unavailable");
		});

		render(AdminUserMenu, {
			displayName: "Ada Lovelace",
			roles: ["admin"],
			appName: "Underlay",
			onLogout,
		});

		await fireEvent.click(screen.getByRole("button", { name: /Ada Lovelace/i }));
		const menu = await screen.findByRole("navigation", { name: "User menu" });
		await fireEvent.click(within(menu).getByRole("button", { name: "Log out" }));

		await fireEvent.click(await screen.findByTestId("stub-confirm"));

		expect(await screen.findByText("Session service unavailable")).toBeTruthy();
		// The dialog stays open so the user can retry rather than believing they logged out.
		expect(screen.getByRole("alertdialog")).toBeTruthy();
		expect(screen.getByTestId("stub-confirm").hasAttribute("disabled")).toBe(false);
	});

	it("falls back to a generic message when onLogout rejects without a message", async () => {
		const onLogout = vi.fn(async () => {
			throw new Error("");
		});

		render(AdminUserMenu, {
			displayName: "Ada Lovelace",
			roles: ["admin"],
			appName: "Underlay",
			onLogout,
		});

		await fireEvent.click(screen.getByRole("button", { name: /Ada Lovelace/i }));
		const menu = await screen.findByRole("navigation", { name: "User menu" });
		await fireEvent.click(within(menu).getByRole("button", { name: "Log out" }));
		await fireEvent.click(await screen.findByTestId("stub-confirm"));

		expect(await screen.findByText("Log out failed. Please try again.")).toBeTruthy();
	});

	it("sanitizes an unsafe accountHref and defaults the avatar gradient to the shared token", async () => {
		render(AdminUserMenu, {
			displayName: "Ada Lovelace",
			roles: ["admin"],
			appName: "Underlay",
			accountHref: "javascript:alert(1)",
			onLogout: vi.fn(),
		});

		await fireEvent.click(screen.getByRole("button", { name: /Ada Lovelace/i }));
		const menu = await screen.findByRole("navigation", { name: "User menu" });
		const accountLink = within(menu).getByRole("link", { name: "Account" });
		expect(accountLink.getAttribute("href")).toBe("/account");

		const avatar = document.querySelector<HTMLElement>(".admin-nav__user-avatar");
		expect(avatar?.getAttribute("style") ?? "").toContain(
			"var(--underlay-admin-nav-badge-overview",
		);
	});

	it("keeps a safe accountHref unchanged", async () => {
		render(AdminUserMenu, {
			displayName: "Ada Lovelace",
			roles: ["admin"],
			accountHref: "/settings/profile",
			onLogout: vi.fn(),
		});

		await fireEvent.click(screen.getByRole("button", { name: /Ada Lovelace/i }));
		const menu = await screen.findByRole("navigation", { name: "User menu" });
		expect(within(menu).getByRole("link", { name: "Account" }).getAttribute("href")).toBe(
			"/settings/profile",
		);
	});
});
