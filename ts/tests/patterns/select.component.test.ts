// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import SelectHarness from "../fixtures/SelectHarness.svelte";

describe("nightfire/Select.svelte", () => {
  it("renders native select fallback and emits change/input", async () => {
    const onChange = vi.fn();
    const onInput = vi.fn();

    const view = render(SelectHarness, {
      items: null,
      withNativeChildren: true,
      initialValue: "",
      onChange,
      onInput
    });

    const native = view.container.querySelector("select.underlay-select") as HTMLSelectElement;
    expect(native).toBeTruthy();
    await fireEvent.change(native, { target: { value: "y" } });
    expect(onChange).toHaveBeenCalledWith("y");
    expect(onInput).toHaveBeenCalledWith("y");
    expect(screen.getByTestId("select-value").textContent).toBe("y");
  });

  it("renders bits-select trigger with clear button and resets value", async () => {
    const onChange = vi.fn();
    const onInput = vi.fn();

    render(SelectHarness, {
      initialValue: "a",
      clearable: true,
      defaultValue: "",
      onChange,
      onInput
    });

    const trigger = screen.getByRole("button", { name: "Choose" });
    expect(trigger.textContent).toContain("Alpha");
    expect(screen.getByLabelText("Clear selection")).toBeTruthy();

    await fireEvent.click(screen.getByLabelText("Clear selection"));
    await waitFor(() => {
      expect(screen.getByTestId("select-value").textContent).toBe("");
    });
    expect(onChange).toHaveBeenCalledWith("");
    expect(onInput).toHaveBeenCalledWith("");
  });
});
