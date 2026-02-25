// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import DurationInputHarness from "../fixtures/DurationInputHarness.svelte";

describe("components/DurationInput.svelte", () => {
  it("composes hours/minutes/seconds into total seconds and hidden value", async () => {
    const view = render(DurationInputHarness, {
      unit: "seconds",
      name: "duration_seconds"
    });

    await fireEvent.input(screen.getByLabelText("Hours"), { target: { value: "1" } });
    await fireEvent.input(screen.getByLabelText("Minutes"), { target: { value: "2" } });
    await fireEvent.input(screen.getByLabelText("Seconds"), { target: { value: "3" } });

    expect(screen.getByTestId("duration-value").textContent).toBe("3723");
    expect((view.container.querySelector('input[type="hidden"][name="duration_seconds"]') as HTMLInputElement).value).toBe("3723");
  });

  it("supports minutes mode with no seconds segment and blur-based total clamping", async () => {
    const view = render(DurationInputHarness, {
      unit: "minutes",
      max: 62,
      name: "duration_minutes",
      initialValue: "62"
    });

    expect(screen.queryByLabelText("Seconds")).toBeNull();
    const minutes = screen.getByLabelText("Minutes");
    await fireEvent.focus(minutes);
    await fireEvent.click(screen.getByRole("button", { name: "Increment" }));
    expect(screen.getByTestId("duration-value").textContent).toBe("63");

    await fireEvent.blur(minutes);
    expect(screen.getByTestId("duration-value").textContent).toBe("62");
    expect((view.container.querySelector('input[type="hidden"][name="duration_minutes"]') as HTMLInputElement).value).toBe("62");
  });
});
