/// <reference types="vitest" />
// @vitest-environment jsdom

import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import PasskeyManager from "../../src/components/auth/PasskeyManager.svelte";

describe("components/auth/PasskeyManager.svelte", () => {
  it("renders empty-state registration CTA", async () => {
    const onRegister = vi.fn(async () => undefined);

    render(PasskeyManager, {
      passkeys: [],
      onRegister,
    });

    await fireEvent.click(screen.getAllByRole("button", { name: "Add passkey" })[0]);

    await waitFor(() => {
      expect(onRegister).toHaveBeenCalledTimes(1);
    });
  });

  it("supports rename and delete actions for existing passkeys", async () => {
    const onRename = vi.fn(async () => undefined);
    const onDelete = vi.fn(async () => undefined);

    render(PasskeyManager, {
      passkeys: [
        {
          id: "pk-1",
          name: "Work laptop",
          createdAt: "2026-03-11T10:00:00.000Z",
          lastUsedAt: "2026-03-11T12:00:00.000Z",
          deviceLabel: "Safari on macOS",
        },
      ],
      onRename,
      onDelete,
    });

    expect(screen.getByText("Work laptop")).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Rename" }));
    await fireEvent.input(screen.getByLabelText("Rename Work laptop"), {
      target: { value: "Primary laptop" },
    });
    await fireEvent.click(screen.getByRole("button", { name: "Save" }));

    await waitFor(() => {
      expect(onRename).toHaveBeenCalledWith("pk-1", "Primary laptop");
    });

    await fireEvent.click(screen.getByRole("button", { name: "Delete" }));
    await waitFor(() => {
      expect(onDelete).toHaveBeenCalledWith("pk-1");
    });
  });
});
