// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import FieldSetGridHarness from "../fixtures/FieldSetGridHarness.svelte";

describe("components/FieldSetGrid.svelte", () => {
	it("renders grid children and fixed column classes", () => {
		const view = render(FieldSetGridHarness, {
			columns: 3,
		});

		expect(screen.getByTestId("grid-child-a")).toBeTruthy();
		expect(screen.getByTestId("grid-child-b")).toBeTruthy();
		const root = view.container.querySelector(".underlay-fieldset-grid") as HTMLElement;
		expect(root.classList.contains("underlay-fieldset-grid--cols-3")).toBe(true);
		expect(root.classList.contains("underlay-fieldset--cols-3")).toBe(true);
	});

	it("supports auto columns plus full/class modifiers", () => {
		const view = render(FieldSetGridHarness, {
			columns: "auto",
			full: true,
			className: "extra-grid",
		});

		const root = view.container.querySelector(".underlay-fieldset-grid") as HTMLElement;
		expect(root.classList.contains("underlay-fieldset-grid--full")).toBe(true);
		expect(root.classList.contains("underlay-fieldset--full")).toBe(true);
		expect(root.classList.contains("extra-grid")).toBe(true);
		expect(root.className.includes("--cols-")).toBe(false);
	});
});
