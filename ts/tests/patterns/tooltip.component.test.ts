// @vitest-environment jsdom
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import TooltipHarness from "../fixtures/TooltipHarness.svelte";

describe("components/Tooltip.svelte", () => {
  let originalResizeObserver: typeof ResizeObserver | undefined;

  beforeEach(() => {
    originalResizeObserver = globalThis.ResizeObserver;
    class ResizeObserverMock {
      observe() {
        return undefined;
      }
      disconnect() {
        return undefined;
      }
    }
    // @ts-expect-error test shim
    globalThis.ResizeObserver = ResizeObserverMock;
  });

  afterEach(() => {
    globalThis.ResizeObserver = originalResizeObserver as typeof ResizeObserver;
  });

  it("renders default trigger and open tooltip content", () => {
    render(TooltipHarness, {
      initialOpen: true,
      content: "Helpful hint",
      triggerLabel: "?"
    });

    const trigger = screen.getByRole("button", { name: "Helpful hint" });
    expect(trigger.className).toContain("underlay-tooltip-trigger");
    expect(trigger.textContent).toContain("?");
    expect(screen.getByText("Helpful hint")).toBeTruthy();
    expect(screen.getByTestId("tooltip-open-state").textContent).toBe("open");
  });

  it("uses inline trigger class for inline mode", () => {
    const inlineView = render(TooltipHarness, {
      initialOpen: false,
      inline: true,
      className: "custom-tooltip"
    });

    const inlineTrigger = inlineView.container.querySelector('[aria-label="Tooltip content"]') as HTMLElement;
    expect(inlineTrigger.className).toContain("underlay-tooltip-trigger--inline");
    expect(inlineTrigger.className).toContain("custom-tooltip");
  });

  it("supports custom trigger snippet", () => {
    render(TooltipHarness, {
      withTriggerSnippet: true,
      content: "Custom content"
    });
    expect(screen.getByTestId("tooltip-custom-trigger").textContent).toContain("Hover target");
  });
});
