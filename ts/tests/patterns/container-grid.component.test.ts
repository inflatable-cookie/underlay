// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import ContainerGridHarness from "../fixtures/ContainerGridHarness.svelte";

describe("components/ContainerGrid.svelte", () => {
	it("renders wrapper/grid with child content and style vars", () => {
		const view = render(ContainerGridHarness, {
			breakpoint: 640,
			gap: "2rem",
			stretch: false,
			className: "extra-grid",
		});

		expect(screen.getByTestId("grid-item-a")).toBeTruthy();
		expect(screen.getByTestId("grid-item-b")).toBeTruthy();

		const wrapper = view.container.querySelector(".underlay-container-grid-wrapper") as HTMLElement;
		const grid = view.container.querySelector(".underlay-container-grid");
		expect(wrapper).toBeTruthy();
		expect(grid).toBeTruthy();
		expect(wrapper.classList.contains("extra-grid")).toBe(true);
		expect(wrapper.getAttribute("style")).toContain("--grid-gap: 2rem;");
		expect(wrapper.getAttribute("style")).toContain("--grid-breakpoint: 640px;");
		expect(wrapper.classList.contains("underlay-container-grid-wrapper--stretch")).toBe(false);
	});

	it("applies stretch modifier class when enabled", () => {
		const view = render(ContainerGridHarness, {
			stretch: true,
		});
		const wrapper = view.container.querySelector(".underlay-container-grid-wrapper");
		expect(wrapper?.classList.contains("underlay-container-grid-wrapper--stretch")).toBe(true);
	});
});
