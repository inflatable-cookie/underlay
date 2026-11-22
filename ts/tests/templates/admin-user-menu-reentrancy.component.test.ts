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
});
