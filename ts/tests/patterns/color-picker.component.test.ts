// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import ColorPickerHarness from "../fixtures/ColorPickerHarness.svelte";

describe("components/ColorPicker.svelte", () => {
  it("normalizes typed values and falls back on invalid blur", async () => {
    const onInput = vi.fn();
    const onChange = vi.fn();
    const view = render(ColorPickerHarness, {
      initialValue: "#123456",
      onInput,
      onChange
    });

    const text = view.container.querySelector("input.underlay-color-picker__text-input") as HTMLInputElement;
    await fireEvent.input(text, { target: { value: "abc" } });
    expect(screen.getByTestId("color-picker-value").textContent).toBe("#abc");
    expect(onInput).toHaveBeenLastCalledWith("#abc");

    await fireEvent.blur(text);
    expect(screen.getByTestId("color-picker-value").textContent).toBe("#aabbcc");
    expect(onChange).toHaveBeenLastCalledWith("#aabbcc");

    await fireEvent.input(text, { target: { value: "bad!" } });
    await fireEvent.blur(text);
    expect(screen.getByTestId("color-picker-value").textContent).toBe("#000000");
  });

  it("supports native color input and preset selection", async () => {
    const onInput = vi.fn();
    const onChange = vi.fn();
    const view = render(ColorPickerHarness, {
      initialValue: "#112233",
      presets: ["#ff0000", "#00ff00"],
      onInput,
      onChange
    });

    const native = view.container.querySelector('input[type="color"]') as HTMLInputElement;
    await fireEvent.change(native, { target: { value: "#445566" } });
    expect(screen.getByTestId("color-picker-value").textContent).toBe("#445566");

    await fireEvent.click(screen.getByRole("option", { name: "Select color #00ff00" }));
    expect(screen.getByTestId("color-picker-value").textContent).toBe("#00ff00");
    expect(onInput).toHaveBeenLastCalledWith("#00ff00");
    expect(onChange).toHaveBeenLastCalledWith("#00ff00");
  });
});
