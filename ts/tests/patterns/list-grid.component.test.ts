// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import ListGridHarness from "../fixtures/ListGridHarness.svelte";

describe("components/ListGrid.svelte", () => {
  it("renders header/actions and formats default variant style vars", () => {
    const view = render(ListGridHarness, {
      variant: "default",
      minItemWidth: 24,
      gap: 12
    });

    expect(screen.getByTestId("list-grid-action")).toBeTruthy();
    expect(screen.getByTestId("list-grid-item-a")).toBeTruthy();
    expect(screen.getByTestId("list-grid-item-b")).toBeTruthy();

    const header = view.container.querySelector(".underlay-list-grid-header");
    const grid = view.container.querySelector(".underlay-list-grid") as HTMLElement;
    expect(header).toBeTruthy();
    expect(grid).toBeTruthy();
    expect(grid.classList.contains("underlay-list-grid--compact")).toBe(false);
    expect(grid.getAttribute("style")).toContain("--underlay-list-grid-min: 24em;");
    expect(grid.getAttribute("style")).toContain("--underlay-list-grid-gap: 12px;");
  });

  it("omits header when actions are absent and suppresses min width style in compact mode", () => {
    const view = render(ListGridHarness, {
      variant: "compact",
      minItemWidth: "18rem",
      gap: "0.75rem",
      withActions: false
    });

    const header = view.container.querySelector(".underlay-list-grid-header");
    const grid = view.container.querySelector(".underlay-list-grid") as HTMLElement;
    expect(header).toBeNull();
    expect(grid.classList.contains("underlay-list-grid--compact")).toBe(true);
    expect(grid.getAttribute("style")).not.toContain("--underlay-list-grid-min");
    expect(grid.getAttribute("style")).toContain("--underlay-list-grid-gap: 0.75rem;");
  });
});
