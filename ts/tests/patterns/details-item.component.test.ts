// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import DetailsItemHarness from "../fixtures/DetailsItemHarness.svelte";

describe("components/DetailsItem.svelte", () => {
  it("renders code value, muted/class modifiers, and numeric span style", () => {
    const view = render(DetailsItemHarness, {
      label: "ID",
      value: "abc-123",
      code: true,
      span: 2,
      muted: true,
      className: "custom-details-item"
    });

    const root = view.container.querySelector(".details-item") as HTMLElement;
    const code = view.container.querySelector(".underlay-details-item__code");

    expect(screen.getByText("ID")).toBeTruthy();
    expect(code?.textContent).toContain("abc-123");
    expect(root.classList.contains("details-item--muted")).toBe(true);
    expect(root.classList.contains("custom-details-item")).toBe(true);
    expect(root.getAttribute("style")).toContain("grid-column: span 2");
  });

  it("supports full-span style and custom children", () => {
    const childrenView = render(DetailsItemHarness, {
      span: "full",
      withChildren: true,
      value: "ignored"
    });

    const root = childrenView.container.querySelector(".details-item") as HTMLElement;
    expect(root.getAttribute("style")).toContain("grid-column: span var(--details-grid-columns, 4)");
    expect(screen.getByTestId("details-item-custom")).toBeTruthy();
    expect(childrenView.container.querySelector(".underlay-details-item__empty")).toBeNull();
  });

  it("shows empty fallback when value is null", () => {
    render(DetailsItemHarness, {
      value: null,
      withChildren: false
    });
    expect(screen.getByText("—")).toBeTruthy();
  });
});
