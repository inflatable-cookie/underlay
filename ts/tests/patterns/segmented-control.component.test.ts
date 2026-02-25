// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import SegmentedControlHarness from "../fixtures/SegmentedControlHarness.svelte";

describe("components/SegmentedControl.svelte", () => {
  it("renders radiogroup + hidden input and updates selection on click", async () => {
    const view = render(SegmentedControlHarness, {
      name: "mode",
      value: "one",
      ariaLabel: "Display mode"
    });

    const group = screen.getByRole("radiogroup", { name: "Display mode" });
    expect(group).toBeTruthy();

    const hidden = view.container.querySelector('input[type="hidden"][name="mode"]') as HTMLInputElement;
    expect(hidden.value).toBe("one");
    expect(screen.getByRole("radio", { name: "One" }).getAttribute("aria-checked")).toBe("true");

    await fireEvent.click(screen.getByRole("radio", { name: "Two" }));
    expect(screen.getByTestId("segmented-value").textContent).toBe("two");
    expect(hidden.value).toBe("two");
  });

  it("prevents selection changes when disabled", async () => {
    render(SegmentedControlHarness, {
      value: "one",
      disabled: true
    });

    await fireEvent.click(screen.getByRole("radio", { name: "Three" }));
    expect(screen.getByTestId("segmented-value").textContent).toBe("one");
  });
});
