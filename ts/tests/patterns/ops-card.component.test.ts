// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import OpsCardHarness from "../fixtures/OpsCardHarness.svelte";

describe("patterns/OpsCard.svelte", () => {
	it("renders title and class even without body snippet", () => {
		const view = render(OpsCardHarness, {
			title: "Health",
			class: "card-extra",
			withBody: false,
		});

		const card = view.container.querySelector(".underlay-ops-card");
		expect(card).toBeTruthy();
		expect(card?.classList.contains("card-extra")).toBe(true);
		expect(view.container.querySelector(".underlay-ops-card__title")?.textContent).toContain("Health");
		expect(view.container.querySelector('[data-testid="ops-card-body"]')).toBeNull();
	});

	it("renders snippet content in body", () => {
		const view = render(OpsCardHarness, {
			title: "Metrics",
			withBody: true,
		});

		expect(screen.getByTestId("ops-card-body").textContent).toContain("Body content");
		expect(view.container.querySelector(".underlay-ops-card__body")?.textContent).toContain("Body content");
	});
});
