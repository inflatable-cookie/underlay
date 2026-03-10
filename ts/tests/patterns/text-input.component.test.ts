// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import TextInputHarness from "../fixtures/TextInputHarness.svelte";

describe("components/TextInput.svelte", () => {
  it("supports debounced onchange and clear affordance in search mode", async () => {
    vi.useFakeTimers();
    const onInput = vi.fn();
    const onChange = vi.fn();

    try {
      const view = render(TextInputHarness, {
        initialValue: "abc",
        search: true,
        debounce: 200,
        onInput,
        onChange
      });

      const input = view.container.querySelector("input.underlay-input") as HTMLInputElement;
      await fireEvent.input(input, { target: { value: "abcdef" } });
      expect(screen.getByTestId("text-input-value").textContent).toBe("abcdef");
      expect(onInput).toHaveBeenCalledWith("abcdef");
      expect(screen.getByRole("button", { name: "Clear" })).toBeTruthy();

      vi.advanceTimersByTime(200);
      expect(onChange).toHaveBeenCalledWith("abcdef");

      await fireEvent.click(screen.getByRole("button", { name: "Clear" }));
      expect(screen.getByTestId("text-input-value").textContent).toBe("");
      expect(onChange).toHaveBeenCalledWith("");
    } finally {
      vi.useRealTimers();
    }
  });

  it("shows validation error message when async validate returns invalid", async () => {
    vi.useFakeTimers();
    const validate = vi.fn(async () => ({ valid: false, message: "Not available" }));

    try {
      const view = render(TextInputHarness, {
        initialValue: "",
        validate,
        showValidationStatus: true,
        withSuffix: true
      });

      const input = view.container.querySelector("input.underlay-input") as HTMLInputElement;
      await fireEvent.input(input, { target: { value: "taken" } });
      vi.advanceTimersByTime(300);

      await waitFor(() => {
        expect(screen.getByText("Not available")).toBeTruthy();
      });
      expect(validate).toHaveBeenCalledWith("taken", undefined);
      expect(screen.getByTestId("text-input-suffix")).toBeTruthy();
      expect(view.container.querySelector(".underlay-input-wrapper--suffixed")).toBeTruthy();
    } finally {
      vi.useRealTimers();
    }
  });

  it("flushes pending validation immediately on change instead of restarting debounce", async () => {
    vi.useFakeTimers();
    const validate = vi.fn(async () => ({ valid: true }));

    try {
      const view = render(TextInputHarness, {
        initialValue: "",
        validate,
        showValidationStatus: true
      });

      const input = view.container.querySelector("input.underlay-input") as HTMLInputElement;
      await fireEvent.input(input, { target: { value: "C" } });
      expect(validate).toHaveBeenCalledTimes(0);

      await fireEvent.change(input);

      await waitFor(() => {
        expect(validate).toHaveBeenCalledTimes(1);
      });
      expect(validate).toHaveBeenCalledWith("C", undefined);
    } finally {
      vi.useRealTimers();
    }
  });

  it("does not revalidate on change when the current value was already validated", async () => {
    vi.useFakeTimers();
    const validate = vi.fn(async () => ({ valid: true }));

    try {
      const view = render(TextInputHarness, {
        initialValue: "",
        validate,
        showValidationStatus: true
      });

      const input = view.container.querySelector("input.underlay-input") as HTMLInputElement;
      await fireEvent.input(input, { target: { value: "C" } });
      vi.advanceTimersByTime(300);

      await waitFor(() => {
        expect(validate).toHaveBeenCalledTimes(1);
      });

      await fireEvent.change(input);
      vi.runOnlyPendingTimers();

      expect(validate).toHaveBeenCalledTimes(1);
    } finally {
      vi.useRealTimers();
    }
  });
});
