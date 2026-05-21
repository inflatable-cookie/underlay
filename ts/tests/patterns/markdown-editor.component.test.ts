// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import MarkdownEditorHarness from "../fixtures/MarkdownEditorHarness.svelte";

describe("nightfire/markup/MarkdownEditorSurface.svelte", () => {
  it("renders textarea mode with label/hint and updates bound value", async () => {
    const view = render(MarkdownEditorHarness, {
      showPreview: false,
      label: "Description",
      hint: "Markdown supported",
      initialValue: "Start"
    });

    expect(screen.getByText("Description")).toBeTruthy();
    expect(screen.getByText("Markdown supported")).toBeTruthy();

    const textarea = view.container.querySelector("textarea") as HTMLTextAreaElement;
    await fireEvent.input(textarea, { target: { value: "Updated text" } });
    expect(screen.getByTestId("markdown-value").textContent).toBe("Updated text");
  });

  it("disables the editor while loading", () => {
    const view = render(MarkdownEditorHarness, {
      loading: true,
      showPreview: true
    });

    expect(view.container.querySelector("textarea")).toBeTruthy();
    expect((view.container.querySelector("textarea") as HTMLTextAreaElement).disabled).toBe(true);
  });
});
