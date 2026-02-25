// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import DetailsSectionHarness from "../fixtures/DetailsSectionHarness.svelte";

describe("components/DetailsSection.svelte", () => {
  it("renders items container and children without legend by default", () => {
    const view = render(DetailsSectionHarness, {});

    const section = view.container.querySelector(".underlay-details-section");
    const items = view.container.querySelector(".underlay-details-section__items");
    const legend = view.container.querySelector(".underlay-details-section__legend");

    expect(section).toBeTruthy();
    expect(items).toBeTruthy();
    expect(legend).toBeNull();
    expect(screen.getByTestId("details-section-item-a")).toBeTruthy();
    expect(screen.getByTestId("details-section-item-b")).toBeTruthy();
  });

  it("renders legend text and passthrough class when provided", () => {
    const view = render(DetailsSectionHarness, {
      legend: "Meta",
      className: "custom-details-section"
    });

    const section = view.container.querySelector(".underlay-details-section") as HTMLElement;
    expect(section.classList.contains("custom-details-section")).toBe(true);
    expect(screen.getByText("Meta")).toBeTruthy();
  });
});
