// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import CompactGroupedBarHarness from "../fixtures/CompactGroupedBarHarness.svelte";

describe("components/CompactGroupedBar.svelte", () => {
  it("renders grouped controls with joined segments and separation styling", () => {
    const view = render(CompactGroupedBarHarness);

    const bar = view.container.querySelector(".underlay-compact-grouped-bar");
    expect(bar).toBeTruthy();
    expect(bar?.classList.contains("underlay-compact-grouped-bar--tight")).toBe(true);
    expect(bar?.classList.contains("underlay-compact-grouped-bar--separated")).toBe(true);

    expect(screen.getByText("Range")).toBeTruthy();
    expect(screen.getByText("Outcome")).toBeTruthy();

    const joinedGroups = view.container.querySelectorAll(
      ".underlay-compact-grouped-bar-group__items--joined"
    );
    expect(joinedGroups.length).toBe(2);
  });

  it("applies per-group priority ordering metadata for responsive wrapping", () => {
    const view = render(CompactGroupedBarHarness);

    const groups = Array.from(
      view.container.querySelectorAll(".underlay-compact-grouped-bar-group")
    ) as HTMLElement[];
    expect(groups.length).toBe(3);

    const rangeGroup = groups.find((group) =>
      group.textContent?.includes("Range")
    );
    const outcomeGroup = groups.find((group) =>
      group.textContent?.includes("Outcome")
    );
    const statusGroup = groups.find((group) =>
      group.textContent?.includes("Status")
    );

    expect(rangeGroup?.style.order).toBe("10");
    expect(outcomeGroup?.style.order).toBe("15");
    expect(statusGroup?.style.order).toBe("40");
  });
});
