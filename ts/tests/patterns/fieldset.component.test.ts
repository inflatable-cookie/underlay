// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import FieldSetHarness from "../fixtures/FieldSetHarness.svelte";

describe("components/FieldSet.svelte", () => {
	it("renders fieldset with children and optional legend", () => {
		const view = render(FieldSetHarness, {
			legend: "Basic Info",
		});

		expect(view.container.querySelector("fieldset")).toBeTruthy();
		expect(screen.getByText("Basic Info")).toBeTruthy();
		expect(screen.getByTestId("fieldset-child")).toBeTruthy();
		expect(view.container.querySelector(".underlay-fieldset__fields")).toBeTruthy();
	});

	it("applies full/class modifiers when configured", () => {
		const view = render(FieldSetHarness, {
			full: true,
			className: "extra-fieldset",
		});

		const fieldset = view.container.querySelector(".underlay-fieldset") as HTMLElement;
		expect(fieldset.classList.contains("underlay-fieldset--full")).toBe(true);
		expect(fieldset.classList.contains("extra-fieldset")).toBe(true);
	});
});
