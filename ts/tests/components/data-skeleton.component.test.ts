// @vitest-environment jsdom
import { afterEach, describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import DataSkeletonHarness from "../fixtures/DataSkeletonHarness.svelte";
import {
  registerDataSkeletonPreset,
  unregisterDataSkeletonPreset
} from "../../src/components/data-skeleton";

describe("components/DataSkeleton.svelte", () => {
  afterEach(() => {
    unregisterDataSkeletonPreset("user-grid");
  });

  it("renders avatar-text list presets with the requested item count", () => {
    const view = render(DataSkeletonHarness, {
      type: "list",
      pattern: "avatar-text",
      count: 4
    });

    expect(view.container.querySelectorAll("[data-testid='data-skeleton-item']").length).toBe(4);
    expect(view.container.querySelectorAll(".underlay-skeleton--avatar").length).toBe(4);
  });

  it("renders table skeletons with header and row cells", () => {
    const view = render(DataSkeletonHarness, {
      type: "table",
      columns: 4,
      rows: 3,
      header: true
    });

    expect(view.container.querySelectorAll(".underlay-data-skeleton__table-row").length).toBe(4);
    expect(view.container.querySelectorAll(".underlay-data-skeleton__table-row--header .underlay-skeleton").length).toBe(4);
  });

  it("renders detail skeletons for selected sections only", () => {
    render(DataSkeletonHarness, {
      type: "detail",
      sections: ["header", "actions"]
    });

    expect(screen.getByTestId("data-skeleton-section-header")).toBeTruthy();
    expect(screen.getByTestId("data-skeleton-section-actions")).toBeTruthy();
    expect(screen.queryByTestId("data-skeleton-section-stats")).toBeNull();
    expect(screen.queryByTestId("data-skeleton-section-description")).toBeNull();
  });

  it("supports registered reusable presets over built-in layouts", () => {
    registerDataSkeletonPreset("user-grid", {
      type: "grid",
      pattern: "product-card",
      count: 2,
      columns: 2
    });

    const view = render(DataSkeletonHarness, {
      pattern: "user-grid"
    });

    const grid = view.container.querySelector(".underlay-data-skeleton__grid") as HTMLElement;
    expect(view.container.querySelectorAll("[data-testid='data-skeleton-item']").length).toBe(2);
    expect(grid.getAttribute("style")).toContain("repeat(2, minmax(0, 1fr))");
  });
});
