// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import ConfirmActionHarness from "../fixtures/ConfirmActionHarness.svelte";

describe("components/ConfirmAction.svelte", () => {
  it("opens alert dialog from trigger and confirms action", async () => {
    const onConfirm = vi.fn();
    render(ConfirmActionHarness, {
      triggerLabel: "Remove",
      title: "Confirm removal",
      confirmLabel: "Yes",
      cancelLabel: "No",
      onConfirm
    });

    await fireEvent.click(screen.getByRole("button", { name: "Remove" }));
    await waitFor(() => {
      expect(screen.getByText("Confirm removal")).toBeTruthy();
    });

    await fireEvent.click(screen.getByRole("button", { name: "Yes" }));
    expect(onConfirm).toHaveBeenCalledTimes(1);
  });

  it("propagates cancel callback", async () => {
    const onCancel = vi.fn();
    render(ConfirmActionHarness, {
      onCancel
    });

    await fireEvent.click(screen.getByRole("button", { name: "Remove" }));
    await fireEvent.click(screen.getByRole("button", { name: "No" }));
    expect(onCancel).toHaveBeenCalledTimes(1);
  });
});
