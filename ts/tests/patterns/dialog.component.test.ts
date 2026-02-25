// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import DialogHarness from "../fixtures/DialogHarness.svelte";

describe("components/Dialog.svelte", () => {
  it("opens from trigger and renders title/description/body/footer", async () => {
    const view = render(DialogHarness, {
      triggerLabel: "Launch dialog",
      withFooter: true,
      withChildren: true
    });

    await fireEvent.click(screen.getByRole("button", { name: "Launch dialog" }));

    await waitFor(() => {
      expect(screen.getByText("Dialog title")).toBeTruthy();
      expect(screen.getByText("Dialog description")).toBeTruthy();
    });

    expect(screen.getByTestId("dialog-body").textContent).toContain("Dialog body content");
    expect(screen.getByTestId("dialog-footer-button").textContent).toContain("Confirm");
    expect(view.container.querySelector('[data-testid="dialog-open-state"]')?.textContent).toBe("open");

    await fireEvent.click(screen.getByRole("button", { name: "Close" }));
    await waitFor(() => {
      expect(view.container.querySelector('[data-testid="dialog-open-state"]')?.textContent).toBe("closed");
    });
  });

  it("supports open-without-trigger, custom classes, and optional close-x", () => {
    render(DialogHarness, {
      initialOpen: true,
      showTrigger: false,
      showCloseX: false,
      contentClassName: "custom-dialog-content",
      overlayClassName: "custom-dialog-overlay",
      title: "Open dialog"
    });

    expect(screen.queryByRole("button", { name: "Open dialog" })).toBeNull();
    expect(document.querySelector(".underlay-dialog-content")?.classList.contains("custom-dialog-content")).toBe(true);
    expect(document.querySelector(".underlay-dialog-overlay")?.classList.contains("custom-dialog-overlay")).toBe(true);
    expect(document.querySelector(".underlay-dialog-close-x")).toBeNull();
    expect(screen.getByText("Open dialog")).toBeTruthy();
  });
});
