// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import NumberInputHarness from "../fixtures/NumberInputHarness.svelte";

describe("components/NumberInput.svelte", () => {
  it("increments/decrements within min/max bounds and updates button disabled state", async () => {
    render(NumberInputHarness, {
      initialValue: "1",
      min: 0,
      max: 2,
      step: 1
    });

    const increment = screen.getByRole("button", { name: "Increment" }) as HTMLButtonElement;
    const decrement = screen.getByRole("button", { name: "Decrement" }) as HTMLButtonElement;

    await fireEvent.click(increment);
    expect(screen.getByTestId("number-value").textContent).toBe("2");
    expect(increment.disabled).toBe(true);

    await fireEvent.click(decrement);
    await fireEvent.click(decrement);
    expect(screen.getByTestId("number-value").textContent).toBe("0");
    expect(decrement.disabled).toBe(true);
  });

  it("sanitizes decimal input, snaps to step on blur, and clamps max on blur", async () => {
    const view = render(NumberInputHarness, {
      step: 0.5,
      min: 0,
      max: 10
    });

    const input = view.container.querySelector("input.underlay-input") as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "1a.5" } });
    expect(screen.getByTestId("number-value").textContent).toBe("1.5");

    await fireEvent.input(input, { target: { value: "1.6" } });
    expect(screen.getByTestId("number-value").textContent).toBe("1.6");
    await fireEvent.blur(input);
    expect(screen.getByTestId("number-value").textContent).toBe("1.5");

    await fireEvent.input(input, { target: { value: "11" } });
    await fireEvent.blur(input);
    expect(screen.getByTestId("number-value").textContent).toBe("10");
  });
});
