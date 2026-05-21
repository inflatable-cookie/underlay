// @vitest-environment jsdom
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";

import NightfireEditorHarness from "../fixtures/NightfireEditorHarness.svelte";

describe("nightfire/NightfireEditor slash commands", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("stays inert by default", async () => {
    const view = render(NightfireEditorHarness);
    const textarea = view.container.querySelector("textarea") as HTMLTextAreaElement;

    textarea.value = "/media";
    textarea.selectionStart = textarea.value.length;
    textarea.selectionEnd = textarea.value.length;
    await fireEvent.input(textarea);

    expect(screen.queryByRole("dialog", { name: "Slash commands" })).toBeNull();
  });

  it("opens the slash palette and inserts a new block below the active markdown block", async () => {
    const view = render(NightfireEditorHarness, {
      slashCommands: {
        enabled: true
      }
    });
    const textarea = view.container.querySelector("textarea") as HTMLTextAreaElement;

    textarea.value = "/media";
    textarea.selectionStart = textarea.value.length;
    textarea.selectionEnd = textarea.value.length;
    await fireEvent.input(textarea);

    const dialog = await screen.findByRole("dialog", { name: "Slash commands" });
    expect(dialog).toBeTruthy();

    await fireEvent.click(within(dialog).getByRole("option", { name: /media/i }));

    await waitFor(() => {
      const payload = JSON.parse(screen.getByTestId("nightfire-value").textContent ?? "{}");
      expect(payload.blocks).toHaveLength(2);
      expect(payload.blocks[0].data.text).toBe("");
      expect(payload.blocks[1].type).toBe("media");
    });
  });
});
