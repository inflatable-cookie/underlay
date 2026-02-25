// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import type { PaginationController } from "../../src/patterns/pagination-types";
import PaginatedListHarness from "../fixtures/PaginatedListHarness.svelte";

type Item = { id: string; label: string };

function makeController(overrides: Partial<PaginationController<Item>> = {}): PaginationController<Item> {
  return {
    items: [],
    currentPage: 1,
    pageSize: 20,
    hasNextPage: false,
    hasPrevPage: false,
    total: 0,
    loading: false,
    error: null,
    showingFrom: 0,
    showingTo: 0,
    totalPages: 1,
    nextPage: vi.fn(async () => {}),
    prevPage: vi.fn(async () => {}),
    goToPage: vi.fn(() => {}),
    setPageSize: vi.fn(() => {}),
    refresh: vi.fn(async () => {}),
    reset: vi.fn(async () => {}),
    ...overrides
  };
}

describe("components/PaginatedList.svelte", () => {
  it("renders loading branch", () => {
    const loadingController = makeController({ loading: true });
    render(PaginatedListHarness, {
      controller: loadingController,
      loadingMessage: "Loading records..."
    });
    expect(screen.getByText("Loading records...")).toBeTruthy();
  });

  it("renders loading branch custom snippet", () => {
    const loadingController = makeController({ loading: true });
    render(PaginatedListHarness, {
      controller: loadingController,
      withLoadingSnippet: true
    });
    expect(screen.getByTestId("paginated-loading")).toBeTruthy();
  });

  it("renders error branch", () => {
    const errorController = makeController({ error: "Request failed" });
    render(PaginatedListHarness, {
      controller: errorController
    });
    expect(screen.getByText("Request failed")).toBeTruthy();
  });

  it("renders error branch custom snippet", () => {
    const errorController = makeController({ error: "Request failed" });
    render(PaginatedListHarness, {
      controller: errorController,
      withErrorSnippet: true
    });
    expect(screen.getByTestId("paginated-error").textContent).toContain("Custom error: Request failed");
  });

  it("renders empty state default message", () => {
    const controller = makeController({ items: [] });
    render(PaginatedListHarness, {
      controller,
      emptyMessage: "Nothing available."
    });
    expect(screen.getByText("Nothing available.")).toBeTruthy();
  });

  it("renders empty state custom snippet", () => {
    const controller = makeController({ items: [] });
    render(PaginatedListHarness, {
      controller,
      withEmptySnippet: true
    });
    expect(screen.getByTestId("paginated-empty").textContent).toContain("Custom empty");
  });

  it("renders items in grid mode and shows pagination when applicable", () => {
    const controller = makeController({
      items: [
        { id: "1", label: "First" },
        { id: "2", label: "Second" }
      ],
      hasNextPage: true,
      total: 42,
      showingFrom: 1,
      showingTo: 20,
      totalPages: 3
    });

    const view = render(PaginatedListHarness, {
      controller,
      layout: "grid",
      gap: 1.5,
      gridMinWidth: 18,
      className: "custom-paginated-list",
      paginationVariant: "full",
      showLimitSelector: true
    });

    const list = view.container.querySelector(".underlay-paginated-list") as HTMLElement;
    const items = screen.getAllByTestId("paginated-item");
    expect(list.classList.contains("underlay-paginated-list--grid")).toBe(true);
    expect(list.classList.contains("custom-paginated-list")).toBe(true);
    expect(list.getAttribute("style")).toContain("--paginated-list-gap: 1.5rem;");
    expect(list.getAttribute("style")).toContain("--paginated-list-grid-min: 18rem;");
    expect(items.length).toBe(2);
    expect(screen.getByRole("navigation", { name: "Pagination" })).toBeTruthy();
  });
});
