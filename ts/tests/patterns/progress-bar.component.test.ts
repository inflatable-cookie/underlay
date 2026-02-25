// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import ProgressBarHarness from "../fixtures/ProgressBarHarness.svelte";

describe("components/ProgressBar.svelte", () => {
  it("renders progress semantics, classes, and formatted label", () => {
    const view = render(ProgressBarHarness, {
      value: 30,
      max: 60,
      variant: "success",
      size: "lg",
      showLabel: true,
      animated: true,
      className: "custom-progress",
      withFormatLabel: true
    });

    const root = view.container.querySelector(".underlay-progress") as HTMLElement;
    const fill = view.container.querySelector(".underlay-progress-fill") as HTMLElement;

    expect(root).toBeTruthy();
    expect(root.classList.contains("underlay-progress--success")).toBe(true);
    expect(root.classList.contains("underlay-progress--lg")).toBe(true);
    expect(root.classList.contains("custom-progress")).toBe(true);
    expect(root.getAttribute("aria-valuenow")).toBe("30");
    expect(root.getAttribute("aria-valuemax")).toBe("60");
    expect(root.getAttribute("aria-label")).toBe("30/60 (50%)");
    expect(fill.classList.contains("underlay-progress-fill--animated")).toBe(true);
    expect(fill.getAttribute("style")).toContain("width: 50%");
    expect(screen.getByText("30/60 (50%)")).toBeTruthy();
  });

  it("clamps width and supports custom label snippet", () => {
    const view = render(ProgressBarHarness, {
      value: 150,
      max: 100,
      withCustomLabel: true
    });

    const fill = view.container.querySelector(".underlay-progress-fill") as HTMLElement;
    expect(fill.getAttribute("style")).toContain("width: 100%");
    expect(screen.getByTestId("progress-custom-label").textContent).toContain("150 of 100");
  });
});
