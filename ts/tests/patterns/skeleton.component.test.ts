// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import SkeletonHarness from "../fixtures/SkeletonHarness.svelte";

describe("components/Skeleton.svelte", () => {
  it("renders multiline text skeleton with animation and trailing shorter line", () => {
    const view = render(SkeletonHarness, {
      variant: "text",
      lines: 3,
      animate: true,
      className: "custom-skeleton"
    });

    const lines = view.container.querySelectorAll(".underlay-skeleton--text");
    expect(lines.length).toBe(3);
    expect((lines[0] as HTMLElement).classList.contains("underlay-skeleton--animate")).toBe(true);
    expect((lines[2] as HTMLElement).getAttribute("style")).toContain("width: 75%;");
    expect(view.container.querySelector(".custom-skeleton")).toBeTruthy();
  });

  it("renders card variant with children and custom dimensions", () => {
    const view = render(SkeletonHarness, {
      variant: "card",
      width: "90%",
      radius: "1rem",
      withChildren: true,
      animate: false
    });

    const card = view.container.querySelector(".underlay-skeleton--card") as HTMLElement;
    expect(card).toBeTruthy();
    expect(card.getAttribute("style")).toContain("width: 90%;");
    expect(card.getAttribute("style")).toContain("border-radius: 1rem;");
    expect(card.classList.contains("underlay-skeleton--animate")).toBe(false);
    expect(screen.getByTestId("skeleton-card-child-a")).toBeTruthy();
    expect(screen.getByTestId("skeleton-card-child-b")).toBeTruthy();
  });
});
