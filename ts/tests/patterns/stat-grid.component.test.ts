// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import StatGridHarness from "../fixtures/StatGridHarness.svelte";

describe("components/StatGrid.svelte", () => {
  it("renders children with default column and width CSS vars", () => {
    const view = render(StatGridHarness, {});

    expect(screen.getByTestId("stat-grid-item-a").textContent).toContain("A");
    expect(screen.getByTestId("stat-grid-item-b").textContent).toContain("B");

    const grid = view.container.querySelector(".underlay-stat-grid") as HTMLElement;
    expect(grid).toBeTruthy();
    expect(grid.getAttribute("style")).toContain("--min-item-width: 250px;");
    expect(grid.getAttribute("style")).toContain("--max-columns: 4;");
  });

  it("applies custom column and min width values", () => {
    const view = render(StatGridHarness, {
      columns: 3,
      minItemWidth: 320
    });

    const grid = view.container.querySelector(".underlay-stat-grid") as HTMLElement;
    expect(grid.getAttribute("style")).toContain("--min-item-width: 320px;");
    expect(grid.getAttribute("style")).toContain("--max-columns: 3;");
  });
});
