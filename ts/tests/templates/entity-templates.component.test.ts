// @vitest-environment jsdom
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";

import { configureAuth } from "../../src/patterns/auth";
import EntityDetailPageHarness from "../fixtures/EntityDetailPageHarness.svelte";
import EntityListCardHarness from "../fixtures/EntityListCardHarness.svelte";
import EntityListControlledFilterHarness from "../fixtures/EntityListControlledFilterHarness.svelte";
import EntityListSummaryFallbackHarness from "../fixtures/EntityListSummaryFallbackHarness.svelte";

describe("templates", () => {
  beforeEach(() => {
    configureAuth({
      getToken: () => "token-1",
      onRefresh: async () => "token-1",
      getAuthLoading: () => false,
      getCurrentUser: () => ({ id: "user-1" })
    });
  });

  it("switches detail tabs when a non-default tab is selected", async () => {
    render(EntityDetailPageHarness);

    expect(await screen.findByText("Status")).toBeTruthy();
    expect(screen.queryByTestId("related-tab-content")).toBeNull();

    await fireEvent.click(screen.getByRole("tab", { name: "Related" }));

    expect(await screen.findByTestId("related-tab-content")).toBeTruthy();
    expect(screen.queryByText("Status")).toBeNull();
  });

  it("refetches with the latest controlled filter values", async () => {
    const onQuery = vi.fn();
    render(EntityListControlledFilterHarness, { onQuery });

    await waitFor(() => {
      expect(onQuery).toHaveBeenCalledWith({ page: 1, limit: 30, filters: [], sort: [] });
    });

    await fireEvent.click(screen.getByRole("button", { name: "Show filters" }));

    const input = screen.getByRole("searchbox", { name: "Search" });
    await fireEvent.input(input, { target: { value: "mars" } });

    await waitFor(() => {
      expect(onQuery).toHaveBeenLastCalledWith({
        page: 1,
        limit: 30,
        sort: [],
        filters: [
          {
            field: "search",
            operator: "like",
            value: "%mars%"
          }
        ]
      });
    });
  });

  it("falls back to visible count when a paged response omits total", async () => {
    render(EntityListSummaryFallbackHarness);

    await waitFor(() => {
      expect(screen.getByText("Showing 1-5 of 5")).toBeTruthy();
    });
  });

  it("renders the shared entity card surface with selection and context actions", async () => {
    render(EntityListCardHarness);

    expect(screen.getByText("Active")).toBeTruthy();
    expect(screen.getByText("Development")).toBeTruthy();
    expect(screen.getByText("Last updated today")).toBeTruthy();
    expect(screen.getByText("3")).toBeTruthy();
    expect(screen.getByText("12")).toBeTruthy();

    const card = screen.getByRole("button", { name: "Project Apollo" });
    expect(screen.getByTestId("selected-state").textContent).toBe("idle");

    await fireEvent.click(card);
    expect(screen.getByTestId("selected-state").textContent).toBe("selected");

    await fireEvent.contextMenu(card);
    await fireEvent.click(await screen.findByRole("menuitem", { name: "Archive" }));

    expect(screen.getByTestId("context-action").textContent).toBe("archive");
  });

  it("collapses shared entity cards in reorder mode", async () => {
    render(EntityListCardHarness);

    expect(screen.getByText("Project Mercury")).toBeTruthy();
    expect(screen.queryByText("This should be hidden in reorder mode")).toBeNull();
    expect(screen.queryByText("Archived")).toBeNull();
    expect(screen.queryByText("9")).toBeNull();
  });

  it("supports explicit selection-mode display config on shared entity cards", async () => {
    render(EntityListCardHarness);

    expect(screen.getByText("Project Gemini")).toBeTruthy();
    expect(screen.getByText("Research")).toBeTruthy();
    expect(screen.queryByText("Should hide subtitle in selection mode")).toBeNull();
    expect(screen.queryByText("This should be hidden in selection mode")).toBeNull();
  });
});
