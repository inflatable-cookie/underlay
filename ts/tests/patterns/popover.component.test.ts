// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import PopoverHarness from "../fixtures/PopoverHarness.svelte";

describe("components/Popover.svelte", () => {
  it("opens from trigger and renders children content", async () => {
    const view = render(PopoverHarness, {
      triggerLabel: "Open popover",
      triggerAriaLabel: "Open details"
    });

    await fireEvent.click(screen.getByRole("button", { name: "Open details" }));
    await waitFor(() => {
      expect(screen.getByTestId("popover-content").textContent).toContain("Popover content");
    });
    expect(view.container.querySelector('[data-testid="popover-open-state"]')?.textContent).toBe("open");
  });

  it("supports open-without-trigger and custom content/trigger classes", () => {
    render(PopoverHarness, {
      initialOpen: true,
      showTrigger: false,
      contentClassName: "custom-popover-content",
      className: "custom-popover-trigger"
    });

    expect(screen.queryByRole("button", { name: "Popover trigger" })).toBeNull();
    const content = document.querySelector(".underlay-popover-content") as HTMLElement;
    expect(content.classList.contains("custom-popover-content")).toBe(true);
    expect(screen.getByTestId("popover-content")).toBeTruthy();
  });
});
