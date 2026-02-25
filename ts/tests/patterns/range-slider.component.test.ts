// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import RangeSliderHarness from "../fixtures/RangeSliderHarness.svelte";

describe("components/RangeSlider.svelte", () => {
  it("renders selected label/value and hidden input", () => {
    const view = render(RangeSliderHarness, {
      name: "priority",
      initialValue: "med",
      showValue: true
    });

    expect(screen.getByText("Medium")).toBeTruthy();
    expect(screen.getByTestId("range-slider-value").textContent).toBe("med");
    expect((view.container.querySelector('input[type="hidden"][name="priority"]') as HTMLInputElement).value).toBe("med");
  });

  it("maps range index to option values and fires input/change callbacks", async () => {
    const onInput = vi.fn();
    const onChange = vi.fn();
    const view = render(RangeSliderHarness, {
      initialValue: "low",
      onInput,
      onChange
    });

    const slider = view.container.querySelector('input[type="range"]') as HTMLInputElement;
    await fireEvent.input(slider, { target: { value: "2" } });
    expect(screen.getByTestId("range-slider-value").textContent).toBe("high");
    expect(onInput).toHaveBeenCalledWith("high");

    await fireEvent.change(slider, { target: { value: "1" } });
    expect(screen.getByTestId("range-slider-value").textContent).toBe("med");
    expect(onChange).toHaveBeenCalledWith("med");
  });
});
