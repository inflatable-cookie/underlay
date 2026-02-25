// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import DateRangeHarness from "../fixtures/DateRangeHarness.svelte";

describe("components/DateRange.svelte", () => {
  it("renders adaptive same-month ranges with ordinal formatting", () => {
    const view = render(DateRangeHarness, {
      startDate: "2026-01-01T00:00:00.000Z",
      endDate: "2026-01-03T00:00:00.000Z",
      style: "adaptive",
      className: "custom-range"
    });

    const el = view.container.querySelector(".underlay-date-range") as HTMLElement;
    expect(el.classList.contains("custom-range")).toBe(true);
    expect(screen.getByText("1st to 3rd Jan 2026")).toBeTruthy();
  });

  it("renders full style and falls back to empty text when missing endpoints", () => {
    const full = render(DateRangeHarness, {
      startDate: "2026-04-01T00:00:00.000Z",
      endDate: "2026-05-02T00:00:00.000Z",
      style: "full"
    });
    expect(screen.getByText("1st Apr 2026 to 2nd May 2026")).toBeTruthy();
    full.unmount();

    const empty = render(DateRangeHarness, {
      startDate: "2026-04-01T00:00:00.000Z",
      endDate: null,
      emptyText: "No range"
    });
    expect(screen.getByText("No range")).toBeTruthy();
    expect(empty.container.querySelector(".underlay-date-range--empty")).toBeTruthy();
  });
});
