// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import DetailsCardHarness from "../fixtures/DetailsCardHarness.svelte";

describe("components/DetailsCard.svelte", () => {
  it("renders base class and child content", () => {
    const view = render(DetailsCardHarness, {});

    const card = view.container.querySelector(".underlay-details-card");
    expect(card).toBeTruthy();
    expect(screen.getByTestId("details-card-content").textContent).toContain("Card detail");
  });

  it("applies passthrough class", () => {
    const view = render(DetailsCardHarness, {
      className: "custom-details-card",
      text: "Custom text"
    });

    const card = view.container.querySelector(".underlay-details-card") as HTMLElement;
    expect(card.classList.contains("custom-details-card")).toBe(true);
    expect(screen.getByText("Custom text")).toBeTruthy();
  });
});
