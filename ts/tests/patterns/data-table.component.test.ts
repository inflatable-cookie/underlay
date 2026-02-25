// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import DataTableHarness from "../fixtures/DataTableHarness.svelte";

describe("components/DataTable.svelte", () => {
  it("renders rows, supports sort/select/row-click callbacks, and custom cell/extended snippets", async () => {
    const onSort = vi.fn();
    const onSelect = vi.fn();
    const onRowClick = vi.fn();

    const view = render(DataTableHarness, {
      selectable: true,
      onSort,
      onSelect,
      onRowClick,
      withCellSnippet: true,
      withExtendedRow: true
    });

    expect(screen.getByText("Name")).toBeTruthy();
    expect(screen.getByText("Status")).toBeTruthy();
    expect(screen.getByText("Ada")).toBeTruthy();
    expect(screen.getByText("Linus")).toBeTruthy();
    expect(view.container.querySelector('[data-testid="extended-1"]')?.textContent).toContain("Extended: Ada");

    await fireEvent.click(screen.getByRole("button", { name: /Name/ }));
    expect(onSort).toHaveBeenCalledWith({ key: "name", direction: "asc" });

    const checkboxes = view.container.querySelectorAll('input[type="checkbox"]');
    await fireEvent.click(checkboxes[0] as Element);
    expect(onSelect).toHaveBeenCalledTimes(1);
    expect(screen.getByTestId("selected-count").textContent).toBe("2");

    await fireEvent.click(screen.getByText("Ada"));
    expect(onRowClick).toHaveBeenCalledWith({ id: "1", name: "Ada", status: "active" });
  });

  it("handles toolbar controls, export callback, and pagination callbacks", async () => {
    const onPage = vi.fn();
    const onExport = vi.fn();
    const createObjectURL = vi.fn(() => "blob:mock");
    const revokeObjectURL = vi.fn();
    const anchorClick = vi.spyOn(HTMLAnchorElement.prototype, "click").mockImplementation(() => {});
    const originalCreate = URL.createObjectURL;
    const originalRevoke = URL.revokeObjectURL;
    URL.createObjectURL = createObjectURL;
    URL.revokeObjectURL = revokeObjectURL;

    try {
      const view = render(DataTableHarness, {
        showColumnToggle: true,
        showExport: true,
        pagination: { page: 1, limit: 10, total: 25 },
        onPage,
        onExport
      });

      await fireEvent.click(screen.getByRole("button", { name: /Columns/ }));
      expect(view.container.querySelector(".underlay-column-menu")).toBeTruthy();

      const statusColumnToggle = screen.getByLabelText("Status");
      await fireEvent.click(statusColumnToggle);
      await waitFor(() => {
        expect(screen.queryByRole("columnheader", { name: "Status" })).toBeNull();
      });

      await fireEvent.click(screen.getByRole("button", { name: /Export CSV/ }));
      expect(onExport).toHaveBeenCalledTimes(1);
      expect(createObjectURL).toHaveBeenCalledTimes(1);
      expect(revokeObjectURL).toHaveBeenCalledTimes(1);

      await fireEvent.click(screen.getByRole("button", { name: "Next page" }));
      expect(onPage).toHaveBeenCalledWith(2);

      await fireEvent.click(screen.getByRole("button", { name: "Last page" }));
      expect(onPage).toHaveBeenCalledWith(3);

      await fireEvent.click(screen.getByRole("button", { name: "First page" }));
      expect(onPage).toHaveBeenCalledWith(1);

      await fireEvent.click(screen.getByRole("button", { name: "Previous page" }));
      expect(onPage).not.toHaveBeenCalledWith(0);
    } finally {
      URL.createObjectURL = originalCreate;
      URL.revokeObjectURL = originalRevoke;
      anchorClick.mockRestore();
    }
  });
});
