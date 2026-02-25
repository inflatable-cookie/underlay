// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import DetailItemHarness from "../fixtures/DetailItemHarness.svelte";

describe("components/DetailItem.svelte", () => {
  it("renders label/value and applies value modifier classes", () => {
    const view = render(DetailItemHarness, {
      label: "State",
      value: "active",
      code: true,
      capitalize: true,
      className: "custom-detail-item"
    });

    expect(screen.getByText("State")).toBeTruthy();
    expect(screen.getByText("active")).toBeTruthy();

    const root = view.container.querySelector(".underlay-detail-item") as HTMLElement;
    const value = view.container.querySelector(".underlay-detail-item__value") as HTMLElement;
    expect(root.classList.contains("custom-detail-item")).toBe(true);
    expect(value.classList.contains("underlay-detail-item__value--code")).toBe(true);
    expect(value.classList.contains("underlay-detail-item__value--capitalize")).toBe(true);
  });

  it("renders children over value", () => {
    const childrenView = render(DetailItemHarness, {
      value: "ignored",
      withChildren: true
    });

    expect(screen.getByTestId("detail-item-custom")).toBeTruthy();
    expect(childrenView.container.querySelector(".underlay-detail-item__empty")).toBeNull();
  });

  it("falls back to empty placeholder", () => {
    render(DetailItemHarness, {
      label: "Unset",
      value: null,
      withChildren: false
    });
    expect(screen.getByText("Not set")).toBeTruthy();
  });
});
