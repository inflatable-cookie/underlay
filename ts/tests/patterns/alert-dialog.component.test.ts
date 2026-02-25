// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import AlertDialogHarness from "../fixtures/AlertDialogHarness.svelte";

describe("components/AlertDialog.svelte", () => {
  it("opens from trigger, renders content, and closes on cancel", async () => {
    const onCancel = vi.fn();
    const view = render(AlertDialogHarness, {
      triggerLabel: "Open alert",
      cancelLabel: "Cancel",
      onCancel
    });

    await fireEvent.click(screen.getByRole("button", { name: "Open alert" }));
    await waitFor(() => {
      expect(screen.getByText("Delete item?")).toBeTruthy();
    });
    expect(screen.getByTestId("alert-dialog-child")).toBeTruthy();
    expect(view.container.querySelector('[data-testid="alert-open-state"]')?.textContent).toBe("open");

    await fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
    await waitFor(() => {
      expect(view.container.querySelector('[data-testid="alert-open-state"]')?.textContent).toBe("closed");
    });
    expect(onCancel).toHaveBeenCalledTimes(1);
  });

  it("runs async confirm and closes after resolution", async () => {
    const onConfirm = vi.fn(async () => {
      await Promise.resolve();
    });
    const view = render(AlertDialogHarness, {
      initialOpen: true,
      showTrigger: false,
      confirmLabel: "Delete",
      onConfirm
    });

    await fireEvent.click(screen.getByRole("button", { name: "Delete" }));
    expect(onConfirm).toHaveBeenCalledTimes(1);

    await waitFor(() => {
      expect(view.container.querySelector('[data-testid="alert-open-state"]')?.textContent).toBe("closed");
    });
  });
});
