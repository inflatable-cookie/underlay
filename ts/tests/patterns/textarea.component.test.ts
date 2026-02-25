// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import TextAreaHarness from "../fixtures/TextAreaHarness.svelte";

describe("components/TextArea.svelte", () => {
  it("renders textarea with id/class and updates bound value via input/change callbacks", async () => {
    const onInput = vi.fn();
    const onChange = vi.fn();
    const view = render(TextAreaHarness, {
      initialValue: "Initial",
      id: "notes",
      className: "custom-textarea",
      onInput,
      onChange
    });

    const textarea = view.container.querySelector("textarea.underlay-textarea") as HTMLTextAreaElement;
    expect(textarea.id).toBe("notes");
    expect(textarea.classList.contains("custom-textarea")).toBe(true);

    await fireEvent.input(textarea, { target: { value: "Updated notes" } });
    expect(screen.getByTestId("textarea-value").textContent).toBe("Updated notes");
    expect(onInput).toHaveBeenCalledWith("Updated notes");

    await fireEvent.change(textarea);
    expect(onChange).toHaveBeenCalledWith("Updated notes");
  });

  it("passes through disabled/readOnly attributes", () => {
    const view = render(TextAreaHarness, {
      initialValue: "Read-only"
    });

    const textarea = view.container.querySelector("textarea.underlay-textarea") as HTMLTextAreaElement;
    expect(textarea.disabled).toBe(false);
    expect(textarea.readOnly).toBe(false);
  });
});
