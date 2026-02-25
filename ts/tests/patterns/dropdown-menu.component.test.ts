// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import DropdownMenuHarness from "../fixtures/DropdownMenuHarness.svelte";

describe("components/DropdownMenu.svelte", () => {
  it("renders item list mode and runs action handlers", async () => {
    const onEdit = vi.fn();
    const onDelete = vi.fn();

    render(DropdownMenuHarness, {
      triggerLabel: "Menu",
      triggerAriaLabel: "Open row actions",
      items: [
        { label: "Edit", onSelect: onEdit },
        { separator: true },
        { label: "Delete", destructive: true, onSelect: onDelete }
      ],
      contentClassName: "custom-dropdown-content"
    });

    await fireEvent.click(screen.getByRole("button", { name: "Open row actions" }));
    await waitFor(() => {
      expect(screen.getByText("Edit")).toBeTruthy();
    });

    const content = document.querySelector(".underlay-dropdown-menu-content") as HTMLElement;
    expect(content.classList.contains("custom-dropdown-content")).toBe(true);
    expect(document.querySelector(".underlay-dropdown-menu-separator")).toBeTruthy();
    expect(document.querySelector(".underlay-dropdown-menu-item--destructive")?.textContent).toContain("Delete");

    await fireEvent.click(screen.getByText("Edit"));
    await fireEvent.click(screen.getByRole("button", { name: "Open row actions" }));
    await fireEvent.click(screen.getByText("Delete"));
    expect(onEdit).toHaveBeenCalledTimes(1);
    expect(onDelete).toHaveBeenCalledTimes(1);
  });

  it("supports children mode and custom trigger snippet", async () => {
    render(DropdownMenuHarness, {
      items: undefined,
      withTriggerSnippet: true,
      withChildren: true
    });

    expect(screen.getByTestId("dropdown-custom-trigger")).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Open menu" }));
    expect(screen.getByTestId("dropdown-custom-child").textContent).toContain("Child action");
  });
});
