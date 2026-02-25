// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import OpsCardGridHarness from "../fixtures/OpsCardGridHarness.svelte";

describe("patterns/OpsCardGrid.svelte", () => {
	it("renders default grid styles and class", () => {
		const view = render(OpsCardGridHarness, {
			class: "grid-extra",
			withChildren: false,
		});

		const grid = view.container.querySelector(".underlay-ops-card-grid");
		expect(grid).toBeTruthy();
		expect(grid?.classList.contains("grid-extra")).toBe(true);
		expect(grid?.getAttribute("style")).toContain("--underlay-ops-card-grid-min: 18rem");
		expect(view.container.querySelector('[data-testid="ops-grid-child-a"]')).toBeNull();
	});

	it("renders children and custom min column width", () => {
		const view = render(OpsCardGridHarness, {
			minColumnWidth: "22rem",
			withChildren: true,
		});

		expect(view.container.querySelector(".underlay-ops-card-grid")?.getAttribute("style")).toContain(
			"--underlay-ops-card-grid-min: 22rem"
		);
		expect(screen.getByTestId("ops-grid-child-a").textContent).toContain("A");
		expect(screen.getByTestId("ops-grid-child-b").textContent).toContain("B");
	});
});
