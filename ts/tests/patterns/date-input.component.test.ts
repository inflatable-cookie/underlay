// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import DateInputHarness from "../fixtures/DateInputHarness.svelte";

describe("components/DateInput.svelte", () => {
  it("defaults to today when configured and syncs hidden form value", async () => {
    const onChange = vi.fn();
    const view = render(DateInputHarness, {
      initialValue: "",
      defaultToNow: true,
      name: "due_date",
      onChange
    });

    const hidden = view.container.querySelector('input[type="hidden"][name="due_date"]') as HTMLInputElement;
    await waitFor(() => {
      expect(hidden.value).toMatch(/^\d{4}-\d{2}-\d{2}$/);
    });
    expect(onChange).toHaveBeenCalledTimes(1);
    expect(onChange.mock.calls[0]?.[0]).toBe(hidden.value);
  });

  it("updates value and emits onchange from date input interactions", async () => {
    const onChange = vi.fn();
    const view = render(DateInputHarness, {
      initialValue: "2024-01-01",
      name: "start_date",
      onChange
    });

    const dateInput = view.container.querySelector('input[type="date"]') as HTMLInputElement;
    await fireEvent.input(dateInput, { target: { value: "2026-03-14" } });

    expect(screen.getByTestId("date-input-value").textContent).toBe("2026-03-14");
    expect((view.container.querySelector('input[type="hidden"][name="start_date"]') as HTMLInputElement).value).toBe("2026-03-14");
    expect(onChange).toHaveBeenLastCalledWith("2026-03-14");
  });
});
