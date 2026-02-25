// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import DetailListHarness from "../fixtures/DetailListHarness.svelte";

describe("components/DetailList.svelte", () => {
  it("renders title, wrapper class, and children inside definition list", () => {
    const view = render(DetailListHarness, {
      title: "Metadata",
      className: "custom-detail-list",
      withChildren: true
    });

    const root = view.container.querySelector(".underlay-detail-list") as HTMLElement;
    const items = view.container.querySelector(".underlay-detail-list__items");

    expect(root.classList.contains("custom-detail-list")).toBe(true);
    expect(screen.getByText("Metadata")).toBeTruthy();
    expect(items?.tagName.toLowerCase()).toBe("dl");
    expect(screen.getByTestId("detail-list-item-a")).toBeTruthy();
    expect(screen.getByTestId("detail-list-item-b")).toBeTruthy();
  });

  it("omits title and renders empty items container when children are absent", () => {
    const view = render(DetailListHarness, {
      title: undefined,
      withChildren: false
    });

    expect(view.container.querySelector(".underlay-detail-list__title")).toBeNull();
    expect(view.container.querySelector(".underlay-detail-list__items")).toBeTruthy();
    expect(view.container.querySelector('[data-testid="detail-list-item-a"]')).toBeNull();
  });
});
