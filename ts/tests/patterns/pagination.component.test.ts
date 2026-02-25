// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import type { PaginationController } from "../../src/patterns/pagination-types";
import PaginationHarness from "../fixtures/PaginationHarness.svelte";

type Item = { id: string };

function makeController(overrides: Partial<PaginationController<Item>> = {}): PaginationController<Item> {
  return {
    items: [],
    currentPage: 2,
    pageSize: 20,
    hasNextPage: true,
    hasPrevPage: true,
    total: 100,
    loading: false,
    error: null,
    showingFrom: 21,
    showingTo: 40,
    totalPages: 5,
    nextPage: vi.fn(async () => {}),
    prevPage: vi.fn(async () => {}),
    goToPage: vi.fn(() => {}),
    setPageSize: vi.fn(() => {}),
    refresh: vi.fn(async () => {}),
    reset: vi.fn(async () => {}),
    ...overrides
  };
}

describe("components/Pagination.svelte", () => {
  it("supports props mode callbacks and limit selector", async () => {
    const onPage = vi.fn();
    const onLimit = vi.fn();

    const view = render(PaginationHarness, {
      page: 2,
      limit: 10,
      total: 45,
      variant: "simple",
      showLimitSelector: true,
      limitOptions: [10, 25],
      onPage,
      onLimit
    });

    expect(screen.getByText("Showing 11 to 20 of 45")).toBeTruthy();
    const pageSummary = (view.container.querySelector(".underlay-pagination-page") as HTMLElement).textContent ?? "";
    expect(pageSummary).toContain("11–20");
    expect(pageSummary).toContain("of 45");

    await fireEvent.click(screen.getByRole("button", { name: "Previous page" }));
    await fireEvent.click(screen.getByRole("button", { name: "Next page" }));
    expect(onPage).toHaveBeenNthCalledWith(1, 1);
    expect(onPage).toHaveBeenNthCalledWith(2, 3);

    const select = view.container.querySelector("#pagination-limit") as HTMLSelectElement;
    await fireEvent.change(select, { target: { value: "25" } });
    expect(onLimit).toHaveBeenCalledWith(25);
  });

  it("supports controller mode navigation controls in full variant", async () => {
    const controller = makeController();
    render(PaginationHarness, {
      controller,
      variant: "full",
      className: "custom-pagination",
      compact: true
    });

    const nav = screen.getByRole("navigation", { name: "Pagination" });
    expect(nav.classList.contains("custom-pagination")).toBe(true);
    expect(nav.classList.contains("underlay-compact")).toBe(true);
    expect(screen.getByText("Page 2 of 5")).toBeTruthy();

    await fireEvent.click(screen.getByRole("button", { name: "First page" }));
    await fireEvent.click(screen.getByRole("button", { name: "Previous page" }));
    await fireEvent.click(screen.getByRole("button", { name: "Next page" }));
    await fireEvent.click(screen.getByRole("button", { name: "Last page" }));

    expect(controller.goToPage).toHaveBeenCalledWith(1);
    expect(controller.prevPage).toHaveBeenCalledTimes(1);
    expect(controller.nextPage).toHaveBeenCalledTimes(1);
    expect(controller.goToPage).toHaveBeenCalledWith(5);
  });

  it("does not render nav when there is one page and no limit selector", () => {
    const view = render(PaginationHarness, {
      page: 1,
      limit: 20,
      total: 20,
      showLimitSelector: false
    });

    expect(view.container.querySelector(".underlay-pagination")).toBeNull();
  });
});
