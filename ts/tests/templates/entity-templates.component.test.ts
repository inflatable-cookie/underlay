// @vitest-environment jsdom
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";

import { configureAuth } from "../../src/patterns/auth";
import { clearNavigationContext, pushNavigationContext } from "../../src/patterns/navigation";
import { MetadataDialogTrigger } from "../../src/templates";
import FormShellHarness from "../fixtures/FormShellHarness.svelte";
import EntityDetailPageHarness from "../fixtures/EntityDetailPageHarness.svelte";
import EntityInlineListModuleHarness from "../fixtures/EntityInlineListModuleHarness.svelte";
import EntityListCardHarness from "../fixtures/EntityListCardHarness.svelte";
import EntityListControlledFilterHarness from "../fixtures/EntityListControlledFilterHarness.svelte";
import EntityListQueryVariantHarness from "../fixtures/EntityListQueryVariantHarness.svelte";
import EntityListSummaryFallbackHarness from "../fixtures/EntityListSummaryFallbackHarness.svelte";
import EntityListLogHarness from "../fixtures/EntityListLogHarness.svelte";

describe("templates", () => {
  beforeEach(() => {
    clearNavigationContext();
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

  it("lazily loads a data-driven detail tab on activation", async () => {
    render(EntityDetailPageHarness);

    expect(screen.queryByTestId("lazy-tab-content")).toBeNull();

    await fireEvent.click(await screen.findByRole("tab", { name: "Lazy" }));

    expect(await screen.findByTestId("lazy-tab-content")).toBeTruthy();
    expect(screen.getByText("Loaded: lazy-data")).toBeTruthy();
  });

  it("does not render a trailing separator for the last detail tab", async () => {
    const { container } = render(EntityDetailPageHarness);

    expect(await screen.findByRole("tab", { name: "Overview" })).toBeTruthy();
    // Three tabs → two separators (between tabs), never one after the last.
    expect(
      container.querySelectorAll(".poodle-tabs__list:not(.poodle-tabs__list--measure) .poodle-tabs__separator")
    ).toHaveLength(2);
  });

  it("renders a clickable metadata trigger in detail meta and opens the dialog", async () => {
    render(EntityDetailPageHarness);

    const trigger = await screen.findByRole("button", { name: "Metadata" });
    await fireEvent.click(trigger);

    expect(await screen.findByRole("dialog")).toBeTruthy();
    expect(screen.getByText("Project metadata")).toBeTruthy();
    expect(screen.getByText(/"provider": "manual"/)).toBeTruthy();
  });

  it("renders a disabled metadata trigger when the metadata value is empty", () => {
    render(MetadataDialogTrigger, {
      value: {},
      title: "Empty metadata"
    });

    const trigger = screen.getByRole("button", { name: "Metadata: none" }) as HTMLButtonElement;
    expect(trigger.disabled).toBe(true);
  });

  it("uses contextual back info when navigation context exists", async () => {
    pushNavigationContext({
      label: "Media Library",
      href: "/media?view=trash",
      type: "list"
    });

    render(EntityDetailPageHarness);

    const backLink = await screen.findByRole("link", { name: /back to media library/i });
    expect(backLink.getAttribute("href")).toBe("/media?view=trash");
    expect(backLink.querySelector(".poodle-page-header__context-dot")).toBeTruthy();
  });

  it("keeps the context dot when callers pass precomputed contextual back info", async () => {
    render(EntityDetailPageHarness, {
      backHref: "/pathways/acca-2027?tab=modules",
      backLabel: "Back to ACCA 2027",
      backIsContextual: true,
    });

    const backLink = await screen.findByRole("link", { name: /back to acca 2027/i });
    expect(backLink.getAttribute("href")).toBe("/pathways/acca-2027?tab=modules");
    expect(backLink.querySelector(".poodle-page-header__context-dot")).toBeTruthy();
  });

  it("uses contextual back info in shared form shells", async () => {
    pushNavigationContext({
      label: "Users",
      href: "/users?page=2",
      type: "list"
    });

    render(FormShellHarness);

    const backLink = await screen.findByRole("link", { name: /back to users/i });
    expect(backLink.getAttribute("href")).toBe("/users?page=2");
    expect(backLink.querySelector(".poodle-page-header__context-dot")).toBeTruthy();
  });

  it("does not render a context dot when using fallback back info", async () => {
    render(EntityDetailPageHarness);

    const backLink = await screen.findByRole("link", { name: /back to projects/i });
    expect(backLink.getAttribute("href")).toBe("/projects");
    expect(backLink.querySelector(".poodle-page-header__context-dot")).toBeNull();
  });

  it("refetches with the latest controlled filter values", async () => {
    const onQuery = vi.fn();
    render(EntityListControlledFilterHarness, { onQuery });

    await waitFor(() => {
      expect(onQuery).toHaveBeenCalledWith({ page: 1, limit: 30, filters: [], sort: [] });
    });

    const showFiltersButton = screen.queryByRole("button", { name: "Show filters" });
    if (showFiltersButton) {
      await fireEvent.click(showFiltersButton);
    }

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

  it("debounces free-text search into a single refetch", async () => {
    const onQuery = vi.fn();
    render(EntityListControlledFilterHarness, { onQuery });

    await waitFor(() => {
      expect(onQuery).toHaveBeenCalledWith({ page: 1, limit: 30, filters: [], sort: [] });
    });

    const showFiltersButton = screen.queryByRole("button", { name: "Show filters" });
    if (showFiltersButton) {
      await fireEvent.click(showFiltersButton);
    }

    const input = screen.getByRole("searchbox", { name: "Search" });
    const initialCalls = onQuery.mock.calls.length;

    // Rapid keystrokes: only the final value should trigger a refetch.
    await fireEvent.input(input, { target: { value: "m" } });
    await fireEvent.input(input, { target: { value: "ma" } });
    await fireEvent.input(input, { target: { value: "mar" } });
    await fireEvent.input(input, { target: { value: "mars" } });

    await waitFor(() => {
      expect(onQuery).toHaveBeenLastCalledWith({
        page: 1,
        limit: 30,
        sort: [],
        filters: [{ field: "search", operator: "like", value: "%mars%" }]
      });
    });

    // Intermediate values never reached the loader.
    const searchedValues = onQuery.mock.calls
      .flatMap((call) => call[0].filters ?? [])
      .map((filter: { value?: string }) => filter.value);
    expect(searchedValues).not.toContain("%m%");
    expect(searchedValues).not.toContain("%ma%");
    expect(searchedValues).not.toContain("%mar%");
    // One debounced refetch beyond the baseline load.
    expect(onQuery.mock.calls.length).toBe(initialCalls + 1);
  });

  it("applies query variants as baseline query state", async () => {
    const onQuery = vi.fn();
    render(EntityListQueryVariantHarness, { onQuery });

    await waitFor(() => {
      expect(onQuery).toHaveBeenCalledWith({ page: 3, limit: 30, variant: "pending", filters: [], sort: [] });
    });

    await fireEvent.click(screen.getByRole("button", { name: /marked/i }));

    await waitFor(() => {
      expect(onQuery).toHaveBeenLastCalledWith({
        page: 1,
        limit: 30,
        variant: "marked",
        filters: [],
        sort: []
      });
    });
  });

  it("applies API-published query variant capabilities", async () => {
    const onQuery = vi.fn();
    render(EntityListQueryVariantHarness, { onQuery, useCapabilities: true });

    await waitFor(() => {
      expect(onQuery).toHaveBeenLastCalledWith({ page: 3, limit: 30, variant: "marked", filters: [], sort: [] });
    });
  });

  it("falls back to visible count when a paged response omits total", async () => {
    render(EntityListSummaryFallbackHarness);

    await waitFor(() => {
      expect(screen.getByText("Showing 1-5 of 5")).toBeTruthy();
    });
  });

  it("renders the log presentation via the generic toLogEntries mapper", async () => {
    render(EntityListLogHarness);

    await waitFor(() => {
      expect(screen.getByText('"Apollo"')).toBeTruthy();
      expect(screen.getByText('"Mercury"')).toBeTruthy();
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
    expect(screen.queryByRole("menuitem", { name: "Archive" })).toBeNull();

    const actionCard = screen.getByRole("button", { name: "Project Artemis" });
    await fireEvent.contextMenu(actionCard);
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

  it("supports managed inline list modules with dialog add, item actions, and compact pagination", async () => {
    render(EntityInlineListModuleHarness);

    expect(await screen.findByText("Knowledge")).toBeTruthy();
    expect(screen.queryByText("Showing 1-5 of 6")).toBeNull();

    await fireEvent.click(screen.getByRole("button", { name: "Add level" }));
    expect(await screen.findByText("Create a new level.")).toBeTruthy();

    await fireEvent.click(screen.getByRole("button", { name: "Create level" }));
    await waitFor(() => {
      expect(screen.queryByText("Create a new level.")).toBeNull();
    });

    const actionButtons = screen.getAllByRole("button", { name: "Levels item actions" });
    await fireEvent.click(actionButtons[0]);
    await fireEvent.click(await screen.findByRole("menuitem", { name: "Rename Knowledge" }));
    expect(screen.getByTestId("inline-list-last-action").textContent).toBe("rename:level-1");

    await fireEvent.click(screen.getAllByRole("button", { name: "Levels item actions" })[0]);
    await fireEvent.click(await screen.findByRole("menuitem", { name: "Delete level" }));
    await fireEvent.click(await screen.findByRole("button", { name: "Delete level" }));

    await waitFor(() => {
      expect(screen.getByTestId("inline-list-last-action").textContent).toBe("delete:level-1");
      expect(screen.queryByText("Knowledge")).toBeNull();
    });

    await fireEvent.click(screen.getByRole("button", { name: "Next page" }));
    expect(await screen.findByText("Reflection")).toBeTruthy();
  });
});
