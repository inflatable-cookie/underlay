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
    expect(view.container.querySelector(".underlay-preview-hidden")).toBeTruthy();

    const textarea = view.container.querySelector("textarea.underlay-markdown-editor-textarea") as HTMLTextAreaElement;
    await fireEvent.input(textarea, { target: { value: "Updated text" } });
    expect(screen.getByTestId("markdown-value").textContent).toBe("Updated text");
  });

  it("shows loading spinner state and keeps textarea in DOM", () => {
    const view = render(MarkdownEditorHarness, {
      loading: true,
      showPreview: true
    });

    expect(screen.getByText("Loading markdown editor...")).toBeTruthy();
    expect(view.container.querySelector(".underlay-markdown-editor-spinner")).toBeTruthy();
    expect(view.container.querySelector("textarea.underlay-markdown-editor-textarea")).toBeTruthy();
  });
});
