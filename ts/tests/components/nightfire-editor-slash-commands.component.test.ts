// @vitest-environment jsdom
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";

class FakeEasyMDE {
  element: HTMLTextAreaElement;
  listeners = new Map<string, Array<() => void>>();

  codemirror = {
    on: (event: string, handler: () => void) => {
      const existing = this.listeners.get(event) ?? [];
      existing.push(handler);
      this.listeners.set(event, existing);
    },
    getCursor: (which: "from" | "to" = "from") => {
      const position = which === "to"
        ? (this.element.selectionEnd ?? this.element.value.length)
        : (this.element.selectionStart ?? this.element.value.length);
      return { line: 0, ch: position };
    },
    indexFromPos: (position: { ch: number }) => position.ch
  };

  constructor(options: { element: HTMLTextAreaElement; initialValue?: string | null }) {
    this.element = options.element;
    this.element.value = options.initialValue ?? "";
    this.element.addEventListener("input", this.handleInput);
    this.element.addEventListener("keyup", this.handleCursorActivity);
    this.element.addEventListener("click", this.handleCursorActivity);
    this.element.addEventListener("select", this.handleCursorActivity);
  }

  handleInput = () => {
    this.emit("change");
    this.emit("cursorActivity");
  };

  handleCursorActivity = () => {
    this.emit("cursorActivity");
  };

  emit(event: string) {
    for (const listener of this.listeners.get(event) ?? []) {
      listener();
    }
  }

  value() {
    return this.element.value;
  }

  toTextArea() {
    this.element.removeEventListener("input", this.handleInput);
    this.element.removeEventListener("keyup", this.handleCursorActivity);
    this.element.removeEventListener("click", this.handleCursorActivity);
    this.element.removeEventListener("select", this.handleCursorActivity);
  }
}

vi.mock("../../src/nightfire/markup/lazy-load-easymde", () => ({
  lazyLoadEasyMde: vi.fn(async () => FakeEasyMDE)
}));

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
