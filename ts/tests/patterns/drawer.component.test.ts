// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import DrawerHarness from "../fixtures/DrawerHarness.svelte";

describe("components/Drawer.svelte", () => {
  it("renders open drawer with title/actions/content and closes via close button", async () => {
    const onclose = vi.fn();
    const view = render(DrawerHarness, {
      initialOpen: true,
      title: "Details panel",
      position: "left",
      width: "32rem",
      className: "custom-drawer",
      onclose
    });

    const drawer = view.container.querySelector(".underlay-drawer") as HTMLElement;
    expect(drawer.classList.contains("underlay-drawer--open")).toBe(true);
    expect(drawer.classList.contains("underlay-drawer--left")).toBe(true);
    expect(drawer.classList.contains("custom-drawer")).toBe(true);
    expect(drawer.getAttribute("style")).toContain("--drawer-width: 32rem;");
    expect(screen.getByText("Details panel")).toBeTruthy();
    expect(screen.getByTestId("drawer-action")).toBeTruthy();
    expect(screen.getByTestId("drawer-content")).toBeTruthy();

    await fireEvent.click(screen.getByRole("button", { name: "Close panel" }));
    await waitFor(() => {
      expect(view.container.querySelector('[data-testid="drawer-open-state"]')?.textContent).toBe("closed");
    });
    expect(onclose).toHaveBeenCalledTimes(1);
  });

  it("handles backdrop click close when open", async () => {
    const onclose = vi.fn();
    const view = render(DrawerHarness, {
      initialOpen: true,
      overlay: true,
      onclose
    });

    const backdrop = view.container.querySelector(".underlay-drawer-backdrop") as HTMLElement;
    expect(backdrop.classList.contains("underlay-drawer-backdrop--force")).toBe(true);

    await fireEvent.click(backdrop);
    await waitFor(() => {
      expect(view.container.querySelector('[data-testid="drawer-open-state"]')?.textContent).toBe("closed");
    });
    expect(onclose).toHaveBeenCalledTimes(1);
  });

  it("handles escape key close when open", async () => {
    const escapeClose = vi.fn();
    const escapeView = render(DrawerHarness, {
      initialOpen: true,
      onclose: escapeClose
    });
    await fireEvent.keyDown(window, { key: "Escape" });
    await waitFor(() => {
      expect(escapeView.container.querySelector('[data-testid="drawer-open-state"]')?.textContent).toBe("closed");
    });
    expect(escapeClose).toHaveBeenCalledTimes(1);
  });
});
