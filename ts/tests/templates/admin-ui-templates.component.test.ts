// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";

import type { ToastStore } from "../../src/patterns/toasts";
import AdminUserMenu from "../../src/templates/AdminUserMenu.svelte";
import UserForm from "../../src/templates/UserForm.svelte";
import AdminNavListHarness from "../fixtures/AdminNavListHarness.svelte";
import EntityTrashPageHarness from "../fixtures/EntityTrashPageHarness.svelte";
import EntityActionsMenuHarness from "../fixtures/EntityActionsMenuHarness.svelte";

function createToastStoreSpy(): ToastStore & { push: ReturnType<typeof vi.fn> } {
	return {
		toasts: { subscribe: () => () => {} },
		push: vi.fn(() => "toast-1"),
		dismiss: vi.fn(),
		clear: vi.fn(),
	};
}

describe("templates/AdminNavList", () => {
	it("expands and collapses a section via its toggle with aria-controls linkage", async () => {
		render(AdminNavListHarness);

		expect(screen.queryByRole("link", { name: "Pages" })).toBeNull();
		const toggle = screen.getByRole("button", { name: "Content" });
		expect(toggle.getAttribute("aria-expanded")).toBe("false");
		expect(toggle.getAttribute("aria-controls")).toBeNull();

		await fireEvent.click(toggle);
		const pagesLink = await screen.findByRole("link", { name: "Pages" });
		expect(pagesLink).toBeTruthy();
		expect(toggle.getAttribute("aria-expanded")).toBe("true");
		const controlsId = toggle.getAttribute("aria-controls");
		expect(controlsId).toBeTruthy();
		const childList = document.getElementById(controlsId!);
		expect(childList?.tagName).toBe("UL");
		expect(childList?.contains(pagesLink)).toBe(true);

		await fireEvent.click(toggle);
		expect(screen.queryByRole("link", { name: "Pages" })).toBeNull();
		expect(toggle.getAttribute("aria-expanded")).toBe("false");
		expect(toggle.getAttribute("aria-controls")).toBeNull();
	});

	it("auto-expands the current section and marks active links with aria-current='page'", async () => {
		render(AdminNavListHarness, {
			currentSection: "content",
			currentPath: "/content/pages",
		});

		const pagesLink = await screen.findByRole("link", { name: "Pages" });
		expect(pagesLink.className).toContain("admin-nav__link--active");
		expect(pagesLink.getAttribute("aria-current")).toBe("page");
		const overviewLink = screen.getByRole("link", { name: "Overview" });
		expect(overviewLink.getAttribute("aria-current")).toBeNull();
		// Other sections stay collapsed.
		expect(screen.queryByRole("link", { name: "Jobs" })).toBeNull();
	});

	it("marks active top-level links with aria-current='page'", () => {
		render(AdminNavListHarness, {
			currentPath: "/",
		});

		const overviewLink = screen.getByRole("link", { name: "Overview" });
		expect(overviewLink.getAttribute("aria-current")).toBe("page");
	});

	it("suppresses the active state for excluded child hrefs", async () => {
		render(AdminNavListHarness, {
			currentSection: "content",
			currentPath: "/content/archived/hidden",
		});

		const archivedLink = await screen.findByRole("link", { name: "Archived" });
		expect(archivedLink.className).not.toContain("admin-nav__link--active");
	});

	it("calls onNavigate when a nav link is clicked", async () => {
		const onNavigate = vi.fn();
		render(AdminNavListHarness, { currentSection: "content", onNavigate });

		const pagesLink = await screen.findByRole("link", { name: "Pages" });
		pagesLink.addEventListener("click", (event) => event.preventDefault(), { once: true });
		await fireEvent.click(pagesLink);
		expect(onNavigate).toHaveBeenCalledTimes(1);
	});
});

describe("templates/EntityTrashPage", () => {
	it("renders loading and error states instead of the list", async () => {
		const { unmount } = render(EntityTrashPageHarness, { loading: true });
		expect(screen.getByText("Loading...")).toBeTruthy();
		unmount();

		render(EntityTrashPageHarness, { error: "Trash failed to load" });
		expect(await screen.findByText("Trash failed to load")).toBeTruthy();
		expect(screen.queryByText("Trash is empty")).toBeNull();
	});

	it("renders the empty state when there are no items", async () => {
		render(EntityTrashPageHarness, { items: [] });

		expect(await screen.findByText("Trash is empty")).toBeTruthy();
		expect(screen.getByText("Deleted chapters will appear here.")).toBeTruthy();
	});

	it("renders each item through renderItem and shows the status message", async () => {
		render(EntityTrashPageHarness, {
			items: [
				{ id: "1", title: "G2019 Machines in motion" },
				{ id: "2", title: "G2020 Waves" },
			],
			statusMessage: "Chapter restored",
		});

		expect(await screen.findByTestId("trash-item-1")).toBeTruthy();
		expect(screen.getByTestId("trash-item-2")).toBeTruthy();
		expect(screen.getByText("Chapter restored")).toBeTruthy();
		expect(screen.queryByText("Trash is empty")).toBeNull();
	});
});

describe("templates/EntityActionsMenu", () => {
	it("opens the confirm dialog from the menu and executes on confirm", async () => {
		const execute = vi.fn(async () => {});
		const onDeleteSuccess = vi.fn();
		render(EntityActionsMenuHarness, { execute, onDeleteSuccess });

		await fireEvent.click(screen.getByTestId("action-delete"));

		const dialog = await screen.findByRole("alertdialog");
		expect(dialog.textContent).toContain("Purge chapter?");
		expect(dialog.textContent).toContain("G2019 Machines in motion");

		await fireEvent.click(within(dialog).getByRole("button", { name: "Purge" }));

		await waitFor(() => {
			expect(execute).toHaveBeenCalledTimes(1);
			expect(onDeleteSuccess).toHaveBeenCalledTimes(1);
		});
		expect(screen.queryByRole("alertdialog")).toBeNull();
	});

	it("cancels the dialog without executing", async () => {
		const execute = vi.fn(async () => {});
		render(EntityActionsMenuHarness, { execute });

		await fireEvent.click(screen.getByTestId("action-delete"));
		expect(await screen.findByRole("alertdialog")).toBeTruthy();

		await fireEvent.click(screen.getByRole("button", { name: "Cancel" }));

		await waitFor(() => {
			expect(screen.queryByRole("alertdialog")).toBeNull();
		});
		expect(execute).not.toHaveBeenCalled();
	});

	it("pushes an error toast and keeps the dialog open when execute fails", async () => {
		const toastStore = createToastStoreSpy();
		const execute = vi.fn(async () => {
			throw new Error("Purge failed: still referenced");
		});
		render(EntityActionsMenuHarness, { execute, toastStore });

		await fireEvent.click(screen.getByTestId("action-delete"));
		const dialog = await screen.findByRole("alertdialog");
		await fireEvent.click(within(dialog).getByRole("button", { name: "Purge" }));

		await waitFor(() => {
			expect(toastStore.push).toHaveBeenCalledWith({
				variant: "error",
				message: "Purge failed: still referenced",
			});
		});
		expect(screen.getByRole("alertdialog")).toBeTruthy();
	});

	it("runs the edit action directly from the menu", async () => {
		const onEdit = vi.fn();
		render(EntityActionsMenuHarness, { onEdit });

		await fireEvent.click(screen.getByTestId("action-edit"));
		expect(onEdit).toHaveBeenCalledTimes(1);
	});
});

describe("templates/AdminUserMenu", () => {
	it("marks decorative menu icons with aria-hidden and guards confirmLogout against re-entrancy", async () => {
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

		const trigger = screen.getByRole("button", { name: /Ada Lovelace/i });
		await fireEvent.click(trigger);

		const menu = await screen.findByRole("navigation", { name: "User menu" });
		const icons = menu.querySelectorAll(".admin-nav__user-menu-icon");
		expect(icons.length).toBe(2);
		for (const icon of icons) {
			expect(icon.getAttribute("aria-hidden")).toBe("true");
		}

		await fireEvent.click(within(menu).getByRole("button", { name: "Log out" }));
		const dialog = await screen.findByRole("alertdialog");
		const confirmButton = within(dialog).getByRole("button", { name: "Log out" });

		await fireEvent.click(confirmButton);
		await fireEvent.click(confirmButton);
		expect(onLogout).toHaveBeenCalledTimes(1);

		resolveLogout();
		await waitFor(() => {
			expect(screen.queryByRole("alertdialog")).toBeNull();
		});
	});
});

describe("templates/UserForm", () => {
	it("scopes field ids per instance using $props.id() instead of hardcoded globals", () => {
		const first = render(UserForm, { mode: "create" });
		const second = render(UserForm, { mode: "create" });

		const firstEmail = within(first.container).getByLabelText(/^Email/) as HTMLInputElement;
		const firstDisplayName = within(first.container).getByLabelText(
			/^Display Name/,
		) as HTMLInputElement;
		const secondEmail = within(second.container).getByLabelText(/^Email/) as HTMLInputElement;

		expect(firstEmail.id).not.toBe("user-email");
		expect(firstDisplayName.id).not.toBe("user-display-name");
		expect(firstEmail.id).not.toBe(secondEmail.id);
	});

	it("syncs field values when async edit data arrives without clobbering user edits", async () => {
		const { rerender } = render(UserForm, {
			mode: "edit",
			values: {},
		});

		const emailInput = screen.getByLabelText(/^Email/) as HTMLInputElement;
		const displayNameInput = screen.getByLabelText(/^Display Name/) as HTMLInputElement;
		expect(emailInput.value).toBe("");
		expect(displayNameInput.value).toBe("");

		await rerender({
			mode: "edit",
			values: {
				email: "alice@example.com",
				displayName: "Alice",
				role: "admin",
				status: "active",
			},
		});

		expect(emailInput.value).toBe("alice@example.com");
		expect(displayNameInput.value).toBe("Alice");

		await fireEvent.input(displayNameInput, { target: { value: "Alice Edited" } });
		expect(displayNameInput.value).toBe("Alice Edited");

		// Unrelated re-render with a fresh values object carrying the same server values.
		await rerender({
			mode: "edit",
			values: {
				email: "alice@example.com",
				displayName: "Alice",
				role: "admin",
				status: "active",
			},
		});
		expect(displayNameInput.value).toBe("Alice Edited");
	});
});
