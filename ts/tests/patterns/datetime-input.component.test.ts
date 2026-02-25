// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import DateTimeInputHarness from "../fixtures/DateTimeInputHarness.svelte";

describe("components/DateTimeInput.svelte", () => {
  it("defaults to current datetime and syncs RFC3339 hidden value", async () => {
    const onChange = vi.fn();
    const view = render(DateTimeInputHarness, {
      defaultToNow: true,
      name: "starts_at",
      onChange
    });

    const hidden = view.container.querySelector('input[type="hidden"][name="starts_at"]') as HTMLInputElement;
    await waitFor(() => {
      expect(hidden.value).toContain("T");
      expect(hidden.value.endsWith("Z")).toBe(true);
    });
    expect(onChange).toHaveBeenCalledTimes(1);
    expect(onChange.mock.calls[0]?.[0]).toBe(hidden.value);
  });

  it("converts datetime-local input changes to RFC3339 value", async () => {
    const onChange = vi.fn();
    const view = render(DateTimeInputHarness, {
      name: "published_at",
      onChange
    });

    const local = "2026-05-01T09:30";
    const input = view.container.querySelector('input[type="datetime-local"]') as HTMLInputElement;
    await fireEvent.input(input, { target: { value: local } });

    const expected = new Date(local).toISOString();
    expect(screen.getByTestId("datetime-input-value").textContent).toBe(expected);
    expect((view.container.querySelector('input[type="hidden"][name="published_at"]') as HTMLInputElement).value).toBe(expected);
    expect(onChange).toHaveBeenLastCalledWith(expected);
  });
});
