// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";

import type { ToastStore } from "../../src/patterns/toasts";
import AdminNavListHarness from "../fixtures/AdminNavListHarness.svelte";
import EntityTrashPageHarness from "../fixtures/EntityTrashPageHarness.svelte";
import EntityActionsMenuHarness from "../fixtures/EntityActionsMenuHarness.svelte";

function createToastStoreSpy(): ToastStore & { push: ReturnType<typeof vi.fn> } {
  return {
    toasts: { subscribe: () => () => {} },
    push: vi.fn(() => "toast-1"),
    dismiss: vi.fn(),
    clear: vi.fn()
  };
}

describe("templates/AdminNavList", () => {
  it("expands and collapses a section via its toggle", async () => {
    render(AdminNavListHarness);

    expect(screen.queryByRole("link", { name: "Pages" })).toBeNull();
    const toggle = screen.getByRole("button", { name: "Content" });
    expect(toggle.getAttribute("aria-expanded")).toBe("false");

    await fireEvent.click(toggle);
    expect(await screen.findByRole("link", { name: "Pages" })).toBeTruthy();
    expect(toggle.getAttribute("aria-expanded")).toBe("true");

    await fireEvent.click(toggle);
    expect(screen.queryByRole("link", { name: "Pages" })).toBeNull();
    expect(toggle.getAttribute("aria-expanded")).toBe("false");
  });

  it("auto-expands the current section and marks the active child", async () => {
    render(AdminNavListHarness, {
      currentSection: "content",
      currentPath: "/content/pages"
    });

    const pagesLink = await screen.findByRole("link", { name: "Pages" });
    expect(pagesLink.className).toContain("admin-nav__link--active");
    // Other sections stay collapsed.
    expect(screen.queryByRole("link", { name: "Jobs" })).toBeNull();
  });

  it("suppresses the active state for excluded child hrefs", async () => {
    render(AdminNavListHarness, {
      currentSection: "content",
      currentPath: "/content/archived/hidden"
    });

    const archivedLink = await screen.findByRole("link", { name: "Archived" });
    expect(archivedLink.className).not.toContain("admin-nav__link--active");
  });

  it("calls onNavigate when a nav link is clicked", async () => {
    const onNavigate = vi.fn();
    render(AdminNavListHarness, { currentSection: "content", onNavigate });

    await fireEvent.click(await screen.findByRole("link", { name: "Pages" }));
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
        { id: "2", title: "G2020 Waves" }
      ],
      statusMessage: "Chapter restored"
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
        message: "Purge failed: still referenced"
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
