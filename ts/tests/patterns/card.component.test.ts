// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import CardHarness from "../fixtures/CardHarness.svelte";

describe("components/Card.svelte", () => {
	it("renders default section element with base class and children", () => {
		const view = render(CardHarness, {
			as: "section",
			className: "",
		});

		const root = screen.getByTestId("card-root");
		expect(root.tagName.toLowerCase()).toBe("section");
		expect(root.classList.contains("underlay-card")).toBe(true);
		expect(screen.getByTestId("card-title").textContent).toContain("Card Title");
		expect(view.container.querySelector(".underlay-card")).toBeTruthy();
	});

	it("supports alternate element type and class passthrough", () => {
		render(CardHarness, {
			as: "article",
			className: "extra-card",
			title: "Article Card",
		});

		const root = screen.getByTestId("card-root");
		expect(root.tagName.toLowerCase()).toBe("article");
		expect(root.classList.contains("underlay-card")).toBe(true);
		expect(root.classList.contains("extra-card")).toBe(true);
		expect(screen.getByText("Article Card")).toBeTruthy();
	});
});
