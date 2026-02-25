// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import SegmentedControlEdgeHarness from "../fixtures/SegmentedControlEdgeHarness.svelte";

describe("components/SegmentedControl.svelte (edge cases)", () => {
  it("handles empty options while still rendering group and hidden input", () => {
    const view = render(SegmentedControlEdgeHarness, {
      name: "mode",
      value: "",
      options: [],
      ariaLabel: "Editor mode"
    });

    expect(screen.getByRole("radiogroup", { name: "Editor mode" })).toBeTruthy();
    expect(view.container.querySelectorAll('[role="radio"]').length).toBe(0);
    expect((view.container.querySelector('input[type="hidden"][name="mode"]') as HTMLInputElement).value).toBe("");
  });

  it("applies before-active class on segment preceding selected option", async () => {
    const view = render(SegmentedControlEdgeHarness, {
      name: "mode",
      value: "b",
      options: [
        { label: "A", value: "a" },
        { label: "B", value: "b" },
        { label: "C", value: "c" }
      ]
    });

    const radios = view.container.querySelectorAll(".underlay-segmented__option");
    expect(radios[0]?.classList.contains("underlay-segmented__option--before-active")).toBe(true);
    expect(radios[1]?.classList.contains("underlay-segmented__option--active")).toBe(true);

    await fireEvent.click(screen.getByRole("radio", { name: "C" }));
    expect(screen.getByTestId("segmented-edge-value").textContent).toBe("c");
    expect(radios[1]?.classList.contains("underlay-segmented__option--before-active")).toBe(true);
  });
});
